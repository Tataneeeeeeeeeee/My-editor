use std::fs;
use std::path::PathBuf;
use serde_json;

pub fn get_settings(setting_name: Vec<&str>) -> Result<String, String>
{
    // Resolve home directory manually (~/ is not expanded by Rust)
    let home = std::env::var("HOME")
        .map_err(|_| "HOME environment variable not set".to_string())?;

    let settings_path = PathBuf::from(home).join(".my-editor").join("settings.json");

    let settings_content = fs::read_to_string(&settings_path)
        .map_err(|e| format!("Failed to read settings file {:?}: {}", settings_path, e))?;

    let settings: serde_json::Value = serde_json::from_str(&settings_content)
        .map_err(|e| format!("Failed to parse settings file: {}", e))?;

    // Extract the requested settings
    let mut current_value = settings;
    for key in setting_name {
        if let Some(value) = current_value.get(&key) {
            current_value = value.clone();
        } else {
            return Err(format!("Setting '{}' not found", key));
        }
    }

    Ok(current_value.to_string().trim_matches('"').to_string())
}
