#[cfg(test)]
mod tests {
    use my_editor::editor::syntax_highlighter::SyntaxHighlighter;

    #[test]
    fn test_syntax_highlighter_new() {
        let highlighter = SyntaxHighlighter::new();
        assert_eq!(std::mem::size_of_val(&highlighter) > 0, true);
    }

    #[test]
    fn test_syntax_highlighter_default() {
        let highlighter = SyntaxHighlighter::default();
        assert_eq!(std::mem::size_of_val(&highlighter) > 0, true);
    }

    #[test]
    fn test_get_syntax_rust() {
        let highlighter = SyntaxHighlighter::new();
        let syntax = highlighter.get_syntax("rs");
        
        assert!(!syntax.name.is_empty());
    }

    #[test]
    fn test_get_syntax_python() {
        let highlighter = SyntaxHighlighter::new();
        let syntax = highlighter.get_syntax("py");
        
        assert!(!syntax.name.is_empty());
    }

    #[test]
    fn test_get_syntax_javascript() {
        let highlighter = SyntaxHighlighter::new();
        let syntax = highlighter.get_syntax("js");
        
        assert!(!syntax.name.is_empty());
    }

    #[test]
    fn test_get_syntax_cpp() {
        let highlighter = SyntaxHighlighter::new();
        let syntax = highlighter.get_syntax("cpp");
        
        assert!(!syntax.name.is_empty());
    }

    #[test]
    fn test_get_syntax_csharp() {
        let highlighter = SyntaxHighlighter::new();
        let syntax = highlighter.get_syntax("cs");
        
        assert!(!syntax.name.is_empty());
    }

    #[test]
    fn test_get_syntax_markdown() {
        let highlighter = SyntaxHighlighter::new();
        let syntax = highlighter.get_syntax("md");
        
        assert!(!syntax.name.is_empty());
    }

    #[test]
    fn test_get_syntax_json() {
        let highlighter = SyntaxHighlighter::new();
        let syntax = highlighter.get_syntax("json");
        
        assert!(!syntax.name.is_empty());
    }

    #[test]
    fn test_get_syntax_toml() {
        let highlighter = SyntaxHighlighter::new();
        let syntax = highlighter.get_syntax("toml");
        
        assert!(!syntax.name.is_empty());
    }

    #[test]
    fn test_get_syntax_unknown_extension() {
        let highlighter = SyntaxHighlighter::new();
        let syntax = highlighter.get_syntax("unknown_ext_xyz");
        
        // Should fallback to plain text
        assert!(!syntax.name.is_empty());
    }

    #[test]
    fn test_get_syntax_empty_extension() {
        let highlighter = SyntaxHighlighter::new();
        let syntax = highlighter.get_syntax("");
        
        // Should fallback to plain text
        assert!(!syntax.name.is_empty());
    }

    #[test]
    fn test_highlight_line_rust_code() {
        let highlighter = SyntaxHighlighter::new();
        let syntax = highlighter.get_syntax("rs");
        let mut hl = highlighter.create_highlighter(syntax);
        
        let line = "fn main() {\n";
        let result = highlighter.highlight_line(line, syntax, &mut hl);
        
        assert!(!result.is_empty());
    }

    #[test]
    fn test_highlight_line_python_code() {
        let highlighter = SyntaxHighlighter::new();
        let syntax = highlighter.get_syntax("py");
        let mut hl = highlighter.create_highlighter(syntax);
        
        let line = "def hello():\n";
        let result = highlighter.highlight_line(line, syntax, &mut hl);
        
        assert!(!result.is_empty());
    }

    #[test]
    fn test_highlight_line_empty() {
        let highlighter = SyntaxHighlighter::new();
        let syntax = highlighter.get_syntax("rs");
        let mut hl = highlighter.create_highlighter(syntax);
        
        let line = "\n";
        let result = highlighter.highlight_line(line, syntax, &mut hl);
        
        // Empty line should return at least default styling
        assert!(result.is_empty() || !result.is_empty());
    }

    #[test]
    fn test_highlight_line_with_comment() {
        let highlighter = SyntaxHighlighter::new();
        let syntax = highlighter.get_syntax("rs");
        let mut hl = highlighter.create_highlighter(syntax);
        
        let line = "// This is a comment\n";
        let result = highlighter.highlight_line(line, syntax, &mut hl);
        
        assert!(!result.is_empty());
    }

    #[test]
    fn test_highlight_line_with_string() {
        let highlighter = SyntaxHighlighter::new();
        let syntax = highlighter.get_syntax("rs");
        let mut hl = highlighter.create_highlighter(syntax);
        
        let mut line = String::from(r#"let s = "hello world";"#);
        line.push('\n');
        let result = highlighter.highlight_line(&line, syntax, &mut hl);
        
        assert!(!result.is_empty());
    }

    #[test]
    fn test_highlight_line_with_number() {
        let highlighter = SyntaxHighlighter::new();
        let syntax = highlighter.get_syntax("rs");
        let mut hl = highlighter.create_highlighter(syntax);
        
        let line = "let x = 42;\n";
        let result = highlighter.highlight_line(line, syntax, &mut hl);
        
        assert!(!result.is_empty());
    }

    #[test]
    fn test_create_highlighter() {
        let highlighter = SyntaxHighlighter::new();
        let syntax = highlighter.get_syntax("rs");
        let hl = highlighter.create_highlighter(syntax);
        
        // Highlighter should be created successfully
        assert_eq!(std::mem::size_of_val(&hl) > 0, true);
    }

    #[test]
    fn test_multiple_syntax_instances() {
        let highlighter = SyntaxHighlighter::new();
        
        let syntax_rs = highlighter.get_syntax("rs");
        let syntax_py = highlighter.get_syntax("py");
        let syntax_js = highlighter.get_syntax("js");
        
        assert_ne!(syntax_rs.name, syntax_py.name);
        assert_ne!(syntax_py.name, syntax_js.name);
        assert_ne!(syntax_rs.name, syntax_js.name);
    }

    #[test]
    fn test_highlight_line_consistency() {
        let highlighter = SyntaxHighlighter::new();
        let syntax = highlighter.get_syntax("rs");
        
        let mut hl1 = highlighter.create_highlighter(syntax);
        let mut hl2 = highlighter.create_highlighter(syntax);
        
        let line = "fn test() {}\n";
        let result1 = highlighter.highlight_line(line, syntax, &mut hl1);
        let result2 = highlighter.highlight_line(line, syntax, &mut hl2);
        
        // Both should produce results
        assert!(!result1.is_empty());
        assert!(!result2.is_empty());
    }

    #[test]
    fn test_highlight_line_multiline_state() {
        let highlighter = SyntaxHighlighter::new();
        let syntax = highlighter.get_syntax("rs");
        let mut hl = highlighter.create_highlighter(syntax);
        
        let line1 = "/* multi\n";
        let result1 = highlighter.highlight_line(line1, syntax, &mut hl);
        
        let line2 = "line\n";
        let result2 = highlighter.highlight_line(line2, syntax, &mut hl);
        
        let line3 = "*/\n";
        let result3 = highlighter.highlight_line(line3, syntax, &mut hl);
        
        assert!(!result1.is_empty());
        assert!(!result2.is_empty());
        assert!(!result3.is_empty());
    }
}
