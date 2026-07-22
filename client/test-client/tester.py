#!/usr/bin/env python3
"""
Usage:
  python tester.py [source_file]
"""

import argparse
import asyncio
import json
import os

import requests
import websockets

# ---------------------------------------------------------------------------
# Configuration
# ---------------------------------------------------------------------------

# TODO: get this via CLI args / or a json file 
BASE_URL = "https://tw.semaka.ro:2053"
WS_URL = "wss://tw.semaka.ro:2053/ws"

RUNNER_SLUG = "admin-runner"
RUNNER_API_KEY = "REDACTED"

USER_EMAIL = "admin@tw.semaka.ro"
CLIENT_API_KEY = "REDACTED"

# REST helpers

def rest_get_client_jwt() -> str:
    """Exchange user email + client API key for a JWT (ApiClient role)."""
    resp = requests.post(
        f"{BASE_URL}/api/auth/newUserSession",
        json={"userEmail": USER_EMAIL, "clientApiKey": CLIENT_API_KEY},
    )
    resp.raise_for_status()
    token = resp.json()["token"]
    print("[REST] Got client JWT")
    return token


def rest_get_runner_jwt() -> str:
    """Exchange runner slug + runner API key for a JWT (ApiRunner role)."""
    resp = requests.post(
        f"{BASE_URL}/api/auth/newRunnerSession",
        json={"runnerSlug": RUNNER_SLUG, "runnerApiKey": RUNNER_API_KEY},
    )
    resp.raise_for_status()
    token = resp.json()["token"]
    print("[REST] Got runner JWT")
    return token


def rest_upload_source(client_jwt: str, source_path: str) -> str:
    """Upload a source file and return its source ID (UUID string)."""
    with open(source_path, "rb") as f:
        resp = requests.post(
            f"{BASE_URL}/client_api/source",
            headers={"Authorization": f"Bearer {client_jwt}"},
            params={"RunnerSlug": RUNNER_SLUG},
            files={
                "sourceFile": (
                    os.path.basename(source_path),
                    f,
                    "application/octet-stream",
                )
            },
        )
    resp.raise_for_status()
    result = resp.json()
    print(f"[REST] Source uploaded -> sourceId={result['sourceId']}")
    return result["sourceId"]


def rest_download_source(runner_jwt: str, source_id: str, dest_path: str) -> None:
    """Download a source file using the runner JWT."""
    resp = requests.get(
        f"{BASE_URL}/runner_api/source/{source_id}/download",
        headers={"Authorization": f"Bearer {runner_jwt}"},
        stream=True,
    )
    resp.raise_for_status()
    with open(dest_path, "wb") as f:
        for chunk in resp.iter_content(chunk_size=8192):
            f.write(chunk)
    print(f"[REST] Runner downloaded source -> {dest_path}")


def rest_upload_artifact(runner_jwt: str, job_id: str, display_identifier: str, content: str) -> None:
    """Upload an in-memory artifact for a job using the runner JWT."""
    resp = requests.post(
        f"{BASE_URL}/runner_api/artifact/job/{job_id}",
        headers={"Authorization": f"Bearer {runner_jwt}"},
        params={"DisplayIdentifier": display_identifier},
        files={"artifactFile": (display_identifier, content.encode(), "text/plain")},
    )
    resp.raise_for_status()
    print(f"[REST] Runner uploaded artifact '{display_identifier}'")


def rest_get_job(client_jwt: str, job_id: str) -> dict:
    """Fetch job info (including artifact list) using the client JWT."""
    resp = requests.get(
        f"{BASE_URL}/client_api/job/{job_id}",
        headers={"Authorization": f"Bearer {client_jwt}"},
    )
    resp.raise_for_status()
    return resp.json()


def rest_download_artifact(client_jwt: str, artifact_id: str) -> bytes:
    """Download artifact bytes using the client JWT (no disk write)."""
    resp = requests.get(
        f"{BASE_URL}/client_api/artifact/{artifact_id}/download",
        headers={"Authorization": f"Bearer {client_jwt}"},
    )
    resp.raise_for_status()
    return resp.content


# WebSocket helpers

async def ws_send_recv(ws, payload: dict) -> dict:
    await ws.send(json.dumps(payload))
    raw = await ws.recv()
    return json.loads(raw)


# Runner WebSocket session

async def run_runner_session(runner_ready: asyncio.Event, stop: asyncio.Event, runner_jwt: str) -> None:
    """
    Connect as a runner, complete CONFIG_RUNNER handshake, signal ready,
    then sit and print any messages until stop is set.
    """
    async with websockets.connect(WS_URL) as ws:
        # HELLO_RUNNER
        resp = await ws_send_recv(ws, {
            "command": "HELLO_RUNNER",
            "socket_protocol_version": "1.0.0",
        })
        print(f"[Runner WS] HELLO_RUNNER -> {resp}")

        # CONFIG_RUNNER
        resp = await ws_send_recv(ws, {
            "command": "CONFIG_RUNNER",
            "api_key": RUNNER_API_KEY,
            "runner_slug": RUNNER_SLUG,
        })
        print(f"[Runner WS] CONFIG_RUNNER -> {resp}")

        runner_ready.set()
        print("[Runner WS] Ready - listening for dispatched jobs...")

        while not stop.is_set():
            try:
                raw = await asyncio.wait_for(ws.recv(), timeout=1.0)
                msg = json.loads(raw)
                print(f"[Runner WS] Received: {msg}")

                if msg.get("command") == "NEW_JOB":
                    job_id = msg["job_identifier"]
                    job_desc = msg.get("job_description", {})
                    print(f"[Runner WS] ACK-ing job {job_id}")
                    await ws.send(json.dumps({
                        "response": "NEW_JOB_ACK",
                        "data": {"job_identifier": job_id},
                    }))

                    await asyncio.sleep(3)

                    resp = await ws_send_recv(ws, {"command": "JOB_STARTED", "job_identifier": job_id})
                    print(f"[Runner WS] JOB_STARTED -> {resp}")

                    await asyncio.sleep(3)

                    # Upload dummy artifacts before marking the job finished
                    loop = asyncio.get_event_loop()
                    if job_desc.get("stdout_artifact"):
                        await loop.run_in_executor(None, rest_upload_artifact, runner_jwt, job_id,
                                                   "stdout", "These are the contents of stdout: <Dummy Content>")
                    if job_desc.get("stderr_artifact"):
                        await loop.run_in_executor(None, rest_upload_artifact, runner_jwt, job_id,
                                                   "stderr", "These are the contents of stderr: <Dummy Content>")
                    for artifact_path in job_desc.get("artifacts", []):
                        name = os.path.basename(artifact_path)
                        await loop.run_in_executor(None, rest_upload_artifact, runner_jwt, job_id,
                                                   artifact_path, f"These are the contents of {name}: <Dummy Content>")

                    resp = await ws_send_recv(ws, {"command": "JOB_FINISHED", "job_identifier": job_id})
                    print(f"[Runner WS] JOB_FINISHED -> {resp}")
            except asyncio.TimeoutError:
                continue
            except websockets.ConnectionClosed:
                break

        # GOODBYE
        resp = await ws_send_recv(ws, {"command": "GOODBYE"})
        print(f"[Runner WS] GOODBYE -> {resp}")


# Client WebSocket session

async def run_client_session(source_id: str, client_jwt: str, job_description: dict) -> str | None:
    """
    Connect as a client, schedule a job, and return the job ID on success.
    """
    async with websockets.connect(WS_URL) as ws:
        # HELLO_CLIENT
        resp = await ws_send_recv(ws, {
            "command": "HELLO_CLIENT",
            "socket_protocol_version": "1.0.0",
        })
        print(f"[Client WS] HELLO_CLIENT -> {resp}")

        # CONFIG_CLIENT
        resp = await ws_send_recv(ws, {
            "command": "CONFIG_CLIENT",
            "user_identifier": USER_EMAIL,
            "api_key": CLIENT_API_KEY,
            "runner_slug": RUNNER_SLUG,
        })
        print(f"[Client WS] CONFIG_CLIENT -> {resp}")

        # SCHEDULE_JOB
        resp = await ws_send_recv(ws, {
            "command": "SCHEDULE_JOB",
            "job_data_identifier": source_id,
            "job_description": job_description,
        })
        print(f"[Client WS] SCHEDULE_JOB -> {resp}")

        job_id = None
        if resp.get("response") == "ACK" and resp.get("data"):
            job_id = resp["data"].get("job_identifier")
            print(f"[Client WS] Job scheduled -> jobId={job_id}")

            print("[Client WS] Waiting for COMPLETED or FAILED notification...")
            while True:
                try:
                    raw = await ws.recv()
                    msg = json.loads(raw)

                    if msg.get("command") == "NOTIFY":
                        await ws.send(json.dumps({"response": "NOTIFY_ACK"}))
                        status = msg.get("job_status")
                        print(f"[Client WS] NOTIFY job_status={status} message={msg.get('job_status_message', '')!r}")
                        if status in ("COMPLETED", "FAILED"):
                            if status == "COMPLETED":
                                loop = asyncio.get_event_loop()
                                job_info = await loop.run_in_executor(None, rest_get_job, client_jwt, job_id)
                                for artifact in job_info.get("artifacts", []):
                                    content = await loop.run_in_executor(
                                        None, rest_download_artifact, client_jwt, artifact["id"]
                                    )
                                    print(f"[Client] Artifact '{artifact['displayIdentifier']}': {content.decode()}")
                            break
                    else:
                        print(f"[Client WS] Received: {msg}")
                except websockets.ConnectionClosed:
                    print("[Client WS] Connection closed while waiting for job completion")
                    return job_id

        # GOODBYE
        resp = await ws_send_recv(ws, {"command": "GOODBYE"})
        print(f"[Client WS] GOODBYE -> {resp}")

        return job_id


# Main

async def main() -> None:
    script_dir = os.path.dirname(os.path.abspath(__file__))
    default_job_description = os.path.join(script_dir, "..", "sample-jobs", "nrf52_blinky", "job.json")
    default_job_data = os.path.join(script_dir, "..",  "sample-jobs", "nrf52_blinky.zip")

    parser = argparse.ArgumentParser(description="Hilltop broker WebSocket tester.")
    parser.add_argument(
        "--client",
        action="store_true",
        default=False,
        help="Run only the client session (assumes a runner is already connected).",
    )
    parser.add_argument(
        "--job-description",
        default=None,
        help="Path to job.json (must be paired with --job-data).",
    )
    parser.add_argument(
        "--job-data",
        default=None,
        help="Path to job data zip (must be paired with --job-description).",
    )
    args = parser.parse_args()

    if (args.job_description is None) != (args.job_data is None):
        parser.error("--job-description and --job-data must both be specified or neither.")

    job_description_path = args.job_description or default_job_description
    source_path = args.job_data or default_job_data

    with open(job_description_path) as f:
        job_description = json.load(f)
    print(f"[Config] Job description: {job_description_path}")
    print(f"[Config] Job data: {source_path}")

    # REST: obtain tokens and upload source before touching WebSockets
    client_jwt = rest_get_client_jwt()
    runner_jwt = rest_get_runner_jwt()
    source_id = rest_upload_source(client_jwt, source_path)

    if args.client:
        job_id = await run_client_session(source_id, client_jwt, job_description)
    else:
        # Run runner WS and client WS concurrently
        runner_ready = asyncio.Event()
        stop_runner = asyncio.Event()

        runner_task = asyncio.create_task(
            run_runner_session(runner_ready, stop_runner, runner_jwt)
        )

        # Wait for runner to finish its CONFIG_RUNNER handshake before the
        # client tries to schedule a job (server would return RUNNER_OFFLINE otherwise)
        await runner_ready.wait()

        job_id = await run_client_session(source_id, client_jwt, job_description)

        # Let the runner session wind down
        stop_runner.set()
        await runner_task

    print("\n[Done]")


if __name__ == "__main__":
    asyncio.run(main())
