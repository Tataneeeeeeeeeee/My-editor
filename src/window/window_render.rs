use gpui::*;

pub struct AppState {
    pub next_id: usize,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            next_id: 0,
        }
    }

    pub fn create_editor_window(&mut self, name: String, cx: &mut App) -> WindowHandle<crate::editor::editor_window::EditorWindow> {
        let id = self.next_id;
        self.next_id += 1;

        let window_handle = cx.open_window(WindowOptions::default(), |_, cx| {
            cx.new(|cx| crate::editor::editor_window::EditorWindow::new(id, name.clone(), cx))
        }).unwrap();

        window_handle
    }
}

impl Global for AppState {}