use std::{
    fs::{self, File},
    io::{self, Write},
    path::Path,
    process,
};

pub fn add_cmd(file: &str) -> std::io::Result<()> {
    let file_path = ".sk/changes";
    if Path::new(file_path).exists() {
        loop {
            print!("The file exists. Do you want to rewrite it? [Y/n] ");
            io::stdout().flush()?;
            let mut rewrite_input = String::new();
            io::stdin().read_line(&mut rewrite_input)?;
            if ["", "y", "Y"].contains(&rewrite_input.trim()) {
                fs::remove_file(file_path)?;
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

    let mut change_file = File::create(file_path)?;
    change_file.write_all(changes_template.as_bytes())?;

    Ok(())
}
