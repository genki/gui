use std::{env, fs, process};

use gui::{page_nodes, parse_document, validate_document};

fn main() {
    let mut args = env::args();
    let program = args.next().unwrap_or_else(|| "gui".to_string());
    let Some(command) = args.next() else {
        eprintln!("usage: {program} <check|pages> <file.gui>");
        process::exit(2);
    };
    if command != "check" && command != "pages" {
        eprintln!("unknown command: {command}");
        eprintln!("usage: {program} <check|pages> <file.gui>");
        process::exit(2);
    }
    let Some(path) = args.next() else {
        eprintln!("usage: {program} <check|pages> <file.gui>");
        process::exit(2);
    };
    if args.next().is_some() {
        eprintln!("usage: {program} <check|pages> <file.gui>");
        process::exit(2);
    }

    let input = match fs::read_to_string(&path) {
        Ok(input) => input,
        Err(err) => {
            eprintln!("failed to read `{path}`: {err}");
            process::exit(1);
        }
    };

    let doc = match parse_document(&input) {
        Ok(doc) => doc,
        Err(err) => {
            eprintln!("syntax error: {}", err.message);
            process::exit(1);
        }
    };

    match command.as_str() {
        "check" => {
            if let Err(errors) = validate_document(&doc) {
                for err in errors {
                    eprintln!("validation error: {}", err.message);
                }
                process::exit(1);
            }

            println!("ok: {path}");
        }
        "pages" => {
            if let Err(errors) = validate_document(&doc) {
                for err in errors {
                    eprintln!("validation error: {}", err.message);
                }
                process::exit(1);
            }
            match page_nodes(&doc) {
                Ok(pages) => {
                    for page in pages {
                        println!("{page}");
                    }
                }
                Err(errors) => {
                    for err in errors {
                        eprintln!("validation error: {}", err.message);
                    }
                    process::exit(1);
                }
            }
        }
        _ => unreachable!(),
    }
}
