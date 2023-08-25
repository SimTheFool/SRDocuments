#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error(transparent)]
    Other(#[from] anyhow::Error),
    #[error("{0}")]
    ParseYml(String),
}

impl AppError {
    pub fn as_other<T>(e: T) -> AppError
    where
        T: Into<anyhow::Error>,
    {
        AppError::Other(e.into())
    }
}

pub type AppResult<T> = Result<T, AppError>;
