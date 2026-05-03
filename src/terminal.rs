use std::collections::BTreeMap;

use crate::error::QuicktermError;
use crate::expand::{env_map, expand_command};

pub fn quoted(s: &str) -> String {
    format!("'{s}'")
}

pub fn term_title(shell: &str) -> String {
    format!("{shell} - quickterm")
}

pub fn build_terminal_command(
    term: &str,
    title: &str,
    reentry_argv: &[String],
) -> Result<Vec<String>, QuicktermError> {
    match term {
        "foot" => Ok(vec![
            "footclient".to_string(),
            format!("--title={title}"),
            "--hold".to_string(),
            "--app-id=foot-sp".to_string(),
        ]
        .into_iter()
        .chain(reentry_argv.iter().cloned())
        .collect()),
        "alacritty" => Ok(vec![
            "alacritty".to_string(),
            "-t".to_string(),
            title.to_string(),
            "-e".to_string(),
        ]
        .into_iter()
        .chain(reentry_argv.iter().cloned())
        .collect()),
        custom => {
            let mut vars: BTreeMap<String, String> = env_map();
            let expanded = reentry_argv.join(" ");
            vars.insert("title".to_string(), quoted(title));
            vars.insert("expanded".to_string(), expanded);
            vars.insert("string".to_string(), quoted(&reentry_argv.join(" ")));
            expand_command(custom, &vars)
        }
    }
}
