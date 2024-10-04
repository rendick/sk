use serde::Deserialize;
use std::{fs, path::Path};
use toml;

use crate::utilities::constants::{BOLD, COMMIT_PATH, CONFIG_PATH, ENDCOLOR, SK_PATH};

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

    if !Path::new(SK_PATH).is_dir() {
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
        let src_path = Path::new(x.trim());
        let rel_path = src_path;
        let dest_path = Path::new(&conf.project.repository).join(rel_path);

        // Deleting files or directories if they exist
        if src_path.is_dir() {
            println!("Removing directory: {}", src_path.display());
            fs::remove_dir_all(src_path)?;
        } else if src_path.is_file() {
            println!("Removing file: {}", src_path.display());
            fs::remove_file(src_path)?;
        } else {
            eprintln!("Skipping: {} is not a file or directory", src_path.display());
        }
    }

    Ok(())
}
