use std::env;
use std::process;

mod commands {
    pub mod add;
    pub mod clean;
    pub mod clone;
    pub mod commit;
    pub mod info;
    pub mod init;
    pub mod logs;
    pub mod pull;
    pub mod push;
    pub mod rm;
}

use commands::add;
use commands::clean;
use commands::clone;
use commands::commit;
use commands::info;
use commands::init;
use commands::logs;
use commands::pull;
use commands::push;
use commands::rm;

mod utilities {
    pub mod constants;
    pub mod prompt;
    pub mod www;
}

use crate::utilities::constants::{HELP, VERSION};

fn main() {
    let supported_archs = ["x86_64", "x86", "arm", "riscv64", "aarch64"];
    if !supported_archs.contains(&env::consts::ARCH) {
        println!("sk doesn't support your CPU currently.");
        process::exit(1)
    }

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("{}", HELP);
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
            if let Err(e) = commit::commit_cmd(&commit_text) {
                eprintln!("{e}")
            }
        }
        Some("push") => push::push_cmd().expect("Error parsing the command."),
        Some("add") => {
            let files: String = args[2..].join("\",\"");
            if let Err(e) = add::add_cmd(&files) {
                eprintln!("Error adding file: {}", e)
            }
        }
        Some("clone") => clone::clone_cmd(),
        Some("logs") => logs::logs_cmd(),
        Some("info") => info::info_cmd().expect("Error parsing the command."),
        Some("pull") => pull::pull_cmd(),
        Some("clean") => clean::clean_cmd(),
        Some("help") | Some("--help") | Some("-h") => println!("{}", HELP),
        Some("version") | Some("--version") | Some("-v") => println!("{}", VERSION),
        Some("rm") => {
            let files: Vec<String> = args[2..].to_vec();
            rm::rm_cmd(files).expect("Error running this function.")
        }
        _ => println!(
            "sk: {} is not a sk command. Try 'sk --help' for more information.",
            args.get(1).as_slice()[0]
        ),
    }
}
