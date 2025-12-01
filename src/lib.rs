use clap::ArgMatches;
use regex::{Regex, RegexBuilder};
use std::error::Error;
use std::fs;

pub struct Config {
    pattern: String,
    files: Vec<String>,
    ignore_case: bool,
    invert_match: bool,
}

impl Config {
    pub fn from_args(args: ArgMatches) -> Self {
        let pattern = args
            .get_one::<String>("pattern")
            .expect("--pattern is a required argument")
            .clone();
        let files = args
            .get_many::<String>("file")
            .expect("--file is a required argument")
            .cloned()
            .collect();

        let ignore_case = !args.get_flag("no-ignore-case") && args.get_flag("ignore-case");
        let invert_match = !args.get_flag("no-invert-match") && args.get_flag("invert-match");

        Self {
            pattern,
            files,
            ignore_case,
            invert_match,
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let pattern = RegexBuilder::new(&config.pattern)
        .case_insensitive(config.ignore_case)
        .build()?;

    for file in &config.files {
        let file_contents = fs::read_to_string(file)?; // TODO: Specify the file that gave an error
        let file_name = if config.files.len() > 1 {
            format!("{file} ")
        } else {
            "".to_string()
        };
        for line in search(&pattern, &file_contents, config.invert_match) {
            println!("{file_name}{line}");
        }
    }

    Ok(())
}

pub fn search<'a>(pattern: &'a Regex, contents: &'a str, invert_match: bool) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| {
            if invert_match {
                !pattern.is_match(line)
            } else {
                pattern.is_match(line)
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive_no_invert() {
        let pattern = Regex::new(r"treasure").unwrap();
        let contents = "\
        I am not obsessed with treasure.\n\
        Not all Treasure is silver and gold, mate.";
        assert_eq!(
            vec!["I am not obsessed with treasure."],
            search(&pattern, contents, false)
        )
    }

    #[test]
    fn case_insensitive_no_invert() {
        let pattern = RegexBuilder::new(r"nOtHiNg")
            .case_insensitive(true)
            .build()
            .unwrap();
        let contents = "\
        For too long I've been parched of thirst and unable to quench it.\n\
        Too long I've been starving to death and haven't died.\n\
        I feel nothing.\n\
        Not the wind on my face nor the spray of the sea.";
        assert_eq!(vec!["I feel nothing."], search(&pattern, contents, false))
    }

    #[test]
    fn case_sensitive_invert() {
        let pattern = Regex::new(r"bRiLlIaNcE").unwrap();
        let contents = "\
        This is either madness... or brilliance.\n\
        It's remarkable how often those two traits coincide.";
        assert_eq!(
            vec![
                "This is either madness... or brilliance.",
                "It's remarkable how often those two traits coincide."
            ],
            search(&pattern, contents, true)
        )
    }

    #[test]
    fn case_insensitive_invert() {
        let pattern = RegexBuilder::new(r"yOu")
            .case_insensitive(true)
            .build()
            .unwrap();
        let contents = "\
        You didn't beat me.\n\
        You ignored the rules of engagement.\n\
        In a fair fight, I'd kill you.\n\
        That's not much incentive for me to fight fair, then, is it?";
        assert_eq!(
            vec!["That's not much incentive for me to fight fair, then, is it?"],
            search(&pattern, contents, true)
        )
    }
}