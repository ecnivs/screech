# Screech

A Rust-based web API for capturing screenshots and recording screencasts of websites using headless Chrome.

## Features

- **Screenshot Capture**: Take screenshots of any website
- **Screencast Recording**: Record videos of websites with customizable duration
- **RESTful API**: Clean JSON API with proper error handling
- **Data-First Approach**: Returns structured data for flexible usage

## Prerequisites

- **Rust** (latest stable version)
- **Chrome/Chromium** browser
- **FFmpeg** (for video recording functionality)

### Installing Prerequisites

#### Ubuntu/Debian:
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Install Chrome
sudo apt update
sudo apt install google-chrome-stable

# Install FFmpeg
sudo apt install ffmpeg
```

#### Arch Linux:
```bash
# Install Rust
sudo pacman -S rust

# Install Chrome
sudo pacman -S google-chrome

# Install FFmpeg
sudo pacman -S ffmpeg
```

## Installation

1. Clone or download this project
2. Navigate to the project directory
3. Build the project:

```bash
cargo build --release
```

## Running the API

Start the server:

```bash
cargo run
```

The API will be available at `http://localhost:3000`

## API Endpoints

### Health Check

**GET** `/health`

Check if the API is running.

```bash
curl http://localhost:3000/health
```

### Take Screenshot

**POST** `/screenshot`

Captures a screenshot of the specified URL.

```bash
curl -X POST http://localhost:3000/screenshot \
  -H "Content-Type: application/json" \
  -d '{
    "url": "https://github.com",
    "width": 1920,
    "height": 1080
  }'
```

**Parameters:**
- `url` (required): The website URL to screenshot
- `width` (optional): Viewport width in pixels
- `height` (optional): Viewport height in pixels

### Record Screencast

**POST** `/recording`

Records a video of the specified URL for the given duration.

```bash
curl -X POST http://localhost:3000/recording \
  -H "Content-Type: application/json" \
  -d '{
    "url": "https://github.com",
    "duration": 10,
    "width": 1920,
    "height": 1080
  }'
```

**Parameters:**
- `url` (required): The website URL to record
- `duration` (required): Recording duration in seconds (1-300)
- `width` (optional): Viewport width in pixels
- `height` (optional): Viewport height in pixels

## Response Format

All endpoints return JSON responses in this format:

```json
{
  "success": true,
  "data": {
    "id": "unique-uuid",
    "url": "https://example.com",
    "file_path": "/tmp/server/path/file.png",
    "timestamp": "2025-10-28T06:43:17Z",
    "duration": 5
  },
  "error": null
}
```

**Response Fields:**
- `success`: Boolean indicating if the request was successful
- `data`: Object containing the result data (null if error)
- `error`: Error message string (null if successful)

**Data Fields:**
- `id`: Unique identifier for the capture/recording
- `url`: The URL that was captured
- `file_path`: Server path where the file is stored
- `timestamp`: ISO 8601 timestamp of when the capture was made
- `duration`: Duration in seconds (only for recordings)

## Error Handling

The API returns appropriate HTTP status codes and error messages:

- **400 Bad Request**: Invalid URL format
- **422 Unprocessable Entity**: Screenshot/recording failed
- **500 Internal Server Error**: Server-side errors

**Error Response Format:**
```json
{
  "success": false,
  "data": null,
  "error": "Error description"
}
```

## Usage Examples

### Basic Screenshot
```bash
curl -X POST http://localhost:3000/screenshot \
  -H "Content-Type: application/json" \
  -d '{"url": "https://example.com"}'
```

### Custom Viewport Screenshot
```bash
curl -X POST http://localhost:3000/screenshot \
  -H "Content-Type: application/json" \
  -d '{
    "url": "https://example.com",
    "width": 1280,
    "height": 720
  }'
```

### Short Recording
```bash
curl -X POST http://localhost:3000/recording \
  -H "Content-Type: application/json" \
  -d '{
    "url": "https://example.com",
    "duration": 5
  }'
```

### Long Recording with Custom Viewport
```bash
curl -X POST http://localhost:3000/recording \
  -H "Content-Type: application/json" \
  -d '{
    "url": "https://example.com",
    "duration": 30,
    "width": 1920,
    "height": 1080
  }'
```

## File Management

- Screenshots are saved as PNG files
- Recordings are saved as MP4 files
- Files are stored in temporary directories
- Files are automatically cleaned up when the server shuts down
- Each file has a unique UUID-based filename

## Configuration

The API runs on port 3000 by default. To change this, modify the `main.rs` file:

```rust
let listener = tokio::net::TcpListener::bind("0.0.0.0:YOUR_PORT").await?;
```

## Security Considerations

- The API runs Chrome in headless mode with sandbox disabled
- URLs are validated to ensure they start with http:// or https://
- Duration is limited to prevent excessive resource usage
- Files are stored in temporary directories

## Troubleshooting

### Common Issues

1. **Chrome not found**: Make sure Chrome/Chromium is installed and accessible
2. **FFmpeg not found**: Install FFmpeg for video recording functionality
3. **Permission denied**: Ensure the application has write permissions to temp directories
4. **Port already in use**: Change the port in the configuration

### Debug Mode

To run with debug logging, set the `RUST_LOG` environment variable:

```bash
RUST_LOG=debug cargo run
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

This project is open source and available under the MIT License.
