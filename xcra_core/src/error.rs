#[derive(thiserror::Error, Debug)]
pub enum XcraError {
    #[error("SqliteError {0}")]
    SqliteError(#[from] rusqlite::Error),
    #[error("IoError {0}")]
    IoError(#[from] std::io::Error),
    #[error("SerializationError {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("xcresult file not found")]
    XcresultNotFound,
}
