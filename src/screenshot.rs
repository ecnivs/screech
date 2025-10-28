use crate::error::{ApiError, Result};
use thirtyfour::{DesiredCapabilities, WebDriver};
use std::time::Duration;

pub struct ScreenshotService;

impl ScreenshotService {
    pub fn new() -> Self {
        Self
    }

    pub async fn capture_screenshot(
        &self,
        url: &str,
        width: Option<u32>,
        height: Option<u32>,
    ) -> Result<Vec<u8>> {
        if !url.starts_with("http://") && !url.starts_with("https://") {
            return Err(ApiError::InvalidUrl("URL must start with http:// or https://".to_string()));
        }

        let viewport_width = width.unwrap_or(1920);
        let viewport_height = height.unwrap_or(1080);

        let mut caps = DesiredCapabilities::chrome();
        caps.add_chrome_arg("--headless")?;
        caps.add_chrome_arg("--no-sandbox")?;
        caps.add_chrome_arg("--disable-dev-shm-usage")?;
        caps.add_chrome_arg(&format!("--window-size={},{}", viewport_width, viewport_height))?;

        let driver = WebDriver::new("http://localhost:9515", caps)
            .await
            .map_err(|e| ApiError::BrowserError(format!("Failed to create driver: {}", e)))?;

        driver.goto(url)
            .await
            .map_err(|e| ApiError::ScreenshotError(format!("Failed to navigate to URL: {}", e)))?;

        tokio::time::sleep(Duration::from_secs(2)).await;

        let temp_file = tempfile::NamedTempFile::new()
            .map_err(|e| ApiError::FileSystemError(format!("Failed to create temp file: {}", e)))?;
        
        let temp_path = temp_file.path();
        
        driver.screenshot(temp_path)
            .await
            .map_err(|e| ApiError::ScreenshotError(format!("Failed to capture screenshot: {}", e)))?;

        let screenshot_data = tokio::fs::read(temp_path)
            .await
            .map_err(|e| ApiError::FileSystemError(format!("Failed to read screenshot: {}", e)))?;

        driver.quit()
            .await
            .map_err(|e| ApiError::BrowserError(format!("Failed to quit driver: {}", e)))?;

        Ok(screenshot_data)
    }
}