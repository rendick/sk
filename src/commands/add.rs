use serde::Deserialize;
use std::{
    fs::{self, File},
    io::{self, Write},
    path::Path,
    process,
};
use toml;

extern crate glob;
use glob::glob;

use crate::utilities::constants::{
    BOLD,
    ENDCOLOR,
    CHANGES_PATH,
    SKIGNORE_PATH
};

#[derive(Debug, Deserialize)]
struct SkignoreConfig {
    skignore: Skignore,
}

#[derive(Debug, Deserialize)]
struct Skignore {
    ignored: Vec<String>,
}

fn parse_dir(file: &str, collected_paths: &mut Vec<String>) -> io::Result<()> {
    if !Path::new(SKIGNORE_PATH).exists() {
        return Ok(());
    }

    let all_dirs_pattern = format!("{}/{}", file, "*");
    let contents = fs::read_to_string(SKIGNORE_PATH)?;
    let config: SkignoreConfig = match toml::de::from_str(&contents) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error parsing {SKIGNORE_PATH}: {:?}", e);
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid {SKIGNORE_PATH} format",
            ));
        }
    };

    for entry in glob(&all_dirs_pattern).expect("Error checking files.") {
        match entry {
            Ok(path) => {
                if path.is_dir() {
                    let file_name = path
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string();
                    if config.skignore.ignored.contains(&file_name) {
                        continue;
                    }

                    println!("{}", path.display());
                    collected_paths.push(path.display().to_string());
                    parse_dir(&path.display().to_string(), collected_paths)?;
                } else {
                    println!("{}", path.display());
                    collected_paths.push(path.display().to_string())
                }
            }
            Err(e) => eprintln!("Error: {:?}", e),
        }
    }
    Ok(())
}

pub fn add_cmd(file: &str) -> std::io::Result<()> {
    println!("{}", file.replace("\",\"", " "));
    if Path::new(CHANGES_PATH).exists() {
        loop {
            print!(
                "The file {BOLD}{CHANGES_PATH}{ENDCOLOR} exists. Do you want to rewrite it? [Y/n] "
            );

            io::stdout().flush()?;
            let mut rewrite_input = String::new();
            io::stdin().read_line(&mut rewrite_input)?;

            if ["", "y", "Y"].contains(&rewrite_input.trim()) {
                fs::remove_file(CHANGES_PATH)?;
                break;
            } else if ["n", "N"].contains(&rewrite_input.trim()) {
                process::exit(1);
            }
        }
    }

    let mut collected_paths = Vec::new();

    if ["."].contains(&file) {
        parse_dir(file, &mut collected_paths)?;
    } else {
        for paths_to_check in file.replace("\",\"", " ").split(" ") {
            if Path::new(paths_to_check).exists() {
                println!("Exist: {paths_to_check}");
                collected_paths.push(paths_to_check.to_string())
            }
        }
    }

    let file_to_vec_toml = collected_paths
        .iter()
        .map(|f| format!("\"{}\"", f.trim()))
        .collect::<Vec<_>>()
        .join(", ");

    let changes_template = format!(
        r#"[modifications]
changes = [{}]
"#,
        file_to_vec_toml
    );

    let mut change_file = File::create(CHANGES_PATH)?;
    change_file.write_all(changes_template.as_bytes())?;

    Ok(())
}
