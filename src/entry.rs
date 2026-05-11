use ctor::ctor;
use std::ffi::c_void;
use std::io::Cursor;
use std::thread;
use std::time::Duration;
use windows::Win32::System::Diagnostics::Debug::{MessageBeep, ReadProcessMemory};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::System::ProcessStatus::{GetModuleInformation, MODULEINFO};
use windows::Win32::System::Threading::GetCurrentProcess;
use windows::Win32::UI::Input::KeyboardAndMouse::{GetAsyncKeyState, VK_F10};
use windows::Win32::UI::WindowsAndMessaging::MESSAGEBOX_STYLE;
use windows::core::{w, PCWSTR};
use anyhow::{Context, Result, anyhow};
use il2cpp_runtime::api::ApiIndexTable;

#[ctor(unsafe)]
#[cfg(not(test))]
fn entry() {
    thread::spawn(|| init());
}

fn get_module_handle(name: PCWSTR) -> Result<usize> {
    unsafe {
        GetModuleHandleW(name)
            .map(|v| v.0 as usize)
            .context("Failed to get module handle")
    }
}

fn init() {
    let _ = simplelog::WriteLogger::init(
        simplelog::LevelFilter::Info,
        simplelog::Config::default(),
        std::fs::File::create("relic_dumper.log").unwrap(),
    );

    log::info!("Relic Dumper Initialized! Waiting for GameAssembly...");

    unsafe {
        while GetModuleHandleW(w!("GameAssembly")).is_err() || GetModuleHandleW(w!("UnityPlayer")).is_err() {
            thread::sleep(Duration::from_secs(3));
        }

        let table = ApiIndexTable {
            il2cpp_assembly_get_image: 22, il2cpp_class_get_fields: 31,
            il2cpp_class_get_methods: 35, il2cpp_class_get_name: 37,
            il2cpp_class_get_parent: 40, il2cpp_class_from_type: 49,
            il2cpp_class_get_type: 51, il2cpp_domain_get: 63,
            il2cpp_domain_get_assemblies: 65, il2cpp_field_get_name: 73,
            il2cpp_field_get_offset: 75, il2cpp_field_get_type: 76,
            il2cpp_field_get_value_object: 77, il2cpp_method_get_return_type: 116,
            il2cpp_method_get_name: 117, il2cpp_method_get_param_count: 123,
            il2cpp_method_get_param: 124, il2cpp_object_new: 130,
            il2cpp_thread_attach: 154, il2cpp_type_get_name: 161,
            il2cpp_image_get_class_count: 169, il2cpp_image_get_class: 170,
        };

        if let Ok(offset) = get_il2cpp_table_offset() {
            if let Err(e) = il2cpp_runtime::init(offset, table) {
                log::error!("Il2cpp init failed: {}", e);
                return;
            }
            log::info!("Il2cpp initialized successfully!");
        }
    }

    thread::spawn(|| {
        hotkey_listener_loop();
    });
}

fn hotkey_listener_loop() {
    log::info!("Press F10 to dump Relics...");
    loop {
        unsafe {
            if (GetAsyncKeyState(VK_F10.0 as i32) as u16 & 0x8000) != 0 {
                log::info!("F10 Pressed! Starting Dump...");
                
                let domain = il2cpp_runtime::api::il2cpp_domain_get();
                il2cpp_runtime::api::il2cpp_thread_attach(domain);

                if let Err(e) = crate::helpers::dump_all_equipment_on_demand() {
                    log::error!("Extract items failed: {:#?}", e);
                } else {
                    if let Err(e) = crate::relic_utils::dump_and_convert_data() {
                        log::error!("Convert & Save failed: {:#?}", e);
                    } else {
                        log::info!("Dump successful!");
                        let _ = MessageBeep(MESSAGEBOX_STYLE(0xFFFFFFFF));
                    }
                }
                thread::sleep(Duration::from_secs(3));
            }
        }
        thread::sleep(Duration::from_millis(100));
    }
}

fn get_il2cpp_table_offset() -> Result<usize> {
    unsafe {
        let unityplayer_offset = get_module_handle(w!("UnityPlayer"))
            .map_err(|e| anyhow!(e.to_string()))
            .context("Failed to resolve UnityPlayer module")?;
        let module = windows::Win32::Foundation::HMODULE(unityplayer_offset as *mut c_void);

        let process_handle = GetCurrentProcess();
        let mut lp_mod_info = MODULEINFO::default();

        GetModuleInformation(
            process_handle,
            module,
            &mut lp_mod_info,
            size_of::<MODULEINFO>() as u32,
        )
        .context("Failed to read module information")?;

        let buffer = vec![0u8; lp_mod_info.SizeOfImage as usize];
        let mut bytes_read = 0usize;

        ReadProcessMemory(
            process_handle,
            module.0,
            buffer.as_ptr() as _,
            lp_mod_info.SizeOfImage as usize,
            Some(&mut bytes_read),
        )
        .context("Failed to read module memory")?;

        static PATTERN: &str = "48 8B 05 ? ? ? ? 48 8D 0D ? ? ? ? FF D0";
        let locs = patternscan::scan(Cursor::new(buffer), &PATTERN)
            .context("Failed to scan for il2cpp pattern")?;
        let addr = locs
            .get(0)
            .context("Pattern not found in UnityPlayer module")?
            + module.0 as usize;

        let qword_addr = addr + 7 + std::ptr::read_unaligned((addr + 3) as *const i32) as usize;
        Ok(qword_addr)
    }
}