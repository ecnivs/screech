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

```json
{
  "success": true,
  "data": {
    "id": "uuid",
    "url": "https://example.com",
    "file_path": "/tmp/screech/file.png",
    "timestamp": "2025-10-28T06:43:17Z",
    "duration": 5
  },
  "error": null
}
```

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
git clone https://github.com/yourusername/screech
cd screech
cargo build --release
```

## 🚀 Running Screech
```
cargo run
```
