use crate::settings::settings::SettingsGlobal;
use gpui::*;

#[derive(Clone, Copy, PartialEq)]
pub enum MenuAction {
    // File menu actions
    NewFile,
    OpenFile,
    OpenFolder,
    SaveFile,

    // Settings menu actions
    OpenSettings,
}

pub struct MenuBar {
    pub file_menu_open: bool,
    pub setting_menu_open: bool,
}

impl MenuBar {
    pub fn new() -> Self {
        Self {
            file_menu_open: false,
            setting_menu_open: false,
        }
    }

    pub fn render(
        &self,
        _file_menu_open: bool,
        cx: &mut Context<crate::editor::editor_window::EditorWindow>,
    ) -> impl IntoElement + use<> {
        let settings_global = cx.global::<SettingsGlobal>().clone();

        let menu_bar_bg = settings_global
            .get_color(vec!["ui", "panels", "menu_bar", "background"])
            .unwrap_or(0x2d2d30);
        let menu_bar_border = settings_global
            .get_color(vec!["ui", "panels", "menu_bar", "border_color"])
            .unwrap_or(0x1e1e1e);
        let menu_text = settings_global
            .get_color(vec!["ui", "colors", "text_secondary"])
            .unwrap_or(0xcccccc);
        let menu_hover_bg = settings_global
            .get_color(vec!["ui", "hover_effects", "menu_hover_background"])
            .unwrap_or(0x3e3e42);

        div()
            .h(px(30.0))
            .w_full()
            .bg(rgb(menu_bar_bg))
            .flex()
            .flex_row()
            .items_center()
            .px_2()
            .border_b_1()
            .border_color(rgb(menu_bar_border))
            .child(
                // Menu "File"
                div()
                    .id("file-menu-button")
                    .px_3()
                    .py_1()
                    .text_color(rgb(menu_text))
                    .hover(|style| style.bg(rgb(menu_hover_bg)))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |this, _event: &MouseDownEvent, _window, cx| {
                            this.menu_bar.file_menu_open = !this.menu_bar.file_menu_open;
                            this.menu_bar.setting_menu_open = false; // Close settings menu if open
                            cx.notify();
                        }),
                    )
                    .child("File"),
            )
            .child(
                // Menu "Settings"
                div()
                    .id("settings-menu-button")
                    .px_3()
                    .py_1()
                    .text_color(rgb(menu_text))
                    .hover(|style| style.bg(rgb(menu_hover_bg)))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |this, _event: &MouseDownEvent, _window, cx| {
                            this.menu_bar.setting_menu_open = !this.menu_bar.setting_menu_open;
                            this.menu_bar.file_menu_open = false; // Close file menu if open
                            cx.notify();
                        }),
                    )
                    .child("Settings"),
            )
    }
}
