#!/usr/bin/env python3
"""
List all users (admin only).

Usage:
  python list_users.py --base-url http://localhost:5288 \
      --email admin@example.com --password secret
"""

import argparse
import json
import requests


def main():
    parser = argparse.ArgumentParser(description="List all users.")
    parser.add_argument("--base-url", required=True)
    parser.add_argument("--email", required=True)
    parser.add_argument("--password", required=True)
    args = parser.parse_args()

    # Login
    resp = requests.post(f"{args.base_url}/api/auth/login",
                         json={"email": args.email, "password": args.password})
    resp.raise_for_status()
    token = resp.json()["token"]
    print(f"Logged in as {args.email}\n")

    # List users
    resp = requests.get(f"{args.base_url}/api/user/",
                        headers={"Authorization": f"Bearer {token}"})
    resp.raise_for_status()
    users = resp.json()

    print(f"{'ID':<38} {'Name':<25} {'Email':<30} {'Role'}")
    print("-" * 100)
    for u in users:
        print(f"{u['id']:<38} {u['name']:<25} {u['email']:<30} {u.get('role', '')}")

    print(f"\nTotal: {len(users)} user(s)")


if __name__ == "__main__":
    main()
