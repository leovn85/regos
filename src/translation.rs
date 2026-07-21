use std::collections::HashMap;
use std::sync::LazyLock;
use crate::types::RPG_Client_TextID;
use crate::types::RPG_GameCore_TextmapExcelTable;

// Offline translation lookup cache, populated locally if the minimized file exists
pub static TEXT_MAP_EN: LazyLock<HashMap<u64, String>> = LazyLock::new(|| {
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(dir) = exe_path.parent() {
            let local_path = dir.join("TextMapMinimizedEN.json");
            if let Ok(json_str) = std::fs::read_to_string(local_path) {
                if let Ok(map) = serde_json::from_str(&json_str) {
                    log::info!("Successfully loaded offline TextMapMinimizedEN.json at runtime.");
                    return map;
                }
            }
        }
    }
    log::warn!("TextMapMinimizedEN.json not found locally! Fallback to empty translation dictionary.");
    HashMap::new()
});

// Retrieves the English text for a given hash from the offline dictionary
pub fn get_en_text(text_id: &RPG_Client_TextID) -> String {
    if let Some(en_text) = TEXT_MAP_EN.get(&text_id.hash64) {
        en_text.clone()
    } else {
        "Unknown".to_string() 
    }
}

// Verifies if the active client game language is set to English
pub unsafe fn is_language_en() -> bool {
    unsafe {
        let is_loaded = RPG_GameCore_TextmapExcelTable::IsDataLoaded().unwrap_or(false);
        if !is_loaded {
            let _ = RPG_GameCore_TextmapExcelTable::LoadData();
        }
        
        if let Ok(lang_il2cpp) = RPG_GameCore_TextmapExcelTable::GetLanguage() {
            let lang = lang_il2cpp.to_string();
			log::info!("Current active client language is: '{}'", lang);
            return lang.eq_ignore_ascii_case("EN");
        }
        false
    }
}