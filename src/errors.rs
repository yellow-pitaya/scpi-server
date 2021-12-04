pub type Result<T = Option<String>> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Redpitaya error: {0}")]
    Redpitaya(#[from] redpitaya::Error),
    #[error("{0}")]
    Misc(String),
    #[error("Missing parameter")]
    MissingParameter,
    #[error("Unknow command")]
    UnknowCommand,
}
