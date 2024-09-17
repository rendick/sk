use std::path::Path;

const BOLD: &str = "\x1b[1m";
const ENDCOLOR: &str = "\x1b[0m";

const CONFIG_PATH: &str = ".sk/config";
const CHANGES_PATH: &str = ".sk/changes";
const LOGS_PATH: &str = ".sk/logs";
const COMMIT_PATH: &str = ".sk/commit";

pub fn push_cmd() {
    if Path::new(".sk").is_dir() {
        if Path::new(COMMIT_PATH).exists() && Path::new(CONFIG_PATH).exists() {
            println!("zsdhskdjfh")
        }
    } else {
        println!("The directory doesn't exist! Use {BOLD}sk init{ENDCOLOR} to initialize it.")
    }
}
