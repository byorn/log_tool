#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
    Critical,
}

impl LogLevel {
    pub fn from_str(str: &str) -> Result<LogLevel, String> {
        match str {
            "INFO" => Ok(LogLevel::Info),
            "WARN" => Ok(LogLevel::Warn),
            "ERROR" => Ok(LogLevel::Error),
            "DEBUG" => Ok(LogLevel::Debug),
            _ => Err("Couldnt match a log level from the file".to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub level: LogLevel,
    pub user: String,
    pub message: String,
}
