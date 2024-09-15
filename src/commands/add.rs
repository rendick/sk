use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

pub fn add_cmd(file: &str) -> std::io::Result<()> {
    let file_path = ".sk/changes";
    if Path::new(file_path).exists() {
        fs::remove_file(file_path)?;
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
