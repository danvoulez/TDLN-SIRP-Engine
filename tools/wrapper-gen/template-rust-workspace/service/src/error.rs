use axum::{http::StatusCode, response::{IntoResponse, Response}};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("{0}")]
    Anyhow(#[from] anyhow::Error),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}
