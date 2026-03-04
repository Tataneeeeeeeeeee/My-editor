mod window;
mod editor;
mod assets;

use crate::window::window_render::AppState;
use crate::assets::asset::Assets;
use gpui::*;
use std::path::PathBuf;

fn main() {
    let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets");

    Application::new()
        .with_assets(Assets { base: assets_dir })
        .run(|cx: &mut App| {
            cx.set_global(AppState::new());

            let _window = cx.update_global::<AppState, _>(|state, cx| {
                state.create_editor_window("Untitled".to_string(), cx)
            });

            cx.activate(true);
        });
}
