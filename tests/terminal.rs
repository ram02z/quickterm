use quickterm::terminal::{build_terminal_command, term_title};

#[test]
fn term_title_matches_expected_format() {
    assert_eq!(term_title("python"), "python - quickterm");
}

#[test]
fn builds_alacritty_command() {
    let argv = build_terminal_command(
        "alacritty",
        "python - quickterm",
        &[
            "quickterm".to_string(),
            "--in-place".to_string(),
            "python".to_string(),
        ],
    )
    .unwrap();

    assert_eq!(
        argv,
        vec![
            "alacritty",
            "-t",
            "python - quickterm",
            "-e",
            "quickterm",
            "--in-place",
            "python",
        ]
    );
}

#[test]
fn builds_foot_command() {
    let argv = build_terminal_command(
        "foot",
        "python - quickterm",
        &[
            "quickterm".to_string(),
            "--in-place".to_string(),
            "python".to_string(),
        ],
    )
    .unwrap();

    assert_eq!(
        argv,
        vec![
            "footclient",
            "--title=python - quickterm",
            "--hold",
            "--app-id=foot-sp",
            "quickterm",
            "--in-place",
            "python",
        ]
    );
}

#[test]
fn builds_custom_terminal_command_from_template() {
    let argv = build_terminal_command(
        "urxvt -T {title} -e {expanded}",
        "python - quickterm",
        &[
            "quickterm".to_string(),
            "--in-place".to_string(),
            "python".to_string(),
        ],
    )
    .unwrap();

    assert_eq!(
        argv,
        vec![
            "urxvt",
            "-T",
            "python - quickterm",
            "-e",
            "quickterm",
            "--in-place",
            "python",
        ]
    );
}
