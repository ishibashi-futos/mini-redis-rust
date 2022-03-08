use quickreplace::*;
use std::fs;
use text_colorizer::*;

fn main() {
    let args = parse_args();

    let data = match fs::read_to_string(&args.filename) {
        Ok(text) => text,
        Err(e) => {
            eprintln!("{} failed to read from file '{}': {:?}", "Error:".red().bold(), args.filename, e);
            std::process::exit(1);
        }
    };

    let data = match replace(&args.target, &args.replacement, &data) {
        Ok(text) => text,
        Err(e) => {
            eprintln!("{} failed to replace text: {:?}", "Error:".red().bold(), e);
            std::process::exit(1);
        }
    };

    match fs::write(&args.output, &data) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("{} failed to write to file '{}': {:?}", "Error:".red().bold(), args.filename, e);
            std::process::exit(1);
        }
    }
}
