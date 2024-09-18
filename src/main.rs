use std::env;
use std::path::Path;

mod commands {
    pub mod add;
    pub mod commit;
    pub mod info;
    pub mod init;
    pub mod logs;
    pub mod push;
}

use commands::add;
use commands::commit;
use commands::info;
use commands::init;
use commands::logs;
use commands::push;

mod utilities {
    pub mod skignore;
}

use utilities::skignore;

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
                        let _ = commit::commit_cmd(name);
                    }
                }
                Some("push") => push::push_cmd(),
                Some("add") => {
                    let files: String = args[2..].join("\",\"");
                    if let Err(e) = add::add_cmd(&files) {
                        eprintln!("Error adding file: {}", e)
                    }
                }
                Some("clone") => println!("clone"),
                Some("pull") => println!("pull"),
                Some("logs") => logs::logs_cmd(),
                Some("info") => info::info_cmd().expect("dsfdf"),
                Some("ignore") => if let Err(e) = skignore::skignore_cmd() {
                    eprintln!("{e}")
                },
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
