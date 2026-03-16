use gpui::Global;
use json_comments::StripComments;
use serde_json;
use std::fs;
use std::path::PathBuf;

/// Global settings accessible throughout the app via GPUI's global system
#[derive(Clone)]
pub struct SettingsGlobal {
    pub settings: serde_json::Value,
}

impl SettingsGlobal {
    pub fn new(settings: serde_json::Value) -> Self {
        Self { settings }
    }

    pub fn get(&self, path: Vec<&str>) -> Result<String, String> {
        let mut current_value = self.settings.clone();
        for key in path {
            if let Some(value) = current_value.get(&key) {
                current_value = value.clone();
            } else {
                return Err(format!("Setting '{}' not found", key));
            }
        }
        Ok(current_value.to_string().trim_matches('"').to_string())
    }

    pub fn get_f32(&self, path: Vec<&str>) -> Result<f32, String> {
        let mut current_value = self.settings.clone();
        for key in path {
            if let Some(value) = current_value.get(&key) {
                current_value = value.clone();
            } else {
                return Err(format!("Setting '{}' not found", key));
            }
        }
        current_value
            .as_f64()
            .ok_or_else(|| format!("Setting is not a number"))
            .map(|v| v as f32)
    }

    pub fn get_color(&self, path: Vec<&str>) -> Result<u32, String> {
        let color_str = self.get(path)?;
        // Parse hex color: "0x1e1e1e" or "0x1e1e1eff"
        u32::from_str_radix(color_str.trim_start_matches("0x"), 16)
            .map_err(|_| format!("Invalid color format: {}", color_str))
    }
}

impl Global for SettingsGlobal {}

pub fn load_settings() -> Result<SettingsGlobal, String> {
    // Resolve home directory manually (~/ is not expanded by Rust)
    let home =
        std::env::var("HOME").map_err(|_| "HOME environment variable not set".to_string())?;

    let settings_path = PathBuf::from(home).join(".my-editor").join("settings.json");

    let settings_content = fs::read_to_string(&settings_path)
        .map_err(|e| format!("Failed to read settings file {:?}: {}", settings_path, e))?;

    let settings_content_stripped = StripComments::new(settings_content.as_bytes());
    let settings: serde_json::Value = serde_json::from_reader(settings_content_stripped)
        .map_err(|e| format!("Failed to parse settings file: {}", e))?;

    Ok(SettingsGlobal::new(settings))
}
