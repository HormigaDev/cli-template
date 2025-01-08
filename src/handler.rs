use crate::commands;
use clap::ArgMatches;
use colored::*;

pub fn execute(matches: ArgMatches) {
    match matches.subcommand() {
        Some(("example", sub_matches)) => {
            //Your code here
        }
        _ => {
            eprintln!("{}", "Unknown or unsupported command".red());
            std::process::exit(1);
        }
    }
}
