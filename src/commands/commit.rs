use std::{
    fs::{self},
    io::{self},
    path::Path,
};

use chrono::Local;

const BOLD: &str = "\x1b[1m";
const ENDCOLOR: &str = "\x1b[0m";

fn write_logs(commit_name: &str) -> std::io::Result<()> {
    let mut log_file_content = if Path::new(".sk/logs").exists() {
        fs::read_to_string(".sk/logs")?
    } else {
        String::new()
    };

    let logtmpl = format!(
        "{}: {}\n",
        Local::now().format("%H:%M:%S %Y-%m-%d"),
        commit_name
    );

    log_file_content.push_str(&logtmpl);
    fs::write(".sk/logs", log_file_content)?;
    Ok(())
}

pub fn commit_cmd(name: &str) -> io::Result<()> {
    let changes_file = ".sk/changes";
    if Path::new(changes_file).exists() {
        if !Path::new(".sk/commit").exists() {
            write_logs(name)?;
            let mut commit_file_content = fs::read_to_string(changes_file)?;
            commit_file_content.push_str(&format!("commit = \"{}\"\n", name));
            fs::write(".sk/commit", commit_file_content.clone())?;
            Ok(())
        } else {
            println!(".sl/commit file already created.");
            Ok(())
        }
    } else {
        println!("{changes_file} not created.\nChanges not staged for commit:\nUse {BOLD}git add <file>...{ENDCOLOR} to update what will be committed.");
        Ok(())
    }
}
