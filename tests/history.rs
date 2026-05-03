use std::collections::BTreeSet;
use std::fs;

use quickterm::history::{LockedHistoryFile, reorder_shells, validate_history};
use tempfile::tempdir;

#[test]
fn history_is_reordered_with_selected_shell_first() {
    let result = reorder_shells(
        vec!["js".to_string(), "python".to_string(), "shell".to_string()],
        "shell",
    );

    assert_eq!(result, vec!["shell", "js", "python"]);
}

#[test]
fn history_is_rejected_if_shell_set_changes() {
    let known: BTreeSet<String> = ["js", "python"].into_iter().map(str::to_string).collect();
    let loaded = vec!["js".to_string(), "python".to_string(), "shell".to_string()];

    assert!(validate_history(&loaded, &known).is_none());
}

#[test]
fn malformed_history_is_ignored() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("history.json");
    fs::write(&path, "not-json").unwrap();

    let mut file = LockedHistoryFile::open(&path).unwrap();
    assert_eq!(file.read_order().unwrap(), None);
}
