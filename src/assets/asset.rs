use std::path::PathBuf;
use std::borrow::Cow;
use gpui::*;

pub struct Assets {
    pub base: PathBuf,
}

impl AssetSource for Assets {
    fn load(&self, path: &str) -> anyhow::Result<Option<Cow<'static, [u8]>>> {
        std::fs::read(self.base.join(path))
            .map(|data| Some(Cow::Owned(data)))
            .map_err(|e| e.into())
    }

    fn list(&self, path: &str) -> anyhow::Result<Vec<SharedString>> {
        std::fs::read_dir(self.base.join(path))
            .map(|entries| {
                entries
                    .filter_map(|entry| {
                        entry.ok()
                            .and_then(|e| e.file_name().into_string().ok())
                            .map(SharedString::from)
                    })
                    .collect()
            })
            .map_err(|e| e.into())
    }
}
