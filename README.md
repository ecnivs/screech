<h1 align="center">Screech API</h1>

<p align="center"><em>capture screenshots of websites</em></p>


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
  -d '{"url":"https://github.com","width":1920,"height":1080,"delay":2}'
```

**Params**
| Field | Type | Required | Description |
|--------|------|-----------|-------------|
| `url` | string | Yes | Website URL |
| `width` | number | No | Viewport width (100-4096) |
| `height` | number | No | Viewport height (100-4096) |
| `delay` | number | No | Delay in seconds before screenshot (0-60, default: 2) |


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

**Note:** The `image_data` field contains base64-encoded binary data that can be decoded and saved as PNG files.

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

#### ChromeDriver Setup

Download and install ChromeDriver:

```bash
# Download ChromeDriver (replace with your Chrome version)
wget https://chromedriver.storage.googleapis.com/LATEST_RELEASE
CHROME_VERSION=$(cat LATEST_RELEASE)
wget https://chromedriver.storage.googleapis.com/${CHROME_VERSION}/chromedriver_linux64.zip
unzip chromedriver_linux64.zip
sudo mv chromedriver /usr/local/bin/
sudo chmod +x /usr/local/bin/chromedriver
```

Or use package manager:

```bash
# Ubuntu/Debian
sudo apt-get install chromium-chromedriver

# Arch Linux
sudo pacman -S chromium
```

## 🚀 Running Screech
```
cargo run
```
