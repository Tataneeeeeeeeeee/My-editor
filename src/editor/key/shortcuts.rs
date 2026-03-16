use crate::editor::editor_window::EditorWindow;
use gpui::Context;
use gpui::KeyDownEvent;

/// Returns true if the event was a Ctrl shortcut and was handled
pub fn handle_ctrl(
    this: &mut EditorWindow,
    event: &KeyDownEvent,
    cx: &mut Context<EditorWindow>,
) -> bool {
    if !event.keystroke.modifiers.control {
        return false;
    }

    match event.keystroke.key.as_str() {
        "t" => {
            this.add_tab("Untitled".to_string(), cx);
            true
        }
        "w" => {
            if this.tabs.len() > 1 {
                this.close_tab(this.active_tab_index, cx);
            }
            true
        }
        "tab" => {
            let next_index = (this.active_tab_index + 1) % this.tabs.len();
            this.set_active_tab(next_index, cx);
            true
        }
        "s" => {
            this.save_current_file(cx);
            true
        }
        "o" => {
            this.open_file(cx);
            true
        }
        "n" => {
            this.add_tab("Untitled".to_string(), cx);
            true
        }
        _ => false,
    }
}
