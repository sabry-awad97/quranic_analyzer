use thiserror::Error;

#[derive(Debug, Error)]
pub enum QuranError {
    #[error("failed to open file: {0}")]
    FileOpenError(String),
    #[error("failed to parse JSON: {0}")]
    JsonError(String),
}
