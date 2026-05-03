use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

use serde::Deserialize;

use crate::error::QuicktermError;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Position {
    Top,
    Bottom,
    Center,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Config {
    pub menu: String,
    pub term: String,
    pub history: Option<String>,
    pub ratio: f64,
    pub pos: Position,
    pub shells: BTreeMap<String, String>,
}

#[derive(Debug, Default, Deserialize)]
struct PartialConfig {
    menu: Option<String>,
    term: Option<String>,
    history: Option<Option<String>>,
    ratio: Option<f64>,
    pos: Option<Position>,
    shells: Option<BTreeMap<String, String>>,
}

pub fn default_config() -> Config {
    let mut shells = BTreeMap::new();
    shells.insert("haskell".to_string(), "ghci".to_string());
    shells.insert("js".to_string(), "node".to_string());
    shells.insert("python".to_string(), "ipython3 --no-banner".to_string());
    shells.insert("shell".to_string(), "{$SHELL}".to_string());

    Config {
        menu: "rofi -dmenu -p 'quickterm: ' -no-custom -auto-select".to_string(),
        term: "urxvt".to_string(),
        history: Some("{$HOME}/.cache/i3-quickterm.order".to_string()),
        ratio: 0.25,
        pos: Position::Top,
        shells,
    }
}

pub fn config_path_for(home: &str, xdg_config_dir: Option<&str>) -> String {
    match xdg_config_dir {
        Some(dir) => format!("{dir}/i3-quickterm.json"),
        None => format!("{home}/.config/i3-quickterm.json"),
    }
}

pub fn config_path_from_env() -> Result<PathBuf, QuicktermError> {
    let home = std::env::var("HOME")
        .map_err(|_| QuicktermError::InvalidConfig("HOME is not set".to_string()))?;
    let xdg = std::env::var("XDG_CONFIG_DIR").ok();
    Ok(PathBuf::from(config_path_for(&home, xdg.as_deref())))
}

pub fn load_config() -> Result<Config, QuicktermError> {
    let path = config_path_from_env()?;
    if !path.exists() {
        return Ok(default_config());
    }

    let text = fs::read_to_string(path)?;
    let partial: PartialConfig = serde_json::from_str(&text)
        .map_err(|err| QuicktermError::InvalidConfig(err.to_string()))?;

    let mut config = default_config();
    if let Some(value) = partial.menu {
        config.menu = value;
    }
    if let Some(value) = partial.term {
        config.term = value;
    }
    if let Some(value) = partial.history {
        config.history = value;
    }
    if let Some(value) = partial.ratio {
        config.ratio = value;
    }
    if let Some(value) = partial.pos {
        config.pos = value;
    }
    if let Some(value) = partial.shells {
        config.shells = value;
    }

    Ok(config)
}
