<h1 align="center">Screech</h1>

<p align="center"><em>API for capturing screenshots and recording screencasts of websites</em></p>


## 🛠️ Prerequisites

- **Rust** (latest stable)
- **Google Chrome / Chromium**
- **FFmpeg**

## API Overview

#### Health Check

**GET** `/health`  
Check if the server is running.

```bash
curl http://localhost:3000/health
```

#### Screenshot

**POST** `/screenshot`

Take a screenshot of a given URL.

```bash
curl -X POST http://localhost:3000/screenshot \
  -H "Content-Type: application/json" \
  -d '{"url":"https://github.com","width":1920,"height":1080}'
```

**Params**
| Field | Type | Required | Description |
|--------|------|-----------|-------------|
| `url` | string | Yes | Website URL |
| `width` | number | No | Viewport width |
| `height` | number | No | Viewport height |


#### Recording

**POST** `/recording`

Record a video of a website for a specific duration.

```bash
curl -X POST http://localhost:3000/recording \
  -H "Content-Type: application/json" \
  -d '{"url":"https://github.com","duration":10,"width":1920,"height":1080}'
```

**Params**
| Field | Type | Required | Description |
|--------|------|-----------|-------------|
| `url` | string | Yes | Website URL |
| `duration` | number | Yes | Recording duration (1–300 sec) |
| `width` | number | No | Viewport width |
| `height` | number | No | Viewport height |


#### Response Format

**Screenshot Response**
```json
{
  "success": true,
  "data": {
    "id": "uuid",
    "url": "https://example.com",
    "image_data": "iVBORw0KGgoAAAANSUhEUgAA...",
    "timestamp": "2025-10-28T06:43:17Z"
  },
  "error": null
}
```

**Recording Response**
```json
{
  "success": true,
  "data": {
    "id": "uuid",
    "url": "https://example.com",
    "video_data": "AAAAIGZ0eXBpc29tAAACAGlzb21pc28y...",
    "duration": 5,
    "timestamp": "2025-10-28T06:43:17Z"
  },
  "error": null
}
```

**Note:** The `image_data` and `video_data` fields contain base64-encoded binary data that can be decoded and saved as PNG/MP4 files.

**Error Example**

```json
{
  "success": false,
  "data": null,
  "error": "Invalid URL format"
}
```

## 📦 Installation

```bash
git clone https://github.com/ecnivs/screech
cd screech
cargo build --release
```

## 🚀 Running Screech
```
cargo run
```
