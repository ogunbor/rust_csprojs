use regex::Regex;
use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    // Detect and print the type of blockchain address
    let evm_regex = Regex::new(r"^0x[a-fA-F0-9]{40}$").unwrap();
    let solana_regex = Regex::new(r"^[1-9A-HJ-NP-Za-km-z]{44}$").unwrap();

    if evm_regex.is_match(&config.query) {
        println!("✅ The provided address is an **EVM address**.");
    } else if solana_regex.is_match(&config.query) {
        println!("✅ The provided address is a **Solana address**.");
    } else {
        println!("❌ Invalid blockchain address! Please enter a valid EVM or Solana address.");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}
