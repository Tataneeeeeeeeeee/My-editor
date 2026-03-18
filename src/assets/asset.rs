use gpui::*;
use std::borrow::Cow;
use std::path::PathBuf;

use crate::editor::log::log_info;

pub struct Assets {
    pub base: PathBuf,
}

impl AssetSource for Assets {
    fn load(&self, path: &str) -> anyhow::Result<Option<Cow<'static, [u8]>>> {
        log_info(format!("Loading asset: {}", path).as_str());

        std::fs::read(self.base.join(path))
            .map(|data| Some(Cow::Owned(data)))
            .map_err(|e| e.into())
    }

    fn list(&self, path: &str) -> anyhow::Result<Vec<SharedString>> {
        std::fs::read_dir(self.base.join(path))
            .map(|entries| {
                entries
                    .filter_map(|entry| {
                        entry
                            .ok()
                            .and_then(|e| e.file_name().into_string().ok())
                            .map(SharedString::from)
                    })
                    .collect()
            })
            .map_err(|e| e.into())
    }
}
