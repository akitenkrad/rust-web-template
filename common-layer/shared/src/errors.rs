use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("{0}")]
    Unprocessable(String),

    // from anyhow
    #[error("Error: {0}")]
    AnyhowError(#[from] anyhow::Error),

    // from axum
    #[error("Axum error: {0}")]
    AxumError(#[from] axum::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let status_code = match self {
            AppError::Unprocessable(_) => {
                tracing::error!("Unprocessable entity error: {}", self);
                StatusCode::UNPROCESSABLE_ENTITY
            }
            AppError::AnyhowError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::AxumError(_) => StatusCode::BAD_REQUEST,
        };
        status_code.into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;
