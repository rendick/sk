use std::{
    fs,
    path::Path,
};

use crate::utilities::prompt::prompt_input;

use crate::utilities::constants::{
    BOLD,
    ENDCOLOR,
    CHANGES_PATH,
    COMMIT_PATH
};

pub fn clean_cmd() {
    if !Path::new(COMMIT_PATH).exists() && !Path::new(CHANGES_PATH).exists() {
        for exist_files in [COMMIT_PATH, CHANGES_PATH] {
            if !Path::new(exist_files).exists() {
                println!("{exist_files} doesn't exists.");
            }
        }
        return;
    }

    let clean_input =
        match prompt_input(&format!("Do you want to clear the {BOLD}{CHANGES_PATH}{ENDCOLOR} and {BOLD}{COMMIT_PATH}{ENDCOLOR} the files? [y/N] ")) {
            Ok(input) => input,
            Err(_) => {
                println!("Failed to read input.");
                return;
            }
        };

    if ["y", "Y"].contains(&clean_input.as_str()) {
        for clean_input_values in [COMMIT_PATH, CHANGES_PATH] {
            if let Err(e) = fs::remove_file(clean_input_values) {
                eprintln!("Error removing {BOLD}{clean_input_values}{ENDCOLOR} file: {e}")
            } else {
                println!("{BOLD}{clean_input_values}{ENDCOLOR} successfully removed.")
            }
        }
    }
}
