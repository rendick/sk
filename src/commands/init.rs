use std::{
    env,
    fs::{self, File},
    io::{self, Write},
    path::Path,
    process,
};

use crate::utilities::constants::{BOLD, CONFIG_PATH, ENDCOLOR, SK_PATH};
use crate::utilities::prompt::prompt_input;
use crate::utilities::www::{
    COMMIT, COMMIT_PATH, CONFIG, CONFIG_DIR, FETCH, FETCH_PATH, INDEX, INDEX_PATH, JSON_CONFIG,
    JSON_CONFIG_PATH, MAIN, MAIN_PATH, STYLE_CSS, STYLE_PATH,
};

fn check_file() -> io::Result<bool> {
    let file_path = Path::new(CONFIG_PATH);
    if file_path.exists() {
        println!("File {BOLD}{CONFIG_PATH}{ENDCOLOR} already exists!");
        print!("Do you want to remake it? [y/{BOLD}N{ENDCOLOR}] ");
        io::stdout().flush()?;

        let mut remake_input = String::new();
        io::stdin().read_line(&mut remake_input)?;

        if ["y", "Y"].contains(&remake_input.trim()) {
            return Ok(true);
        } else {
            return Ok(false);
        }
    }

    Ok(true)
}

fn create_folder() -> io::Result<()> {
    let dir_path = Path::new(SK_PATH);

    if dir_path.exists() {
        println!("Directory {BOLD}.sk{ENDCOLOR} already exists.");
    } else {
        match fs::create_dir(dir_path) {
            Ok(_) => println!("Directory {BOLD}.sk{ENDCOLOR} created successfully."),
            Err(e) => {
                eprintln!("Failed to create directory {BOLD}.sk{ENDCOLOR}: {}", e);
                return Err(e);
            }
        }
    }
    Ok(())
}

pub fn init_cmd() -> io::Result<()> {
    if !check_file()? {
        return Ok(());
    }

    let web_server = prompt_input(
        "Would you like to create a sk repository with the ability to be hosted on the internet? [Y/n] ",
    )?;

    if ["", "Y", "y"].contains(&web_server.as_str()) {
        let dirs_to_create = ["./public/js", "./public/style", "./repo"];

        for dir in dirs_to_create {
            if !Path::new(dir).exists() {
                fs::create_dir_all(dir)?;
            }
        }

        let files_to_write = [
            (INDEX, INDEX_PATH),
            (FETCH, FETCH_PATH),
            (MAIN, MAIN_PATH),
            (COMMIT, COMMIT_PATH),
            (CONFIG, CONFIG_DIR),
            (STYLE_CSS, STYLE_PATH),
            (JSON_CONFIG, JSON_CONFIG_PATH),
        ];

        for (content, file_path) in files_to_write {
            if let Some(parent_dir) = Path::new(file_path).parent() {
                fs::create_dir_all(parent_dir)?;
            }

            fs::write(file_path, content)?;
        }
    }

    let mut splited_authors: Vec<String> = Vec::new();
    let mut splited_license: Vec<String> = Vec::new();

    let project_name = prompt_input("Project name: ")?;
    let author = prompt_input("Authors: [name <name@example.org-name2 <name2@example.org>] ")?;
    let license = prompt_input("License: [LICENSE1-LICENSE2-LICENSE3] ")?;

    let path = env::current_dir()?;
    let repository = format!("{}/repo/", path.display());

    for (license_words, author_words) in license.split("-").zip(author.split("-")) {
        splited_license.push(license_words.to_string());
        splited_authors.push(author_words.to_string());
    }

    let setup_template = format!(
        r#"[project]
name = "{}"
authors = {:?} 
license = {:?}
repository = "{}""#,
        project_name, splited_authors, splited_license, repository
    );

    create_folder()?;

    match File::create(CONFIG_PATH) {
        Ok(mut file) => {
            if let Err(e) = file.write_all(setup_template.trim().as_bytes()) {
                eprintln!(
                    "Failed to write to file {BOLD}{CONFIG_PATH}{ENDCOLOR}: {}",
                    e
                );
                return Err(e);
            } else {
                println!(
                    r#"
Your {BOLD}.sk{ENDCOLOR} was successfully created.
The project directory where the configuration file is located named {BOLD}.sk{ENDCOLOR}
The front-end and back-end parts of the online server are located in a {BOLD}public{ENDCOLOR} directory.

If you want to start the server, you can write {BOLD}npm install && npm start{ENDCOLOR}. Make sure that {BOLD}npm{ENDCOLOR} & {BOLD}node.js{ENDCOLOR} is already installed."#
                );
                process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Failed to create file {BOLD}{CONFIG_PATH}{ENDCOLOR}: {}", e);
            return Err(e);
        }
    }
}
