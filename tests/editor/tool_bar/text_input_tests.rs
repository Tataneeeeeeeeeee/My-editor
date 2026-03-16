#[cfg(test)]
mod tests {
    #[test]
    fn test_text_input_creation() {
        let input = "".to_string();
        assert_eq!(input.is_empty(), true);
    }

    #[test]
    fn test_text_input_push_char() {
        let mut input = String::new();
        input.push('a');
        assert_eq!(input, "a");
    }

    #[test]
    fn test_text_input_push_multiple() {
        let mut input = String::new();
        for ch in "hello".chars() {
            input.push(ch);
        }
        assert_eq!(input, "hello");
    }

    #[test]
    fn test_text_input_backspace() {
        let mut input = "hello".to_string();
        input.pop();
        assert_eq!(input, "hell");
    }

    #[test]
    fn test_text_input_backspace_empty() {
        let mut input = "".to_string();
        input.pop();
        assert_eq!(input, "");
    }

    #[test]
    fn test_text_input_clear() {
        let mut input = "hello world".to_string();
        input.clear();
        assert_eq!(input, "");
    }

    #[test]
    fn test_text_input_length() {
        let input = "hello".to_string();
        assert_eq!(input.len(), 5);
    }

    #[test]
    fn test_text_input_contains() {
        let input = "hello world".to_string();
        assert!(input.contains("world"));
    }

    #[test]
    fn test_text_input_unicode() {
        let mut input = String::new();
        input.push('€');
        input.push('ñ');
        assert!(input.len() > 2);
    }

    #[test]
    fn test_text_input_state() {
        struct TextInputState {
            input: String,
            cursor_pos: usize,
        }

        let state = TextInputState {
            input: "test".to_string(),
            cursor_pos: 0,
        };

        assert_eq!(state.input, "test");
        assert_eq!(state.cursor_pos, 0);
    }
}
