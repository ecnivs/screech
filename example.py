#!/usr/bin/env python3
"""
Screech API Example Script
"""

import requests
import json
import time
import os
import argparse

API_BASE_URL = "http://localhost:3000"

def test_health():
    print("Testing health endpoint...")
    response = requests.get(f"{API_BASE_URL}/health")
    print(f"Status: {response.status_code}")
    print(f"Response: {response.json()}")
    print()

def take_screenshot(url, width=1920, height=1080, convert_to_file=False):
    print(f"Taking screenshot of {url}...")
    
    payload = {
        "url": url,
        "width": width,
        "height": height
    }
    
    response = requests.post(
        f"{API_BASE_URL}/screenshot",
        headers={"Content-Type": "application/json"},
        data=json.dumps(payload)
    )
    
    print(f"Status: {response.status_code}")
    result = response.json()
    
    if result["success"]:
        print(f"API Response - Screenshot ID: {result['data']['id']}")
        print(f"Server file path: {result['data']['file_path']}")
        print(f"Timestamp: {result['data']['timestamp']}")
        
        if convert_to_file:
            convert_api_data_to_file(result['data'], 'screenshot')
    else:
        print(f"Error: {result['error']}")
    
    print()
    return result

def record_screencast(url, duration, width=1920, height=1080, convert_to_file=False):
    print(f"Recording {duration}s screencast of {url}...")
    
    payload = {
        "url": url,
        "duration": duration,
        "width": width,
        "height": height
    }
    
    response = requests.post(
        f"{API_BASE_URL}/recording",
        headers={"Content-Type": "application/json"},
        data=json.dumps(payload)
    )
    
    print(f"Status: {response.status_code}")
    result = response.json()
    
    if result["success"]:
        print(f"API Response - Recording ID: {result['data']['id']}")
        print(f"Server file path: {result['data']['file_path']}")
        print(f"Duration: {result['data']['duration']}s")
        print(f"Timestamp: {result['data']['timestamp']}")
        
        if convert_to_file:
            convert_api_data_to_file(result['data'], 'recording')
    else:
        print(f"Error: {result['error']}")
    
    print()
    return result

def convert_api_data_to_file(api_data, file_type):
    try:
        os.makedirs("downloads", exist_ok=True)
        
        extension = "png" if file_type == "screenshot" else "mp4"
        local_filename = f"{file_type}_{api_data['id']}.{extension}"
        local_path = os.path.join("downloads", local_filename)
        if os.path.exists(api_data['file_path']):
            with open(api_data['file_path'], 'rb') as src, open(local_path, 'wb') as dst:
                dst.write(src.read())
            print(f"✅ Converted to local file: {local_path}")
            print(f"   File size: {os.path.getsize(local_path)} bytes")
        else:
            print(f"❌ Server file not found: {api_data['file_path']}")
    except Exception as e:
        print(f"❌ Error converting file: {e}")

def demonstrate_data_usage(api_data, file_type):
    print(f"📊 API Data Usage Example for {file_type}:")
    print(f"   ID: {api_data['id']}")
    print(f"   URL: {api_data['url']}")
    print(f"   Timestamp: {api_data['timestamp']}")
    if file_type == "recording":
        print(f"   Duration: {api_data['duration']}s")
    print(f"   Server Path: {api_data['file_path']}")
    print(f"   You can use this data to:")
    print(f"   - Store in database with ID: {api_data['id']}")
    print(f"   - Create download links")
    print(f"   - Track file metadata")
    print(f"   - Process files as needed")
    print()

def main():
    parser = argparse.ArgumentParser(description="Screech API Example Script")
    parser.add_argument("--convert", action="store_true", 
                       help="Convert API data to local files")
    parser.add_argument("--url", default="https://github.com", 
                       help="URL to capture (default: https://github.com)")
    parser.add_argument("--duration", type=int, default=5, 
                       help="Recording duration in seconds (default: 5)")
    parser.add_argument("--width", type=int, default=1920, 
                       help="Viewport width (default: 1920)")
    parser.add_argument("--height", type=int, default=1080, 
                       help="Viewport height (default: 1080)")
    
    args = parser.parse_args()
    
    print("Screech API Example Script")
    print("=" * 50)
    print(f"Convert to files: {args.convert}")
    print(f"Target URL: {args.url}")
    print(f"Recording duration: {args.duration}s")
    print(f"Viewport: {args.width}x{args.height}")
    print()
    
    # Test health endpoint
    test_health()
    
    # Test screenshot
    print("🖼️  SCREENSHOT EXAMPLE")
    print("-" * 30)
    screenshot_result = take_screenshot(
        args.url, 
        width=args.width, 
        height=args.height, 
        convert_to_file=args.convert
    )
    
    if screenshot_result["success"]:
        demonstrate_data_usage(screenshot_result["data"], "screenshot")
    
    # Test recording
    print("🎥 RECORDING EXAMPLE")
    print("-" * 30)
    recording_result = record_screencast(
        args.url, 
        args.duration, 
        width=args.width, 
        height=args.height, 
        convert_to_file=args.convert
    )
    
    if recording_result["success"]:
        demonstrate_data_usage(recording_result["data"], "recording")
    
    print("✅ Examples completed!")
    
    if args.convert:
        print(f"\n📁 Files converted to: {os.path.abspath('downloads')}")
    else:
        print("\n💡 Tip: Use --convert to save files locally")
        print("💡 The API returns data that you can use however you need!")

if __name__ == "__main__":
    main()
