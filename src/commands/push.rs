use serde::Deserialize;
use std::{fs, path::Path};
use toml;

const BOLD: &str = "\x1b[1m";
const ENDCOLOR: &str = "\x1b[0m";

const CONFIG_PATH: &str = ".sk/config";
const COMMIT_PATH: &str = ".sk/commit";

#[derive(Deserialize)]
struct Modifications {
    changes: Vec<String>,
}

#[derive(Deserialize)]
struct CommitConfig {
    modifications: Modifications,
}

#[derive(Deserialize)]
struct Project {
    repository: String,
}

#[derive(Deserialize)]
struct ConfigConfig {
    project: Project,
}

pub fn push_cmd() -> std::io::Result<()> {
    let config_file_content = fs::read_to_string(CONFIG_PATH)?;
    let conf: ConfigConfig =
        toml::from_str(&config_file_content).expect("Error parsing {CONFIG_PATH} TOML file.");

    if !Path::new(".sk").is_dir() {
        println!("The directory doesn't exist! Use {BOLD}sk init{ENDCOLOR} to initialize it.");

        return Ok(());
    }

    if !Path::new(COMMIT_PATH).exists() && !Path::new(CONFIG_PATH).exists() {
        return Ok(());
    }

    if !Path::new(&conf.project.repository).exists() {
        eprintln!(
            "sk cannot find the given directory {BOLD}{}{ENDCOLOR} in your file system.\n\
            Please check {BOLD}{CONFIG_PATH}{ENDCOLOR} for the existence of a repository path.",
            conf.project.repository
        );

        return Ok(());
    }

    let commit_file_content = fs::read_to_string(COMMIT_PATH)?;
    let commit: CommitConfig =
        toml::from_str(&commit_file_content).expect("Error parsing {COMMIT_PATH} TOML file.");

    for x in commit.modifications.changes.iter() {
        let src_path = Path::new(x);
        let rel_path = src_path;
        let dest_path = Path::new(&conf.project.repository).join(rel_path);

        if Path::new(x).is_dir() {
            println!("Creating directory: {}", dest_path.display());
            fs::create_dir_all(&dest_path)?;
        } else if src_path.is_file() {
            println!("Copying file {} to {}", x, dest_path.display());

            if let Some(parent) = dest_path.parent() {
                fs::create_dir_all(parent)?;
            }

            fs::copy(x, &dest_path)?;
        } else {
            eprintln!("Skipping: {} is not a file or directory", x);
        }
    }

    Ok(())
}
