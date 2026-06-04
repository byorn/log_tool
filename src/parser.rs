use crate::model::{LogEntry, LogLevel};

pub fn parse_line(line: &str) -> Result<LogEntry, String> {
    let level = line
        .split(' ')
        .nth(1)
        .unwrap_or("")
        .parse::<LogLevel>()
        .unwrap_or(LogLevel::Critical);
    let user = find_user_in_line(line)
        .trim_start_matches("user_id=")
        .to_string();

    Ok(LogEntry {
        level,
        user,
        message: line.to_string(),
    })
}

fn find_user_in_line(line: &str) -> &str {
    line.split(' ')
        .find(|token| token.starts_with("user_id="))
        .unwrap_or("")
}
