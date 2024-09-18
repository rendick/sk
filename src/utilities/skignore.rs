use serde::Deserialize;
use std::{fs, path::Path};
use toml;

#[derive(Debug, Deserialize)]
struct SkignoreConfig {
    skignore: Skignore,
}

#[derive(Debug, Deserialize)]
struct Skignore {
    ignored: Vec<String>,
}

pub fn skignore_cmd() -> Result<(), Box<dyn std::error::Error>> {
    if Path::new(".skignore").exists() {
        let contents = fs::read_to_string(".skignore")?;
        let config: SkignoreConfig = toml::de::from_str(&contents)?;
        println!("{:?}", config.skignore.ignored);
    } else {
        println!("No .skignore file found.");
    }

    Ok(())
}
