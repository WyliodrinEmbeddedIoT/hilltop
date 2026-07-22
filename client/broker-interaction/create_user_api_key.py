#!/usr/bin/env python3
"""
Generate a new user (client) API key.

Usage:
  python create_user_api_key.py --base-url http://localhost:5288 \
      --email user@example.com --password secret \
      --name "my-key" --expiry-days 365
"""

import argparse
import requests


def main():
    parser = argparse.ArgumentParser(description="Generate a user API key.")
    parser.add_argument("--base-url", required=True)
    parser.add_argument("--email", required=True)
    parser.add_argument("--password", required=True)
    parser.add_argument("--name", required=True, help="Display name for the key")
    parser.add_argument("--expiry-days", type=int, default=365)
    args = parser.parse_args()

    # Login
    resp = requests.post(f"{args.base_url}/api/auth/login",
                         json={"email": args.email, "password": args.password})
    resp.raise_for_status()
    token = resp.json()["token"]
    print(f"Logged in as {args.email}")

    # Create client API key
    resp = requests.post(f"{args.base_url}/api/client/key",
                         headers={"Authorization": f"Bearer {token}"},
                         json={"displayName": args.name, "expiryInDays": args.expiry_days})
    resp.raise_for_status()
    api_key = resp.json()["apiKey"]
    print(f"API Key: {api_key}")


if __name__ == "__main__":
    main()
