use serde::Deserialize;
use std::{fs, path::Path};

const BOLD: &str = "\x1b[1m";
const ENDCOLOR: &str = "\x1b[0m";

const CONFIG_PATH: &str = ".sk/config";

#[derive(Deserialize)]
struct Project {
    name: String,
    authors: Vec<String>,
    license: Vec<String>,
    repository: String,
}

#[derive(Deserialize)]
struct Config {
    project: Project,
}

pub fn info_cmd() -> std::io::Result<()> {
    if Path::new(CONFIG_PATH).exists() {
        let config_file_content = fs::read_to_string(CONFIG_PATH)?;
        let conf: Config = toml::from_str(&config_file_content).expect("Error parsing TOML configuration file.");

        println!(
            "{}",
            format!(
                r#"{BOLD}Name:{ENDCOLOR} {}
{BOLD}Authors:{ENDCOLOR} {:?}
{BOLD}License:{ENDCOLOR} {:?}
{BOLD}Repository:{ENDCOLOR} {}"#,
                conf.project.name,
                conf.project.authors,
                conf.project.license,
                conf.project.repository
            )
        );
    }

    Ok(())
}
