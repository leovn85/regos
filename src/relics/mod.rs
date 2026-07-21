use anyhow::{Result, anyhow};
use il2cpp_runtime::{Il2CppObject, get_cached_class, api::{il2cpp_class_get_fields, il2cpp_field_get_type, il2cpp_field_get_offset}};
use il2cpp_runtime::prelude::*;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::ffi::c_void;
use std::sync::OnceLock;

use crate::types::{
    RPG_Client_EquipmentItemData, RPG_Client_RelicItemData, RPG_GameCore_AvatarPropertyExcelTable,
    RPG_GameCore_FixPoint, RPG_GameCore_GamePlayStatic, RPG_GameCore_RelicBaseTypeExcelTable,
    RPG_GameCore_RelicSetConfigExcelTable, RPG_GameCore_RelicSubAffixConfigExcelTable, SystemObjectDummy,
    RPG_GameCore_RelicConfigExcelTable, RPG_GameCore_RelicConfigRow
};
use crate::misc::{LightCone, Relic, RelicMainStat, RelicRolls, RelicSubstat, ReliquaryLightCone, ReliquaryRelic, Substat};
use crate::translation::get_en_text;
use crate::helpers::{extract_rows_from_dict_ram, sanitize_entity_name, get_textmap_content};

// Thread-safe static database cache for Relics
pub fn get_relics() -> &'static RwLock<HashMap<String, Relic>> {
    static RELICS: OnceLock<RwLock<HashMap<String, Relic>>> = OnceLock::new();
    RELICS.get_or_init(|| RwLock::new(HashMap::new()))
}

// Thread-safe static database cache for Light Cones (Equipment)
pub fn get_light_cones() -> &'static RwLock<HashMap<String, LightCone>> {
    static LIGHT_CONES: OnceLock<RwLock<HashMap<String, LightCone>>> = OnceLock::new();
    LIGHT_CONES.get_or_init(|| RwLock::new(HashMap::new()))
}

// Calculates the initial upgrade rolls of a relic based on current level and total rolls
pub fn calc_initial_rolls(level: u32, total_rolls: u32) -> u32 {
    total_rolls - (level / 3)
}

// Mathematical solver to find low, mid, and high rolling step combinations
pub fn solve_low_mid_high(step: u32, count: u32) -> Vec<(u32, u32, u32)> {
    let high_min = step.saturating_sub(count); 
    let high_max = step / 2;

    if high_min > high_max {
        return Vec::new();
    }

    (high_min..=high_max)
        .map(|high| {
            let mid = step - 2 * high; 
            let low = (count + high) - step; 
            (low, mid, high)
        })
        .collect()
}

// Selects the most optimal rolling step layout from matching solver calculations
pub fn pick_low_mid_high(step: u32, count: u32) -> (u32, u32, u32) {
    solve_low_mid_high(step, count)
        .last()
        .copied()
        .unwrap_or((0, 0, 0))
}

// Captures a thread-safe snapshot clone of the static Relics database
pub fn get_relics_snapshot() -> Vec<Relic> {
    let relics_map = get_relics().read();
    relics_map.values().cloned().collect()
}

// Captures a thread-safe snapshot clone of the static Light Cones database
pub fn get_light_cones_snapshot() -> Vec<LightCone> {
    let light_cones_map = get_light_cones().read();
    light_cones_map.values().cloned().collect()
}

#[derive(Clone, Copy, Debug)]
struct RelicAffixOffsets {
    count: usize,
    step: usize,
    property_id: usize,
}

static RELIC_AFFIX_OFFSETS: OnceLock<RelicAffixOffsets> = OnceLock::new();

// Evaluates class layouts at runtime to locate count, step, and property_id field offsets
unsafe fn resolve_relic_affix_offsets() -> Result<RelicAffixOffsets> {
    
	let relic_data_class = get_cached_class("RPG.Client.RelicItemData")?;
	let method = relic_data_class.find_method("_GetAvatarPropertyTypeByRelicAffix", &["*"])?;
	let affix_class = method.arg(0).class();

	let mut uint_offsets = Vec::new();
	let field_iter: *const c_void = std::ptr::null();
	
	loop {
		let field = il2cpp_class_get_fields(affix_class, &field_iter);
		if field.0.is_null() { break; }

		let f_type = il2cpp_field_get_type(field);
		if f_type.name() == "System.UInt32" {
			uint_offsets.push(il2cpp_field_get_offset(field) as usize);
		}
	}

	if uint_offsets.len() >= 3 {
		uint_offsets.sort(); 
		Ok(RelicAffixOffsets {
			step: uint_offsets[0],
			count: uint_offsets[1],
			property_id: uint_offsets[2],
		})
	} else {
		Err(anyhow!("Failed to dynamically resolve RelicAffix fields!"))
	}
}

// Cached proxy accessor for raw RelicAffix structure offsets inside the game engine
unsafe fn get_relic_affix_offsets() -> Result<RelicAffixOffsets> {
    unsafe {
        if let Some(offsets) = RELIC_AFFIX_OFFSETS.get() {
            return Ok(*offsets);
        }
        let offsets = resolve_relic_affix_offsets()?;
        let _ = RELIC_AFFIX_OFFSETS.set(offsets);
        Ok(offsets)
    }
}

// Converts high-precision game fixed points into floating point representations
impl Into<f64> for RPG_GameCore_FixPoint {
    fn into(self) -> f64 {
        const FLOAT_CONVERSION_CONSTANT: f64 = 1.0 / 4294967296.0;
        let raw_value = self.m_rawValue;
        let hi = ((raw_value as u64 & 0xFFFFFFFF00000000) >> 32) as u32;
        let lo = (raw_value as u64 & 0x00000000FFFFFFFF) as u32;
        let raw = hi as f64 + lo as f64 * FLOAT_CONVERSION_CONSTANT;
        raw / 2.0 // Scaling updated for 4.3+ client compatibility
    }
}

// Extracts and processes full Relic metadata from active memory pointers
pub fn process_relic_data_english(this: RPG_Client_RelicItemData) -> Result<ReliquaryRelic> {
    unsafe {
        let relic_row = this.get_RelicRow()?;
        let set_id = relic_row.SetID()?.try_deref()?.0;
        let location = this.get_BelongAvatarID()?;
        let lock = this.get_IsProtected()?;
        let discard = this.get_IsDiscard()?;
        let uid = this.as_base().get_UID()?;
        let rarity = (*relic_row.Rarity()?.try_deref()?) as u32;
        let level = this.get_Level()?;
        let relic_set_config_data = RPG_GameCore_RelicSetConfigExcelTable::GetData(set_id)?;
        let relic_set_name = get_en_text(relic_set_config_data.SetName()?.try_deref()?);
        let main_affix_property = this.get_MainAffixPropertyType()?;
        let main_row_data = RPG_GameCore_AvatarPropertyExcelTable::GetData(main_affix_property)?;
        let main_stat_name = get_en_text(main_row_data.PropertyName()?.try_deref()?);
        let relic_type_row = RPG_GameCore_RelicBaseTypeExcelTable::GetData(*relic_row.Type()?.try_deref()?)?;
        let slot_name = get_en_text(relic_type_row.BaseTypeText()?.try_deref()?);
        
        let parse_affix_array = |array: Il2CppArray| -> Result<Option<Vec<Substat>>> {
            if array.as_ptr().is_null() || array.len() == 0 {
                return Ok(None);
            }
            
            let mut subs = Vec::new();
            let offsets = get_relic_affix_offsets()?;
            
            for i in 0..array.len() {
                let affix_obj: &SystemObjectDummy = array.get(i);
                let ptr = affix_obj.as_ptr() as *const u8;
                
                let count = *(ptr.add(offsets.count) as *const u32);
                let step = *(ptr.add(offsets.step) as *const u32);
                let affix_id = *(ptr.add(offsets.property_id) as *const u32);
                
                if affix_id == 0 {
                    continue;
                }

                let sub_property = this._GetPropertyTypeBySubAffixID(affix_id)?;
                let sub_row_data = RPG_GameCore_AvatarPropertyExcelTable::GetData(sub_property)?;
                
                if sub_row_data.0.is_null() {
                    continue;
                }
                
                let property_name = get_en_text(&*sub_row_data.PropertyName()?.try_deref()?);

                let relic_sub_affix_config = RPG_GameCore_RelicSubAffixConfigExcelTable::GetData(relic_row.SubAffixGroup()?.try_deref()?.0, affix_id)?;
                
                if relic_sub_affix_config.0.is_null() {
                    continue;
                }
                
                let mut value: f64 = RPG_GameCore_GamePlayStatic::CalcRelicSubAffixValue(*relic_sub_affix_config.BaseValue()?.try_deref()?, *relic_sub_affix_config.StepValue()?.try_deref()?, count, step)?.into();
                
                let mut key = property_name;
                if value < 1.0 { key.push('_'); value *= 100.0; } 

                subs.push(Substat {
                    key,
                    value, 
                    count,
                    step,
                });
            }
            Ok(Some(subs))
        };
        
        let reroll_substats = parse_affix_array(this.get_ReforgeSubAffixes()?)?;
        let preview_substats = parse_affix_array(this.get_PreviewSubAffixList()?)?;
        let raw_main_substats = parse_affix_array(this.get_SubAffixList()?)?.unwrap_or_default();
        
        let mut ui_substats = Vec::new();
        let mut total_count: u32 = 0;

        for sub in raw_main_substats {
            total_count = total_count.saturating_add(sub.count);
            let (low, mid, high) = pick_low_mid_high(sub.step, sub.count);
            
            ui_substats.push(RelicSubstat { 
                stat: sub.key, 
                value: sub.value, 
                rolls: RelicRolls { high, mid, low }, 
                added_rolls: if sub.count > 1 { sub.count - 1 } else { 0 },
                raw_count: sub.count, 
                raw_step: sub.step,
            });
        }

        let initial_rolls = if total_count > 0 { calc_initial_rolls(level, total_count) } else { 0 };
        let mut main_value: f64 = (this.GetMainAffixPropertyValue()?).into();
        let main_stat = main_stat_name.to_string();
        if main_value < 1.0 { main_value *= 100.0; }

        let relic = Relic {
            part: slot_name.to_string(), 
            set_id: set_id.to_string(), 
            set: relic_set_name.to_string(), 
            enhance: level, 
            grade: rarity, 
            main: RelicMainStat { stat: main_stat, value: main_value }, 
            substats: ui_substats, 
            reroll_substats,	   
            preview_substats,	   
            equipped_by: if location > 0 { location.to_string() } else { String::new() }, 
            verified: true, 
            id: uid.to_string(), 
            age_index: uid, 
            initial_rolls, 
            lock, 
            discard,
        };
        
        get_relics().write().insert(uid.to_string(), relic.clone());
        Ok(ReliquaryRelic::from(&relic))
    }
}

// Extracts and processes full Light Cone (Equipment) metadata from active memory pointers
pub fn process_lc_data_english(this: RPG_Client_EquipmentItemData) -> anyhow::Result<ReliquaryLightCone> {
    unsafe {
        let uid = this.as_base().get_UID()?;
        let location = this.get_BelongAvatarID()?;
        let lock = this.get_IsProtected()?;
        let rank = this._Rank()?.try_deref()?.0;
        let level = this.get_Level()?;
        let promotion = this.get_Promotion()?;
        let equipment_row = this.get_EquipmentRow()?;
        
        let name = get_en_text(&*equipment_row.EquipmentName()?.try_deref()?);
        let id = equipment_row.EquipmentID()?.try_deref()?.0;
        
        let light_cone = LightCone {
            id: id.to_string(),
            name: name.to_string(),
            level,
            promotion,
            rank,
            equipped_by: if location > 0 { location.to_string() } else { String::new() },
            lock,
            uid: uid.to_string(),
        };

        let live_light_cone = ReliquaryLightCone::from(&light_cone);
        get_light_cones().write().insert(uid.to_string(), light_cone);
        Ok(live_light_cone)
    }
}

pub struct RelicMappingInfo {
    pub relic_id: u32,
    pub relic_type: i32,
}

// Builds the relational ID table mapping set definitions and rarity configs back to specific database rows
pub unsafe fn build_relic_id_mapping() -> anyhow::Result<HashMap<(u32, u32, String), RelicMappingInfo>> {
    unsafe {
        let mut map = HashMap::new();
        let dict_ptr = RPG_GameCore_RelicConfigExcelTable::get_dataDict()?;
        let rows = extract_rows_from_dict_ram(dict_ptr).unwrap_or_default();

        for row_ptr in rows {
            let row = RPG_GameCore_RelicConfigRow(row_ptr as _);
            
            let id = (*row.ID()?).0;
            let set_id = (*row.SetID()?).0;
            let rarity = *row.Rarity()? as u32; 
            let type_enum_val = *row.Type()? as i32;
            
            let safe_type = microseh::try_seh(|| RPG_GameCore_RelicBaseTypeExcelTable::GetData(*row.Type()?));
            let mut slot_name = String::new();
            if let Ok(Ok(type_row)) = safe_type {
                if !type_row.0.is_null() {
                    let safe_text_id = microseh::try_seh(|| type_row.BaseTypeText());
                    if let Ok(Ok(text_id)) = safe_text_id {
                        let safe_text = microseh::try_seh(|| get_textmap_content(&*text_id));
                        if let Ok(Ok(name)) = safe_text {
                            slot_name = sanitize_entity_name(name);
                        }
                    }
                }
            }
            
            if !slot_name.is_empty() {
                map.insert((set_id, rarity, slot_name), RelicMappingInfo {
                    relic_id: id,
                    relic_type: type_enum_val,
                });
            }
        }
        Ok(map)
    }
}