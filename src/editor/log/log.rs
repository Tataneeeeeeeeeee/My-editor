use lazy_static::lazy_static;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;

pub enum LogLevel {
    Info,
    Warning,
    Error,
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Info => write!(f, "[INFO]"),
            LogLevel::Warning => write!(f, "[WARNING]"),
            LogLevel::Error => write!(f, "[ERROR]"),
        }
    }
}

pub struct Log {
    pub messages: Vec<String>,
    pub level: LogLevel,
}

pub struct LogManager {
    pub logs: Vec<Log>,
    pub log_file: Option<File>,
}

impl Log {
    pub fn new(level: LogLevel) -> Self {
        Self {
            messages: Vec::new(),
            level,
        }
    }

    pub fn add_message(&mut self, message: String) {
        self.messages.push(message);
    }

    pub fn write_to_file(&self, log_file: &mut File) {
        for message in &self.messages {
            let formatted = format!("{} {}\n", self.level, message);
            let _ = log_file.write_all(formatted.as_bytes());
        }
        let _ = log_file.flush();
    }
}

impl LogManager {
    pub fn new() -> Self {
        Self {
            logs: Vec::new(),
            log_file: load_log_file().ok(),
        }
    }

    pub fn add_log(&mut self, message: String, level: LogLevel) {
        let mut log = Log::new(level);
        log.add_message(message);

        if let Some(ref mut file) = self.log_file {
            log.write_to_file(file);
        }

        self.logs.push(log);
    }
}

impl Default for LogManager {
    fn default() -> Self {
        Self::new()
    }
}

pub fn load_log_file() -> Result<File, Box<dyn std::error::Error>> {
    let log_path = std::env::var("HOME").map(|home| {
        PathBuf::from(home)
            .join(".my-editor")
            .join("logs")
            .join("editor.log")
    })?;

    // Create directory if it doesn't exist
    if let Some(parent) = log_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // Open file in append mode, create if it doesn't exist
    Ok(std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)?)
}

// Global singleton LogManager
lazy_static! {
    pub static ref LOG_MANAGER: Mutex<LogManager> = Mutex::new(LogManager::new());
}

/// Convenience function to log from anywhere in the app
pub fn log(message: &str, level: LogLevel) {
    if let Ok(mut manager) = LOG_MANAGER.lock() {
        manager.add_log(message.to_string(), level);
    }
}

/// Log an info message
pub fn log_info(message: &str) {
    log(message, LogLevel::Info);
}

/// Log a warning message
pub fn log_warning(message: &str) {
    log(message, LogLevel::Warning);
}

/// Log an error message
pub fn log_error(message: &str) {
    log(message, LogLevel::Error);
}
