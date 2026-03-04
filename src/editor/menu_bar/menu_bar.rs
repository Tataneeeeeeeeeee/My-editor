use gpui::*;

#[derive(Clone, Copy, PartialEq)]
pub enum MenuAction {
    NewFile,
    OpenFile,
    SaveFile,
}

pub struct MenuBar {
    pub file_menu_open: bool,
}

impl MenuBar {
    pub fn new() -> Self {
        Self {
            file_menu_open: false,
        }
    }

    pub fn render(&self, _file_menu_open: bool, cx: &mut Context<crate::editor::editor_window::EditorWindow>) -> impl IntoElement {
        div()
            .h(px(30.0))
            .w_full()
            .bg(rgb(0x2d2d30))
            .flex()
            .flex_row()
            .items_center()
            .px_2()
            .border_b_1()
            .border_color(rgb(0x1e1e1e))
            .child(
                // Menu "File"
                div()
                    .px_3()
                    .py_1()
                    .text_color(rgb(0xcccccc))
                    .hover(|style| style.bg(rgb(0x3e3e42)))
                    .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event: &MouseDownEvent, _window, cx| {
                        this.menu_bar.file_menu_open = !this.menu_bar.file_menu_open;
                        cx.notify();
                    }))
                    .child("File")
            )
    }
}
