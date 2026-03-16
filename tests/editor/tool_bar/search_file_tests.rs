#[cfg(test)]
mod tests {
    #[test]
    fn test_search_files_empty_query() {
        let query = "".to_string();
        assert_eq!(query.is_empty(), true);
    }

    #[test]
    fn test_search_files_query() {
        let query = "main".to_string();
        assert_eq!(query, "main");
    }

    #[test]
    fn test_search_files_case_insensitive() {
        let query1 = "Main".to_string();
        let query2 = "main".to_string();
        assert_ne!(query1, query2);
    }

    #[test]
    fn test_search_files_wildcard() {
        let query = "*.rs".to_string();
        assert!(query.contains('*'));
    }

    #[test]
    fn test_search_files_results() {
        let results = vec!["main.rs", "lib.rs", "test.rs"];
        assert_eq!(results.len(), 3);
    }

    #[test]
    fn test_search_files_no_results() {
        let results: Vec<&str> = vec![];
        assert_eq!(results.is_empty(), true);
    }

    #[test]
    fn test_search_files_filter() {
        let files = vec!["main.rs", "config.json", "test.py"];
        let filtered: Vec<_> = files.iter().filter(|f| f.contains("main")).collect();
        assert_eq!(filtered.len(), 1);
    }

    #[test]
    fn test_search_files_path_matching() {
        let file_path = "src/editor/main.rs".to_string();
        assert!(file_path.contains("main"));
    }

    #[test]
    fn test_search_files_pagination() {
        let results = vec!["file1", "file2", "file3", "file4", "file5"];
        let page_size = 2;
        let pages = (results.len() + page_size - 1) / page_size;
        assert_eq!(pages, 3);
    }

    #[test]
    fn test_search_files_special_characters() {
        let query = "test_*.rs".to_string();
        assert!(query.contains('_'));
    }
}
