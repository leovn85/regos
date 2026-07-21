use std::collections::{HashMap, BTreeMap};
use chrono::Local;
use serde_json::json;
use il2cpp_runtime::Il2CppObject;

use crate::helpers::{
    sanitize_entity_name, get_textmap_content, extract_primitive_dict_ram, get_type_handle, extract_rows_from_dict_ram
};
use crate::relics::{
    get_relics, get_light_cones, process_relic_data_english, process_lc_data_english,
    get_relics_snapshot, get_light_cones_snapshot, build_relic_id_mapping
};
use crate::translation::is_language_en;
use crate::types::{
    RPG_Client_GlobalVars, RPG_AvatarSystem_IAvatar, RPG_Client_AvatarExtensions,
    RPG_Client_AvatarHelper, RPG_GameCore_AvatarExcelTable, RPG_GameCore_AvatarBaseType,
    RPG_GameCore_AvatarSkillTreeExcelTable, RPG_GameCore_ItemMainType, RPG_Client_RelicItemData,
    RPG_Client_EquipmentItemData, RPG_GameCore_RelicSetConfigExcelTable, RPG_GameCore_RelicBaseTypeExcelTable,
    RPG_GameCore_RelicConfigExcelTable, RPG_GameCore_RelicConfigRow, RPG_Client_TextmapStatic
};
use crate::misc::{
    FribbelsCharacter, FribbelsSkills, FribbelsTraces, FribbelsMemosprite,
    ReliquaryRelic, ReliquaryLightCone, FribbelsArchive, FribbelsMetadata,
    RelicConfigDumpEntry
};

// Iterates and scans equipment inventories in RAM on demand, caching relics and light cones
pub unsafe fn dump_all_equipment_on_demand() -> anyhow::Result<()> {
    unsafe {
        if !is_language_en() {
            log::warn!("F10 Aborted: Active game language is not 'EN'. Please switch your game language to English before dumping equipment.");
            return Ok(());
        }
        
        let module_manager = RPG_Client_GlobalVars::s_ModuleManager()?;
        let inventory_module = module_manager.InventoryModule()?;
        let type_handle = get_type_handle("RPG.GameCore.ItemMainType")?;
        
        get_relics().write().clear();
        get_light_cones().write().clear();

        log::info!("Scanning all Relics...");
        let mut relic_types_arr = il2cpp_runtime::types::Il2CppArray::create_instance(type_handle, 1)?;
        *(relic_types_arr.get_mut::<i32>(0)) = RPG_GameCore_ItemMainType::Relic as i32;
        
        let relic_list = inventory_module.get_items_by_main_types(relic_types_arr)?;
        if !relic_list.as_ptr().is_null() {
            let relic_data_vec = relic_list.to_vec::<RPG_Client_RelicItemData>();
            log::info!("Found {} Relics.", relic_data_vec.len());
            
            for relic_data in relic_data_vec {
                if relic_data.0.is_null() { continue; }
                let _ = process_relic_data_english(relic_data);
            }
        }

        log::info!("Scanning all Light Cones...");
        let mut lc_types_arr = il2cpp_runtime::types::Il2CppArray::create_instance(type_handle, 1)?;
        *(lc_types_arr.get_mut::<i32>(0)) = RPG_GameCore_ItemMainType::Equipment as i32;
        
        let lc_list = inventory_module.get_items_by_main_types(lc_types_arr)?;
        if !lc_list.as_ptr().is_null() {
            let lc_data_vec = lc_list.to_vec::<RPG_Client_EquipmentItemData>();
            log::info!("Found {} Light Cones.", lc_data_vec.len());
            
            for lc_data in lc_data_vec {
                if lc_data.0.is_null() { continue; }
                let _ = process_lc_data_english(lc_data);
            }
        }

        Ok(())
    }
}

// Scans active player character avatars and processes their trace configurations
pub unsafe fn dump_fribbels_characters() -> anyhow::Result<(Vec<FribbelsCharacter>, u32, String)> {
    unsafe {
        let domain = il2cpp_runtime::api::il2cpp_domain_get();
        il2cpp_runtime::api::il2cpp_thread_attach(domain);
        let mut characters_map: BTreeMap<u32, FribbelsCharacter> = BTreeMap::new();
        let mut player_uid: u32 = 0;
        let mut account_name = "Unknown".to_string();
        let mut trailblazer_gender = "Stelle".to_string();

        let safe_dump = microseh::try_seh(|| {
            let module_manager = RPG_Client_GlobalVars::s_ModuleManager()?;
            
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

            for avatar_obj in all_avatars {
                if avatar_obj.0.is_null() { continue; }

                let base_id = match RPG_Client_AvatarExtensions::GetAvatarID(avatar_obj) {
                    Ok(id) => id,
                    Err(_) => continue,
                };

                if base_id >= 8000 && base_id < 9000 {
                    let gender = if base_id % 2 == 0 { "Stelle" } else { "Caelus" };
                    trailblazer_gender = gender.to_string();
                }

                let avatar_row = RPG_GameCore_AvatarExcelTable::GetData(base_id)?;
                if avatar_row.0.is_null() { continue; }

                let path_enum_val = *avatar_row.AvatarBaseType()?.try_deref()?;
                
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
                
                let mut name = format!("Avatar_{}", base_id);
                if let Ok(name_str) = RPG_Client_AvatarExtensions::GetName(avatar_obj) {
                    let clean = sanitize_entity_name(name_str.to_string());
                    if !clean.is_empty() { name = clean; }
                } else if base_id >= 8000 {
                    name = format!("{} MC", path_str); 
                }
                
                let mut skills = FribbelsSkills { basic: 1, skill: 1, ult: 1, talent: 1, elation: None };
                let mut traces = FribbelsTraces {
                    ability_1: false, ability_2: false, ability_3: false,
                    stat_1: false, stat_2: false, stat_3: false, stat_4: false, stat_5: false,
                    stat_6: false, stat_7: false, stat_8: false, stat_9: false, stat_10: false, special: false,
                };
                let mut memosprite = None;

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

        let trailblazer_meta = format!("{} ({})", account_name, trailblazer_gender);
        Ok((characters_map.into_values().collect::<Vec<_>>(), player_uid, trailblazer_meta))
    }
}

// Converts scanned memory entities into the standardized JSON schemas expected by Fribbels Relic Optimizer
pub fn dump_and_convert_data() -> anyhow::Result<()> {
    log::info!("=== BEGIN FRIBBELS DUMP ===");

    let relics: Vec<ReliquaryRelic> = get_relics_snapshot()
        .iter().map(|r| ReliquaryRelic::from(r)).collect();
    log::info!("Dumped {} Relics from snapshot.", relics.len());

    let light_cones: Vec<ReliquaryLightCone> = get_light_cones_snapshot()
        .iter().map(|lc| ReliquaryLightCone::from(lc)).collect();
    log::info!("Dumped {} Light Cones from snapshot.", light_cones.len());

    let (characters, player_uid, tb_meta) = unsafe { 
        dump_fribbels_characters().unwrap_or_else(|e| {
            log::error!("Failed to dump characters: {}", e);
            (Vec::new(), 0, "Unknown (Stelle)".to_string())
        }) 
    };

    let archive = FribbelsArchive {
        source: "reliquary_archiver".to_string(),
        build: "0.16.0".to_string(),
        version: 4,
        metadata: FribbelsMetadata { 
            uid: player_uid, 
            trailblazer: tb_meta 
        },
        light_cones,
        relics,
        characters,
    };

    let now_str = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    let archive_filename = format!("archive_output-{}.json", now_str);
    let archive_json = serde_json::to_string_pretty(&archive)?;
    std::fs::write(&archive_filename, &archive_json)?;
    log::info!("Created Fribbels file successfully: {}", archive_filename);

    log::info!("Generating Private Server config...");
    generate_private_server_config(&archive)?;
    
    log::info!("=== END FRIBBELS DUMP ===");
    Ok(())
}

// Generates config.json profile datasets designed specifically for local private simulation environments
fn generate_private_server_config(archive: &FribbelsArchive) -> anyhow::Result<()> {
    let relic_lookup = unsafe { build_relic_id_mapping().unwrap_or_default() };

    let sub_affix_map: HashMap<&str, u32> = HashMap::from([
        ("HP", 1), ("ATK", 2), ("DEF", 3), ("HP_", 4), ("ATK_", 5), ("DEF_", 6),
        ("SPD", 7), ("CRIT Rate_", 8), ("CRIT DMG_", 9), ("Effect Hit Rate_", 10),
        ("Effect RES_", 11), ("Break Effect_", 12)
    ]);

    let get_main_affix = |relic_type: i32, main: &str| -> u32 {
        match relic_type {
            1 | 2 => 1,
            3 => match main { "HP"=>1, "ATK"=>2, "DEF"=>3, "CRIT Rate"=>4, "CRIT DMG"=>5, "Outgoing Healing Boost"=>6, "Effect Hit Rate"=>7, _=>1 },
            4 => match main { "HP"=>1, "ATK"=>2, "DEF"=>3, "SPD"=>4, _=>1 },
            5 => match main { "HP"=>1, "ATK"=>2, "DEF"=>3, "Physical DMG Boost"=>4, "Fire DMG Boost"=>5, "Ice DMG Boost"=>6, "Lightning DMG Boost"=>7, "Wind DMG Boost"=>8, "Quantum DMG Boost"=>9, "Imaginary DMG Boost"=>10, _=>1 },
            6 => match main { "Break Effect"=>1, "Energy Regeneration Rate"=>2, "HP"=>3, "ATK"=>4, "DEF"=>5, _=>1 },
            _ => 1
        }
    };

    let mut lc_map = HashMap::new();
    for lc in &archive.light_cones {
        if !lc.location.is_empty() {
            lc_map.insert(lc.location.clone(), lc.clone());
        }
    }

    let mut relic_map: HashMap<String, Vec<&ReliquaryRelic>> = HashMap::new();
    for r in &archive.relics {
        if !r.location.is_empty() {
            relic_map.entry(r.location.clone()).or_default().push(r);
        }
    }

    let mut avatar_configs = Vec::new();

    for char in &archive.characters {
        let char_id = &char.id;
        let Some(lc) = lc_map.get(char_id) else { continue };

        let mut char_relics = Vec::new();
        if let Some(equipped_relics) = relic_map.get(char_id) {
            for r in equipped_relics {
                let set_id_u32 = r.set_id.parse::<u32>().unwrap_or(0);
                let mapping_info = relic_lookup.get(&(set_id_u32, r.rarity, r.slot.clone()));
                
                if let Some(info) = mapping_info {
                    let main_id = get_main_affix(info.relic_type, &r.mainstat);
                    
                    let mut sub_strs = Vec::new();
                    for sub in &r.substats {
                        if let Some(sub_id) = sub_affix_map.get(sub.key.as_str()) {
                            sub_strs.push(format!("{}:{}:{}", sub_id, sub.count, sub.step));
                        }
                    }
                    
                    let sub_str = sub_strs.join(",");
                    let relic_format = format!("{},{},{},{},{}", info.relic_id, r.level, main_id, r.substats.len(), sub_str);
                    char_relics.push((info.relic_id, relic_format));
                } else {
                    log::warn!("Could not find relic_id for set_id: {}, rarity: {}, slot: {}", r.set_id, r.rarity, r.slot);
                }
            }
        }

        if char_relics.len() != 6 { continue; }
        char_relics.sort_by_key(|k| k.0);

        let final_relic_strings: Vec<String> = char_relics.into_iter().map(|x| x.1).collect();

        avatar_configs.push(json!({
            "name": char.name,
            "id": char.id.parse::<u32>().unwrap_or(1001),
            "hp": 100, "sp": 50,
            "level": char.level,
            "promotion": char.ascension,
            "rank": char.eidolon,
            "lightcone": {
                "id": lc.id.parse::<u32>().unwrap_or(0),
                "rank": lc.superimposition,
                "level": lc.level,
                "promotion": lc.ascension
            },
            "relics": final_relic_strings,
            "use_technique": true
        }));
    }

    let final_data = json!({
        "avatar_config": avatar_configs,
        "battle_config": {
            "battle_id": 1,
            "stage_id": 1052086,
            "cycle_count": 30,
            "monster_wave": [[4035010]],
            "monster_level": 82,
            "blessings": []
        }
    });

    std::fs::write("config.json", serde_json::to_string_pretty(&final_data)?)?;
    log::info!("Created config.json for Private Server");

    Ok(())
}

// Dumps static index sets mapping internal game IDs to relic names
pub unsafe fn dump_relic_sets() -> anyhow::Result<()> {
    unsafe {
        let mut relic_sets = BTreeMap::new();
        let set_ids = (101..=200).chain(301..=400);

        for id in set_ids {
            if let Ok(row) = RPG_GameCore_RelicSetConfigExcelTable::GetData(id) {
                if !row.0.is_null() {
                    if let Ok(name_id_boxed) = row.SetName() {
                        if let Ok(name_id) = name_id_boxed.try_deref() {
                            if name_id.hash != 0 {
                                if let Ok(name) = get_textmap_content(name_id) {
                                    relic_sets.insert(id, sanitize_entity_name(name));
                                }
                            }
                        }
                    }
                }
            }
        }

        std::fs::write("relic_sets.json", serde_json::to_string_pretty(&relic_sets)?)?;
        log::info!("Dumped {} relic sets!", relic_sets.len());
        Ok(())
    }
}

// Scans and compiles global relic structural parameters into relational configuration output
pub unsafe fn dump_relic_config() -> anyhow::Result<()> {
    unsafe {
        let mut relic_configs = BTreeMap::new();
        log::debug!("Getting dataDict...");
        let dict_ptr = RPG_GameCore_RelicConfigExcelTable::get_dataDict()?;
        
        log::debug!("Extracting rows from RAM...");
        let rows = extract_rows_from_dict_ram(dict_ptr).unwrap_or_default();
        log::debug!("Found {} rows to process.", rows.len());
        
        let rows_len = rows.len();
        for (index, row_ptr) in rows.into_iter().enumerate() {
            log::debug!("--- Processing Row {}/{} (Ptr: {:p}) ---", index + 1, rows_len, row_ptr);
            
            let row = RPG_GameCore_RelicConfigRow(row_ptr as _);
            let id = row.ID()?.try_deref()?.0;
            let set_id = row.SetID()?.try_deref()?.0;
            let rarity_val = (*row.Rarity()?.try_deref()?) as i32 + 1;
            let type_enum_val = *row.Type()?.try_deref()?;

            let type_enum_obj = RPG_GameCore_RelicBaseTypeExcelTable::GetData(type_enum_val)?;
            let type_str = RPG_Client_TextmapStatic::get_text(type_enum_obj.BaseTypeText()?.try_deref()?, std::ptr::null())?.to_string();
            let max_level = row.MaxLevel()?.try_deref()?.0;
            let main_affix_id = row.MainAffixGroup()?.try_deref()?.0;
            let sub_affix_id = row.SubAffixGroup()?.try_deref()?.0;

            let mut set_name_str = format!("Set {}", set_id);
            let safe_set = microseh::try_seh(|| RPG_GameCore_RelicSetConfigExcelTable::GetData(set_id) );
            
            if let Ok(Ok(set_row)) = safe_set {
                if !set_row.0.is_null() {
                    let safe_name_id = microseh::try_seh(|| set_row.SetName());
                    if let Ok(Ok(name_id_boxed)) = safe_name_id {
                        if let Ok(name_id) = name_id_boxed.try_deref() {
                            let safe_text = microseh::try_seh(|| get_textmap_content(name_id));
                            if let Ok(Ok(name)) = safe_text {
                                set_name_str = sanitize_entity_name(name);
                            }
                        }
                    }
                }
            }

            let mut slot_name_str = type_str.clone();
            let safe_type = microseh::try_seh(|| RPG_GameCore_RelicBaseTypeExcelTable::GetData(type_enum_val) );
            
            if let Ok(Ok(type_row)) = safe_type {
                if !type_row.0.is_null() {
                    let safe_text_id = microseh::try_seh(|| type_row.BaseTypeText());
                    if let Ok(Ok(text_id_boxed)) = safe_text_id {
                        if let Ok(text_id) = text_id_boxed.try_deref() {
                            let safe_text = microseh::try_seh(|| get_textmap_content(text_id));
                            if let Ok(Ok(name)) = safe_text {
                                slot_name_str = sanitize_entity_name(name);
                            }
                        }
                    }
                }
            }

            let final_name = format!("{} - {}", set_name_str, slot_name_str);
            let icon = "****".to_string();

            relic_configs.insert(
                id.to_string(),
                RelicConfigDumpEntry {
                    id, set_id, rarity: rarity_val, relic_type: type_str, max_level,
                    main_affix_id, sub_affix_id, icon, name: final_name,
                }
            );
        }

        log::debug!("Serialization to JSON...");
        let json = serde_json::to_string_pretty(&relic_configs)?;
        std::fs::write("relic_config_dump.json", json)?;
        log::info!("Dumped {} relic configs dynamically!", relic_configs.len());
        Ok(())
    }
}