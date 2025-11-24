//! Error handling for the macro host

use thiserror::Error;

/// Result type for macro operations
pub type Result<T> = std::result::Result<T, MacroError>;

/// Errors that can occur in the macro host
#[derive(Debug, Error)]
pub enum MacroError {
    /// Macro not found in registry
    #[error("Macro '{module}::{name}' not found in registry")]
    MacroNotFound { module: String, name: String },

    /// Invalid macro configuration
    #[error("Invalid macro configuration: {0}")]
    InvalidConfig(String),

    /// Macro execution failed
    #[error("Macro execution failed: {0}")]
    ExecutionFailed(String),

    /// ABI version mismatch
    #[error("ABI version mismatch: expected {expected}, got {actual}")]
    AbiVersionMismatch { expected: u32, actual: u32 },

    /// Invalid macro manifest
    #[error("Invalid macro manifest: {0}")]
    InvalidManifest(String),

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// JSON error
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// TOML error
    #[error("TOML error: {0}")]
    Toml(#[from] toml::de::Error),

    /// TOML serialization error
    #[error("TOML serialization error: {0}")]
    TomlSer(#[from] toml::ser::Error),

    /// Generic error
    #[error("{0}")]
    Other(#[from] anyhow::Error),
}
