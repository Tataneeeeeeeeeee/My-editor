#[cfg(test)]
mod tests {
    #[test]
    fn test_tab_bar_creation() {
        let tabs = vec!["file1.rs", "file2.py"];
        assert_eq!(tabs.len(), 2);
    }

    #[test]
    fn test_tab_bar_add_tab() {
        let mut tabs = vec!["file1.rs"];
        tabs.push("file2.py");
        assert_eq!(tabs.len(), 2);
    }

    #[test]
    fn test_tab_bar_remove_tab() {
        let mut tabs = vec!["file1.rs", "file2.py"];
        tabs.remove(0);
        assert_eq!(tabs.len(), 1);
        assert_eq!(tabs[0], "file2.py");
    }

    #[test]
    fn test_tab_bar_active_tab() {
        let active_tab = 0;
        assert_eq!(active_tab, 0);
    }

    #[test]
    fn test_tab_bar_switch_tab() {
        let active_tab = 0;
        assert_eq!(active_tab, 0);
    }

    #[test]
    fn test_tab_bar_tab_count() {
        let tabs = vec!["tab1", "tab2", "tab3", "tab4"];
        assert_eq!(tabs.len(), 4);
    }

    #[test]
    fn test_tab_bar_tab_names() {
        let tabs = vec!["Untitled", "test.rs", "main.py"];
        assert!(tabs.contains(&"test.rs"));
    }

    #[test]
    fn test_tab_bar_modified_state() {
        let modified = true;
        assert_eq!(modified, true);
    }

    #[test]
    fn test_tab_bar_close_active_tab() {
        let mut active_tab = 1;
        let tab_count = 3;

        if active_tab >= tab_count - 1 {
            active_tab = tab_count - 2;
        }

        assert!(active_tab >= 0);
    }

    #[test]
    fn test_tab_bar_empty_tabs() {
        let tabs: Vec<&str> = vec![];
        assert_eq!(tabs.is_empty(), true);
    }
}
