use clap::builder::FalseyValueParser;
use clap::{command, Arg, ArgAction, ArgMatches};
use grep_rs::Config;
use std::{env, process};

fn main() {
    let args = read_args();
    let config = Config::from_args(args);
    if let Err(err) = grep_rs::run(config) {
        eprintln!("Error while searching: {err}");
        process::exit(1);
    }
}

fn read_args() -> ArgMatches {
    command!()
        .about("Finds and returns matching lines for regex patterns in files")
        .arg(
            Arg::new("pattern")
                .required(true)
                .help("The regex pattern to search for"),
        )
        .arg(
            Arg::new("file")
                .required(true)
                .num_args(1..)
                .action(ArgAction::Append)
                .help("The file(s) to search within"),
        )
        .arg(
            Arg::new("ignore-case")
                .short('i')
                .long("ignore-case")
                .env("IGNORE_CASE")
                .num_args(0)
                .action(ArgAction::SetTrue)
                .value_parser(FalseyValueParser::new())
                .help("Ignore character casing"),
        )
        .arg(
            Arg::new("no-ignore-case")
                .long("no-ignore-case")
                .num_args(0)
                .action(ArgAction::SetTrue)
                .value_parser(FalseyValueParser::new())
                .conflicts_with("ignore-case")
                .help("Do not ignore character casing"),
        )
        .arg(
            Arg::new("invert-match")
                .short('v')
                .long("invert-match")
                .env("INVERT_MATCH")
                .num_args(0)
                .action(ArgAction::SetTrue)
                .value_parser(FalseyValueParser::new())
                .help("Return non-matching lines instead"),
        )
        .arg(
            Arg::new("no-invert-match")
                .long("no-invert-match")
                .num_args(0)
                .action(ArgAction::SetTrue)
                .value_parser(FalseyValueParser::new())
                .conflicts_with("invert-match")
                .help("Do not return non-matching lines"),
        )
        .get_matches()
}