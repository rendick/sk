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

const BOLD: &str = "\x1b[1m";
const ENDCOLOR: &str = "\x1b[0m";

const CHANGES_PATH: &str = ".sk/changes";

#[derive(Debug, Deserialize)]
struct SkignoreConfig {
    skignore: Skignore,
}

#[derive(Debug, Deserialize)]
struct Skignore {
    ignored: Vec<String>,
}

// fn skignore() {
//     println!("sdfhsdkfh")
// }

fn parse_dir(file: &str, collected_paths: &mut Vec<String>) -> io::Result<()> {
    let all_dirs_pattern = format!("{}/{}", file, "*");
    let contents = fs::read_to_string(".skignore")?;
    let config: SkignoreConfig = match toml::de::from_str(&contents) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error parsing .skignore: {:?}", e);
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid .skignore format",
            ));
        }
    };

    for entry in glob(&all_dirs_pattern).expect("sdkfldsj") {
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
        collected_paths.push(file.to_string())
    }

    let file_vec_toml = collected_paths
        .iter()
        .map(|f| format!("\"{}\"", f.trim()))
        .collect::<Vec<_>>()
        .join(", ");

    let changes_template = format!(
        r#"[modifications]
changes = [{}]
"#,
        file_vec_toml
    );

    let mut change_file = File::create(CHANGES_PATH)?;
    change_file.write_all(changes_template.as_bytes())?;

    Ok(())
}
