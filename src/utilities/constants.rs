pub const BOLD: &str = "\x1b[1m";
pub const ENDCOLOR: &str = "\x1b[0m";

pub const SK_PATH: &str = "./.sk";
pub const CHANGES_PATH: &str = "./.sk/changes";
pub const LOGS_PATH: &str = "./.sk/commits";
pub const COMMIT_PATH: &str = "./.sk/modifications";
pub const CONFIG_PATH: &str = "./.sk/config";
pub const SKIGNORE_PATH: &str = "./.skignore";

pub const HELP: &str = r#"Usage: sk [ARGS]...

init     test
commit   test
add      test
push     test
info     test 
clean    test
logs     test
clone    test

version  test
help     test"#;
pub const VERSION: &str = "v0.7.0";
