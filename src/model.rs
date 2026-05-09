#[derive(Debug)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
    Critical,
}
#[derive(Debug)]
pub struct LogEntry {
    pub level: LogLevel,
    pub user: String,
    pub message: String,
}
