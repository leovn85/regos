use crate::helpers::{
    extract_rows_from_dict_ram, get_textmap_content, sanitize_entity_name
};
use crate::translation::is_language_en;
use crate::types::{
    RPG_GameCore_TextmapExcelTable, RPG_GameCore_EquipmentExcelTable, RPG_Client_TextID,
    RPG_GameCore_RelicSetConfigExcelTable, RPG_GameCore_AvatarPropertyExcelTable,
    RPG_GameCore_RelicBaseTypeExcelTable, RPG_GameCore_AvatarExcelTable
};

// Evaluates class layout metadata at runtime to recover structural offsets.
unsafe fn get_row_u32_field(row_ptr: *mut std::ffi::c_void, field_names: &[&str]) -> anyhow::Result<u32> {
    unsafe {
        let class_ptr = *(row_ptr as *const *const std::ffi::c_void);
        let class = il2cpp_runtime::Il2CppClass(class_ptr);
        let field_iter: *const std::ffi::c_void = std::ptr::null();
        loop {
            let field = il2cpp_runtime::api::il2cpp_class_get_fields(class, &field_iter);
            if field.0.is_null() { break; }
            let f_name = field.name();
            if field_names.contains(&f_name.as_str()) {
                let offset = il2cpp_runtime::api::il2cpp_field_get_offset(field) as usize;
                let val = *((row_ptr as *mut u8).add(offset) as *const u32);
                return Ok(val);
            }
        }
        log::warn!("get_row_u32_field: Could not find fields {:?} in class '{}'", field_names, class.name());
        anyhow::bail!("Could not resolve u32 field in {:?}", field_names)
    }
}

// Evaluates class layout metadata at runtime to recover structural offsets.
unsafe fn get_row_i32_field(row_ptr: *mut std::ffi::c_void, field_names: &[&str]) -> anyhow::Result<i32> {
    unsafe {
        let class_ptr = *(row_ptr as *const *const std::ffi::c_void);
        let class = il2cpp_runtime::Il2CppClass(class_ptr);
        let field_iter: *const std::ffi::c_void = std::ptr::null();
        loop {
            let field = il2cpp_runtime::api::il2cpp_class_get_fields(class, &field_iter);
            if field.0.is_null() { break; }
            let f_name = field.name();
            if field_names.contains(&f_name.as_str()) {
                let offset = il2cpp_runtime::api::il2cpp_field_get_offset(field) as usize;
                let val = *((row_ptr as *mut u8).add(offset) as *const i32);
                return Ok(val);
            }
        }
        log::warn!("get_row_i32_field: Could not find fields {:?} in class '{}'", field_names, class.name());
        anyhow::bail!("Could not resolve i32 field in {:?}", field_names)
    }
}

// Evaluates class layout metadata at runtime to recover structural offsets.
unsafe fn get_row_text_id_field(row_ptr: *mut std::ffi::c_void, field_names: &[&str]) -> anyhow::Result<RPG_Client_TextID> {
    unsafe {
        let class_ptr = *(row_ptr as *const *const std::ffi::c_void);
        let class = il2cpp_runtime::Il2CppClass(class_ptr);
        let field_iter: *const std::ffi::c_void = std::ptr::null();
        loop {
            let field = il2cpp_runtime::api::il2cpp_class_get_fields(class, &field_iter);
            if field.0.is_null() { break; }
            let f_name = field.name();
            if field_names.contains(&f_name.as_str()) {
                let offset = il2cpp_runtime::api::il2cpp_field_get_offset(field) as usize;
                let text_id = *((row_ptr as *mut u8).add(offset) as *const RPG_Client_TextID);
                return Ok(text_id);
            }
        }
        log::warn!("get_row_text_id_field: Could not find fields {:?} in class '{}'", field_names, class.name());
        anyhow::bail!("Could not resolve RPG_Client_TextID field in {:?}", field_names)
    }
}

struct TextmapRowOffsets {
    hash_offset: usize,
    text_offset: usize,
}

// Dynamically extracts the structure layouts of active game TextmapRows in RAM
unsafe fn resolve_textmap_row_offsets() -> anyhow::Result<TextmapRowOffsets> {

	let row_class = il2cpp_runtime::get_cached_class("RPG.GameCore.TextmapRow")?;
	let mut hash_offset = 0;
	let mut text_offset = 0;
	
	let field_iter: *const std::ffi::c_void = std::ptr::null();
	loop {
		let field = il2cpp_runtime::api::il2cpp_class_get_fields(row_class, &field_iter);
		if field.0.is_null() { break; }
		
		let f_name = field.name();
		let f_type = il2cpp_runtime::api::il2cpp_field_get_type(field);
		let f_type_name = f_type.name();
		
		let is_u64 = f_type_name == "System.UInt64" || f_type_name == "ulong" || f_name.to_lowercase().contains("hash");
		let is_string = f_type_name == "System.String" || f_type_name == "string" || f_name.to_lowercase().contains("text") || f_name.to_lowercase().contains("content");
		
		if is_u64 {
			hash_offset = il2cpp_runtime::api::il2cpp_field_get_offset(field) as usize;
		} else if is_string {
			text_offset = il2cpp_runtime::api::il2cpp_field_get_offset(field) as usize;
		}
	}
	
	if hash_offset == 0 || text_offset == 0 {
		log::warn!("Could not dynamically resolve TextmapRow fields perfectly. Using guessed layout offsets.");
		if hash_offset == 0 { hash_offset = 0x10; }
		if text_offset == 0 { text_offset = 0x18; }
	}
	
	Ok(TextmapRowOffsets { hash_offset, text_offset })

}

// Maps internal active enum flags back to human-readable property definitions
fn get_property_type_name(val: i32) -> String {
    match val {
        1 => "MaxHP".to_string(),
        2 => "Attack".to_string(),
        3 => "Defence".to_string(),
        4 => "Speed".to_string(),
        5 => "CriticalChance".to_string(),
        6 => "CriticalDamage".to_string(),
        7 => "HealRatio".to_string(),
        8 => "StanceBreakAddedRatio".to_string(),
        9 => "SPRatio".to_string(),
        10 => "StatusProbability".to_string(),
        11 => "StatusResistance".to_string(),
        12 => "PhysicalAddedRatio".to_string(),
        13 => "PhysicalResistance".to_string(),
        14 => "FireAddedRatio".to_string(),
        15 => "FireResistance".to_string(),
        16 => "IceAddedRatio".to_string(),
        17 => "IceResistance".to_string(),
        18 => "ThunderAddedRatio".to_string(),
        19 => "ThunderResistance".to_string(),
        20 => "WindAddedRatio".to_string(),
        21 => "WindResistance".to_string(),
        22 => "QuantumAddedRatio".to_string(),
        23 => "QuantumResistance".to_string(),
        24 => "ImaginaryAddedRatio".to_string(),
        25 => "ImaginaryResistance".to_string(),
        26 => "BaseHP".to_string(),
        27 => "HPDelta".to_string(),
        28 => "BaseAttack".to_string(),
        29 => "AttackDelta".to_string(),
        30 => "BaseDefence".to_string(),
        31 => "DefenceDelta".to_string(),
        32 => "HPAddedRatio".to_string(),
        33 => "AttackAddedRatio".to_string(),
        34 => "DefenceAddedRatio".to_string(),
        35 => "BaseSpeed".to_string(),
        36 => "HealTakenRatio".to_string(),
        _ => format!("Property_{}", val),
    }
}

// Scans and extracts complete static database dictionaries from the game client
pub unsafe fn dump_game_excel_files() -> anyhow::Result<()> {
    unsafe {
        if !is_language_en() {
            log::warn!("F11 Aborted: Active game language is not 'EN'. Please switch your game language to English before extracting.");
            return Ok(());
        }
        
        log::info!("=== STARTING EXCEL TABLES DUMP ===");

        log::info!("Step 1/6: Processing TextMapEN.json...");
        if !RPG_GameCore_TextmapExcelTable::IsDataLoaded()? {
            log::info!("TextmapExcelTable is not loaded. Loading now...");
            RPG_GameCore_TextmapExcelTable::LoadData()?;
        }
        let textmap_dict = RPG_GameCore_TextmapExcelTable::get_dataDict()?;
        let textmap_rows = extract_rows_from_dict_ram(textmap_dict).unwrap_or_default();
        log::info!("Found {} raw rows in TextmapExcelTable.", textmap_rows.len());

        let textmap_offsets = resolve_textmap_row_offsets()?;
        let mut textmap_json = serde_json::Map::new();
        let mut textmap_fails = 0;

        for row_ptr in textmap_rows {
            let hash = *((row_ptr as *mut u8).add(textmap_offsets.hash_offset) as *const u64);
            let text_ptr = *((row_ptr as *mut u8).add(textmap_offsets.text_offset) as *const *const std::ffi::c_void);
            if text_ptr.is_null() {
                textmap_fails += 1;
                continue;
            }
            let il2cpp_str = il2cpp_runtime::types::Il2CppString(text_ptr);
            textmap_json.insert(hash.to_string(), serde_json::Value::String(il2cpp_str.to_string()));
        }
        if textmap_fails > 0 {
            log::warn!("Skipped {} TextMap rows due to null string pointers.", textmap_fails);
        }
        let textmap_len = textmap_json.len();
        std::fs::write("TextMapEN.json", serde_json::to_string_pretty(&serde_json::Value::Object(textmap_json))?)?;
        log::info!("Successfully dumped TextMapEN.json with {} entries.", textmap_len);

        log::info!("Step 2/6: Processing EquipmentConfig.json...");
        if !RPG_GameCore_EquipmentExcelTable::IsDataLoaded()? {
            log::info!("EquipmentExcelTable is not loaded. Loading now...");
            RPG_GameCore_EquipmentExcelTable::LoadData()?;
        }
        let eq_dict = RPG_GameCore_EquipmentExcelTable::get_dataDict()?;
        let eq_rows = extract_rows_from_dict_ram(eq_dict).unwrap_or_default();
        let mut eq_json = serde_json::Map::new();
        let eq_fails = 0;

        for row_ptr in eq_rows {
            if let Ok(id) = get_row_u32_field(row_ptr, &["EquipmentID", "ID"]) {
                if let Ok(name_id) = get_row_text_id_field(row_ptr, &["EquipmentName", "Name"]) {
                    let name_text = match get_textmap_content(&name_id) {
                        Ok(text) => sanitize_entity_name(text),
                        Err(_) => "Unknown".to_string(),
                    };
                    let row_data = serde_json::json!({
                        "EquipmentID": id,
                        "EquipmentName": {
                            "Hash": name_id.hash64,
                            "Text": name_text
                        }
                    });
                    eq_json.insert(id.to_string(), row_data);
                }
            }
        }
        if eq_fails > 0 {
            log::warn!("Skipped {} equipment rows due to field resolution issues.", eq_fails);
        }
        let eq_len = eq_json.len();
        std::fs::write("EquipmentConfig.json", serde_json::to_string_pretty(&serde_json::Value::Object(eq_json))?)?;
        log::info!("Successfully dumped EquipmentConfig.json with {} entries.", eq_len);

        log::info!("Step 3/6: Processing RelicSetConfig.json...");
        if !RPG_GameCore_RelicSetConfigExcelTable::IsDataLoaded()? {
            log::info!("RelicSetConfigExcelTable is not loaded. Loading now...");
            RPG_GameCore_RelicSetConfigExcelTable::LoadData()?;
        }
        let relic_set_dict = RPG_GameCore_RelicSetConfigExcelTable::get_dataDict()?;
        let relic_set_rows = extract_rows_from_dict_ram(relic_set_dict).unwrap_or_default();
        let mut relic_set_json = serde_json::Map::new();
        let set_fails = 0;

        for row_ptr in relic_set_rows {
            if let Ok(id) = get_row_u32_field(row_ptr, &["ID", "SetID", "RelicSetID"]) {
                if let Ok(name_id) = get_row_text_id_field(row_ptr, &["SetName", "Name"]) {
                    let name_text = match get_textmap_content(&name_id) {
                        Ok(text) => sanitize_entity_name(text),
                        Err(_) => "Unknown".to_string(),
                    };
                    let row_data = serde_json::json!({
                        "ID": id,
                        "SetName": {
                            "Hash": name_id.hash64,
                            "Text": name_text
                        }
                    });
                    relic_set_json.insert(id.to_string(), row_data);
                }
            }
        }
        if set_fails > 0 {
            log::warn!("Skipped {} relic set rows due to field resolution issues.", set_fails);
        }
        let relic_set_len = relic_set_json.len();
        std::fs::write("RelicSetConfig.json", serde_json::to_string_pretty(&serde_json::Value::Object(relic_set_json))?)?;
        log::info!("Successfully dumped RelicSetConfig.json with {} entries.", relic_set_len);

        log::info!("Step 4/6: Processing AvatarPropertyConfig.json...");
        if !RPG_GameCore_AvatarPropertyExcelTable::IsDataLoaded()? {
            log::info!("AvatarPropertyExcelTable is not loaded. Loading now...");
            RPG_GameCore_AvatarPropertyExcelTable::LoadData()?;
        }
        let prop_dict = RPG_GameCore_AvatarPropertyExcelTable::get_dataDict()?;
        let prop_rows = extract_rows_from_dict_ram(prop_dict).unwrap_or_default();
        let mut prop_json = serde_json::Map::new();
        let prop_fails = 0;

        for row_ptr in prop_rows {
            if let Ok(prop_type_val) = get_row_i32_field(row_ptr, &["PropertyType", "Type"]) {
                if let Ok(name_id) = get_row_text_id_field(row_ptr, &["PropertyName", "Name"]) {
                    let prop_name = get_property_type_name(prop_type_val);
                    let name_text = match get_textmap_content(&name_id) {
                        Ok(text) => sanitize_entity_name(text),
                        Err(_) => "Unknown".to_string(),
                    };
                    let row_data = serde_json::json!({
                        "PropertyType": prop_name,
                        "PropertyName": {
                            "Hash": name_id.hash64,
                            "Text": name_text
                        }
                    });
                    prop_json.insert(prop_name, row_data);
                }
            }
        }
        if prop_fails > 0 {
            log::warn!("Skipped {} property rows due to field resolution issues.", prop_fails);
        }
        let prop_len = prop_json.len();
        std::fs::write("AvatarPropertyConfig.json", serde_json::to_string_pretty(&serde_json::Value::Object(prop_json))?)?;
        log::info!("Successfully dumped AvatarPropertyConfig.json with {} entries.", prop_len);

        log::info!("Step 5/6: Processing RelicBaseType.json...");
        if !RPG_GameCore_RelicBaseTypeExcelTable::IsDataLoaded()? {
            log::info!("RelicBaseTypeExcelTable is not loaded. Loading now...");
            RPG_GameCore_RelicBaseTypeExcelTable::LoadData()?;
        }
        let base_type_dict = RPG_GameCore_RelicBaseTypeExcelTable::get_dataDict()?;
        let base_type_rows = extract_rows_from_dict_ram(base_type_dict).unwrap_or_default();
        let mut base_type_json = serde_json::Map::new();
        let base_type_fails = 0;

        for row_ptr in base_type_rows {
            if let Ok(base_type_val) = get_row_i32_field(row_ptr, &["Type", "BaseType"]) {
                if let Ok(name_id) = get_row_text_id_field(row_ptr, &["BaseTypeText", "Name"]) {
                    let base_type_str = match base_type_val {
                        0 => { continue; },
                        1 => "HEAD",
                        2 => "HAND",
                        3 => "BODY",
                        4 => "FOOT",
                        5 => "NECK",
                        6 => "OBJECT",
                        _ => { continue; }
                    };
                    let name_text = match get_textmap_content(&name_id) {
                        Ok(text) => sanitize_entity_name(text),
                        Err(_) => "Unknown".to_string(),
                    };
                    let row_data = serde_json::json!({
                        "BaseType": base_type_str,
                        "BaseTypeText": {
                            "Hash": name_id.hash64,
                            "Text": name_text
                        }
                    });
                    base_type_json.insert(base_type_str.to_string(), row_data);
                }
            }
        }
        if base_type_fails > 0 {
            log::warn!("Skipped {} base type rows due to field resolution issues.", base_type_fails);
        }
        let base_type_len = base_type_json.len();
        std::fs::write("RelicBaseType.json", serde_json::to_string_pretty(&serde_json::Value::Object(base_type_json))?)?;
        log::info!("Successfully dumped RelicBaseType.json with {} entries.", base_type_len);

        log::info!("Step 6/6: Processing AvatarConfig.json...");
        if !RPG_GameCore_AvatarExcelTable::IsDataLoaded()? {
            log::info!("AvatarExcelTable is not loaded. Loading now...");
            RPG_GameCore_AvatarExcelTable::LoadData()?;
        }
        let avatar_dict = RPG_GameCore_AvatarExcelTable::get_dataDict()?;
        let avatar_rows = extract_rows_from_dict_ram(avatar_dict).unwrap_or_default();
        let mut avatar_json = serde_json::Map::new();
        let avatar_fails = 0;

        for row_ptr in avatar_rows {
            if let Ok(id) = get_row_u32_field(row_ptr, &["AvatarID", "ID"]) {
                if let Ok(name_id) = get_row_text_id_field(row_ptr, &["AvatarName", "Name"]) {
                    let name_text = match get_textmap_content(&name_id) {
                        Ok(text) => sanitize_entity_name(text),
                        Err(_) => "Unknown".to_string(),
                    };
                    let row_data = serde_json::json!({
                        "AvatarID": id,
                        "AvatarName": {
                            "Hash": name_id.hash64,
                            "Text": name_text
                        }
                    });
                    avatar_json.insert(id.to_string(), row_data);
                }
            }
        }
        if avatar_fails > 0 {
            log::warn!("Skipped {} avatar rows due to field resolution issues.", avatar_fails);
        }
        let avatar_len = avatar_json.len();
        std::fs::write("AvatarConfig.json", serde_json::to_string_pretty(&serde_json::Value::Object(avatar_json))?)?;
        log::info!("Successfully dumped AvatarConfig.json with {} entries.", avatar_len);

        log::info!("Final step: Generating TextMapMinimizedEN.json...");
        if let Err(e) = generate_local_minimized_textmap() {
            log::warn!("Failed to generate local minimized TextMap: {:?}", e);
        }

        log::info!("=== ALL EXCEL FILES DUMPED SUCCESSFULLY ===");
        Ok(())
    }
}

// Scans and extracts an optimized lightweight translation map to minimize memory overhead during deployment
pub unsafe fn generate_local_minimized_textmap() -> anyhow::Result<()> {
    unsafe {
        log::info!("=== STARTING LOCAL MINIMIZED TEXTMAP GENERATION ===");
        let mut hashes = std::collections::HashSet::new();
        
        if !RPG_GameCore_EquipmentExcelTable::IsDataLoaded()? { 
            RPG_GameCore_EquipmentExcelTable::LoadData()?; 
        }
        let eq_rows = extract_rows_from_dict_ram(RPG_GameCore_EquipmentExcelTable::get_dataDict()?).unwrap_or_default();
        for r in eq_rows {
            if let Ok(name_id) = get_row_text_id_field(r, &["EquipmentName", "Name"]) {
                hashes.insert(name_id.hash64);
            }
        }

        if !RPG_GameCore_RelicSetConfigExcelTable::IsDataLoaded()? { 
            RPG_GameCore_RelicSetConfigExcelTable::LoadData()?; 
        }
        let relic_rows = extract_rows_from_dict_ram(RPG_GameCore_RelicSetConfigExcelTable::get_dataDict()?).unwrap_or_default();
        for r in relic_rows {
            if let Ok(name_id) = get_row_text_id_field(r, &["SetName", "Name"]) {
                hashes.insert(name_id.hash64);
            }
        }

        if !RPG_GameCore_AvatarPropertyExcelTable::IsDataLoaded()? { 
            RPG_GameCore_AvatarPropertyExcelTable::LoadData()?; 
        }
        let prop_rows = extract_rows_from_dict_ram(RPG_GameCore_AvatarPropertyExcelTable::get_dataDict()?).unwrap_or_default();
        for r in prop_rows {
            if let Ok(name_id) = get_row_text_id_field(r, &["PropertyName", "Name"]) {
                hashes.insert(name_id.hash64);
            }
        }

        if !RPG_GameCore_RelicBaseTypeExcelTable::IsDataLoaded()? { 
            RPG_GameCore_RelicBaseTypeExcelTable::LoadData()?; 
        }
        let base_rows = extract_rows_from_dict_ram(RPG_GameCore_RelicBaseTypeExcelTable::get_dataDict()?).unwrap_or_default();
        for r in base_rows {
            if let Ok(name_id) = get_row_text_id_field(r, &["BaseTypeText", "Name"]) {
                hashes.insert(name_id.hash64);
            }
        }

        if !RPG_GameCore_AvatarExcelTable::IsDataLoaded()? { 
            RPG_GameCore_AvatarExcelTable::LoadData()?; 
        }
        let avatar_rows = extract_rows_from_dict_ram(RPG_GameCore_AvatarExcelTable::get_dataDict()?).unwrap_or_default();
        for r in avatar_rows {
            if let Ok(name_id) = get_row_text_id_field(r, &["AvatarName", "Name"]) {
                hashes.insert(name_id.hash64);
            }
        }

        log::info!("Collected {} unique hashes. Translating via active client language...", hashes.len());

        let mut mini_map = serde_json::Map::new();
        for hash_val in hashes {
            let text_id = RPG_Client_TextID { hash: 0, hash64: hash_val };
            if let Ok(text) = get_textmap_content(&text_id) {
                let clean_text = sanitize_entity_name(text);
                mini_map.insert(hash_val.to_string(), serde_json::Value::String(clean_text));
            }
        }

        std::fs::write("TextMapMinimizedEN.json", serde_json::to_string_pretty(&serde_json::Value::Object(mini_map))?)?;
        log::info!("Local Minimized TextMap generated successfully as 'TextMapMinimizedEN.json'!");
        Ok(())
    }
}