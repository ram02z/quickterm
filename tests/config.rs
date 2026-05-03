use quickterm::config::{Position, config_path_for, default_config};

#[test]
fn default_config_contains_expected_shells() {
    let config = default_config();
    assert_eq!(
        config.menu,
        "rofi -dmenu -p 'quickterm: ' -no-custom -auto-select"
    );
    assert_eq!(config.term, "urxvt");
    assert_eq!(
        config.history.as_deref(),
        Some("{$HOME}/.cache/i3-quickterm.order")
    );
    assert!((config.ratio - 0.25).abs() < f64::EPSILON);
    assert_eq!(config.pos, Position::Top);
    assert_eq!(
        config.shells.get("python").map(String::as_str),
        Some("ipython3 --no-banner")
    );
    assert_eq!(
        config.shells.get("shell").map(String::as_str),
        Some("{$SHELL}")
    );
}

#[test]
fn config_path_prefers_xdg_config_dir() {
    let path = config_path_for("/home/tester", Some("/tmp/xdg-config"));
    assert_eq!(path, "/tmp/xdg-config/i3-quickterm.json");
}

#[test]
fn config_path_falls_back_to_home_config() {
    let path = config_path_for("/home/tester", None);
    assert_eq!(path, "/home/tester/.config/i3-quickterm.json");
}
