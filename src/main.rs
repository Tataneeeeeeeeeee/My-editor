mod window;
mod editor;
mod assets;
mod settings;

use crate::window::window_render::AppState;
use crate::assets::asset::Assets;
use crate::settings::settings::load_settings;
use gpui::*;
use std::path::PathBuf;
use std::env;

fn main() {
    let settings_global = load_settings().expect("Failed to load settings");
    let assets_dir = PathBuf::from(
        settings_global.get(vec!["assets", "path"])
            .expect("Failed to get assets path setting")
    );
    let args: Vec<String> = env::args().collect();

    let root_dir = if args.len() > 1 {
        PathBuf::from(&args[1])
    } else {
        env::current_dir().unwrap()
    };

    Application::new()
        .with_assets(Assets { base: assets_dir })
        .run(move |cx: &mut App| {
            // Set settings as a global
            cx.set_global(settings_global.clone());
            
            cx.set_global(AppState::new());

            let _window = cx.update_global::<AppState, _>(|state, cx| {
                state.create_editor_window("Untitled".to_string(), cx, root_dir.clone())
            });

            cx.activate(true);
        });
}
