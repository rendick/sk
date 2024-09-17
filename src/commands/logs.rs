use std::{fs::File, io::Read, path::Path};

const LOGS_PATH: &str = ".sk/logs";

fn read_file() {
    let file_path = Path::new(LOGS_PATH);

    if file_path.exists() {
        let mut log_file = match File::open(file_path) {
            Ok(log_file) => log_file,
            Err(e) => {
                eprintln!("Error opening file: {}", e);
                return;
            }
        };

        let mut contents = Vec::new();
        if let Err(e) = log_file.read_to_end(&mut contents) {
            eprintln!("Error reading file: {}", e)
        } else {
            println!("{}", String::from_utf8_lossy(&contents))
        }
    } else {
        eprintln!("File doesn't exist.")
    }
}

pub fn logs_cmd() {
    read_file()
}
