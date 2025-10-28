use crate::error::{ApiError, Result};
use headless_chrome::Browser;
use headless_chrome::protocol::page::ScreenshotFormat;
use std::path::PathBuf;
use tempfile::TempDir;
use uuid::Uuid;
use std::process::Command;

pub struct RecordingService {
    temp_dir: TempDir,
}

impl RecordingService {
    pub fn new() -> Self {
        let temp_dir = tempfile::tempdir().expect("Failed to create temp directory");
        Self { temp_dir }
    }

    pub async fn record_screencast(
        &self,
        url: &str,
        duration: u32,
        _width: Option<u32>,
        _height: Option<u32>,
    ) -> Result<Vec<u8>> {
        if !url.starts_with("http://") && !url.starts_with("https://") {
            return Err(ApiError::InvalidUrl("URL must start with http:// or https://".to_string()));
        }

        if duration == 0 || duration > 300 {
            return Err(ApiError::RecordingError("Duration must be between 1 and 300 seconds".to_string()));
        }

        let browser = Browser::default()
            .map_err(|e| ApiError::BrowserError(format!("Failed to launch browser: {}", e)))?;

        let tab = browser.new_tab().map_err(|e| ApiError::BrowserError(format!("Failed to create new tab: {}", e)))?;

        tab.navigate_to(url)
            .map_err(|e| ApiError::RecordingError(format!("Failed to navigate to URL: {}", e)))?;

        tab.wait_until_navigated()
            .map_err(|e| ApiError::RecordingError(format!("Failed to wait for navigation: {}", e)))?;

        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        let filename = format!("recording_{}.mp4", Uuid::new_v4());
        let file_path = self.temp_dir.path().join(&filename);

        self.create_video_from_screenshots(&tab, duration, &file_path).await?;

        let video_data = tokio::fs::read(&file_path)
            .await
            .map_err(|e| ApiError::FileSystemError(format!("Failed to read video file: {}", e)))?;

        tokio::fs::remove_file(&file_path)
            .await
            .map_err(|e| ApiError::FileSystemError(format!("Failed to cleanup video file: {}", e)))?;

        Ok(video_data)
    }

    async fn create_video_from_screenshots(
        &self,
        tab: &headless_chrome::Tab,
        duration: u32,
        output_path: &PathBuf,
    ) -> Result<()> {
        let fps = 10;
        let total_frames = duration * fps;
        let frame_duration = 1000 / fps;

        let mut frame_paths = Vec::new();

        for frame_num in 0..total_frames {
            let screenshot_data = tab.capture_screenshot(
                ScreenshotFormat::PNG,
                None,
                true,
            ).map_err(|e| ApiError::RecordingError(format!("Failed to capture frame {}: {}", frame_num, e)))?;

            let frame_filename = format!("frame_{:04}.png", frame_num);
            let frame_path = self.temp_dir.path().join(&frame_filename);

            tokio::fs::write(&frame_path, screenshot_data)
                .await
                .map_err(|e| ApiError::FileSystemError(format!("Failed to save frame {}: {}", frame_num, e)))?;

            frame_paths.push(frame_path.clone());

            tokio::time::sleep(tokio::time::Duration::from_millis(frame_duration as u64)).await;
        }

        self.create_video_from_frames(&frame_paths, output_path, fps).await?;

        for frame_path in frame_paths {
            let _ = tokio::fs::remove_file(frame_path).await;
        }

        Ok(())
    }

    async fn create_video_from_frames(
        &self,
        _frame_paths: &[PathBuf],
        output_path: &PathBuf,
        fps: u32,
    ) -> Result<()> {
        let output = Command::new("ffmpeg")
            .arg("-framerate")
            .arg(fps.to_string())
            .arg("-i")
            .arg(self.temp_dir.path().join("frame_%04d.png").to_string_lossy().as_ref())
            .arg("-vf")
            .arg("scale=1920:1080")
            .arg("-c:v")
            .arg("libx264")
            .arg("-pix_fmt")
            .arg("yuv420p")
            .arg("-preset")
            .arg("fast")
            .arg("-crf")
            .arg("23")
            .arg("-y")
            .arg(output_path)
            .output()
            .map_err(|e| ApiError::RecordingError(format!("Failed to run ffmpeg: {}", e)))?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(ApiError::RecordingError(format!("ffmpeg failed: {}", error_msg)));
        }

        Ok(())
    }
}
