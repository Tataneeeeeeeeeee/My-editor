mod window;
mod editor;

use crate::window::window_render::AppState;
use gpui::*;

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.set_global(AppState::new());
        
        let _window = cx.update_global::<AppState, _>(|state, cx| {
            state.create_editor_window("Untitled".to_string(), cx)
        });
        
        cx.activate(true);
    });
}
