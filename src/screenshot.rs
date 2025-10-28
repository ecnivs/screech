use crate::error::{ApiError, Result};
use thirtyfour::{DesiredCapabilities, WebDriver};
use std::time::Duration;
use tokio::time::timeout;

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

        if url.len() > 2048 {
            return Err(ApiError::InvalidUrl("URL too long (max 2048 characters)".to_string()));
        }

        let viewport_width = width.unwrap_or(1920);
        let viewport_height = height.unwrap_or(1080);

        if viewport_width < 100 || viewport_width > 4096 {
            return Err(ApiError::InvalidRequest("Width must be between 100 and 4096 pixels".to_string()));
        }

        if viewport_height < 100 || viewport_height > 4096 {
            return Err(ApiError::InvalidRequest("Height must be between 100 and 4096 pixels".to_string()));
        }

        let mut caps = DesiredCapabilities::chrome();
        caps.add_chrome_arg("--headless")?;
        caps.add_chrome_arg("--no-sandbox")?;
        caps.add_chrome_arg("--disable-dev-shm-usage")?;
        caps.add_chrome_arg("--disable-gpu")?;
        caps.add_chrome_arg("--disable-extensions")?;
        caps.add_chrome_arg("--disable-plugins")?;
        caps.add_chrome_arg("--disable-images")?;
        caps.add_chrome_arg("--disable-javascript")?;
        caps.add_chrome_arg("--disable-web-security")?;
        caps.add_chrome_arg("--disable-features=VizDisplayCompositor")?;
        caps.add_chrome_arg("--disable-background-timer-throttling")?;
        caps.add_chrome_arg("--disable-renderer-backgrounding")?;
        caps.add_chrome_arg("--disable-backgrounding-occluded-windows")?;
        caps.add_chrome_arg(&format!("--window-size={},{}", viewport_width, viewport_height))?;


        let driver = timeout(
            Duration::from_secs(10),
            WebDriver::new("http://localhost:9515", caps)
        )
        .await
        .map_err(|_| ApiError::BrowserTimeout("Failed to create driver within timeout".to_string()))?
        .map_err(|e| ApiError::BrowserError(format!("Failed to create driver: {}", e)))?;

        timeout(
            Duration::from_secs(30),
            driver.goto(url)
        )
        .await
        .map_err(|_| ApiError::BrowserTimeout("Navigation timeout".to_string()))?
        .map_err(|e| ApiError::ScreenshotError(format!("Failed to navigate to URL: {}", e)))?;

        timeout(
            Duration::from_secs(5),
            tokio::time::sleep(Duration::from_secs(2))
        )
        .await
        .map_err(|_| ApiError::RequestTimeout("Page load timeout".to_string()))?;

        let temp_file = tempfile::NamedTempFile::new()
            .map_err(|e| ApiError::FileSystemError(format!("Failed to create temp file: {}", e)))?;

        let temp_path = temp_file.path();

        timeout(
            Duration::from_secs(15),
            driver.screenshot(temp_path)
        )
        .await
        .map_err(|_| ApiError::BrowserTimeout("Screenshot capture timeout".to_string()))?
        .map_err(|e| ApiError::ScreenshotError(format!("Failed to capture screenshot: {}", e)))?;

        let screenshot_data = timeout(
            Duration::from_secs(10),
            tokio::fs::read(temp_path)
        )
        .await
        .map_err(|_| ApiError::RequestTimeout("File read timeout".to_string()))?
        .map_err(|e| ApiError::FileSystemError(format!("Failed to read screenshot: {}", e)))?;

        let _ = timeout(
            Duration::from_secs(5),
            driver.quit()
        )
        .await;

        Ok(screenshot_data)
    }
}
