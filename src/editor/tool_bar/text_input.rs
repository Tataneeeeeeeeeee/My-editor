/// Generic text input management system for reusable input components
/// Can be used for: file/folder creation, search, renaming, etc.

use gpui::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TextInputType {
    /// Input for creating files
    CreateFile,
    /// Input for creating folders
    CreateFolder,
    /// Input for searching files
    Search,
}

/// Reusable state structure for any text input
#[derive(Clone, Debug)]
pub struct TextInputState {
    /// Type of input (determines behavior and rendering)
    pub input_type: TextInputType,
    /// Current input text
    pub input: String,
}

impl TextInputState {
    /// Create a new text input state
    pub fn new(input_type: TextInputType) -> Self {
        Self {
            input_type,
            input: String::new(),
        }
    }

    /// Push a character to the input
    pub fn push_char(&mut self, ch: char) {
        self.input.push(ch);
    }

    /// Remove the last character (backspace)
    pub fn backspace(&mut self) {
        self.input.pop();
    }

    /// Clear the input
    pub fn clear(&mut self) {
        self.input.clear();
    }

    /// Get the display text (with cursor indicator)
    pub fn display_text(&self) -> String {
        format!("{}_", self.input) // fake cursor
    }

    /// Get the input label based on type
    pub fn get_label(&self) -> &'static str {
        match self.input_type {
            TextInputType::CreateFile => "New file name:",
            TextInputType::CreateFolder => "New folder name:",
            TextInputType::Search => "Search:",
        }
    }

    /// Get hint text for this input type
    pub fn get_hint(&self) -> &'static str {
        match self.input_type {
            TextInputType::CreateFile | TextInputType::CreateFolder => "Enter ✓  Esc ✗",
            TextInputType::Search => "Type to search (4+ chars)…",
        }
    }
}

/// Render a generic text input box
pub fn render_text_input_box(state: &TextInputState) -> impl IntoElement {
    let display = state.display_text();
    
    div()
        .px(px(6.0))
        .py(px(2.0))
        .bg(rgb(0x3c3c3c))
        .border_1()
        .border_color(rgb(0xFFB126))
        .rounded(px(3.0))
        .text_size(px(13.0))
        .text_color(rgb(0xffffff))
        .font_family("monospace")
        .child(display)
}

/// Render a full input section with label, input box, and hint
pub fn render_text_input_section(state: &TextInputState) -> impl IntoElement {
    let label = state.get_label();
    let hint = state.get_hint();

    div()
        .px(px(8.0))
        .py(px(4.0))
        .flex()
        .flex_col()
        .gap(px(2.0))
        .bg(rgb(0x2a2d2e))
        .border_b_1()
        .border_color(rgb(0xFFB126))
        .child(
            div()
                .text_size(px(10.0))
                .text_color(rgb(0xffffff))
                .child(label)
        )
        .child(render_text_input_box(state))
        .child(
            div()
                .text_size(px(10.0))
                .text_color(rgb(0xffffff))
                .child(hint)
        )
        .mb_1()
}
