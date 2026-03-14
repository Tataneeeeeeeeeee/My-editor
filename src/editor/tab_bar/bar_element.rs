use gpui::*;
use crate::editor::editor_window::EditorWindow;
use crate::settings::settings::SettingsGlobal;

pub fn render_bar(
    tabs_info: &[(usize, String, bool, bool)], // (index, title, is_active, is_modified)
    cx: &mut Context<EditorWindow>,
) -> impl IntoElement + use<> {
    let settings_global = cx.global::<SettingsGlobal>().clone();
    
    let tab_bar_bg = settings_global.get_color(vec!["ui", "panels", "tab_bar", "background"]).unwrap_or(0x252526);
    let tab_active_bg = settings_global.get_color(vec!["ui", "panels", "tab_bar", "active_background"]).unwrap_or(0x1e1e1e);
    let tab_inactive_bg = settings_global.get_color(vec!["ui", "panels", "tab_bar", "inactive_background"]).unwrap_or(0x2d2d30);
    let tab_text_active = settings_global.get_color(vec!["ui", "panels", "tab_bar", "text_active"]).unwrap_or(0xffffff);
    let tab_text_inactive = settings_global.get_color(vec!["ui", "panels", "tab_bar", "text_inactive"]).unwrap_or(0x969696);
    let tab_border_color = settings_global.get_color(vec!["ui", "panels", "tab_bar", "border_color"]).unwrap_or(0x1e1e1e);
    let error_color = settings_global.get_color(vec!["ui", "colors", "error"]).unwrap_or(0xff5555);
    let hover_bg = settings_global.get_color(vec!["ui", "hover_effects", "panel_hover_background"]).unwrap_or(0x454545);
    let text_secondary = settings_global.get_color(vec!["ui", "colors", "text_secondary"]).unwrap_or(0xcccccc);
    
    div()
        .h(px(40.0))
        .bg(rgb(tab_bar_bg))
        .flex()
        .flex_row()
        .items_center()
        .children(
            tabs_info.iter().map(|(tab_index, title, is_active, is_modified)| {
                let tab_idx = *tab_index;
                let is_act = *is_active;
                let is_mod = *is_modified;

                let mut name = format!("{}", title);
                if tabs_info.iter().filter(|(_, t, _, _)| t == title).count() > 1 {
                    name.push_str(&format!(" ({})", tab_idx + 1));
                }

                div()
                    .h_full()
                    .px_4()
                    .flex()
                    .items_center()
                    .bg(if is_act { rgb(tab_active_bg) } else { rgb(tab_inactive_bg) })
                    .border_r_1()
                    .border_color(rgb(tab_border_color))
                    .on_mouse_down(MouseButton::Left, cx.listener(move |this, _: &MouseDownEvent, _window, cx| {
                        this.set_active_tab(tab_idx, cx);
                    }))
                    .child(
                        div()
                            .text_color(if is_act { rgb(tab_text_active) } else { rgb(tab_text_inactive) })
                            .child(name),
                    )
                    .child(
                        if is_act {
                            div()
                                .id(("close-tab", tab_idx))
                                .ml_2()
                                .w(px(16.0))
                                .h(px(16.0))
                                .flex()
                                .items_center()
                                .justify_center()
                                .rounded(px(4.0))
                                .text_color(if is_mod { rgb(error_color) } else { rgb(text_secondary) })
                                .hover(|style| style.bg(rgb(hover_bg)).text_color(rgb(tab_text_active)))
                                .on_mouse_down(MouseButton::Left, cx.listener(move |this, _: &MouseDownEvent, _window, cx| {
                                    this.close_tab(tab_idx, cx);
                                }))
                                .child("✖")
                        } else {
                            div()
                                .id(("close-tab", tab_idx))
                                .w(px(16.0))
                                .h(px(16.0))
                        }
                    )
            })
        )
}