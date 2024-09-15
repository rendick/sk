use std::{
    fs::{self, File},
    io::{self, Write},
    path::Path,
};

const BOLD: &str = "\x1b[1m";
const ENDCOLOR: &str = "\x1b[0m";

fn prompt_input(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn check_file() -> io::Result<bool> {
    let file_path = Path::new(".sk/config");
    if file_path.exists() {
        println!("File {BOLD}'./.sk/config'{ENDCOLOR} already exists!");
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
    let dir_path = Path::new(".sk");

    if dir_path.exists() {
        println!("Directory {BOLD}'./.sk'{ENDCOLOR} already exists.");
    } else {
        match fs::create_dir(dir_path) {
            Ok(_) => println!("Directory '.sk' created successfully."),
            Err(e) => {
                eprintln!("Failed to create directory '.sk': {}", e);
                return Err(e);
            }
        }
    }
    Ok(())
}

pub fn init_cmd() -> io::Result<()> {
    create_folder()?;
    if !check_file()? {
        return Ok(());
    }

    let mut splited_authors: Vec<String> = Vec::new();
    let mut splited_license: Vec<String> = Vec::new();

    let project_name = prompt_input("Project name: ")?;
    let author = prompt_input("Authors: ")?;
    let license = prompt_input("License: ")?;
    let repository = prompt_input("Repository URL: ")?;

    for (license_words, author_words) in license.split("-").zip(author.split("-")) {
        println!("{}", license_words);
        splited_license.push(license_words.to_string());

        println!("{}", author_words);
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

    match File::create(".sk/config") {
        Ok(mut file) => {
            if let Err(e) = file.write_all(setup_template.trim().as_bytes()) {
                eprintln!("Failed to write to file '.sk/config': {}", e);
                return Err(e);
            }
        }
        Err(e) => {
            eprintln!("Failed to create file '.sk/config': {}", e);
            return Err(e);
        }
    }

    Ok(())
}
