Log Analyzer CLI — Step-by-Step Guide
🧭 Goal
Build a CLI tool:
logtool stats app.log
logtool errors app.log
logtool top-users app.log

🏗️ Step 0 — Project Setup
cargo new logtool
cd logtool
Suggested structure (refactor into this later):
src/
main.rs
cli.rs
parser.rs
model.rs
analyzer.rs

🧩 Step 1 — Read CLI Arguments
Goal
cargo run -- stats app.log
Tasks
• Use std::env::args
• Extract:
o command
o file path

📂 Step 2 — Read File
Tasks
• Use std::fs::read_to_string
• Avoid unwrap()
Example:
let content = std::fs::read_to_string(file_path)?;

🧱 Step 3 — Define Model
Create model.rs #[derive(Debug, PartialEq)]
pub enum LogLevel {
Info,
Warn,
Error,
}

pub struct LogEntry {
pub level: LogLevel,
pub user: String,
pub action: String,
}

🔍 Step 4 — Parse One Line
Create parser.rs
Goal
Convert:
INFO user=byorn action=login
into LogEntry
Tasks
• Split by whitespace
• Extract level, user, action
Function:
pub fn parse_line(line: &str) -> Result<LogEntry, String>

🔁 Step 5 — Parse Entire File
let entries: Vec<LogEntry> = content
.lines()
.filter_map(|line| parse_line(line).ok())
.collect();

📊 Step 6 — Implement stats
Output
INFO: 10
WARN: 3
ERROR: 2
Tasks
• Count occurrences of each LogLevel

🚨 Step 7 — Implement errors
Tasks
entries.iter()
.filter(|e| e.level == LogLevel::Error)

👤 Step 8 — Implement top-users
Tasks
Use:
HashMap<String, usize>
Pattern:
\*map.entry(user).or_insert(0) += 1;

🧪 Step 9 — Add Tests #[cfg(test)]
mod tests {
use super::\*;

    #[test]
    fn parses_valid_line() {
        let line = "INFO user=byorn action=login";
        let result = parse_line(line).unwrap();
        assert_eq!(result.user, "byorn");
    }

}

🧠 Step 10 — Refactor
Refactor 1 — Reduce Cloning
• Replace String with &str where possible
Refactor 2 — Add Lifetimes
pub struct LogEntry<'a> {
pub level: LogLevel,
pub user: &'a str,
pub action: &'a str,
}
Refactor 3 — Better Error Type
enum ParseError {
InvalidFormat,
MissingField,
}

🧩 Step 11 — CLI Structure
Create enum:
enum Command {
Stats,
Errors,
TopUsers,
}

🚀 Step 12 — Polish
• Format output nicely
• Sort top users
• Ignore empty lines
• Handle malformed logs

🧭 Suggested Timeline
Day 1
• CLI args + file reading
Day 2
• Parser + model
Day 3
• stats + errors
Day 4
• top-users
Day 5
• tests + refactor

💡 Key Advice

1. Get it working first
2. Then refactor:
   o reduce clones
   o improve types
   o clean modules

🚀 Next Step
Start with Step 1 and move forward incrementally. Refactor only after you have a working version.
