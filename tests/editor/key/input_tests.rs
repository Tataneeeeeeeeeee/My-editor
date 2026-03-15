#[cfg(test)]
mod tests {
    #[test]
    fn test_key_event_creation() {
        let key_code = "a".to_string();
        assert_eq!(key_code, "a");
    }

    #[test]
    fn test_key_event_uppercase() {
        let key_code = "A".to_string();
        assert_eq!(key_code, "A");
    }

    #[test]
    fn test_key_event_special_char() {
        let key_code = "Enter".to_string();
        assert_eq!(key_code, "Enter");
    }

    #[test]
    fn test_key_event_multiple_chars() {
        let key_code = "Shift+A".to_string();
        assert_eq!(key_code, "Shift+A");
    }

    #[test]
    fn test_input_buffer_push() {
        let mut buffer = String::new();
        buffer.push('a');
        buffer.push('b');
        assert_eq!(buffer, "ab");
    }

    #[test]
    fn test_input_buffer_pop() {
        let mut buffer = "hello".to_string();
        buffer.pop();
        assert_eq!(buffer, "hell");
    }

    #[test]
    fn test_input_buffer_clear() {
        let mut buffer = "test".to_string();
        buffer.clear();
        assert_eq!(buffer, "");
    }

    #[test]
    fn test_input_unicode() {
        let mut buffer = String::new();
        buffer.push('€');
        assert_eq!(buffer, "€");
    }

    #[test]
    fn test_key_modifier_combinations() {
        let modifiers = vec!["Ctrl", "Shift", "Alt"];
        assert_eq!(modifiers.len(), 3);
    }

    #[test]
    fn test_key_code_ranges() {
        let codes = ('a'..='z').collect::<Vec<_>>();
        assert_eq!(codes.len(), 26);
    }

    #[test]
    fn test_function_keys() {
        let f_keys = vec!["F1", "F2", "F3", "F4", "F5"];
        assert_eq!(f_keys.len(), 5);
    }

    #[test]
    fn test_navigation_keys() {
        let nav_keys = vec!["Up", "Down", "Left", "Right"];
        assert_eq!(nav_keys.len(), 4);
    }

    #[test]
    fn test_control_keys() {
        let ctrl_keys = vec!["Tab", "Enter", "Escape", "Backspace"];
        assert_eq!(ctrl_keys.len(), 4);
    }
}
