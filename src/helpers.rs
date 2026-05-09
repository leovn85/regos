
use std::{collections::HashMap, ptr::null, collections::BTreeMap};

use crate::{
    types::{
        RPG_Client_GlobalVars, RPG_GameCore_AvatarExcelTable, 
        RPG_GameCore_AvatarSkillTreeExcelTable, RPG_GameCore_AvatarBaseType, RPG_GameCore_ItemMainType, RPG_Client_RelicItemData, RPG_Client_EquipmentItemData, RPG_AvatarSystem_IAvatar, RPG_Client_AvatarHelper, RPG_Client_AvatarExtensions
    },
    misc::{FribbelsCharacter, FribbelsSkills, FribbelsTraces, FribbelsMemosprite},
	relics::{process_relic_data_english, process_lc_data_english},
};
use anyhow::Result;
use il2cpp_runtime::{
    Il2CppObject, System_RuntimeType,
    types::System_Type,
};

use super::types::{
    RPG_Client_TextID, RPG_Client_TextmapStatic,
};

pub fn sanitize_entity_name<S: AsRef<str>>(name: S) -> String {
    let name = name.as_ref();
    if !name.contains("<ub>") && !name.contains("</ub>") {
        return name.to_string();
    }

    name.replace("<ub>", "").replace("</ub>", "")
}

pub fn get_textmap_content(hash: &RPG_Client_TextID) -> Result<String> {
    Ok(unsafe { RPG_Client_TextmapStatic::get_text(hash, null()) }.map(|s| s.to_string())?)
}

pub fn get_type_handle<S: AsRef<str>>(type_name: S) -> Result<System_Type> {
    let type_name = type_name.as_ref();
    let runtime_type = System_RuntimeType::from_name(type_name)?;
    let ty = runtime_type.get_il2cpp_type();
    Ok(unsafe { System_Type::get_type_from_handle(ty)? })
}

pub unsafe fn extract_rows_from_dict_ram(dict_ptr: *mut std::ffi::c_void) -> anyhow::Result<Vec<*mut std::ffi::c_void>> {
    if dict_ptr.is_null() {
        return Ok(Vec::new());
    }

    let class_ptr = unsafe { *(dict_ptr as *const *const std::ffi::c_void) };
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
        return Err(anyhow::anyhow!("Không tìm thấy cấu trúc _count hoặc _entries"));
    }

    let count = unsafe { *(dict_ptr.add(count_offset) as *const i32) };
    if count <= 0 { return Ok(Vec::new()); }

    let entries_array_ptr = unsafe { *(dict_ptr.add(entries_offset) as *const *const u8) };
    if entries_array_ptr.is_null() { return Ok(Vec::new()); }

    let mut rows = Vec::new();
    let array_data_start = unsafe { entries_array_ptr.add(0x20) };

    for i in 0..count {
        let entry_ptr = unsafe { array_data_start.add((i as usize) * 0x18) };
        let hash_code = unsafe { *(entry_ptr.add(0x00) as *const i32) };
        
        // Valid data
        if hash_code >= 0 {
            // Read Tvalue at offset 0x10 (Pointer to Row)
            let row_ptr = unsafe { *(entry_ptr.add(0x10) as *const *mut std::ffi::c_void) };
            if !row_ptr.is_null() {
                rows.push(row_ptr);
            }
        }
    }

    Ok(rows)
}

pub unsafe fn dump_fribbels_characters() -> anyhow::Result<(Vec<FribbelsCharacter>, u32, String)> {
    let domain = il2cpp_runtime::api::il2cpp_domain_get();
    il2cpp_runtime::api::il2cpp_thread_attach(domain);
    let mut characters_map: BTreeMap<u32, FribbelsCharacter> = BTreeMap::new();
    let mut player_uid: u32 = 0;
    let mut account_name = "Unknown".to_string();
    let mut trailblazer_gender = "Stelle".to_string();

    unsafe {
        let safe_dump = microseh::try_seh(|| {
            let module_manager = RPG_Client_GlobalVars::s_ModuleManager()?;
            
            // ==========================================
            // 1. GET UID & NAME FROM PLAYER MODULE
            // ==========================================
            let _ = microseh::try_seh(|| {
                let player_module = module_manager.PlayerModule()?;
                if !player_module.0.is_null() {
                    let player_data = player_module.get_PlayerData()?;
                    if !player_data.0.is_null() {
                        if let Ok(uid) = player_data.get_UserID() {
                            player_uid = uid;
                        }
                        if let Ok(name_str) = player_data.get_NickName() {
                            let clean_name = sanitize_entity_name(name_str.to_string());
                            if !clean_name.is_empty() {
                                account_name = clean_name;
                            }
                        }
                    }
                }
                Ok::<(), anyhow::Error>(())
            });

            // ==========================================
            // 2. GET ALL CHARACTERS FROM AVATAR HELPER (4.3++)
            // ==========================================
            let mut all_avatars: Vec<RPG_AvatarSystem_IAvatar> = Vec::new();
            
            if let Ok(avatar_list) = RPG_Client_AvatarHelper::GetAllObtainedSpecificPathAvatars() {
                if !avatar_list.as_ptr().is_null() {
                    all_avatars = avatar_list.to_vec::<RPG_AvatarSystem_IAvatar>();
                }
            }

            if all_avatars.is_empty() {
                log::debug!("[Character Dump] No avatars found. Aborting character dump.");
                return Ok(()); 
            }

            // ==========================================
            // 3. ITERATE AND PROCESS EVERY CHARACTER (4.3++)
            // ==========================================
            for avatar_obj in all_avatars {
                if avatar_obj.0.is_null() { continue; }

                // Get ID via AvatarExtensions
                let base_id = match RPG_Client_AvatarExtensions::GetAvatarID(avatar_obj) {
                    Ok(id) => id,
                    Err(_) => continue,
                };

                // Detect Stelle or Caelus
                if base_id >= 8000 && base_id < 9000 {
                    let gender = if base_id % 2 == 0 { "Stelle" } else { "Caelus" };
                    trailblazer_gender = gender.to_string();
                }

                let avatar_row = RPG_GameCore_AvatarExcelTable::GetData(base_id)?;
                if avatar_row.0.is_null() { continue; }

                let path_enum_val = *avatar_row.AvatarBaseType()?.try_deref()?;
                
                // Read stats via AvatarExtensions
                let level = RPG_Client_AvatarExtensions::GetLevel(avatar_obj).unwrap_or(1);
                let promotion = RPG_Client_AvatarExtensions::GetPromotionLevel(avatar_obj).unwrap_or(0);
                let rank = RPG_Client_AvatarExtensions::GetEidolonLevel(avatar_obj).unwrap_or(0);
                let enhanced_id = RPG_Client_AvatarExtensions::GetEnhancedID(avatar_obj).unwrap_or(0);
                let ability_version = if enhanced_id > 0 && enhanced_id != base_id && base_id < 8000 { 1 } else { 0 };

                let path_str = match path_enum_val {
                    RPG_GameCore_AvatarBaseType::Warrior => "Destruction",
                    RPG_GameCore_AvatarBaseType::Rogue => "Hunt",
                    RPG_GameCore_AvatarBaseType::Mage => "Erudition",
                    RPG_GameCore_AvatarBaseType::Shaman => "Harmony",
                    RPG_GameCore_AvatarBaseType::Warlock => "Nihility",
                    RPG_GameCore_AvatarBaseType::Knight => "Preservation",
                    RPG_GameCore_AvatarBaseType::Priest => "Abundance",
                    RPG_GameCore_AvatarBaseType::Memory => "Remembrance",
                    RPG_GameCore_AvatarBaseType::Elation => "Elation",
                    _ => "Unknown",
                };
                
                // Get character name
                let mut name = format!("Avatar_{}", base_id);
                if let Ok(name_str) = RPG_Client_AvatarExtensions::GetName(avatar_obj) {
                    let clean = sanitize_entity_name(name_str.to_string());
                    if !clean.is_empty() { name = clean; }
                } else if base_id >= 8000 {
                    name = format!("{} MC", path_str); 
                }
                
                // --- SKILL TREE (GET DIRECTLY FROM EXTENSIONS) ---
                let mut skills = FribbelsSkills { basic: 1, skill: 1, ult: 1, talent: 1, elation: None };
                let mut traces = FribbelsTraces {
                    ability_1: false, ability_2: false, ability_3: false,
                    stat_1: false, stat_2: false, stat_3: false, stat_4: false, stat_5: false,
                    stat_6: false, stat_7: false, stat_8: false, stat_9: false, stat_10: false, special: false,
                };
                let mut memosprite = None;

                // GetTraceTreeLevels function returns native C# Dictionary, pass this pointer to function parse RAM
                let level_dict_ptr = RPG_Client_AvatarExtensions::GetTraceTreeLevels(avatar_obj).unwrap_or(std::ptr::null_mut());
                
                if !level_dict_ptr.is_null() {
                    let level_dict = extract_primitive_dict_ram(level_dict_ptr).unwrap_or_default();
                    let mut anchor_to_level: HashMap<u32, u32> = HashMap::new();

                    for (&point_id, &level) in &level_dict {
                        if let Ok(row) = RPG_GameCore_AvatarSkillTreeExcelTable::GetData(point_id, 1) {
                            if !row.0.is_null() {
                                if let Ok(anchor_box) = row.AnchorType() {
                                    let anchor_type = *anchor_box as u32;
                                    let current_max = anchor_to_level.get(&anchor_type).copied().unwrap_or(0);
                                    anchor_to_level.insert(anchor_type, std::cmp::max(current_max, level));
                                }
                            }
                        }
                    }

                    let get_lv = |anchor_type: u32| -> u32 { anchor_to_level.get(&anchor_type).copied().unwrap_or(0) };

                    skills.basic = std::cmp::max(1, get_lv(1));
                    skills.skill = std::cmp::max(1, get_lv(2));
                    skills.ult = std::cmp::max(1, get_lv(3));
                    skills.talent = std::cmp::max(1, get_lv(4));

                    let elation_lv = get_lv(22);
                    skills.elation = if elation_lv > 0 { Some(elation_lv) } else { None };

                    memosprite = FribbelsMemosprite { skill: get_lv(19), talent: get_lv(20) }.if_present();

                    traces.ability_1 = get_lv(6) > 0; traces.ability_2 = get_lv(7) > 0; traces.ability_3 = get_lv(8) > 0;
                    traces.stat_1 = get_lv(9) > 0; traces.stat_2 = get_lv(10) > 0; traces.stat_3 = get_lv(11) > 0;
                    traces.stat_4 = get_lv(12) > 0; traces.stat_5 = get_lv(13) > 0; traces.stat_6 = get_lv(14) > 0;
                    traces.stat_7 = get_lv(15) > 0; traces.stat_8 = get_lv(16) > 0; traces.stat_9 = get_lv(17) > 0;
                    traces.stat_10 = get_lv(18) > 0; traces.special = get_lv(21) > 0;
                }

                characters_map.insert(base_id, FribbelsCharacter {
                    id: base_id.to_string(),
                    name,
                    path: path_str.to_string(),
                    level,
                    ascension: promotion,
                    eidolon: rank,
                    skills,
                    traces,
                    memosprite,
                    ability_version,
                });
            }
            
            Ok::<(), anyhow::Error>(())
        });
        
        if let Err(e) = safe_dump {
            log::error!("[Character Dump] CRITICAL SEH EXCEPTION: {:#?}", e);
        }
    }
    
    let trailblazer_meta = format!("{} ({})", account_name, trailblazer_gender);
    Ok((characters_map.into_values().collect::<Vec<_>>(), player_uid, trailblazer_meta))
}


pub unsafe fn extract_primitive_dict_ram(dict_ptr: *mut std::ffi::c_void) -> anyhow::Result<HashMap<u32, u32>> {

    if dict_ptr.is_null() {
        return Ok(HashMap::new());
    }

    let class_ptr = unsafe { *(dict_ptr as *const *const std::ffi::c_void) };
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
        return Err(anyhow::anyhow!("Cannot found _count or _entries"));
    }

    let count = unsafe { *(dict_ptr.add(count_offset) as *const i32) };
    if count <= 0 { return Ok(HashMap::new()); }

    let entries_array_ptr = unsafe { *(dict_ptr.add(entries_offset) as *const *const u8) };
    if entries_array_ptr.is_null() { 
        return Ok(HashMap::new()); 
    }

    let mut map = HashMap::new();
    let array_data_start = unsafe { entries_array_ptr.add(0x20) };

    for i in 0..count {
        // Entry size for <int, uint> or <uint, uint> is 0x10 (16 bytes)
        // Structure: hashCode (4 bytes) | next (4 bytes) | key (4 bytes) | value (4 bytes)
        let entry_ptr = unsafe { array_data_start.add((i as usize) * 0x10) };
        let hash_code = unsafe { *(entry_ptr.add(0x00) as *const i32) };
        
        if hash_code >= 0 {
            let key = unsafe { *(entry_ptr.add(0x08) as *const u32) };
            let value = unsafe { *(entry_ptr.add(0x0C) as *const u32) };
            map.insert(key, value);
        }
    }
    Ok(map)
}

pub unsafe fn dump_all_equipment_on_demand() -> anyhow::Result<()> {
    let module_manager = RPG_Client_GlobalVars::s_ModuleManager()?;
    let inventory_module = module_manager.InventoryModule()?;
    let type_handle = get_type_handle("RPG.GameCore.ItemMainType")?;
    
    // Clear old cache before starting a new dump
    crate::relic_utils::get_relics().write().clear();
    crate::relic_utils::get_light_cones().write().clear();

    // 1. DUMP RELICS
    log::info!("Scanning all Relics...");
    let mut relic_types_arr = unsafe { il2cpp_runtime::types::Il2CppArray::create_instance(type_handle, 1)? };
    *(relic_types_arr.get_mut::<i32>(0)) = RPG_GameCore_ItemMainType::Relic as i32;
    
    let relic_list = unsafe { inventory_module.get_items_by_main_types(relic_types_arr)? };
    if !relic_list.as_ptr().is_null() {
        let relic_data_vec = relic_list.to_vec::<RPG_Client_RelicItemData>();
        log::info!("Found {} Relics.", relic_data_vec.len());
        
        for relic_data in relic_data_vec {
            if relic_data.0.is_null() { continue; }
            // Process relic data
            let _ = process_relic_data_english(relic_data);
        }
    }

    // 2. DUMP LIGHT CONES
    log::info!("Scanning all Light Cones...");
    let mut lc_types_arr = unsafe { il2cpp_runtime::types::Il2CppArray::create_instance(type_handle, 1)? };
    *(lc_types_arr.get_mut::<i32>(0)) = RPG_GameCore_ItemMainType::Equipment as i32; // Equipment = Light Cone
    
    let lc_list = unsafe { inventory_module.get_items_by_main_types(lc_types_arr)? };
    if !lc_list.as_ptr().is_null() {
        let lc_data_vec = lc_list.to_vec::<RPG_Client_EquipmentItemData>();
        log::info!("Found {} Light Cones.", lc_data_vec.len());
        
        for lc_data in lc_data_vec {
            if lc_data.0.is_null() { continue; }
            // Process light cone data
            let _ = process_lc_data_english(lc_data);
        }
    }

    Ok(())
}