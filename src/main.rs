use std::env;
use std::path::Path;

mod commands {
    pub mod add;
    pub mod clean;
    pub mod commit;
    pub mod info;
    pub mod init;
    pub mod logs;
    pub mod pull;
    pub mod push;
}

use commands::add;
use commands::clean;
use commands::commit;
use commands::info;
use commands::init;
use commands::logs;
use commands::pull;
use commands::push;

// mod utilities {
//     pub mod skignore;
// }
//
// use utilities::skignore;

fn main() {
    let supported_archs = ["x86_64", "x86", "arm", "riscv64"];
    if supported_archs.contains(&env::consts::ARCH) {
        let config_file_path_check = Path::new(".sk/config");
        if config_file_path_check.exists() {
            let args: Vec<String> = env::args().collect();

            if args.len() < 2 {
                println!("dslhfdkhf");
                return;
            }

            match args.get(1).map(String::as_str) {
                Some("init") => {
                    if let Err(e) = init::init_cmd() {
                        eprintln!("Error initializing: {}", e)
                    }
                }
                Some("commit") => {
                    let commit_text: String = args[2..].join(" "); 
                    if let Err(e) = commit::commit_cmd(&commit_text){
                        eprintln!("{e}")
                    }
                }
                Some("push") => push::push_cmd().expect("sdfsd"),
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
                Some("pull") => pull::pull_cmd(),
                Some("clean") => clean::clean_cmd(),
                // Some("ignore") => {
                //     if let Err(e) = skignore::skignore_cmd() {
                //         eprintln!("{e}")
                //     }
                // }
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
