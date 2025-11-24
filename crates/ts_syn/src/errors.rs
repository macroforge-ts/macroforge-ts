use thiserror::Error;

#[derive(Error, Debug)]
pub enum TsSynError {
    #[error("parse error: {0}")]
    Parse(String),

    #[error("unsupported TS syntax: {0}")]
    Unsupported(String),
}
