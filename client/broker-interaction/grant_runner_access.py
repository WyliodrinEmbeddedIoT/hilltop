#!/usr/bin/env python3
"""
Grant a user access to a runner (must be runner owner or admin).

Usage:
  python grant_runner_access.py --base-url http://localhost:5288 \
      --email admin@example.com --password secret \
      --runner-id <runner-uuid> --user-id <user-uuid>
"""

import argparse
import requests


def main():
    parser = argparse.ArgumentParser(description="Grant a user access to a runner.")
    parser.add_argument("--base-url", required=True)
    parser.add_argument("--email", required=True)
    parser.add_argument("--password", required=True)
    parser.add_argument("--runner-id", required=True, help="UUID of the runner")
    parser.add_argument("--user-id", required=True, help="UUID of the user to grant access")
    args = parser.parse_args()

    # Login
    resp = requests.post(f"{args.base_url}/api/auth/login",
                         json={"email": args.email, "password": args.password})
    resp.raise_for_status()
    token = resp.json()["token"]
    print(f"Logged in as {args.email}")

    # Add user to runner
    resp = requests.post(f"{args.base_url}/api/runner/{args.runner_id}/addUser",
                         headers={"Authorization": f"Bearer {token}"},
                         json={"userId": args.user_id})
    resp.raise_for_status()
    print(f"User {args.user_id} granted access to runner {args.runner_id}.")


if __name__ == "__main__":
    main()
