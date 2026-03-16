#[cfg(test)]
mod tests {
    use my_editor::editor::text_buffer::TextBuffer;
    use std::path::PathBuf;

    #[test]
    fn test_text_buffer_new() {
        let buffer = TextBuffer::new();

        assert_eq!(buffer.text, "");
        assert_eq!(buffer.cursor, 0);
        assert_eq!(buffer.line_count, 1);
        assert_eq!(buffer.current_line, 1);
        assert_eq!(buffer.current_col, 0);
        assert_eq!(buffer.scroll_y, 0.0);
        assert!(buffer.file_path.is_none());
    }

    #[test]
    fn test_text_buffer_insert_char() {
        let mut buffer = TextBuffer::new();

        buffer.insert_char('H');
        assert_eq!(buffer.text, "H");
        assert_eq!(buffer.cursor, 1);

        buffer.insert_char('i');
        assert_eq!(buffer.text, "Hi");
        assert_eq!(buffer.cursor, 2);
    }

    #[test]
    fn test_text_buffer_insert_multiple_chars() {
        let mut buffer = TextBuffer::new();

        for ch in "Hello".chars() {
            buffer.insert_char(ch);
        }

        assert_eq!(buffer.text, "Hello");
        assert_eq!(buffer.cursor, 5);
    }

    #[test]
    fn test_text_buffer_backspace() {
        let mut buffer = TextBuffer::new();
        buffer.insert_char('H');
        buffer.insert_char('i');

        buffer.backspace();
        assert_eq!(buffer.text, "H");
        assert_eq!(buffer.cursor, 1);
    }

    #[test]
    fn test_text_buffer_backspace_empty() {
        let mut buffer = TextBuffer::new();

        buffer.backspace();
        assert_eq!(buffer.text, "");
        assert_eq!(buffer.cursor, 0);
    }

    #[test]
    fn test_text_buffer_backspace_multiple() {
        let mut buffer = TextBuffer::new();
        for ch in "Hello".chars() {
            buffer.insert_char(ch);
        }

        buffer.backspace();
        buffer.backspace();

        assert_eq!(buffer.text, "Hel");
        assert_eq!(buffer.cursor, 3);
    }

    #[test]
    fn test_text_buffer_insert_tab() {
        let mut buffer = TextBuffer::new();

        buffer.insert_tab();
        assert_eq!(buffer.text, "    ");
        assert_eq!(buffer.cursor, 4);
    }

    #[test]
    fn test_text_buffer_move_left() {
        let mut buffer = TextBuffer::new();
        for ch in "Hello".chars() {
            buffer.insert_char(ch);
        }

        buffer.move_left();
        assert_eq!(buffer.cursor, 4);

        buffer.move_left();
        assert_eq!(buffer.cursor, 3);
    }

    #[test]
    fn test_text_buffer_move_left_at_start() {
        let mut buffer = TextBuffer::new();
        buffer.insert_char('H');
        buffer.move_left();

        buffer.move_left();
        assert_eq!(buffer.cursor, 0);
    }

    #[test]
    fn test_text_buffer_move_right() {
        let mut buffer = TextBuffer::new();
        for ch in "Hello".chars() {
            buffer.insert_char(ch);
        }

        buffer.cursor = 0;
        buffer.move_right();
        assert_eq!(buffer.cursor, 1);

        buffer.move_right();
        assert_eq!(buffer.cursor, 2);
    }

    #[test]
    fn test_text_buffer_move_right_at_end() {
        let mut buffer = TextBuffer::new();
        buffer.insert_char('H');

        buffer.move_right();
        assert_eq!(buffer.cursor, 1);

        buffer.move_right();
        assert_eq!(buffer.cursor, 1);
    }

    #[test]
    fn test_text_buffer_update_stats() {
        let mut buffer = TextBuffer::new();
        buffer.insert_char('H');
        buffer.insert_char('i');
        buffer.insert_char('\n');
        buffer.insert_char('W');

        buffer.update_stats();
        assert!(buffer.line_count >= 2);
    }

    #[test]
    fn test_text_buffer_single_line() {
        let mut buffer = TextBuffer::new();
        for ch in "Hello World".chars() {
            buffer.insert_char(ch);
        }

        buffer.update_stats();
        assert_eq!(buffer.line_count, 1);
    }

    #[test]
    fn test_text_buffer_multiline() {
        let mut buffer = TextBuffer::new();
        buffer.insert_char('H');
        buffer.insert_char('\n');
        buffer.insert_char('W');

        buffer.update_stats();
        assert_eq!(buffer.line_count, 2);
    }

    #[test]
    fn test_text_buffer_cursor_position_calculation() {
        let mut buffer = TextBuffer::new();
        for ch in "Line1\nLine2".chars() {
            buffer.insert_char(ch);
        }

        buffer.set_cursor_from_position(1, 0);
        assert_eq!(buffer.current_line, 1);
    }

    #[test]
    fn test_text_buffer_scroll_to_cursor_down() {
        let mut buffer = TextBuffer::new();
        for i in 0..100 {
            buffer.insert_char(char::from_digit((i % 10) as u32, 10).unwrap());
            buffer.insert_char('\n');
        }

        buffer.current_line = 50;
        buffer.auto_scroll_to_cursor(600.0, 19.2);

        assert!(buffer.scroll_y > 0.0);
    }

    #[test]
    fn test_text_buffer_scroll_to_cursor_up() {
        let mut buffer = TextBuffer::new();
        for i in 0..100 {
            buffer.insert_char(char::from_digit((i % 10) as u32, 10).unwrap());
            buffer.insert_char('\n');
        }

        buffer.scroll_y = 1000.0;
        buffer.current_line = 1;
        buffer.auto_scroll_to_cursor(600.0, 19.2);

        assert!(buffer.scroll_y < 1000.0);
    }

    #[test]
    fn test_text_buffer_load_from_file() {
        let mut buffer = TextBuffer::new();
        let path = PathBuf::from("/tmp/test.txt");

        buffer.load_from_file(path.clone(), "Hello".to_string());

        assert_eq!(buffer.text, "Hello");
        assert_eq!(buffer.file_path, Some(path));
        assert_eq!(buffer.cursor, 0);
        assert_eq!(buffer.scroll_y, 0.0);
    }

    #[test]
    fn test_text_buffer_get_file_extension() {
        let mut buffer = TextBuffer::new();

        buffer.load_from_file(PathBuf::from("test.rs"), "fn main() {}".to_string());
        assert_eq!(buffer.get_file_extension(), "rs");

        buffer.load_from_file(PathBuf::from("script.py"), "print('hi')".to_string());
        assert_eq!(buffer.get_file_extension(), "py");
    }

    #[test]
    fn test_text_buffer_get_file_extension_no_extension() {
        let mut buffer = TextBuffer::new();

        buffer.load_from_file(PathBuf::from("Makefile"), "".to_string());
        assert_eq!(buffer.get_file_extension(), "txt");
    }

    #[test]
    fn test_text_buffer_default() {
        let buffer = TextBuffer::default();

        assert_eq!(buffer.text, "");
        assert_eq!(buffer.cursor, 0);
    }

    #[test]
    fn test_text_buffer_unicode_characters() {
        let mut buffer = TextBuffer::new();

        buffer.insert_char('€');
        assert_eq!(buffer.text, "€");
        assert!(buffer.cursor > 0);
    }

    #[test]
    fn test_text_buffer_emoji_characters() {
        let mut buffer = TextBuffer::new();

        buffer.insert_char('😀');
        assert_eq!(buffer.text, "😀");
        assert!(buffer.cursor > 0);
    }

    #[test]
    fn test_text_buffer_move_up() {
        let mut buffer = TextBuffer::new();
        let text = "line1\nline2\nline3";
        for ch in text.chars() {
            buffer.insert_char(ch);
        }

        buffer.set_cursor_from_position(3, 1);
        buffer.move_up();
        assert!(buffer.current_line < 3);
    }

    #[test]
    fn test_text_buffer_move_down() {
        let mut buffer = TextBuffer::new();
        let text = "line1\nline2\nline3";
        for ch in text.chars() {
            buffer.insert_char(ch);
        }

        buffer.set_cursor_from_position(1, 1);
        buffer.move_down();
        assert!(buffer.current_line >= 2);
    }

    #[test]
    fn test_text_buffer_clone() {
        let mut buffer = TextBuffer::new();
        buffer.insert_char('A');
        buffer.insert_char('B');

        let cloned = buffer.clone();

        assert_eq!(cloned.text, buffer.text);
        assert_eq!(cloned.cursor, buffer.cursor);
    }
}
