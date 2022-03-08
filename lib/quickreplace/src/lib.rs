use text_colorizer::*;

#[derive(Debug, PartialEq)]
pub struct Arguments {
    pub target: String,
    pub replacement: String,
    pub filename: String,
    pub output: String,
}

pub fn print_usage() {
    eprintln!(
        "{} - change occurrences of one string into another",
        "quickreplace".green()
    );
    eprintln!("Usage: quickreplace <target> <replacement> <INPUT> <OUTPUT>");
}

pub fn parse_args(args: Vec<String>) -> Result<Arguments, usize> {
    if args.len() != 4 {
        print_usage();
        return Err(args.len());
    }

    Ok(Arguments {
        target: args[0].clone(),
        replacement: args[1].clone(),
        filename: args[2].clone(),
        output: args[3].clone(),
    })
}

use regex::Regex;
pub fn replace(target: &str, replacement: &str, text: &str) -> Result<String, regex::Error> {
    let reg = Regex::new(target)?;
    Ok(reg.replace_all(text, replacement).to_string())
}
