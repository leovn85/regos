use std::collections::HashMap;
use std::ptr::null;
use anyhow::Result;
use il2cpp_runtime::System_RuntimeType;
use il2cpp_runtime::types::System_Type;

use crate::types::{RPG_Client_TextID, RPG_Client_TextmapStatic};

// Sanitizes entity names by removing specific format tags (e.g. <ub>, </ub>)
pub fn sanitize_entity_name<S: AsRef<str>>(name: S) -> String {
    let name = name.as_ref();
    if !name.contains("<ub>") && !name.contains("</ub>") {
        return name.to_string();
    }
    name.replace("<ub>", "").replace("</ub>", "")
}

// Direct memory lookup for game translation texts via active textmap static classes
pub fn get_textmap_content(hash: &RPG_Client_TextID) -> Result<String> {
    Ok(unsafe { RPG_Client_TextmapStatic::get_text(hash, null()) }.map(|s| s.to_string())?)
}

// Looks up and returns the C# System.Type handle for an internal IL2CPP class name
pub fn get_type_handle<S: AsRef<str>>(type_name: S) -> Result<System_Type> {
    let type_name = type_name.as_ref();
    let runtime_type = System_RuntimeType::from_name(type_name)?;
    let ty = runtime_type.get_il2cpp_type();
    Ok(unsafe { System_Type::get_type_from_handle(ty)? })
}

// Extracts all structural rows out of a native C# Dictionary in RAM
pub unsafe fn extract_rows_from_dict_ram(dict_ptr: *mut std::ffi::c_void) -> anyhow::Result<Vec<*mut std::ffi::c_void>> {
    unsafe {
        if dict_ptr.is_null() {
            return Ok(Vec::new());
        }

        let class_ptr = *(dict_ptr as *const *const std::ffi::c_void);
        let dict_class = il2cpp_runtime::Il2CppClass(class_ptr);

        let mut count_offset = 0;
        let mut entries_offset = 0;

        let field_iter: *const std::ffi::c_void = std::ptr::null();
        loop {
            let field = il2cpp_runtime::api::il2cpp_class_get_fields(dict_class, &field_iter);
            if field.0.is_null() { break; }
            
            let name = field.name();
            if name == "_count" || name == "count" {
                count_offset = il2cpp_runtime::api::il2cpp_field_get_offset(field) as usize;
            } else if name == "_entries" || name == "entries" {
                entries_offset = il2cpp_runtime::api::il2cpp_field_get_offset(field) as usize;
            }
        }

        if count_offset == 0 || entries_offset == 0 {
            return Err(anyhow::anyhow!("Count or entries metadata offsets could not be resolved."));
        }

        let count = *(dict_ptr.add(count_offset) as *const i32);
        if count <= 0 { return Ok(Vec::new()); }

        let entries_array_ptr = *(dict_ptr.add(entries_offset) as *const *const u8);
        if entries_array_ptr.is_null() { return Ok(Vec::new()); }

        let mut rows = Vec::new();
        let array_data_start = entries_array_ptr.add(0x20);

        for i in 0..count {
            let entry_ptr = array_data_start.add((i as usize) * 0x18);
            let hash_code = *(entry_ptr.add(0x00) as *const i32);
            
            if hash_code >= 0 {
                let row_ptr = *(entry_ptr.add(0x10) as *const *mut std::ffi::c_void);
                if !row_ptr.is_null() {
                    rows.push(row_ptr);
                }
            }
        }

        Ok(rows)
    }
}

// Parses low-level primitive dictionaries (e.g. <uint, uint> layouts) directly from RAM structures
pub unsafe fn extract_primitive_dict_ram(dict_ptr: *mut std::ffi::c_void) -> anyhow::Result<HashMap<u32, u32>> {
    unsafe {
        if dict_ptr.is_null() {
            return Ok(HashMap::new());
        }

        let class_ptr = *(dict_ptr as *const *const std::ffi::c_void);
        let dict_class = il2cpp_runtime::Il2CppClass(class_ptr);

        let mut count_offset = 0;
        let mut entries_offset = 0;

        let field_iter: *const std::ffi::c_void = std::ptr::null();
        loop {
            let field = il2cpp_runtime::api::il2cpp_class_get_fields(dict_class, &field_iter);
            if field.0.is_null() { break; }
            
            let name = field.name();
            if name == "_count" || name == "count" {
                count_offset = il2cpp_runtime::api::il2cpp_field_get_offset(field) as usize;
            } else if name == "_entries" || name == "entries" {
                entries_offset = il2cpp_runtime::api::il2cpp_field_get_offset(field) as usize;
            }
        }

        if count_offset == 0 || entries_offset == 0 {
            return Err(anyhow::anyhow!("Cannot find _count or _entries in primitive dictionary."));
        }

        let count = *(dict_ptr.add(count_offset) as *const i32);
        if count <= 0 { return Ok(HashMap::new()); }

        let entries_array_ptr = *(dict_ptr.add(entries_offset) as *const *const u8);
        if entries_array_ptr.is_null() { 
            return Ok(HashMap::new()); 
        }

        let mut map = HashMap::new();
        let array_data_start = entries_array_ptr.add(0x20);

        for i in 0..count {
            let entry_ptr = array_data_start.add((i as usize) * 0x10);
            let hash_code = *(entry_ptr.add(0x00) as *const i32);
            
            if hash_code >= 0 {
                let key = *(entry_ptr.add(0x08) as *const u32);
                let value = *(entry_ptr.add(0x0C) as *const u32);
                map.insert(key, value);
            }
        }
        Ok(map)
    }
}