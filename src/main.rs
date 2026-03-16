mod run;
mod assets;
mod editor;
mod settings;
mod window;

use crate::run::run_editor;

fn main()
{
    run_editor(None);
}
