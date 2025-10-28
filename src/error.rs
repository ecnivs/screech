use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Screenshot capture failed: {0}")]
    ScreenshotError(String),

    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    #[error("Browser error: {0}")]
    BrowserError(String),

    #[error("File system error: {0}")]
    FileSystemError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("WebDriver error: {0}")]
    WebDriverError(#[from] thirtyfour::error::WebDriverError),
}

#[derive(Serialize)]
struct ErrorResponse {
    success: bool,
    error: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ApiError::InvalidUrl(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            ApiError::ScreenshotError(_) => {
                (StatusCode::UNPROCESSABLE_ENTITY, self.to_string())
            }
            _ => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };

        let body = Json(ErrorResponse {
            success: false,
            error: error_message,
        });

        (status, body).into_response()
    }
}

pub type Result<T> = std::result::Result<T, ApiError>;
