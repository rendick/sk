use std::env;
use std::path::Path;

mod commands {
    pub mod add;
    pub mod init;
    pub mod logs;
    pub mod commit;
}

use commands::add;
use commands::init;
use commands::logs;
use commands::commit;

fn main() {
    let supported_archs = ["x86_64", "x86", "arm", "riscv64"];
    if supported_archs.contains(&env::consts::ARCH) {
        let config_file_path_check = Path::new(".sk/config");
        if config_file_path_check.exists() {
            let args: Vec<String> = env::args().collect();

            match args.get(1).map(String::as_str) {
                Some("init") => {
                    if let Err(e) = init::init_cmd() {
                        eprintln!("Error initializing: {}", e)
                    }
                }
                Some("commit") => {
                    if let Some(name) = args.get(2) {
                        commit::commit_cmd(name);
                    }
                }
                Some("push") => println!("push"),
                Some("add") => {
                    if let Some(file) = args.get(2) {
                        if let Err(e) = add::add_cmd(file) {
                            eprintln!("Error adding file: {}", e)
                        }
                    } else {
                        eprintln!("Missing file name.")
                    }
                }
                Some("clone") => println!("clone"),
                Some("pull") => println!("pull"),
                Some("logs") => logs::logs_cmd(),
                _ => println!(
                    "sk: {} is not a sk command. See 'sk --help'.",
                    args.get(1).as_slice()[0]
                ),
            }
        } else {
            if let Err(e) = init::init_cmd() {
                eprintln!("Error initializing: {}", e)
            }
        }
    } else {
        println!("sk doesn't support your CPU currently.")
    }
}
