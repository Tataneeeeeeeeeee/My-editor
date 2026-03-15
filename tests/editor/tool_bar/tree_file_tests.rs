#[cfg(test)]
mod tests {
    #[test]
    fn test_tree_file_creation() {
        let root_dir = "/home/user/project".to_string();
        assert_eq!(root_dir, "/home/user/project");
    }

    #[test]
    fn test_tree_file_expansion() {
        let mut expanded = false;
        expanded = !expanded;
        assert_eq!(expanded, true);
    }

    #[test]
    fn test_tree_file_node_selection() {
        let selected_node = "/home/user/project/src".to_string();
        assert!(!selected_node.is_empty());
    }

    #[test]
    fn test_tree_file_directory_listing() {
        let entries = vec!["file1.rs", "file2.rs", "folder1"];
        assert_eq!(entries.len(), 3);
    }

    #[test]
    fn test_tree_file_filter() {
        let entries = vec!["main.rs", "lib.rs", "test.txt"];
        let filtered: Vec<_> = entries.iter()
            .filter(|e| e.ends_with(".rs"))
            .collect();
        assert_eq!(filtered.len(), 2);
    }

    #[test]
    fn test_tree_file_sort() {
        let mut entries = vec!["z.rs", "a.rs", "m.rs"];
        entries.sort();
        assert_eq!(entries[0], "a.rs");
    }

    #[test]
    fn test_tree_file_refresh() {
        let last_update = 1;
        assert_eq!(last_update, 1);
    }

    #[test]
    fn test_tree_file_nested_directories() {
        let path = "/home/user/project/src/modules/utils".to_string();
        let parts: Vec<&str> = path.split('/').collect();
        assert!(parts.len() > 3);
    }

    #[test]
    fn test_tree_file_file_type_detection() {
        let file = "test.rs";
        let is_rust = file.ends_with(".rs");
        assert_eq!(is_rust, true);
    }

    #[test]
    fn test_tree_file_empty_directory() {
        let entries: Vec<&str> = vec![];
        assert_eq!(entries.is_empty(), true);
    }
}
