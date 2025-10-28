use axum::{
    extract::State,
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
use tower_http::cors::CorsLayer;
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
    Json(ApiResponse {
        success: true,
        data: Some("API is running"),
        error: None,
    })
}

async fn take_screenshot(
    State(state): State<AppState>,
    Json(request): Json<ScreenshotRequest>,
) -> Result<Json<ApiResponse<ScreenshotResponse>>> {
    let id = Uuid::new_v4().to_string();
    let timestamp = chrono::Utc::now().to_rfc3339();

    let screenshot_data = state
        .screenshot_service
        .capture_screenshot(&request.url, request.width, request.height)
        .await?;

    let response = ScreenshotResponse {
        id,
        url: request.url,
        image_data: base64::Engine::encode(&base64::engine::general_purpose::STANDARD, screenshot_data),
        timestamp,
    };

    Ok(Json(ApiResponse {
        success: true,
        data: Some(response),
        error: None,
    }))
}

fn start_chromedriver() -> Result<()> {
    println!("Starting ChromeDriver...");

    let mut child = Command::new("chromedriver")
        .arg("--port=9515")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| error::ApiError::IoError(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to start ChromeDriver: {}", e)
        )))?;

    thread::sleep(Duration::from_secs(2));

    match child.try_wait() {
        Ok(Some(status)) => {
            return Err(error::ApiError::BrowserError(format!(
                "ChromeDriver exited early with status: {:?}", status
            )));
        }
        Ok(None) => {
            println!("ChromeDriver started successfully on port 9515");
        }
        Err(e) => {
            return Err(error::ApiError::BrowserError(format!(
                "Failed to check ChromeDriver status: {}", e
            )));
        }
    }

    std::mem::forget(child);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
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
                .layer(CorsLayer::permissive())
        )
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await
        .map_err(error::ApiError::IoError)?;

    println!("Screech API server running on http://0.0.0.0:3000");

    axum::serve(listener, app).await
        .map_err(error::ApiError::IoError)?;

    Ok(())
}
