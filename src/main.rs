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
    let args: Vec<String> = env::args().collect();
    //
    if args.len() < 3 {
        println!("Usage: logtoot <command> <filename>");
        return;
    }

    let command = &args[1];

    let commandParsed = Command::from_str(command).unwrap();

    let filename = &args[2];

    let contents = read_file(filename).unwrap_or_else(|_| {
        println!("Error opening file");
        process::exit(1);
    });

    let logentries: Vec<LogEntry> = contents
        .lines()
        .map(|line| parse_line(line).unwrap())
        .collect();

    match commandParsed {
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
            let (user_stats, log_stats) = count_entries_for_stats(logentries.clone());
            for (k, v) in log_stats {
                println!("{:?}:{}", k, v);
            }

            println!("Unique users: {}", user_stats.len() - 1);
            let items: Vec<(&String, &i16)> = user_stats.iter().collect();

            let mut items_filtered: Vec<(&String, &i16)> = items
                .into_iter()
                .filter(|x| *x.0 != "".to_string())
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
            let (user_stats, _) = count_entries_for_stats(logentries.clone());

            let mut user_stats_vec: Vec<(&String, &i16)> = user_stats.iter().collect();

            user_stats_vec.sort_by(|a, b| b.1.cmp(a.1));
            for (k, v) in user_stats_vec {
                println!("{}:{}", k, v);
            }
        }
    }

    println!("########## End ############");
}

fn count_entries_for_stats(
    log_entries: Vec<LogEntry>,
) -> (HashMap<String, i16>, HashMap<LogLevel, i16>) {
    let mut hm = HashMap::new();
    let mut count = HashMap::new();
    for l in log_entries {
        hm.entry(l.level).and_modify(|v| *v += 1).or_insert(1);

        count.entry(l.user).and_modify(|v| *v += 1).or_insert(1);
    }
    (count, hm)
}
