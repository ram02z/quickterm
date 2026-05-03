use std::io::Write;
use std::process::{Command, Stdio};

use crate::error::QuicktermError;

pub fn run_menu(argv: &[String], items: &[String]) -> Result<String, QuicktermError> {
    if argv.is_empty() {
        return Err(QuicktermError::Menu("empty menu command".to_string()));
    }

    let mut child = Command::new(&argv[0])
        .args(&argv[1..])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|err| QuicktermError::Menu(err.to_string()))?;

    {
        let stdin = child
            .stdin
            .as_mut()
            .ok_or_else(|| QuicktermError::Menu("menu stdin unavailable".to_string()))?;

        for item in items {
            stdin.write_all(item.as_bytes())?;
            stdin.write_all(b"\n")?;
        }
    }

    let output = child
        .wait_with_output()
        .map_err(|err| QuicktermError::Menu(err.to_string()))?;

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}
