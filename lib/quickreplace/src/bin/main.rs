use quickreplace::*;
use std::env;
use std::fs;
use text_colorizer::*;

fn main() {
    let args = match parse_args(env::args().skip(1).collect()) {
        Ok(args) => args,
        Err(len) => {
            eprintln!(
                "{} wrong number of arguments: expected 4, got {}.",
                "Error:".red().bold(),
                len
            );
            std::process::exit(1);
        }
    };

    let data = match fs::read_to_string(&args.filename) {
        Ok(text) => text,
        Err(e) => {
            eprintln!(
                "{} failed to read from file '{}': {:?}",
                "Error:".red().bold(),
                args.filename,
                e
            );
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
        Ok(_) => {}
        Err(e) => {
            eprintln!(
                "{} failed to write to file '{}': {:?}",
                "Error:".red().bold(),
                args.filename,
                e
            );
            std::process::exit(1);
        }
    }
}
