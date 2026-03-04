use gpui::*;

/// Renders the file dropdown menu overlay
pub fn render_dropdown(
    on_new_file: impl Fn(&MouseDownEvent, &mut Window, &mut App) + 'static,
    on_open_file: impl Fn(&MouseDownEvent, &mut Window, &mut App) + 'static,
    on_save_file: impl Fn(&MouseDownEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    div()
        .absolute()
        .top(px(30.0))
        .left(px(8.0))
        .w(px(200.0))
        .bg(rgb(0x252526))
        .border_1()
        .border_color(rgb(0x454545))
        .shadow_lg()
        .flex()
        .flex_col()
        .child(
            div()
                .px_4()
                .py_2()
                .text_color(rgb(0xcccccc))
                .hover(|style| style.bg(rgb(0x094771)))
                .on_mouse_down(MouseButton::Left, on_new_file)
                .child("New Text File")
        )
        .child(
            div()
                .px_4()
                .py_2()
                .text_color(rgb(0xcccccc))
                .hover(|style| style.bg(rgb(0x094771)))
                .on_mouse_down(MouseButton::Left, on_open_file)
                .child("Open File...")
        )
        .child(
            div()
                .px_4()
                .py_2()
                .text_color(rgb(0xcccccc))
                .hover(|style| style.bg(rgb(0x094771)))
                .on_mouse_down(MouseButton::Left, on_save_file)
                .child("Save File")
        )
}