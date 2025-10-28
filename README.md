<h1 align="center">Screech API</h1>

<p align="center"><em>capture screenshots of websites</em></p>

<p align="center">
  <a href="https://github.com/ecnivs/screech-api/stargazers">
    <img src="https://img.shields.io/github/stars/ecnivs/screech-api?style=flat-square">
  </a>
  <a href="https://github.com/ecnivs/screech-api/issues">
    <img src="https://img.shields.io/github/issues/ecnivs/screech-api?style=flat-square">
  </a>
  <a href="https://github.com/ecnivs/screech-api/blob/master/LICENSE">
    <img src="https://img.shields.io/github/license/ecnivs/screech-api?style=flat-square">
  </a>
  <img src="https://img.shields.io/github/languages/top/ecnivs/screech-api?style=flat-square">
</p>

## Overview
**Screech API** is a Rust-based service that captures screenshots of websites through a simple HTTP interface. You send it a URL, and it returns a base64-encoded PNG image of the rendered page. It’s built for speed, reliability, and easy integration into automated systems like testing pipelines, preview generators, or monitoring tools.

## 🧩 API Specification

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
  -d '{"url":"https://github.com/ecnivs","width":1920,"height":1080,"delay":2}'
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

## 💖 Support the project
If you find this project helpful and want to support its development, donations are welcome!  
Your support helps keep the project active and enables new features.
<div align="center">
  <a href="https://www.buymeacoffee.com/ecnivs" target="_blank"><img src="https://cdn.buymeacoffee.com/buttons/v2/default-yellow.png" alt="Buy Me A Coffee" style="height: 60px !important;width: 217px !important;" ></a>
</div>

## 🙌 Contributing
Feel free to:
1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Submit a pull request

#### *I'd appreciate any feedback or code reviews you might have!*
