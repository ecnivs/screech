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

    #[error("Invalid request parameters: {0}")]
    InvalidRequest(String),

    #[error("Browser error: {0}")]
    BrowserError(String),

    #[error("Browser timeout: {0}")]
    BrowserTimeout(String),

    #[error("File system error: {0}")]
    FileSystemError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("WebDriver error: {0}")]
    WebDriverError(#[from] thirtyfour::error::WebDriverError),

    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),

    #[error("Request timeout: {0}")]
    RequestTimeout(String),
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
            ApiError::InvalidRequest(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            ApiError::ScreenshotError(_) => (StatusCode::UNPROCESSABLE_ENTITY, self.to_string()),
            ApiError::BrowserTimeout(_) => (StatusCode::REQUEST_TIMEOUT, self.to_string()),
            ApiError::RequestTimeout(_) => (StatusCode::REQUEST_TIMEOUT, self.to_string()),
            ApiError::BrowserError(_) => (StatusCode::SERVICE_UNAVAILABLE, self.to_string()),
            ApiError::ServiceUnavailable(_) => (StatusCode::SERVICE_UNAVAILABLE, self.to_string()),
            ApiError::FileSystemError(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            ApiError::IoError(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            ApiError::WebDriverError(_) => (StatusCode::SERVICE_UNAVAILABLE, self.to_string()),
        };

        let body = Json(ErrorResponse {
            success: false,
            error: error_message,
        });

        (status, body).into_response()
    }
}

pub type Result<T> = std::result::Result<T, ApiError>;
