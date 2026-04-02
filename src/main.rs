use std::{
    env, fs,
    path::{Path, PathBuf},
    process,
};

use gui::{load_documents_from_paths, page_nodes, validate_document};

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
    let paths = match resolve_input_paths(paths) {
        Ok(paths) => paths,
        Err(message) => {
            eprintln!("{message}");
            process::exit(1);
        }
    };

    match command.as_str() {
        "check" => {
            let doc = match load_documents_from_paths(paths.iter()) {
                Ok(doc) => doc,
                Err(err) => {
                    eprintln!("syntax error: {}", err.message);
                    process::exit(1);
                }
            };
            if let Err(errors) = validate_document(&doc) {
                for err in errors {
                    eprintln!("validation error: {}", err.message);
                }
                process::exit(1);
            }
            println!("ok");
        }
        "pages" => {
            let doc = match load_documents_from_paths(paths.iter()) {
                Ok(doc) => doc,
                Err(err) => {
                    eprintln!("syntax error: {}", err.message);
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
    eprintln!("       {program} pages [file.gui ...]");
}

fn resolve_input_paths(paths: Vec<String>) -> Result<Vec<PathBuf>, String> {
    if paths.is_empty() {
        let mut discovered = Vec::new();
        collect_gui_files(Path::new("."), &mut discovered).map_err(|err| err.to_string())?;
        discovered.sort();
        if discovered.is_empty() {
            return Err("no .gui files found under current directory".to_string());
        }
        return Ok(discovered);
    }

    Ok(paths.into_iter().map(PathBuf::from).collect())
}

fn collect_gui_files(dir: &Path, out: &mut Vec<PathBuf>) -> Result<(), std::io::Error> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            collect_gui_files(&path, out)?;
            continue;
        }
        if file_type.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("gui") {
            out.push(path);
        }
    }
    Ok(())
}
