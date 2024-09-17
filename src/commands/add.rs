use std::{
    fs::{self, File},
    io::{self, Write},
    path::Path,
    process,
};

const BOLD: &str = "\x1b[1m";
const ENDCOLOR: &str = "\x1b[0m";

const CHANGES_PATH: &str = ".sk/changes";

pub fn add_cmd(file: &str) -> std::io::Result<()> {
    if Path::new(CHANGES_PATH).exists() {
        loop {
            print!("The file {BOLD}{CHANGES_PATH}{ENDCOLOR} exists. Do you want to rewrite it? [Y/n] ");
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

    let file_vec = vec![file.to_string()];

    let file_vec_toml = file_vec
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
