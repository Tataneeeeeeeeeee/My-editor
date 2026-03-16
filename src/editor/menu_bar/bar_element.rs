use crate::settings::settings::SettingsGlobal;
use gpui::*;

/// Renders the file dropdown menu overlay
pub fn render_file_dropdown(
    on_new_file: impl Fn(&MouseDownEvent, &mut Window, &mut App) + 'static,
    on_open_file: impl Fn(&MouseDownEvent, &mut Window, &mut App) + 'static,
    on_open_directory: impl Fn(&MouseDownEvent, &mut Window, &mut App) + 'static,
    on_save_file: impl Fn(&MouseDownEvent, &mut Window, &mut App) + 'static,
    settings_global: SettingsGlobal,
) -> impl IntoElement {
    div()
        .absolute()
        .top(px(settings_global
            .get_f32(vec!["ui", "dropdown", "padding_x_px"])
            .unwrap_or(30.0)))
        .left(px(settings_global
            .get_f32(vec!["ui", "dropdown", "padding_y_px"])
            .unwrap_or(8.0)))
        .w(px(settings_global
            .get_f32(vec!["ui", "dropdown", "width_px"])
            .unwrap_or(200.0)))
        .bg(rgb(settings_global
            .get_color(vec!["ui", "dropdown", "background"])
            .unwrap_or(0x252526)))
        .border_1()
        .border_color(rgb(settings_global
            .get_color(vec!["ui", "dropdown", "border_color"])
            .unwrap_or(0x454545)))
        .shadow_lg()
        .flex()
        .flex_col()
        .child(
            div()
                .id("new-file-button")
                .px_4()
                .py_2()
                .text_color(rgb(settings_global
                    .get_color(vec!["ui", "dropdown", "text_color"])
                    .unwrap_or(0xcccccc)))
                .hover(|style| {
                    style.bg(rgb(settings_global
                        .get_color(vec!["ui", "dropdown", "hover_background"])
                        .unwrap_or(0x094771)))
                })
                .on_mouse_down(MouseButton::Left, on_new_file)
                .child("New Text File"),
        )
        .child(
            div()
                .id("open-file-button")
                .px_4()
                .py_2()
                .text_color(rgb(settings_global
                    .get_color(vec!["ui", "dropdown", "text_color"])
                    .unwrap_or(0xcccccc)))
                .hover(|style| {
                    style.bg(rgb(settings_global
                        .get_color(vec!["ui", "dropdown", "hover_background"])
                        .unwrap_or(0x094771)))
                })
                .on_mouse_down(MouseButton::Left, on_open_file)
                .child("Open File"),
        )
        .child(
            div()
                .id("open-file-button")
                .px_4()
                .py_2()
                .text_color(rgb(settings_global
                    .get_color(vec!["ui", "dropdown", "text_color"])
                    .unwrap_or(0xcccccc)))
                .hover(|style| {
                    style.bg(rgb(settings_global
                        .get_color(vec!["ui", "dropdown", "hover_background"])
                        .unwrap_or(0x094771)))
                })
                .on_mouse_down(MouseButton::Left, on_open_directory)
                .child("Open Folder"),
        )
        .child(
            div()
                .id("save-file-button")
                .px_4()
                .py_2()
                .text_color(rgb(settings_global
                    .get_color(vec!["ui", "dropdown", "text_color"])
                    .unwrap_or(0xcccccc)))
                .hover(|style| {
                    style.bg(rgb(settings_global
                        .get_color(vec!["ui", "dropdown", "hover_background"])
                        .unwrap_or(0x094771)))
                })
                .on_mouse_down(MouseButton::Left, on_save_file)
                .child("Save File"),
        )
}

/// Renders the settings dropdown menu overlay
pub fn render_setting_dropdown(
    on_open_settings: impl Fn(&MouseDownEvent, &mut Window, &mut App) + 'static,
    settings_global: SettingsGlobal,
) -> impl IntoElement {
    div()
        .absolute()
        .top(px(settings_global
            .get_f32(vec!["ui", "dropdown", "padding_x_px"])
            .unwrap_or(16.0)))
        .left(px(settings_global
            .get_f32(vec!["ui", "dropdown", "padding_y_px"])
            .unwrap_or(8.0)))
        .w(px(settings_global
            .get_f32(vec!["ui", "dropdown", "width_px"])
            .unwrap_or(200.0)))
        .bg(rgb(settings_global
            .get_color(vec!["ui", "dropdown", "background"])
            .unwrap_or(0x252526)))
        .border_1()
        .border_color(rgb(settings_global
            .get_color(vec!["ui", "dropdown", "border_color"])
            .unwrap_or(0x454545)))
        .shadow_lg()
        .flex()
        .flex_col()
        .child(
            div()
                .id("open-settings-button")
                .px_4()
                .py_2()
                .text_color(rgb(settings_global
                    .get_color(vec!["ui", "dropdown", "text_color"])
                    .unwrap_or(0xcccccc)))
                .hover(|style| {
                    style.bg(rgb(settings_global
                        .get_color(vec!["ui", "dropdown", "hover_background"])
                        .unwrap_or(0x094771)))
                })
                .on_mouse_down(MouseButton::Left, on_open_settings)
                .child("Open Settings"),
        )
}
