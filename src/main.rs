use std::{env, process};

use gui::{load_document_from_path, page_nodes, validate_document};

fn main() {
    let mut args = env::args();
    let program = args.next().unwrap_or_else(|| "gui".to_string());
    let Some(command) = args.next() else {
        print_usage(&program);
        process::exit(2);
    };
    if command != "check" && command != "pages" {
        eprintln!("unknown command: {command}");
        print_usage(&program);
        process::exit(2);
    }
    let paths = args.collect::<Vec<_>>();
    if paths.is_empty() {
        print_usage(&program);
        process::exit(2);
    }

    match command.as_str() {
        "check" => {
            let mut failed = false;
            for path in &paths {
                match load_document_from_path(path) {
                    Ok(doc) => {
                        if let Err(errors) = validate_document(&doc) {
                            failed = true;
                            for err in errors {
                                eprintln!("{path}: validation error: {}", err.message);
                            }
                            continue;
                        }
                        println!("ok: {path}");
                    }
                    Err(err) => {
                        failed = true;
                        eprintln!("{path}: syntax error: {}", err.message);
                    }
                }
            }
            if failed {
                process::exit(1);
            }
        }
        "pages" => {
            if paths.len() != 1 {
                eprintln!("usage: {program} pages <file.gui>");
                process::exit(2);
            }
            let path = &paths[0];
            let doc = match load_document_from_path(path) {
                Ok(doc) => doc,
                Err(err) => {
                    eprintln!("{path}: syntax error: {}", err.message);
                    process::exit(1);
                }
            };
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

fn print_usage(program: &str) {
    eprintln!("usage: {program} check <file.gui> [more.gui ...]");
    eprintln!("       {program} pages <file.gui>");
}
