use crate::model::{LogEntry, LogLevel};
pub fn parse_line(line: &str) -> Result<LogEntry, String> {
    let v: Vec<&str> = line.split(' ').collect();

    println!("byorn {},{},{}", v[0], v[1], v[2]);

    return Ok(LogEntry {
        level: LogLevel::Critical,
        user: "csfs".to_string(),
        message: "sfsdd".to_string(),
    });
}
