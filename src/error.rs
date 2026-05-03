use thiserror::Error;

#[derive(Debug, Error)]
pub enum QuicktermError {
    #[error("invalid config file: {0}")]
    InvalidConfig(String),

    #[error("unknown shell: {0}")]
    UnknownShell(String),

    #[error("failed to expand command: {0}")]
    Expansion(String),

    #[error("failed to run menu: {0}")]
    Menu(String),

    #[error("ipc error: {0}")]
    Ipc(String),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
}
