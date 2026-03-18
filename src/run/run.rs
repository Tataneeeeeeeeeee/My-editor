use crate::assets::asset::Assets;
use crate::settings::settings::SettingsGlobal;
use crate::settings::settings::load_settings;
use crate::window::window_render::AppState;
use fork::{Fork, daemon};
use gpui::*;
use std::env;
use std::path::PathBuf;

fn run_app(settings_global: SettingsGlobal, assets_dir: PathBuf, root_dir: PathBuf) {
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

pub fn run_editor(path: Option<String>) {
    let settings_global = load_settings().expect("Failed to load settings");
    let assets_dir = PathBuf::from(
        settings_global
            .get(vec!["assets", "path"])
            .expect("Failed to get assets path setting"),
    );
    let args: Vec<String> = env::args().collect();

    let root_dir = if args.len() > 1 {
        PathBuf::from(&args[1])
    } else {
        env::current_dir().unwrap()
    };

    if let Ok(Fork::Child) = daemon(false, false) {
        // In the child process, run the application
        match path {
            Some(p) => {
                let root_dir = PathBuf::from(p);
                run_app(settings_global, assets_dir, root_dir);
            }
            None => {
                run_app(settings_global, assets_dir, root_dir);
            }
        }
    }
    // Parent process exits here, allowing the current window to close
    std::process::exit(0);
}
