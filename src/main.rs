mod assets;
mod editor;
mod run;
mod settings;
mod window;

use crate::editor::log::log_info;
use crate::run::run_editor;

fn main() {
    log_info("App starting...");
    run_editor(None);
    log_info("App finished.");
}
