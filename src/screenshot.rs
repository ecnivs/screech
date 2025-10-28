use crate::error::{ApiError, Result};
use headless_chrome::Browser;
use headless_chrome::protocol::page::ScreenshotFormat;

pub struct ScreenshotService;

impl ScreenshotService {
    pub fn new() -> Self {
        Self
    }

    pub async fn capture_screenshot(
        &self,
        url: &str,
        _width: Option<u32>,
        _height: Option<u32>,
    ) -> Result<Vec<u8>> {
        if !url.starts_with("http://") && !url.starts_with("https://") {
            return Err(ApiError::InvalidUrl("URL must start with http:// or https://".to_string()));
        }

        let browser = Browser::default()
            .map_err(|e| ApiError::BrowserError(format!("Failed to launch browser: {}", e)))?;

        let tab = browser.new_tab().map_err(|e| ApiError::BrowserError(format!("Failed to create new tab: {}", e)))?;

        tab.navigate_to(url)
            .map_err(|e| ApiError::ScreenshotError(format!("Failed to navigate to URL: {}", e)))?;

        tab.wait_until_navigated()
            .map_err(|e| ApiError::ScreenshotError(format!("Failed to wait for navigation: {}", e)))?;

        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        let screenshot_data = tab.capture_screenshot(
            ScreenshotFormat::PNG,
            None,
            true,
        ).map_err(|e| ApiError::ScreenshotError(format!("Failed to capture screenshot: {}", e)))?;

        Ok(screenshot_data)
    }
}
