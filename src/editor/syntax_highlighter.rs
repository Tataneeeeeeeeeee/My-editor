use syntect::highlighting::{ThemeSet, Style, Theme};
use syntect::parsing::{SyntaxSet, SyntaxReference};
use syntect::easy::HighlightLines;

pub struct SyntaxHighlighter {
    syntax_set: SyntaxSet,
    theme: Theme,
}

impl SyntaxHighlighter {
    pub fn new() -> Self {
        let syntax_set = SyntaxSet::load_defaults_newlines();
        let theme_set = ThemeSet::load_defaults();
        let theme = theme_set.themes["base16-ocean.dark"].clone();
        
        Self {
            syntax_set,
            theme,
        }
    }
    
    pub fn get_syntax(&self, extension: &str) -> &SyntaxReference {
        self.syntax_set
            .find_syntax_by_extension(extension)
            .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text())
    }
    
    pub fn highlight_line(&self, line: &str, _syntax: &SyntaxReference, highlighter: &mut HighlightLines) -> Vec<(Style, String)> {
        // highlight_line already returns colorized segments
        // Just need to pass the line and the syntax_set
        match highlighter.highlight_line(line, &self.syntax_set) {
            Ok(segments) => segments
                .into_iter()
                .map(|(style, text)| (style, text.to_string()))
                .collect(),
            Err(_) => {
                // In case of error, return the entire line without coloring
                vec![(Style::default(), line.to_string())]
            }
        }
    }
    
    pub fn create_highlighter<'a>(&'a self, syntax: &SyntaxReference) -> HighlightLines<'a> {
        HighlightLines::new(syntax, &self.theme)
    }

    // pub fn get_theme(&self) -> &Theme {
    //     &self.theme
    // }
}

impl Default for SyntaxHighlighter {
    fn default() -> Self {
        Self::new()
    }
}
