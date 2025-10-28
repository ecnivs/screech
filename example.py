#!/usr/bin/env python3

import argparse
import base64
import json
import os
from typing import Dict, Any

import requests

API_BASE_URL = "http://localhost:3000"
DEFAULT_VIEWPORT = {"width": 1920, "height": 1080}
DEFAULT_URL = (
    "https://giphy.com/gifs/rickroll-rick-astley-never-gonna-give-you-up-Vuw9m5wXviFIQ"
)


def test_health() -> bool:
    print("Testing health endpoint...")
    try:
        response = requests.get(f"{API_BASE_URL}/health", timeout=10)
        response.raise_for_status()
        print(f"Status: {response.status_code}")
        print(f"Response: {response.json()}")
        print()
        return True
    except requests.RequestException as e:
        print(f"❌ Health check failed: {e}")
        print()
        return False


def take_screenshot(
    url: str,
    width: int = DEFAULT_VIEWPORT["width"],
    height: int = DEFAULT_VIEWPORT["height"],
    save_to_file: bool = True,
) -> Dict[str, Any]:
    print(f"Taking screenshot of {url}...")

    payload = {"url": url, "width": width, "height": height}

    try:
        response = requests.post(
            f"{API_BASE_URL}/screenshot",
            headers={"Content-Type": "application/json"},
            data=json.dumps(payload),
            timeout=30,
        )
        response.raise_for_status()

        print(f"Status: {response.status_code}")
        result = response.json()

        if result["success"]:
            print(f"API Response - Screenshot ID: {result['data']['id']}")
            print(f"Image data size: {len(result['data']['image_data'])} characters")
            print(f"Timestamp: {result['data']['timestamp']}")

            if save_to_file:
                save_image_to_file(result["data"], "screenshot")
        else:
            print(f"Error: {result['error']}")

        print()
        return result

    except requests.RequestException as e:
        print(f"❌ Screenshot request failed: {e}")
        print()
        return {"success": False, "error": str(e)}


def save_image_to_file(api_data: Dict[str, Any], file_type: str) -> bool:
    try:
        os.makedirs("downloads", exist_ok=True)

        local_filename = f"{file_type}_{api_data['id']}.png"
        local_path = os.path.join("downloads", local_filename)

        image_data = base64.b64decode(api_data["image_data"])
        with open(local_path, "wb") as f:
            f.write(image_data)

        print(f"✅ Saved image to: {local_path}")
        print(f"   File size: {os.path.getsize(local_path)} bytes")
        return True
    except Exception as e:
        print(f"❌ Error saving image: {e}")
        return False


def demonstrate_data_usage(api_data: Dict[str, Any]) -> None:
    print(f"📊 API Data Usage Example:")
    print(f"   ID: {api_data['id']}")
    print(f"   URL: {api_data['url']}")
    print(f"   Timestamp: {api_data['timestamp']}")
    data_size = len(api_data["image_data"])
    print(f"   Data size: {data_size} characters (base64)")
    print(f"   Binary size: ~{data_size * 3 // 4} bytes")
    print()


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Screech API Example Script",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  python example.py --url https://example.com
  python example.py --no-save
  python example.py --width 1280 --height 720
        """,
    )
    parser.add_argument(
        "--no-save", action="store_true", help="Don't save files locally"
    )
    parser.add_argument(
        "--url", default=DEFAULT_URL, help=f"URL to capture (default: {DEFAULT_URL})"
    )
    parser.add_argument(
        "--width",
        type=int,
        default=DEFAULT_VIEWPORT["width"],
        help=f"Viewport width (default: {DEFAULT_VIEWPORT['width']})",
    )
    parser.add_argument(
        "--height",
        type=int,
        default=DEFAULT_VIEWPORT["height"],
        help=f"Viewport height (default: {DEFAULT_VIEWPORT['height']})",
    )

    args = parser.parse_args()

    print("Screech API Example Script")
    print("=" * 50)
    print(f"Save to files: {not args.no_save}")
    print(f"Target URL: {args.url}")
    print(f"Viewport: {args.width}x{args.height}")
    print()

    if not test_health():
        print("❌ API is not available. Please ensure the Screech server is running.")
        return

    print("🖼️  SCREENSHOT EXAMPLE")
    print("-" * 30)
    screenshot_result = take_screenshot(
        args.url, width=args.width, height=args.height, save_to_file=not args.no_save
    )

    if screenshot_result.get("success"):
        demonstrate_data_usage(screenshot_result["data"])

    print("✅ Examples completed!")

    if not args.no_save:
        print(f"\n📁 Files saved to: {os.path.abspath('downloads')}")
    else:
        print("\n💡 Files not saved (use default behavior to save files)")
        print("💡 The API returns base64 encoded data that you can use directly!")


if __name__ == "__main__":
    main()
