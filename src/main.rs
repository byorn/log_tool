mod cli;
mod file_reader;
mod model;
mod parser;

use cli::Command;
use file_reader::read_file;
use model::LogEntry;
use parser::parse_line;
use std::{collections::HashMap, env, process};

use crate::model::LogLevel;

fn main() {
    let mut args = env::args().skip(1);

    let command_str = args.next().unwrap_or_else(|| {
        eprintln!("Usage: logtool <command> <filename>");
        process::exit(1);
    });
    let filename = args.next().unwrap_or_else(|| {
        eprintln!("Usage: logtool <command> <filename>");
        process::exit(1);
    });

    let command = command_str.parse::<Command>().unwrap_or_else(|e| {
        eprintln!("{e}");
        process::exit(1);
    });

    let contents = read_file(&filename).unwrap_or_else(|_| {
        eprintln!("Error opening file");
        process::exit(1);
    });

    let logentries: Vec<LogEntry> = contents
        .lines()
        .filter_map(|line| parse_line(line).ok())
        .collect();

    match command {
        Command::Stats => {
            let first = match logentries.first() {
                Some(l) => l,
                None => return,
            };
            let last = match logentries.last() {
                Some(l) => l,
                None => return,
            };
            println!("Total Entries: {}", logentries.len());
            let (user_stats, log_stats) = count_entries_for_stats(&logentries);
            for (k, v) in log_stats {
                println!("{:?}:{}", k, v);
            }

            println!("Unique users: {}", user_stats.len() - 1);
            let items: Vec<(&String, &u32)> = user_stats.iter().collect();

            let mut items_filtered: Vec<(&String, &u32)> = items
                .into_iter()
                .filter(|(user, _)| !user.is_empty())
                .collect();
            items_filtered.sort_by(|a, b| b.1.cmp(a.1));
            let top_user = match items_filtered.first() {
                Some(tu) => tu,
                None => return,
            };

            println!("Top user: {} = {} ", top_user.0, top_user.1);
            let first_timestamp = first.message.split_whitespace().next().unwrap();
            let last_timestamp = last.message.split_whitespace().next().unwrap();

            println!(" First timestamp {}", first_timestamp);
            println!(" Last timestamp {}", last_timestamp);
        }
        Command::Errors => {
            for x in logentries
                .iter()
                .filter(|log_entry| log_entry.level == LogLevel::Error)
            {
                println!("{:?} ", x);
            }
        }
        Command::TopUsers => {
            let (user_stats, _) = count_entries_for_stats(&logentries);

            let mut user_stats_vec: Vec<(&String, &u32)> = user_stats.iter().collect();

            user_stats_vec.sort_by(|a, b| b.1.cmp(a.1));
            for (k, v) in user_stats_vec {
                println!("{}:{}", k, v);
            }
        }
    }

    println!("########## End ############");
}

fn count_entries_for_stats(
    log_entries: &[LogEntry],
) -> (HashMap<String, u32>, HashMap<LogLevel, u32>) {
    let mut log_level_counts: HashMap<LogLevel, u32> = HashMap::new();
    let mut user_counts: HashMap<String, u32> = HashMap::new();
    for l in log_entries.iter() {
        log_level_counts
            .entry(l.level.clone())
            .and_modify(|v| *v += 1)
            .or_insert(1);
        user_counts
            .entry(l.user.clone())
            .and_modify(|v| *v += 1)
            .or_insert(1);
    }
    (user_counts, log_level_counts)
}
