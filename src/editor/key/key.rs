use crate::editor::editor_window::*;
use crate::editor::key::*;
use gpui::*;

pub fn handle_key(
    this: &mut EditorWindow,
    event: &KeyDownEvent,
    window: &mut Window,
    cx: &mut Context<EditorWindow>,
) {
    // If a creation input is active, route keys there
    if this.pending_creation.is_some() {
        match event.keystroke.key.as_str() {
            "enter" => {
                this.confirm_creation(cx);
                return;
            }
            "escape" => {
                this.cancel_creation(cx);
                return;
            }
            "backspace" => {
                this.creation_input_backspace(cx);
                return;
            }
            _ => {}
        }
        if let Some(s) = &event.keystroke.key_char {
            if let Some(ch) = s.chars().next() as Option<char> {
                this.creation_input_push(ch, cx);
                return;
            }
        }
        return;
    }

    // If a search input is active, route keys there
    if this.search_open {
        match event.keystroke.key.as_str() {
            "escape" => {
                this.search_input_clear(cx);
                return;
            }
            "backspace" => {
                this.search_input_backspace(cx);
                return;
            }
            _ => {}
        }
        if let Some(s) = &event.keystroke.key_char {
            if let Some(ch) = s.chars().next() as Option<char> {
                this.search_input_push(ch, cx);
                return;
            }
        }
        return;
    }

    if !key::shortcuts::handle_ctrl(this, event, cx) {
        key::input::handle_input(this, event, window, cx);
    }
}
