/* use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::path::Path;

const BASE_RESOURCE_URL: &str = "https://gitlab.com/Dimbreath/turnbasedgamedata/-/raw/main";

// Configuration tables (Dictionaries)
const TARGET_CONFIG_FILES: &[&str] = &[
    "EquipmentConfig.json",           // Light Cone names
    "RelicSetConfig.json",            // Relic set names
    "AvatarPropertyConfig.json",      // Stat names (CRIT Rate, ATK...)
    "RelicBaseType.json",              // Slot names (Head, Hands...)
    "AvatarConfig.json",              // Character names
];

fn main() {
    // 1. Setup metadata for Windows DLL
    let ver = env!("CARGO_PKG_VERSION").split(".").map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    let sem_ver = ver[0] << 48 | ver[1] << 32 | ver[2] << 16;

    winres::WindowsResource::new()
        .set_version_info(winres::VersionInfo::PRODUCTVERSION, sem_ver)
        .compile()
        .unwrap();
    
    println!("cargo:rerun-if-changed=Cargo.toml");

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("TextMapMinimizedEN.json");

    let mut text_hashes: HashSet<u64> = HashSet::new();

    // 2. Download Dictionaries and collect Hash IDs
    for file_name in TARGET_CONFIG_FILES {
        let url = format!("{}/ExcelOutput/{}", BASE_RESOURCE_URL, file_name);
        println!("cargo:warning=Downloading dictionary reference: {}", file_name);
        
        let response = ureq::get(&url)
            .call()
            .unwrap_or_else(|e| panic!("Failed to download {}: {}", url, e));
            
        // Parse JSON directly from network stream to optimize memory
        let json_val: serde_json::Value = serde_json::from_reader(response.into_reader()).unwrap();
        extract_hashes(&json_val, &mut text_hashes);
    }

    // 3. DOWNLOAD TEXTMAPEN.JSON (approx. 50MB) DIRECTLY AND PROCESS IN RAM
    let textmap_url = format!("{}/TextMap/TextMapEN.json", BASE_RESOURCE_URL);
    println!("cargo:warning=Downloading TextMapEN.json over network... Please wait.");
    
    let response = ureq::get(&textmap_url)
        .call()
        .unwrap_or_else(|e| panic!("Failed to download {}: {}", textmap_url, e));
    
    println!("cargo:warning=Parsing TextMapEN directly from network stream...");
    
    // Parse directly from network stream for maximum RAM efficiency
    let full_text_map: std::collections::HashMap<String, String> = 
        serde_json::from_reader(response.into_reader()).expect("Failed to parse TextMapEN.json");

    // 4. Filter: Only keep entries where the Hash exists in text_hashes
    println!("cargo:warning=Filtering TextMap...");
    let mut mini_text_map: std::collections::HashMap<u64, String> = std::collections::HashMap::new();
    
    for (hash_str, text_val) in full_text_map {
        if let Ok(hash_u64) = hash_str.parse::<u64>() {
            if text_hashes.contains(&hash_u64) {
                mini_text_map.insert(hash_u64, text_val);
            }
        }
    }

    // 5. Write the Minimized file to OUT_DIR
    let mut f = File::create(&dest_path).unwrap();
    serde_json::to_writer(&mut f, &mini_text_map).unwrap();
    
    println!("cargo:warning=Successfully created Minimized TextMap! Kept {} essential entries.", mini_text_map.len());
}

/// Recursive function: Automatically searches for "Hash" keys in the JSON config files
fn extract_hashes(val: &serde_json::Value, hashes: &mut HashSet<u64>) {
    match val {
        serde_json::Value::Object(map) => {
            if let Some(serde_json::Value::Number(num)) = map.get("Hash") {
                if let Some(hash) = num.as_u64() {
                    hashes.insert(hash);
                }
            }
            for (_, v) in map {
                extract_hashes(v, hashes);
            }
        }
        serde_json::Value::Array(arr) => {
            for v in arr {
                extract_hashes(v, hashes);
            }
        }
        _ => {}
    }
} */
fn main() {
    let ver = env!("CARGO_PKG_VERSION")
        .split(".")
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    let sem_ver = ver[0] << 48 | ver[1] << 32 | ver[2] << 16;

    winres::WindowsResource::new()
        .set_version_info(winres::VersionInfo::PRODUCTVERSION, sem_ver)
        .compile()
        .unwrap();
    
    println!("cargo:rerun-if-changed=Cargo.toml");
}