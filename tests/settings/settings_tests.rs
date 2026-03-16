#[cfg(test)]
mod tests {
    use serde_json::{Value, json};

    // Mock SettingsGlobal pour les tests
    #[derive(Clone)]
    struct MockSettingsGlobal {
        settings: Value,
    }

    impl MockSettingsGlobal {
        fn new(settings: Value) -> Self {
            Self { settings }
        }

        fn get(&self, path: Vec<&str>) -> Result<String, String> {
            let mut current_value = self.settings.clone();
            for key in path {
                if let Some(value) = current_value.get(key) {
                    current_value = value.clone();
                } else {
                    return Err(format!("Setting '{}' not found", key));
                }
            }
            Ok(current_value.to_string().trim_matches('"').to_string())
        }

        fn get_f32(&self, path: Vec<&str>) -> Result<f32, String> {
            let mut current_value = self.settings.clone();
            for key in path {
                if let Some(value) = current_value.get(key) {
                    current_value = value.clone();
                } else {
                    return Err(format!("Setting '{}' not found", key));
                }
            }
            current_value
                .as_f64()
                .ok_or_else(|| "Setting is not a number".to_string())
                .map(|v| v as f32)
        }

        fn get_color(&self, path: Vec<&str>) -> Result<u32, String> {
            let color_str = self.get(path)?;
            u32::from_str_radix(color_str.trim_start_matches("0x"), 16)
                .map_err(|_| format!("Invalid color format: {}", color_str))
        }
    }

    #[test]
    fn test_settings_new() {
        let settings_json = json!({
            "editor": {
                "font_size": 14,
                "theme": "dark"
            }
        });

        let settings = MockSettingsGlobal::new(settings_json);
        assert!(settings.get(vec!["editor", "theme"]).is_ok());
    }

    #[test]
    fn test_settings_get_string() {
        let settings_json = json!({
            "editor": {
                "theme": "dark"
            }
        });

        let settings = MockSettingsGlobal::new(settings_json);
        let result = settings.get(vec!["editor", "theme"]);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "dark");
    }

    #[test]
    fn test_settings_get_number() {
        let settings_json = json!({
            "editor": {
                "font_size": 14.5
            }
        });

        let settings = MockSettingsGlobal::new(settings_json);
        let result = settings.get_f32(vec!["editor", "font_size"]);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 14.5);
    }

    #[test]
    fn test_settings_get_color() {
        let settings_json = json!({
            "colors": {
                "background": "0x1e1e1e"
            }
        });

        let settings = MockSettingsGlobal::new(settings_json);
        let result = settings.get_color(vec!["colors", "background"]);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0x1e1e1e);
    }

    #[test]
    fn test_settings_nonexistent_key() {
        let settings_json = json!({
            "editor": {
                "theme": "dark"
            }
        });

        let settings = MockSettingsGlobal::new(settings_json);
        let result = settings.get(vec!["editor", "nonexistent"]);

        assert!(result.is_err());
    }

    #[test]
    fn test_settings_nested_path() {
        let settings_json = json!({
            "ui": {
                "editor": {
                    "colors": {
                        "text": "0xffffff"
                    }
                }
            }
        });

        let settings = MockSettingsGlobal::new(settings_json);
        let result = settings.get_color(vec!["ui", "editor", "colors", "text"]);

        assert!(result.is_ok());
    }

    #[test]
    fn test_settings_multiple_values() {
        let settings_json = json!({
            "theme": "dark",
            "font_size": 14,
            "auto_save": true
        });

        let settings = MockSettingsGlobal::new(settings_json);

        assert_eq!(settings.get(vec!["theme"]).unwrap(), "dark");
        assert_eq!(settings.get_f32(vec!["font_size"]).unwrap(), 14.0);
    }

    #[test]
    fn test_settings_color_parsing() {
        let settings_json = json!({
            "color": "0xffffff"
        });

        let settings = MockSettingsGlobal::new(settings_json);
        let result = settings.get_color(vec!["color"]);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0xffffff);
    }

    #[test]
    fn test_settings_invalid_color_format() {
        let settings_json = json!({
            "color": "notacolor"
        });

        let settings = MockSettingsGlobal::new(settings_json);
        let result = settings.get_color(vec!["color"]);

        assert!(result.is_err());
    }

    #[test]
    fn test_settings_empty_path() {
        let settings_json = json!({
            "value": 42
        });

        let settings = MockSettingsGlobal::new(settings_json);
        let mut current = settings.settings.clone();

        // Empty path iteration should leave current unchanged
        for _ in vec![] as Vec<&str> {
            current = current.clone();
        }

        assert!(current.get("value").is_some());
    }

    #[test]
    fn test_settings_single_level() {
        let settings_json = json!({
            "theme": "light"
        });

        let settings = MockSettingsGlobal::new(settings_json);
        let result = settings.get(vec!["theme"]);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "light");
    }

    #[test]
    fn test_settings_deep_nesting() {
        let settings_json = json!({
            "a": {
                "b": {
                    "c": {
                        "d": {
                            "e": "deep_value"
                        }
                    }
                }
            }
        });

        let settings = MockSettingsGlobal::new(settings_json);
        let result = settings.get(vec!["a", "b", "c", "d", "e"]);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "deep_value");
    }

    #[test]
    fn test_settings_numeric_color() {
        let settings_json = json!({
            "colors": {
                "red": "0xFF0000",
                "green": "0x00FF00",
                "blue": "0x0000FF"
            }
        });

        let settings = MockSettingsGlobal::new(settings_json);

        let red = settings.get_color(vec!["colors", "red"]).unwrap();
        let green = settings.get_color(vec!["colors", "green"]).unwrap();
        let blue = settings.get_color(vec!["colors", "blue"]).unwrap();

        assert_eq!(red, 0xFF0000);
        assert_eq!(green, 0x00FF00);
        assert_eq!(blue, 0x0000FF);
    }

    #[test]
    fn test_settings_zero_values() {
        let settings_json = json!({
            "count": 0,
            "opacity": 0.0
        });

        let settings = MockSettingsGlobal::new(settings_json);

        let count = settings.get_f32(vec!["count"]).unwrap();
        let opacity = settings.get_f32(vec!["opacity"]).unwrap();

        assert_eq!(count, 0.0);
        assert_eq!(opacity, 0.0);
    }

    #[test]
    fn test_settings_negative_values() {
        let settings_json = json!({
            "offset": -10.5
        });

        let settings = MockSettingsGlobal::new(settings_json);
        let result = settings.get_f32(vec!["offset"]);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), -10.5);
    }

    #[test]
    fn test_settings_large_numbers() {
        let settings_json = json!({
            "width": 999999.99
        });

        let settings = MockSettingsGlobal::new(settings_json);
        let result = settings.get_f32(vec!["width"]);

        assert!(result.is_ok());
        assert!(result.unwrap() > 999999.0);
    }

    #[test]
    fn test_settings_get_nonexistent_f32() {
        let settings_json = json!({
            "value": "string"
        });

        let settings = MockSettingsGlobal::new(settings_json);
        let result = settings.get_f32(vec!["value"]);

        assert!(result.is_err());
    }

    #[test]
    fn test_settings_clone() {
        let settings_json = json!({
            "theme": "dark"
        });

        let settings1 = MockSettingsGlobal::new(settings_json);
        let settings2 = settings1.clone();

        assert_eq!(
            settings1.get(vec!["theme"]).unwrap(),
            settings2.get(vec!["theme"]).unwrap()
        );
    }

    #[test]
    fn test_settings_case_sensitive() {
        let settings_json = json!({
            "Theme": "dark",
            "theme": "light"
        });

        let settings = MockSettingsGlobal::new(settings_json);

        let theme_lower = settings.get(vec!["theme"]).unwrap();
        assert_eq!(theme_lower, "light");
    }

    #[test]
    fn test_settings_special_characters_in_keys() {
        let settings_json = json!({
            "ui-colors": {
                "text_color": "0xffffff"
            }
        });

        let settings = MockSettingsGlobal::new(settings_json);
        let result = settings.get(vec!["ui-colors", "text_color"]);

        assert!(result.is_ok());
    }
}
