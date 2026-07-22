#!/usr/bin/env python3
"""
Generate a new runner API key (must be runner owner or admin).

Usage:
  python create_runner_api_key.py --base-url http://localhost:5288 \
      --email admin@example.com --password secret \
      --runner-id <runner-uuid> --name "my-runner-key" --expiry-days 365
"""

import argparse
import requests


def main():
    parser = argparse.ArgumentParser(description="Generate a runner API key.")
    parser.add_argument("--base-url", required=True)
    parser.add_argument("--email", required=True)
    parser.add_argument("--password", required=True)
    parser.add_argument("--runner-id", required=True, help="UUID of the runner")
    parser.add_argument("--name", required=True, help="Display name for the key")
    parser.add_argument("--expiry-days", type=int, default=365)
    args = parser.parse_args()

    # Login
    resp = requests.post(f"{args.base_url}/api/auth/login",
                         json={"email": args.email, "password": args.password})
    resp.raise_for_status()
    token = resp.json()["token"]
    print(f"Logged in as {args.email}")

    # Create runner API key
    resp = requests.post(f"{args.base_url}/api/runner/{args.runner_id}/key",
                         headers={"Authorization": f"Bearer {token}"},
                         json={"displayName": args.name, "expiryInDays": args.expiry_days})
    resp.raise_for_status()
    api_key = resp.json()["apiKey"]
    print(f"Runner API Key: {api_key}")


if __name__ == "__main__":
    main()
