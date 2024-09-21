use std::{
    fs,
    io::{self, Write},
    path::Path,
};

const BOLD: &str = "\x1b[1m";
const ENDCOLOR: &str = "\x1b[0m";

const CHANGES_PATH: &str = ".sk/changes";
const COMMIT_PATH: &str = ".sk/commit";

fn prompt(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

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
        match prompt(&format!("Do you want to clear the {BOLD}{CHANGES_PATH}{ENDCOLOR} and {BOLD}{COMMIT_PATH}{ENDCOLOR} the files? [y/N] ")) {
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
