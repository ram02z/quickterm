use std::collections::BTreeSet;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;

use fs2::FileExt;

use crate::error::QuicktermError;

pub fn validate_history(loaded: &[String], known_shells: &BTreeSet<String>) -> Option<Vec<String>> {
    let loaded_set: BTreeSet<String> = loaded.iter().cloned().collect();
    if &loaded_set == known_shells {
        Some(loaded.to_vec())
    } else {
        None
    }
}

pub fn reorder_shells(shells: Vec<String>, selected: &str) -> Vec<String> {
    std::iter::once(selected.to_string())
        .chain(shells.into_iter().filter(|name| name != selected))
        .collect()
}

pub struct LockedHistoryFile {
    file: File,
}

impl LockedHistoryFile {
    pub fn open(path: &Path) -> Result<Self, QuicktermError> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let file = OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(path)?;

        file.lock_exclusive()?;
        Ok(Self { file })
    }

    pub fn read_order(&mut self) -> Result<Option<Vec<String>>, QuicktermError> {
        self.file.seek(SeekFrom::Start(0))?;
        let mut buf = String::new();
        self.file.read_to_string(&mut buf)?;

        if buf.trim().is_empty() {
            return Ok(None);
        }

        match serde_json::from_str(&buf) {
            Ok(loaded) => Ok(Some(loaded)),
            Err(_) => Ok(None),
        }
    }

    pub fn write_order(&mut self, shells: &[String]) -> Result<(), QuicktermError> {
        self.file.set_len(0)?;
        self.file.seek(SeekFrom::Start(0))?;
        serde_json::to_writer(&mut self.file, shells)?;
        self.file.flush()?;
        Ok(())
    }
}
