#[derive(Debug, thiserror::Error)]
enum AppError {
    #[error("Git has failed, error: {0}")]
    GitError(String),
    #[error("Nix failed to build, output: {0}")]
    NixError(String),
    #[error("Proxmox API error: {0}")]
    ProxmoxError(String),
    #[error("QM error: {0}")]
    QMError(String),
    #[error("File IO error {0}")]
    FileIOError(String),
    #[error("Serialisation error at some point {0}")]
    SerialisationError(String),    
}

