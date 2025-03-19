use minigrep::Config;
use regex::Regex;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // Validate if query is an EVM or Solana address
    let evm_regex = Regex::new(r"^0x[a-fA-F0-9]{40}$").unwrap();
    let solana_regex = Regex::new(r"^[1-9A-HJ-NP-Za-km-z]{44}$").unwrap();

    if !evm_regex.is_match(&config.query) && !solana_regex.is_match(&config.query) {
        eprintln!("❌ Please input a valid EVM or Solana address.");
        process::exit(1);
    }

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
