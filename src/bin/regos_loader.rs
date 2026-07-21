use std::{env, ffi::c_void, fs, mem, path::PathBuf, thread, time::Duration};
use std::os::windows::ffi::OsStrExt;
use sysinfo::System;
use windows::{
    core::w,
    Win32::{
        Foundation::{CloseHandle, GetLastError, HANDLE},
        System::{
            Diagnostics::Debug::WriteProcessMemory,
            LibraryLoader::{GetModuleHandleW, GetProcAddress},
            Memory::{VirtualAllocEx, MEM_COMMIT, MEM_RESERVE, PAGE_READWRITE},
            Threading::{
                CreateRemoteThread, OpenProcess, PROCESS_CREATE_THREAD, PROCESS_QUERY_INFORMATION,
                PROCESS_VM_OPERATION, PROCESS_VM_READ, PROCESS_VM_WRITE,
            },
        },
    },
};

const TARGET_PROCESS: &str = "StarRail.exe";

fn main() {
    println!("==================================================");
    println!("           UNIVERSAL AUTO-INJECTOR LOADER         ");
    println!("==================================================");
    
	//check new version
	auto_update_dll();
	
    let dll_paths = get_all_dlls_in_folder();
    
    if dll_paths.is_empty() {
        println!("[-] Error: No .dll files found in the loader's folder.");
        println!("Press Enter to exit...");
        let _ = std::io::stdin().read_line(&mut String::new());
        return;
    }

    println!("[+] Found {} DLL(s) to inject:", dll_paths.len());
    for dll in &dll_paths {
        println!("    - {}", dll.file_name().unwrap().to_string_lossy());
    }
    
    println!("\nWaiting for the anime train game to start...");

    let mut sys = System::new_all();
    let mut injected_pid: u32 = 0;

    loop {
        sys.refresh_processes();

        if let Some(process) = sys.processes_by_exact_name(TARGET_PROCESS).next() {
            let pid = process.pid().as_u32();

            if pid != injected_pid {
                println!("\n[+] Found target game! PID: {}", pid);

                println!("[+] Injecting DLLs...");
                
                let mut success_count = 0;
                for dll_path in &dll_paths {
                    let dll_name = dll_path.file_name().unwrap().to_string_lossy();
                    print!("    Injecting {}... ", dll_name);
                    
                    if inject_dll(pid, dll_path) {
                        println!("SUCCESS!");
                        success_count += 1;
                    } else {
                        println!("FAILED!");
                    }
                }

                println!("[+] Injection phase completed! ({}/{}) successful.", success_count, dll_paths.len());
                injected_pid = pid;
            }
        } else {
            if injected_pid != 0 {
                println!("\n[-] Game closed. Waiting for the game to start again...");
                injected_pid = 0;
            }
        }

        thread::sleep(Duration::from_secs(2));
    }
}

fn get_all_dlls_in_folder() -> Vec<PathBuf> {
    let mut dlls = Vec::new();
    
    if let Ok(exe_path) = env::current_exe() {
        if let Some(folder_path) = exe_path.parent() {
            if let Ok(entries) = fs::read_dir(folder_path) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() {
                        if let Some(extension) = path.extension() {
                            if extension.to_string_lossy().to_lowercase() == "dll" {
                                dlls.push(path);
                            }
                        }
                    }
                }
            }
        }
    }
    
    dlls
}

fn inject_dll(pid: u32, dll_path: &PathBuf) -> bool {
    unsafe {
        let process_handle: HANDLE = match OpenProcess(
            PROCESS_CREATE_THREAD
                | PROCESS_QUERY_INFORMATION
                | PROCESS_VM_OPERATION
                | PROCESS_VM_READ
                | PROCESS_VM_WRITE,
            false,
            pid,
        ) {
            Ok(h) => h,
            Err(e) => {
                println!("(OpenProcess failed: {:?})", e);
                return false;
            }
        };

        let dll_path_wide: Vec<u16> = dll_path
            .as_os_str()
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();
        let path_size = (dll_path_wide.len() * mem::size_of::<u16>()) as usize;

        let alloc_addr = VirtualAllocEx(
            process_handle,
            None,
            path_size,
            MEM_COMMIT | MEM_RESERVE,
            PAGE_READWRITE,
        );

        if alloc_addr.is_null() {
            println!("(VirtualAllocEx failed: {:?})", GetLastError());
            let _ = CloseHandle(process_handle);
            return false;
        }

        let mut bytes_written = 0;
        let write_res = WriteProcessMemory(
            process_handle,
            alloc_addr,
            dll_path_wide.as_ptr() as *const c_void,
            path_size,
            Some(&mut bytes_written),
        );

        if write_res.is_err() {
            println!("(WriteProcessMemory failed: {:?})", GetLastError());
            let _ = CloseHandle(process_handle);
            return false;
        }

        let kernel32 = GetModuleHandleW(w!("kernel32.dll")).unwrap();
        let load_library_addr = GetProcAddress(kernel32, windows::core::s!("LoadLibraryW")).unwrap();

        let thread_handle = CreateRemoteThread(
            process_handle,
            None,
            0,
            Some(mem::transmute(load_library_addr)),
            Some(alloc_addr),
            0,
            None,
        );

        if thread_handle.is_err() {
            println!("(CreateRemoteThread failed: {:?})", GetLastError());
            let _ = CloseHandle(process_handle);
            return false;
        }

        let _ = CloseHandle(process_handle);
        true
    }
}

fn auto_update_dll() {
    let current_version = env!("CARGO_PKG_VERSION");
    let url = "https://api.github.com/repos/leovn85/regos/releases/latest";

    print!("[*] Checking for updates (Current: v{})... ", current_version);
    use std::io::Write;
    let _ = std::io::stdout().flush();

    if let Ok(response) = ureq::get(url).set("User-Agent", "Regos-Loader").call() {
        if let Ok(json) = response.into_json::<serde_json::Value>() {
            let latest_tag = json["tag_name"].as_str().unwrap_or("").trim_start_matches('v');

            if !latest_tag.is_empty() && latest_tag != current_version {
                println!("\n[!] New version found: v{}! Downloading...", latest_tag);

                if let Some(assets) = json["assets"].as_array() {
                    for asset in assets {
                        let name = asset["name"].as_str().unwrap_or("");
                        
                        if name.ends_with(".dll") {
                            if let Some(download_url) = asset["browser_download_url"].as_str() {
                                if let Ok(dll_res) = ureq::get(download_url).call() {
                                    
                                    if let Ok(exe_path) = std::env::current_exe() {
                                        if let Some(dir) = exe_path.parent() {
                                            let dll_path = dir.join("regos.dll");
                                            
                                            if let Ok(mut dest) = std::fs::File::create(&dll_path) {
                                                if std::io::copy(&mut dll_res.into_reader(), &mut dest).is_ok() {
                                                    println!("[+] Successfully updated regos.dll to v{}!", latest_tag);
                                                    return;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                println!("[-] Failed to download update.");
                return;
            }
        }
    }
    println!("You are up to date.");
}