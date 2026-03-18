#[cfg(test)]
mod tests {
    use my_editor::editor::log::{Log, LogLevel, LogManager, log_info, log_warning, log_error};

    #[test]
    fn test_log_level_display() {
        assert_eq!(LogLevel::Info.to_string(), "[INFO]");
        assert_eq!(LogLevel::Warning.to_string(), "[WARNING]");
        assert_eq!(LogLevel::Error.to_string(), "[ERROR]");
    }

    #[test]
    fn test_log_new() {
        let log = Log::new(LogLevel::Info);
        assert_eq!(log.messages.len(), 0);
        assert!(matches!(log.level, LogLevel::Info));
    }

    #[test]
    fn test_log_add_message() {
        let mut log = Log::new(LogLevel::Warning);
        log.add_message("Test message".to_string());
        
        assert_eq!(log.messages.len(), 1);
        assert_eq!(log.messages[0], "Test message");
    }

    #[test]
    fn test_log_add_multiple_messages() {
        let mut log = Log::new(LogLevel::Error);
        log.add_message("Message 1".to_string());
        log.add_message("Message 2".to_string());
        log.add_message("Message 3".to_string());
        
        assert_eq!(log.messages.len(), 3);
        assert_eq!(log.messages[0], "Message 1");
        assert_eq!(log.messages[1], "Message 2");
        assert_eq!(log.messages[2], "Message 3");
    }

    #[test]
    fn test_log_manager_new() {
        let manager = LogManager::new();
        assert_eq!(manager.logs.len(), 0);
    }

    #[test]
    fn test_log_manager_add_log() {
        let mut manager = LogManager::new();
        manager.add_log("Test log message".to_string(), LogLevel::Info);
        
        assert_eq!(manager.logs.len(), 1);
        assert_eq!(manager.logs[0].messages.len(), 1);
        assert_eq!(manager.logs[0].messages[0], "Test log message");
    }

    #[test]
    fn test_log_manager_add_multiple_logs() {
        let mut manager = LogManager::new();
        manager.add_log("First log".to_string(), LogLevel::Info);
        manager.add_log("Second log".to_string(), LogLevel::Warning);
        manager.add_log("Third log".to_string(), LogLevel::Error);
        
        assert_eq!(manager.logs.len(), 3);
        assert!(matches!(manager.logs[0].level, LogLevel::Info));
        assert!(matches!(manager.logs[1].level, LogLevel::Warning));
        assert!(matches!(manager.logs[2].level, LogLevel::Error));
    }

    #[test]
    fn test_log_manager_default() {
        let manager = LogManager::default();
        assert_eq!(manager.logs.len(), 0);
    }

    #[test]
    fn test_log_info_function() {
        // This test verifies that log_info doesn't panic
        log_info("Test info message");
    }

    #[test]
    fn test_log_warning_function() {
        // This test verifies that log_warning doesn't panic
        log_warning("Test warning message");
    }

    #[test]
    fn test_log_error_function() {
        // This test verifies that log_error doesn't panic
        log_error("Test error message");
    }

    #[test]
    fn test_log_different_levels() {
        let log_info = Log::new(LogLevel::Info);
        let log_warning = Log::new(LogLevel::Warning);
        let log_error = Log::new(LogLevel::Error);
        
        assert!(matches!(log_info.level, LogLevel::Info));
        assert!(matches!(log_warning.level, LogLevel::Warning));
        assert!(matches!(log_error.level, LogLevel::Error));
    }

    #[test]
    fn test_log_manager_with_different_levels() {
        let mut manager = LogManager::new();
        
        manager.add_log("Info message".to_string(), LogLevel::Info);
        manager.add_log("Warning message".to_string(), LogLevel::Warning);
        manager.add_log("Error message".to_string(), LogLevel::Error);
        
        assert_eq!(manager.logs.len(), 3);
        assert!(matches!(manager.logs[0].level, LogLevel::Info));
        assert!(matches!(manager.logs[1].level, LogLevel::Warning));
        assert!(matches!(manager.logs[2].level, LogLevel::Error));
    }

    #[test]
    fn test_log_empty_message() {
        let mut log = Log::new(LogLevel::Info);
        log.add_message(String::new());
        
        assert_eq!(log.messages.len(), 1);
        assert_eq!(log.messages[0], "");
    }

    #[test]
    fn test_log_manager_log_order() {
        let mut manager = LogManager::new();
        
        for i in 0..5 {
            manager.add_log(format!("Message {}", i), LogLevel::Info);
        }
        
        for i in 0..5 {
            assert_eq!(manager.logs[i].messages[0], format!("Message {}", i));
        }
    }
}
