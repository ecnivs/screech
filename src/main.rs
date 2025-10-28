use axum::{
    extract::State,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use uuid::Uuid;

mod screenshot;
mod recording;
mod error;

use error::Result;

#[derive(Clone)]
struct AppState {
    screenshot_service: Arc<screenshot::ScreenshotService>,
    recording_service: Arc<recording::RecordingService>,
}

#[derive(Deserialize)]
struct ScreenshotRequest {
    url: String,
    width: Option<u32>,
    height: Option<u32>,
}

#[derive(Deserialize)]
struct RecordingRequest {
    url: String,
    duration: u32,
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
    file_path: String,
    timestamp: String,
}

#[derive(Serialize)]
struct RecordingResponse {
    id: String,
    url: String,
    file_path: String,
    duration: u32,
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

    let screenshot_path = state
        .screenshot_service
        .capture_screenshot(&request.url, request.width, request.height)
        .await?;

    let response = ScreenshotResponse {
        id,
        url: request.url,
        file_path: screenshot_path,
        timestamp,
    };

    Ok(Json(ApiResponse {
        success: true,
        data: Some(response),
        error: None,
    }))
}

async fn record_screencast(
    State(state): State<AppState>,
    Json(request): Json<RecordingRequest>,
) -> Result<Json<ApiResponse<RecordingResponse>>> {
    let id = Uuid::new_v4().to_string();
    let timestamp = chrono::Utc::now().to_rfc3339();

    let recording_path = state
        .recording_service
        .record_screencast(&request.url, request.duration, request.width, request.height)
        .await?;

    let response = RecordingResponse {
        id,
        url: request.url,
        file_path: recording_path,
        duration: request.duration,
        timestamp,
    };

    Ok(Json(ApiResponse {
        success: true,
        data: Some(response),
        error: None,
    }))
}

#[tokio::main]
async fn main() -> Result<()> {
    let screenshot_service = Arc::new(screenshot::ScreenshotService::new());
    let recording_service = Arc::new(recording::RecordingService::new());

    let app_state = AppState {
        screenshot_service,
        recording_service,
    };

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/screenshot", post(take_screenshot))
        .route("/recording", post(record_screencast))
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
        )
        .with_state(app_state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await
        .map_err(|e| error::ApiError::IoError(e))?;
    println!("Screech API server running on http://0.0.0.0:3000");

    axum::serve(listener, app).await
        .map_err(|e| error::ApiError::IoError(e))?;

    Ok(())
}
