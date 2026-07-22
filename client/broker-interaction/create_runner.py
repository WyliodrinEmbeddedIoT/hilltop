#!/usr/bin/env python3
"""
Create a new runner (admin only).

Usage:
  python create_runner.py --base-url http://localhost:5288 \
      --email admin@example.com --password secret \
      --slug my-runner --name "My Runner" --description "A test runner"
"""

import argparse
import requests


def main():
    parser = argparse.ArgumentParser(description="Create a new runner.")
    parser.add_argument("--base-url", required=True)
    parser.add_argument("--email", required=True)
    parser.add_argument("--password", required=True)
    parser.add_argument("--slug", required=True)
    parser.add_argument("--name", required=True)
    parser.add_argument("--description", default="")
    args = parser.parse_args()

    # Login
    resp = requests.post(f"{args.base_url}/api/auth/login",
                         json={"email": args.email, "password": args.password})
    resp.raise_for_status()
    token = resp.json()["token"]
    print(f"Logged in as {args.email}")

    # Get current user ID to set as owner
    resp = requests.get(f"{args.base_url}/api/auth/me",
                        headers={"Authorization": f"Bearer {token}"})
    resp.raise_for_status()
    owner_id = resp.json()["id"]
    print(f"Owner ID: {owner_id}")

    # Create runner
    resp = requests.post(f"{args.base_url}/api/runner/",
                         headers={"Authorization": f"Bearer {token}"},
                         json={
                             "ownerId": owner_id,
                             "name": args.name,
                             "description": args.description,
                             "slug": args.slug,
                         })
    resp.raise_for_status()
    print(f"Runner '{args.slug}' created successfully.")


if __name__ == "__main__":
    main()
