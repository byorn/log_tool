mod cli;
mod file_reader;
mod model;
mod parser;

use cli::Command;
use file_reader::read_file;
use model::LogEntry;
use parser::parse_line;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

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

    match commandParsed {
        Command::Stats => {
            let logentries: Vec<LogEntry> = contents
                .lines()
                .map(|line| parse_line(line).unwrap())
                .collect();

            for x in logentries {
                println!("{:?}", x);
            }
        }
        Command::Errors => {}
        Command::TopUsers => {}
    }

    for arg in args {
        println!("{}", arg);
    }
    println!("Hello, world!");
}
