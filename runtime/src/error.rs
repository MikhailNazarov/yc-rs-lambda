use thiserror::Error;

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("Environment variable {0} not specified")]
    RequiredEnv(String),

    #[error("Component error: {0}")]
    ComponentError(String),

    #[error(transparent)]
    TimeoutError(#[from] tokio::time::error::Elapsed),

    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

pub type RuntimeResult<T> = Result<T, RuntimeError>;
