use std::{
    fs::{self},
    io::{self},
    path::Path,
};

use chrono::Local;

use crate::utilities::constants::{
    BOLD,
    ENDCOLOR,
    CHANGES_PATH,
    LOGS_PATH,
    COMMIT_PATH
};

fn write_logs(commit_name: &str) -> std::io::Result<()> {
    let mut log_file_content = if Path::new(LOGS_PATH).exists() {
        fs::read_to_string(LOGS_PATH)?
    } else {
        String::new()
    };

    let logtmpl = format!(
        "{}: {}\n",
        Local::now().format("%H:%M:%S %Y-%m-%d"),
        commit_name
    );

    log_file_content.push_str(&logtmpl);
    fs::write(LOGS_PATH, log_file_content)?;
    Ok(())
}

pub fn commit_cmd(name: &str) -> io::Result<()> {
    println!("1");
    if !Path::new(CHANGES_PATH).exists() {
        println!("{BOLD}{CHANGES_PATH}{ENDCOLOR} not created.\nChanges not staged for commit:\nUse {BOLD}git add <file>...{ENDCOLOR} to update what will be committed.");
        return Ok(());
    }
        println!("1");
    if Path::new(COMMIT_PATH).exists() {
        println!("{BOLD}{COMMIT_PATH}{ENDCOLOR} file already created.");
        return Ok(());
    }

    write_logs(name)?;
    let mut commit_file_content = fs::read_to_string(CHANGES_PATH)?;
    commit_file_content.push_str(&format!("commit = \"{}\"\n", name));
    fs::write(COMMIT_PATH, commit_file_content.clone())?;
    Ok(())
}
