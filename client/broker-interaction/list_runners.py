#!/usr/bin/env python3
"""
List all runners.

Usage:
  python list_runners.py --base-url http://localhost:5288 \
      --email user@example.com --password secret
"""

import argparse
import requests


def main():
    parser = argparse.ArgumentParser(description="List all runners.")
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

    # Fetch all runners (page through if needed)
    page, all_runners = 1, []
    while True:
        resp = requests.get(f"{args.base_url}/api/runner/",
                            headers={"Authorization": f"Bearer {token}"},
                            params={"page": page, "pageSize": 100})
        resp.raise_for_status()
        data = resp.json()
        items = data.get("data", [])
        all_runners.extend(items)
        if len(all_runners) >= data.get("totalCount", 0):
            break
        print(f"Fetched {len(all_runners)} runner(s) in total. Need to fetch {data.get('totalCount', 0)} in total. Fetching next page...")
        page += 1

    print(f"{'ID':<38} {'Slug':<25} {'Name':<25} {'Owner'}")
    print("-" * 110)
    for r in all_runners:
        print(f"{r['runnerId']:<38} {r['runnerSlug']:<25} {r['runnerName']:<25} {r['runnerOwnerName']}")

    print(f"\nTotal: {len(all_runners)} runner(s)")


if __name__ == "__main__":
    main()
