use rust_playground::util::str;
use std::{
    env,
    error::Error,
    fs,
    process::{self},
};

struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    fn build(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("miss query string"),
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("miss file path"),
        };
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let results: Vec<&str> = if config.ignore_case {
        str::search_case_insensitive(&config.query, &contents).collect()
    } else {
        str::search(&config.query, &contents).collect()
    };
    for line in results {
        println!("{line}");
    }
    Ok(())
}

// cargo run --bin minigrep -- admiring files/poem.txt
fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
