use crate::editor::editor_window::EditorWindow;
use crate::settings::settings::SettingsGlobal;
use gpui::Context;
use gpui::{KeyDownEvent, Window};

/// Handles regular character input and navigation keys.
/// Returns true if the event was handled.
pub fn handle_input(
    this: &mut EditorWindow,
    event: &KeyDownEvent,
    window: &mut Window,
    cx: &mut Context<EditorWindow>,
) -> bool {
    let settings_global = cx.global::<SettingsGlobal>().clone();
    let line_height = settings_global
        .get_f32(vec!["ui", "editor", "line_height_px"])
        .unwrap_or(19.2);

    let active_tab = &mut this.tabs[this.active_tab_index];
    let buffer = &mut active_tab.buffer;

    // Printable character
    if let Some(s) = &event.keystroke.key_char {
        if let Some(ch) = s.chars().next() {
            buffer.insert_char(ch);
            active_tab.is_modified = true;
            cx.notify();
            return true;
        }
    }

    // Navigation / special keys
    match event.keystroke.key.as_str() {
        "backspace" => buffer.backspace(),
        "enter" => {
            buffer.insert_char('\n');
            let viewport_height: f32 = window.viewport_size().height.into();
            let viewport_height = viewport_height - 100.0;
            buffer.auto_scroll_to_cursor(viewport_height, line_height);
        }
        "tab" => buffer.insert_tab(),
        "left" => buffer.move_left(),
        "right" => buffer.move_right(),
        "up" => buffer.move_up(),
        "down" => buffer.move_down(),
        _ => return false,
    }

    cx.notify();
    true
}
