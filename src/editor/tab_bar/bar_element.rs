use gpui::*;
use crate::editor::editor_window::EditorWindow;

pub fn render_bar(
    tabs_info: &[(usize, String, bool, bool)], // (index, title, is_active, is_modified)
    cx: &mut Context<EditorWindow>,
) -> impl IntoElement + use<> {
    div()
        .h(px(40.0))
        .bg(rgb(0x252526))
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
                    .bg(if is_act { rgb(0x1e1e1e) } else { rgb(0x2d2d30) })
                    .border_r_1()
                    .border_color(rgb(0x1e1e1e))
                    .on_mouse_down(MouseButton::Left, cx.listener(move |this, _: &MouseDownEvent, _window, cx| {
                        this.set_active_tab(tab_idx, cx);
                    }))
                    .child(
                        div()
                            .text_color(if is_act { rgb(0xffffff) } else { rgb(0x969696) })
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
                                .text_color(if is_mod { rgb(0xff5555) } else { rgb(0xcccccc) })
                                .hover(|style| style.bg(rgb(0x454545)).text_color(rgb(0xffffff)))
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