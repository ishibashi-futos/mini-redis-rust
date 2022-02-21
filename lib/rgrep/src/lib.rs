use std::error::Error;
#[allow(unused_imports)]
use std::io::prelude::*;
#[allow(unused_imports)]
use std::{env, result};
use std::{fs::File, io::Read};

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: std::slice::Iter<std::string::String>) -> Result<Config, &str> {
        // 1つめの文字列はプログラム名なので無視
        args.next();
        let query = match args.next() {
            Some(arg) => arg.to_string(),
            None => return Err("Didn't get a query string"),
        };
        let filename = match args.next() {
            Some(arg) => arg.to_string(),
            None => return Err("Didn't get a filename string"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for (i, line) in results.iter().enumerate() {
        println!("{}: {}", i+1, line);
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
    let mut results: Vec<&'a str> = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(&line);
        }
    }

    results
}
