#[cfg(test)]
mod tests {
    use std::borrow::Cow;
    use std::fs;
    use std::path::PathBuf;

    // Mock AssetSource pour tester
    struct MockAssets {
        base: PathBuf,
    }

    impl MockAssets {
        fn new(base: PathBuf) -> Self {
            Self { base }
        }

        fn load(&self, path: &str) -> anyhow::Result<Option<Cow<'static, [u8]>>> {
            std::fs::read(self.base.join(path))
                .map(|data| Some(Cow::Owned(data)))
                .map_err(|e| e.into())
        }

        fn list(&self, path: &str) -> anyhow::Result<Vec<String>> {
            std::fs::read_dir(self.base.join(path))
                .map(|entries| {
                    entries
                        .filter_map(|entry| {
                            entry.ok().and_then(|e| e.file_name().into_string().ok())
                        })
                        .collect()
                })
                .map_err(|e| e.into())
        }
    }

    #[test]
    fn test_asset_path_construction() {
        let base = PathBuf::from("/assets");
        let assets = MockAssets::new(base.clone());

        let full_path = assets.base.join("logo.png");
        assert_eq!(full_path, PathBuf::from("/assets/logo.png"));
    }

    #[test]
    fn test_asset_loading_nonexistent_file() {
        let base = PathBuf::from("/nonexistent");
        let assets = MockAssets::new(base);

        let result = assets.load("missing.png");
        assert!(result.is_err());
    }

    #[test]
    fn test_asset_directory_listing_nonexistent_dir() {
        let base = PathBuf::from("/nonexistent");
        let assets = MockAssets::new(base);

        let result = assets.list("icons");
        assert!(result.is_err());
    }

    #[test]
    fn test_asset_with_subdirectories() {
        let base = PathBuf::from("/assets");
        let assets = MockAssets::new(base);

        let icon_path = assets.base.join("icons").join("logo.png");
        let expected = PathBuf::from("/assets/icons/logo.png");
        assert_eq!(icon_path, expected);
    }

    #[test]
    fn test_asset_base_path_exists() {
        let temp_dir = std::env::temp_dir().join("my_editor_test_assets");
        let _ = fs::create_dir_all(&temp_dir);

        let assets = MockAssets::new(temp_dir.clone());
        assert_eq!(assets.base, temp_dir);

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_asset_multiple_files() {
        let temp_dir = std::env::temp_dir().join("my_editor_test_assets_multi");
        let _ = fs::create_dir_all(&temp_dir);

        // Create test files
        let _ = fs::write(temp_dir.join("file1.txt"), "content1");
        let _ = fs::write(temp_dir.join("file2.txt"), "content2");

        let assets = MockAssets::new(temp_dir.clone());
        let result = assets.list(".");

        assert!(result.is_ok());
        let files = result.unwrap();
        assert!(files.len() >= 2);

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_asset_empty_path() {
        let base = PathBuf::from(".");
        let assets = MockAssets::new(base.clone());

        assert_eq!(assets.base, base);
    }

    #[test]
    fn test_asset_special_characters_in_path() {
        let base = PathBuf::from("/assets");
        let assets = MockAssets::new(base);

        let path_with_space = assets.base.join("my file.png");
        assert_eq!(path_with_space.file_name().unwrap(), "my file.png");
    }
}
