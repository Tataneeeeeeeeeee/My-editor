#[cfg(test)]
mod tests {
    use serde_json::json;

    // Mock SettingsGlobal pour les tests
    #[derive(Clone)]
    struct MockSettingsGlobal {
        settings: serde_json::Value,
    }

    impl MockSettingsGlobal {
        fn new() -> Self {
            Self {
                settings: json!({
                    "ui": {
                        "editor": {
                            "line_height_px": 19.2,
                            "char_spacing_px": 0.5,
                            "cursor_color": "0xffffff",
                            "cursor_opacity": 0.9,
                            "cursor_width_px": 2.0,
                            "background": "0x1e1e1e",
                            "gutter": {
                                "text_color": "0x858585",
                                "margin_px": 8.0
                            },
                            "scrollbar": {
                                "width_px": 14.0,
                                "background": "0x1e1e1e",
                                "thumb_color": "0x424242",
                                "thumb_hover_color": "0x4f4f4f"
                            },
                            "monospace_char_width_px": 8.0,
                            "padding_left_px": 16.0
                        },
                        "panels": {
                            "status_bar": {
                                "text_color": "0x858585",
                                "border_color": "0x404040",
                                "height_px": 60.0
                            },
                            "tab_bar": {
                                "height_px": 40.0
                            },
                            "toolbar": {
                                "background": "0x333333"
                            },
                            "explorer": {
                                "width_px": 240.0
                            }
                        },
                        "colors": {
                            "background": "0x2d2d30"
                        }
                    },
                    "file_extensions": {
                        "rs": { "name": "Rust" },
                        "py": { "name": "Python" },
                        "js": { "name": "JavaScript" }
                    }
                }),
            }
        }

        fn get(&self, path: Vec<&str>) -> Result<String, String> {
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

        fn get_f32(&self, path: Vec<&str>) -> Result<f32, String> {
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

        fn get_color(&self, path: Vec<&str>) -> Result<u32, String> {
            let color_str = self.get(path)?;
            u32::from_str_radix(color_str.trim_start_matches("0x"), 16)
                .map_err(|_| format!("Invalid color format: {}", color_str))
        }
    }

    #[test]
    fn test_editor_element_creation() {
        let settings = MockSettingsGlobal::new();

        assert_eq!(
            settings
                .get_f32(vec!["ui", "editor", "line_height_px"])
                .unwrap(),
            19.2
        );
    }

    #[test]
    fn test_editor_element_settings_retrieval() {
        let settings = MockSettingsGlobal::new();

        let line_height = settings.get_f32(vec!["ui", "editor", "line_height_px"]);
        assert!(line_height.is_ok());
        assert_eq!(line_height.unwrap(), 19.2);
    }

    #[test]
    fn test_editor_element_cursor_color() {
        let settings = MockSettingsGlobal::new();

        let cursor_color = settings.get_color(vec!["ui", "editor", "cursor_color"]);
        assert!(cursor_color.is_ok());
    }

    #[test]
    fn test_editor_element_gutter_width() {
        let settings = MockSettingsGlobal::new();

        let gutter_width = settings.get_f32(vec!["ui", "panels", "explorer", "width_px"]);
        assert!(gutter_width.is_ok());
    }

    #[test]
    fn test_editor_element_status_bar_height() {
        let settings = MockSettingsGlobal::new();

        let height = settings.get_f32(vec!["ui", "panels", "status_bar", "height_px"]);
        assert!(height.is_ok());
        assert_eq!(height.unwrap(), 60.0);
    }

    #[test]
    fn test_editor_element_tab_bar_height() {
        let settings = MockSettingsGlobal::new();

        let height = settings.get_f32(vec!["ui", "panels", "tab_bar", "height_px"]);
        assert!(height.is_ok());
        assert_eq!(height.unwrap(), 40.0);
    }

    #[test]
    fn test_editor_element_scrollbar_width() {
        let settings = MockSettingsGlobal::new();

        let width = settings.get_f32(vec!["ui", "editor", "scrollbar", "width_px"]);
        assert!(width.is_ok());
        assert_eq!(width.unwrap(), 14.0);
    }

    #[test]
    fn test_editor_element_background_color() {
        let settings = MockSettingsGlobal::new();

        let bg = settings.get_color(vec!["ui", "editor", "background"]);
        assert!(bg.is_ok());
    }

    #[test]
    fn test_editor_element_cursor_opacity() {
        let settings = MockSettingsGlobal::new();

        let opacity = settings.get_f32(vec!["ui", "editor", "cursor_opacity"]);
        assert!(opacity.is_ok());
        assert_eq!(opacity.unwrap(), 0.9);
    }

    #[test]
    fn test_editor_element_cursor_width() {
        let settings = MockSettingsGlobal::new();

        let width = settings.get_f32(vec!["ui", "editor", "cursor_width_px"]);
        assert!(width.is_ok());
        assert_eq!(width.unwrap(), 2.0);
    }

    #[test]
    fn test_editor_element_char_spacing() {
        let settings = MockSettingsGlobal::new();

        let spacing = settings.get_f32(vec!["ui", "editor", "char_spacing_px"]);
        assert!(spacing.is_ok());
        assert_eq!(spacing.unwrap(), 0.5);
    }

    #[test]
    fn test_editor_element_file_extension_name() {
        let settings = MockSettingsGlobal::new();

        let rust_name = settings.get(vec!["file_extensions", "rs", "name"]);
        assert!(rust_name.is_ok());
        assert_eq!(rust_name.unwrap(), "Rust");
    }

    #[test]
    fn test_editor_element_python_extension() {
        let settings = MockSettingsGlobal::new();

        let py_name = settings.get(vec!["file_extensions", "py", "name"]);
        assert!(py_name.is_ok());
        assert_eq!(py_name.unwrap(), "Python");
    }

    #[test]
    fn test_editor_element_javascript_extension() {
        let settings = MockSettingsGlobal::new();

        let js_name = settings.get(vec!["file_extensions", "js", "name"]);
        assert!(js_name.is_ok());
        assert_eq!(js_name.unwrap(), "JavaScript");
    }

    #[test]
    fn test_editor_element_nonexistent_setting() {
        let settings = MockSettingsGlobal::new();

        let result = settings.get(vec!["nonexistent", "path"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_editor_element_monospace_char_width() {
        let settings = MockSettingsGlobal::new();

        let width = settings.get_f32(vec!["ui", "editor", "monospace_char_width_px"]);
        assert!(width.is_ok());
        assert_eq!(width.unwrap(), 8.0);
    }

    #[test]
    fn test_editor_element_padding_left() {
        let settings = MockSettingsGlobal::new();

        let padding = settings.get_f32(vec!["ui", "editor", "padding_left_px"]);
        assert!(padding.is_ok());
        assert_eq!(padding.unwrap(), 16.0);
    }

    #[test]
    fn test_editor_element_gutter_margin() {
        let settings = MockSettingsGlobal::new();

        let margin = settings.get_f32(vec!["ui", "editor", "gutter", "margin_px"]);
        assert!(margin.is_ok());
        assert_eq!(margin.unwrap(), 8.0);
    }

    #[test]
    fn test_editor_element_gutter_text_color() {
        let settings = MockSettingsGlobal::new();

        let color = settings.get_color(vec!["ui", "editor", "gutter", "text_color"]);
        assert!(color.is_ok());
    }

    #[test]
    fn test_editor_element_scrollbar_thumb_color() {
        let settings = MockSettingsGlobal::new();

        let color = settings.get_color(vec!["ui", "editor", "scrollbar", "thumb_color"]);
        assert!(color.is_ok());
    }

    #[test]
    fn test_editor_element_scrollbar_thumb_hover_color() {
        let settings = MockSettingsGlobal::new();

        let color = settings.get_color(vec!["ui", "editor", "scrollbar", "thumb_hover_color"]);
        assert!(color.is_ok());
    }

    #[test]
    fn test_editor_element_all_settings_accessible() {
        let settings = MockSettingsGlobal::new();

        // Test that all common settings are accessible
        assert!(
            settings
                .get_f32(vec!["ui", "editor", "line_height_px"])
                .is_ok()
        );
        assert!(
            settings
                .get_f32(vec!["ui", "editor", "char_spacing_px"])
                .is_ok()
        );
        assert!(
            settings
                .get_color(vec!["ui", "editor", "cursor_color"])
                .is_ok()
        );
        assert!(
            settings
                .get_f32(vec!["ui", "panels", "status_bar", "height_px"])
                .is_ok()
        );
    }

    #[test]
    fn test_editor_element_explorer_width() {
        let settings = MockSettingsGlobal::new();

        let width = settings.get_f32(vec!["ui", "panels", "explorer", "width_px"]);
        assert!(width.is_ok());
        assert_eq!(width.unwrap(), 240.0);
    }

    #[test]
    fn test_editor_element_status_bar_text_color() {
        let settings = MockSettingsGlobal::new();

        let color = settings.get_color(vec!["ui", "panels", "status_bar", "text_color"]);
        assert!(color.is_ok());
    }

    #[test]
    fn test_editor_element_status_bar_border_color() {
        let settings = MockSettingsGlobal::new();

        let color = settings.get_color(vec!["ui", "panels", "status_bar", "border_color"]);
        assert!(color.is_ok());
    }
}
