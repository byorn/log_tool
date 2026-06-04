use crate::model::{LogEntry, LogLevel};
pub fn parse_line(line: &str) -> Result<LogEntry, String> {
    let v: Vec<&str> = line.split(' ').collect();

    return Ok(LogEntry {
        level: LogLevel::from_str(v[1]).unwrap_or_else(|_| LogLevel::Critical),
        user: find_user_in_line(v)
            .unwrap_or("")
            .to_string()
            .strip_prefix("user_id=")
            .unwrap_or("")
            .to_string(),
        message: line.to_string(),
    });
}

fn find_user_in_line(v: Vec<&str>) -> Option<&str> {
    v.into_iter().find(|v| v.starts_with("user_id="))
}
