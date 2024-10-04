use serde::Deserialize;
use std::fs::{self, File};
use std::io::{self, Write};
use std::{path::Path, process};

use crate::utilities::constants::COMMIT_PATH;
use crate::utilities::prompt::prompt_input;

#[derive(Deserialize)]
struct Modification {
    changes: Vec<String>,
}

#[derive(Deserialize)]
struct Config {
    modification: Modification,
}

pub fn rm_cmd(files_to_delete: Vec<String>) -> io::Result<()> {
    if !Path::new("./repo").exists() {
        process::exit(1);
    }

    if Path::new(COMMIT_PATH).exists() {
        let delete_or_exit_modification =
            prompt_input(&format!("Do you want to recreate {COMMIT_PATH}? [Y/n] "))?;
        if !["", "Y", "y"].contains(&delete_or_exit_modification.as_str()) {
            process::exit(1);
        }
    }

    let mut filtered_files_to_delete: Vec<String> = Vec::new();

    println!("{:?}", files_to_delete);

    for check_files in files_to_delete.iter() {
        if Path::new(check_files).exists() {
            filtered_files_to_delete.push(check_files.to_string());
        }
    }

    let vector_to_toml_variable = filtered_files_to_delete
        .iter()
        .map(|f| format!("\"{}\"", f.trim()))  // Fix: removed leading space
        .collect::<Vec<_>>()
        .join(", ");

    let changes_template = format!(
        r#"[modifications]
changes = [{}]"#,   // Fix: use [modifications]
        vector_to_toml_variable
    );

    println!("TOML content:\n{changes_template}");

    // Write the changes_template to COMMIT_PATH (creates or overwrites the file)
    let mut commit_file = File::create(COMMIT_PATH)?;
    commit_file.write_all(changes_template.as_bytes())?;
    println!("Written to {COMMIT_PATH}");

    let conf: Config = toml::from_str(&changes_template).expect("Failed to parse TOML");

    for y in conf.modification.changes.iter() {
        if y.starts_with(" ") {
            println!("File name starts with a space: {}", y);
        }
    }

    println!("Filtered files to delete: {:?}", filtered_files_to_delete);

    Ok(())
}
