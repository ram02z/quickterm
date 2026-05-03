use std::collections::BTreeSet;
use std::os::unix::process::CommandExt;
use std::path::PathBuf;
use std::process::Command;

use crate::config::Config;
use crate::error::QuicktermError;
use crate::expand::{env_map, expand_command, expand_string};
use crate::history::{LockedHistoryFile, reorder_shells, validate_history};
use crate::ipc::{Ipc, MARK_QT_PATTERN, mark_for_shell};
use crate::menu::run_menu;
use crate::terminal::{build_terminal_command, term_title};

pub fn run(config: Config, shell: Option<String>, in_place: bool) -> Result<(), QuicktermError> {
    match (shell, in_place) {
        (None, _) => toggle_quickterm_select(&config),
        (Some(shell), false) => toggle_quickterm(&config, &shell),
        (Some(shell), true) => launch_in_place(&config, &shell),
    }
}

pub fn toggle_quickterm_select(config: &Config) -> Result<(), QuicktermError> {
    let mut ipc = Ipc::connect()?;
    let visible = ipc.find_marked_pattern_in_current_workspace(MARK_QT_PATTERN)?;
    if let Some(node) = visible.first() {
        ipc.move_back(&format!("[con_id={}]", node.id))?;
        return Ok(());
    }

    let shell_names: Vec<String> = config.shells.keys().cloned().collect();
    let mut history_file = match history_path(config)? {
        Some(path) => Some(LockedHistoryFile::open(&path)?),
        None => None,
    };
    let ordered = load_ordered_shells(&shell_names, history_file.as_mut())?;
    let menu_argv = expand_command(&config.menu, &env_map())?;
    let selected = run_menu(&menu_argv, &ordered)?;

    if !config.shells.contains_key(&selected) {
        return Ok(());
    }

    if let Some(file) = history_file.as_mut() {
        let reordered = reorder_shells(ordered, &selected);
        file.write_order(&reordered)?;
    }

    toggle_quickterm(config, &selected)
}

pub fn toggle_quickterm(config: &Config, shell: &str) -> Result<(), QuicktermError> {
    if !config.shells.contains_key(shell) {
        return Err(QuicktermError::UnknownShell(shell.to_string()));
    }

    let mark = mark_for_shell(shell);
    let mut ipc = Ipc::connect()?;
    let nodes = ipc.find_marked_exact(&mark)?;

    if nodes.is_empty() {
        let argv0 = std::env::args()
            .next()
            .unwrap_or_else(|| "quickterm".to_string());
        let reentry_argv = vec![argv0, "--in-place".to_string(), shell.to_string()];
        let terminal_argv =
            build_terminal_command(&config.term, &term_title(shell), &reentry_argv)?;

        let err = Command::new(&terminal_argv[0])
            .args(&terminal_argv[1..])
            .exec();
        return Err(QuicktermError::Io(err));
    }

    let node = &nodes[0];
    let current_ws = ipc.current_workspace()?;
    let node_ws_name = ipc.workspace_name_for_node(node.id)?.unwrap_or_default();
    ipc.move_back(&format!("[con_id={}]", node.id))?;
    if node_ws_name != current_ws.name {
        ipc.pop_it(&mark, &config.pos, config.ratio)?;
    }

    Ok(())
}

pub fn launch_in_place(config: &Config, shell: &str) -> Result<(), QuicktermError> {
    let command = config
        .shells
        .get(shell)
        .ok_or_else(|| QuicktermError::UnknownShell(shell.to_string()))?;

    let mark = mark_for_shell(shell);
    let mut ipc = Ipc::connect()?;
    ipc.command(&format!("mark {mark}"))?;
    ipc.move_back(&format!("[con_mark={mark}]"))?;
    ipc.pop_it(&mark, &config.pos, config.ratio)?;

    let argv = expand_command(command, &env_map())?;
    let err = Command::new(&argv[0]).args(&argv[1..]).exec();
    Err(QuicktermError::Io(err))
}

fn history_path(config: &Config) -> Result<Option<PathBuf>, QuicktermError> {
    match &config.history {
        Some(path) => {
            let expanded = expand_string(path, &env_map())?;
            Ok(Some(PathBuf::from(expanded)))
        }
        None => Ok(None),
    }
}

fn load_ordered_shells(
    shell_names: &[String],
    history_file: Option<&mut LockedHistoryFile>,
) -> Result<Vec<String>, QuicktermError> {
    let sorted = {
        let mut items = shell_names.to_vec();
        items.sort();
        items
    };

    let Some(file) = history_file else {
        return Ok(sorted);
    };

    let known: BTreeSet<String> = shell_names.iter().cloned().collect();
    let loaded = file.read_order()?;

    Ok(match loaded {
        Some(order) => validate_history(&order, &known).unwrap_or(sorted),
        None => sorted,
    })
}
