use std::{
    env, fs,
    path::{Path, PathBuf},
    process,
};

use abstract_gui::{
    compare_scan_inputs, compare_scan_inputs_with_config, load_documents_from_paths,
    load_scan_config_from_path, page_nodes, render_compare_report, render_document,
    render_scan_summary, scan_html_paths_with_stage, scan_html_paths_with_stage_and_config,
    validate_document, Document, ScanStage, TreeChild, TreeSection,
};

fn main() {
    let mut args = env::args();
    let program = args.next().unwrap_or_else(|| "gui".to_string());
    let Some(command) = args.next() else {
        print_usage(&program);
        process::exit(2);
    };
    if !matches!(
        command.as_str(),
        "check" | "page" | "drill" | "inherit" | "node" | "nav" | "scan" | "compare"
    ) {
        eprintln!("unknown command: {command}");
        print_usage(&program);
        process::exit(2);
    }
    let raw_args = args.collect::<Vec<_>>();

    let (scan_stage, config_path, raw_paths) = match command.as_str() {
        "scan" => match parse_scan_args(raw_args) {
            Ok(parsed) => parsed,
            Err(message) => {
                eprintln!("{message}");
                process::exit(2);
            }
        },
        "compare" => match parse_compare_args(raw_args) {
            Ok((config_path, paths)) => (ScanStage::Abstract, config_path, paths),
            Err(message) => {
                eprintln!("{message}");
                process::exit(2);
            }
        },
        _ => (ScanStage::Abstract, None, raw_args),
    };

    let config = match config_path {
        Some(path) => match load_scan_config_from_path(&path) {
            Ok(config) => Some(config),
            Err(err) => {
                eprintln!("config error: {}", err.message);
                process::exit(1);
            }
        },
        None => None,
    };

    let paths = match match command.as_str() {
        "scan" => resolve_scan_input_paths(raw_paths),
        "compare" => resolve_compare_input_paths(raw_paths),
        _ => resolve_input_paths(raw_paths),
    } {
        Ok(paths) => paths,
        Err(message) => {
            eprintln!("{message}");
            process::exit(1);
        }
    };

    match command.as_str() {
        "check" => {
            let _doc = load_and_validate(&paths);
            println!("ok");
        }
        "page" => {
            let doc = load_and_validate(&paths);
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
        "drill" => {
            let doc = load_and_validate(&paths);
            print_tree_section(&doc.drill);
        }
        "inherit" => {
            let doc = load_and_validate(&paths);
            print_tree_section(&doc.inherit);
        }
        "node" => {
            let doc = load_and_validate(&paths);
            for node_id in doc.node.keys() {
                println!("{node_id}");
            }
        }
        "nav" => {
            let doc = load_and_validate(&paths);
            for nav_id in doc.nav.keys() {
                println!("{nav_id}");
            }
        }
        "scan" => {
            let result = match config.as_ref() {
                Some(config) => scan_html_paths_with_stage_and_config(paths.iter(), scan_stage, config),
                None => scan_html_paths_with_stage(paths.iter(), scan_stage),
            };
            let result = match result {
                Ok(result) => result,
                Err(err) => {
                    eprintln!("scan error: {}", err.message);
                    process::exit(1);
                }
            };
            let doc = result.document;
            if let Err(errors) = validate_document(&doc) {
                for err in errors {
                    eprintln!("validation error: {}", err.message);
                }
                process::exit(1);
            }
            match scan_stage {
                ScanStage::Abstract => print!("{}", render_document(&doc)),
                ScanStage::Summary => print!("{}", render_scan_summary(&result.summary)),
            }
        }
        "compare" => {
            let report = match config.as_ref() {
                Some(config) => compare_scan_inputs_with_config(&paths[0], &paths[1], config),
                None => compare_scan_inputs(&paths[0], &paths[1]),
            };
            let report = match report {
                Ok(report) => report,
                Err(err) => {
                    eprintln!("compare error: {}", err.message);
                    process::exit(1);
                }
            };
            print!("{}", render_compare_report(&report));
        }
        _ => unreachable!(),
    }
}

fn print_usage(program: &str) {
    eprintln!("usage: {program} check <file.gui> [more.gui ...]");
    eprintln!("       {program} page [file.gui ...]");
    eprintln!("       {program} drill [file.gui ...]");
    eprintln!("       {program} inherit [file.gui ...]");
    eprintln!("       {program} node [file.gui ...]");
    eprintln!("       {program} nav [file.gui ...]");
    eprintln!("       {program} scan [--config config.yaml] <file.html> [more.html ...]");
    eprintln!("       {program} scan --stage summary [--config config.yaml] <file.html|snapshot.yaml> [...]");
    eprintln!("       {program} compare [--config config.yaml] <left.html|left.yaml> <right.html|right.yaml>");
}

fn load_and_validate(paths: &[PathBuf]) -> Document {
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
    doc
}

fn print_tree_section(section: &TreeSection) {
    for (root, children) in section {
        println!("{root}");
        print_tree_children(children, 1);
    }
}

fn print_tree_children(children: &[TreeChild], depth: usize) {
    for child in children {
        let indent = "  ".repeat(depth);
        match child {
            TreeChild::Leaf(id) => println!("{indent}{id}"),
            TreeChild::Branch(id, nested) => {
                println!("{indent}{id}");
                print_tree_children(nested, depth + 1);
            }
        }
    }
}

fn resolve_input_paths(paths: Vec<String>) -> Result<Vec<PathBuf>, String> {
    resolve_paths_by_extension(paths, &["gui"], "no .gui files found under current directory")
}

fn collect_files_with_extensions(
    dir: &Path,
    extensions: &[&str],
    out: &mut Vec<PathBuf>,
) -> Result<(), std::io::Error> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            collect_files_with_extensions(&path, extensions, out)?;
            continue;
        }
        if file_type.is_file()
            && path
                .extension()
                .and_then(|ext| ext.to_str())
                .is_some_and(|ext| {
                    extensions
                        .iter()
                        .any(|candidate| ext.eq_ignore_ascii_case(candidate))
                })
        {
            out.push(path);
        }
    }
    Ok(())
}

fn resolve_scan_input_paths(paths: Vec<String>) -> Result<Vec<PathBuf>, String> {
    resolve_paths_by_extension(
        paths,
        &["html", "htm", "yaml", "yml"],
        "scan requires at least one html or yaml file",
    )
}

fn resolve_compare_input_paths(paths: Vec<String>) -> Result<Vec<PathBuf>, String> {
    let resolved = resolve_paths_by_extension(
        paths,
        &["html", "htm", "yaml", "yml"],
        "compare requires exactly two html or yaml files",
    )?;
    if resolved.len() != 2 {
        return Err("compare requires exactly two html or yaml files".to_string());
    }
    Ok(resolved)
}

fn parse_scan_args(args: Vec<String>) -> Result<(ScanStage, Option<PathBuf>, Vec<String>), String> {
    let mut stage = ScanStage::Abstract;
    let mut config = None;
    let mut paths = Vec::new();
    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        if arg == "--stage" {
            let Some(value) = iter.next() else {
                return Err("--stage requires a value".to_string());
            };
            stage = match value.as_str() {
                "abstract" => ScanStage::Abstract,
                "summary" => ScanStage::Summary,
                other => {
                    return Err(format!(
                        "unknown scan stage: {other} (expected: abstract, summary)"
                    ))
                }
            };
            continue;
        }
        if arg == "--config" {
            let Some(value) = iter.next() else {
                return Err("--config requires a value".to_string());
            };
            config = Some(PathBuf::from(value));
            continue;
        }
        paths.push(arg);
    }
    Ok((stage, config, paths))
}

fn parse_compare_args(args: Vec<String>) -> Result<(Option<PathBuf>, Vec<String>), String> {
    let mut config = None;
    let mut paths = Vec::new();
    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        if arg == "--config" {
            let Some(value) = iter.next() else {
                return Err("--config requires a value".to_string());
            };
            config = Some(PathBuf::from(value));
            continue;
        }
        paths.push(arg);
    }
    Ok((config, paths))
}

fn resolve_paths_by_extension(
    paths: Vec<String>,
    extensions: &[&str],
    empty_message: &str,
) -> Result<Vec<PathBuf>, String> {
    if paths.is_empty() {
        let mut discovered = Vec::new();
        collect_files_with_extensions(Path::new("."), extensions, &mut discovered)
            .map_err(|err| err.to_string())?;
        discovered.sort();
        if discovered.is_empty() {
            return Err(empty_message.to_string());
        }
        return Ok(discovered);
    }

    let mut resolved = Vec::new();
    for raw_path in paths {
        let path = PathBuf::from(raw_path);
        if path.is_dir() {
            collect_files_with_extensions(&path, extensions, &mut resolved)
                .map_err(|err| err.to_string())?;
        } else {
            resolved.push(path);
        }
    }
    resolved.sort();
    resolved.dedup();
    if resolved.is_empty() {
        return Err(empty_message.to_string());
    }
    Ok(resolved)
}

#[cfg(test)]
mod tests {
    use super::{parse_compare_args, parse_scan_args, resolve_compare_input_paths, resolve_input_paths, resolve_scan_input_paths};
    use abstract_gui::ScanStage;
    use std::{
        fs,
        path::PathBuf,
        time::{SystemTime, UNIX_EPOCH},
    };

    #[test]
    fn resolve_input_paths_expands_gui_directories() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("gui-main-gui-dir-{unique}"));
        fs::create_dir_all(dir.join("nested")).expect("mkdir");
        fs::write(
            dir.join("a.gui"),
            "drill:\n  Home:\ninherit:\n  RootLayout:\n    Home:\n",
        )
        .expect("write a");
        fs::write(
            dir.join("nested").join("b.gui"),
            "drill:\n  Page:\ninherit:\n  RootLayout:\n    Page:\n",
        )
        .expect("write b");
        fs::write(dir.join("nested").join("ignore.txt"), "x").expect("write txt");

        let resolved =
            resolve_input_paths(vec![dir.to_string_lossy().into_owned()]).expect("resolve");
        assert_eq!(resolved.len(), 2);
        assert!(resolved.iter().any(|path| path.ends_with("a.gui")));
        assert!(resolved.iter().any(|path| path.ends_with("b.gui")));

        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn parse_scan_args_supports_stage_and_config_flags() {
        let (stage, config, paths) = parse_scan_args(vec![
            "--stage".to_string(),
            "summary".to_string(),
            "--config".to_string(),
            "scan.yaml".to_string(),
            "sample.yaml".to_string(),
        ])
        .expect("parse");
        assert_eq!(stage, ScanStage::Summary);
        assert_eq!(config, Some(PathBuf::from("scan.yaml")));
        assert_eq!(paths, vec!["sample.yaml".to_string()]);
    }

    #[test]
    fn parse_compare_args_supports_config_flag() {
        let (config, paths) = parse_compare_args(vec![
            "--config".to_string(),
            "scan.yaml".to_string(),
            "left.html".to_string(),
            "right.html".to_string(),
        ])
        .expect("parse");
        assert_eq!(config, Some(PathBuf::from("scan.yaml")));
        assert_eq!(paths, vec!["left.html".to_string(), "right.html".to_string()]);
    }

    #[test]
    fn resolve_compare_input_paths_requires_two_inputs() {
        let err = resolve_compare_input_paths(vec!["one.html".to_string()]).expect_err("should fail");
        assert!(err.contains("exactly two"));
    }

    #[test]
    fn resolve_scan_input_paths_expands_html_directories() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("gui-main-html-dir-{unique}"));
        fs::create_dir_all(dir.join("nested")).expect("mkdir");
        fs::write(dir.join("a.html"), "<html></html>").expect("write a");
        fs::write(dir.join("nested").join("b.htm"), "<html></html>").expect("write b");
        fs::write(dir.join("nested").join("ignore.gui"), "x").expect("write ignore");

        let resolved =
            resolve_scan_input_paths(vec![dir.to_string_lossy().into_owned()]).expect("resolve");
        assert_eq!(resolved.len(), 2);
        assert!(resolved.iter().any(|path| path.ends_with("a.html")));
        assert!(resolved.iter().any(|path| path.ends_with("b.htm")));

        fs::remove_dir_all(&dir).ok();
    }
}
