use std::collections::BTreeMap;

use quickterm::expand::{expand_command, expand_string, expand_template};

#[test]
fn expands_environment_placeholders() {
    let mut vars = BTreeMap::new();
    vars.insert("$HOME".to_string(), "/home/tester".to_string());

    let result = expand_template("{$HOME}/bin", &vars).unwrap();
    assert_eq!(result, "/home/tester/bin");
}

#[test]
fn expands_named_placeholders() {
    let mut vars = BTreeMap::new();
    vars.insert("title".to_string(), "'python - quickterm'".to_string());

    let result = expand_template("{title}", &vars).unwrap();
    assert_eq!(result, "'python - quickterm'");
}

#[test]
fn splits_shell_like_arguments() {
    let mut vars = BTreeMap::new();
    vars.insert("$HOME".to_string(), "/home/tester".to_string());

    let argv = expand_command("printf '{$HOME} file' --flag", &vars).unwrap();
    assert_eq!(argv, vec!["printf", "/home/tester file", "--flag"]);
}

#[test]
fn expands_path_without_shell_splitting() {
    let mut vars = BTreeMap::new();
    vars.insert("$HOME".to_string(), "/home/tester".to_string());

    let result = expand_string("{$HOME}/cache file", &vars).unwrap();
    assert_eq!(result, "/home/tester/cache file");
}
