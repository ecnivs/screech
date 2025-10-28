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

#[tokio::main]
async fn main() -> Result<()> {
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
