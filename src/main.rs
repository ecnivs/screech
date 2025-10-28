use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, compression::CompressionLayer};
use uuid::Uuid;

mod screenshot;
mod error;

use error::Result;

#[derive(Clone)]
struct AppState {
    screenshot_service: Arc<screenshot::ScreenshotService>,
}

#[derive(Deserialize)]
struct ScreenshotRequest {
    url: String,
    width: Option<u32>,
    height: Option<u32>,
}

impl ScreenshotRequest {
    fn validate(&self) -> Result<()> {
        if self.url.trim().is_empty() {
            return Err(error::ApiError::InvalidRequest("URL cannot be empty".to_string()));
        }

        if let Some(width) = self.width {
            if width < 100 || width > 4096 {
                return Err(error::ApiError::InvalidRequest("Width must be between 100 and 4096 pixels".to_string()));
            }
        }

        if let Some(height) = self.height {
            if height < 100 || height > 4096 {
                return Err(error::ApiError::InvalidRequest("Height must be between 100 and 4096 pixels".to_string()));
            }
        }

        Ok(())
    }
}

#[derive(Serialize)]
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
}

#[derive(Serialize)]
struct ScreenshotResponse {
    id: String,
    url: String,
    image_data: String,
    timestamp: String,
}
async fn health_check() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert("content-type", "application/json".parse().unwrap());

    (StatusCode::OK, headers, Json(ApiResponse {
        success: true,
        data: Some("API is running"),
        error: None,
    }))
}

async fn take_screenshot(
    State(state): State<AppState>,
    Json(request): Json<ScreenshotRequest>,
) -> Result<impl IntoResponse> {
    request.validate()?;

    let id = Uuid::new_v4().to_string();
    let timestamp = chrono::Utc::now().to_rfc3339();

    println!("Taking screenshot for URL: {} (ID: {})", request.url, id);

    let screenshot_data = state
        .screenshot_service
        .capture_screenshot(&request.url, request.width, request.height)
        .await?;

    let response = ScreenshotResponse {
        id: id.clone(),
        url: request.url,
        image_data: base64::Engine::encode(&base64::engine::general_purpose::STANDARD, screenshot_data),
        timestamp,
    };

    let mut headers = HeaderMap::new();
    headers.insert("content-type", "application/json".parse().unwrap());
    headers.insert("cache-control", "no-cache".parse().unwrap());

    println!("Screenshot completed successfully (ID: {})", id);

    Ok((StatusCode::OK, headers, Json(ApiResponse {
        success: true,
        data: Some(response),
        error: None,
    })))
}

fn start_chromedriver() -> Result<()> {
    println!("Starting ChromeDriver...");

    let mut child = Command::new("chromedriver")
        .arg("--port=9515")
        .arg("--log-level=WARNING")
        .arg("--disable-dev-shm-usage")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| error::ApiError::ServiceUnavailable(format!(
            "Failed to start ChromeDriver: {}. Make sure ChromeDriver is installed and in PATH.", e
        )))?;

    thread::sleep(Duration::from_secs(3));

    match child.try_wait() {
        Ok(Some(status)) => {
            return Err(error::ApiError::ServiceUnavailable(format!(
                "ChromeDriver exited early with status: {:?}. Check if ChromeDriver is properly installed.", status
            )));
        }
        Ok(None) => {
            println!("ChromeDriver started successfully on port 9515");
        }
        Err(e) => {
            return Err(error::ApiError::ServiceUnavailable(format!(
                "Failed to check ChromeDriver status: {}", e
            )));
        }
    }

    std::mem::forget(child);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting Screech API server...");

    start_chromedriver()?;

    let screenshot_service = Arc::new(screenshot::ScreenshotService::new());

    let app_state = AppState {
        screenshot_service,
    };

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/screenshot", post(take_screenshot))
        .layer(
            ServiceBuilder::new()
                .layer(CompressionLayer::new())
                .layer(CorsLayer::permissive())
        )
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await
        .map_err(|e| error::ApiError::IoError(std::io::Error::new(
            std::io::ErrorKind::AddrInUse,
            format!("Failed to bind to port 3000: {}. Port may be in use.", e)
        )))?;

    println!("Screech API server running on http://0.0.0.0:3000");
    println!("Health check: GET /health");
    println!("Screenshot: POST /screenshot");

    axum::serve(listener, app).await
        .map_err(|e| error::ApiError::IoError(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Server error: {}", e)
        )))?;

    Ok(())
}
