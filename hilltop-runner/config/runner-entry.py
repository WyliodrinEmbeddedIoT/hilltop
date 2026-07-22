#!/usr/bin/env python3

# Entrypoint script for hilltop runner

import json
import os
import socket
import subprocess
import time

SOCKET = "/var/run/hilltop.sock"


def connect() -> socket.socket:
    sock = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
    sock.connect(SOCKET)
    return sock


def send(sock: socket.socket, msg: dict) -> None:
    print(f"Sending message: {msg}")
    sock.sendall((json.dumps(msg) + "\n").encode())

def main():
    sock = connect()

    try:
        send(sock, {"status": "container-started"})

        os.makedirs("/artifacts", exist_ok=True)
        os.chdir("/workspace")

        # Wait for job-data.zip to be copied over
        deadline = time.time() + 120
        while not os.path.isfile("./job-data.zip"):
            if time.time() > deadline:
                send(sock, {"status": "error", "message": "job-data.zip missing"})
                raise FileNotFoundError("job-data.zip not found after waiting")
            time.sleep(1)

        # Prepare job
        subprocess.run(["unzip", "./job-data.zip", "-d", "./"], check=True)
        subprocess.run(["chmod", "+x", "./entrypoint.sh"], check=True)

        send(sock, {"status": "job-ready"})

        # Run
        send(sock, {"status": "job-started"})

        with open("/artifacts/stdout.log", "w") as out, open("/artifacts/stderr.log", "w") as err:
            subprocess.run(["./entrypoint.sh"], stdout=out, stderr=err)

        send(sock, {"status": "job-completed"})
        send(sock, {"status": "container-completed"})

    finally:
        print("Closing socket connection")
        sock.close()
        print("Socket connection closed")


if __name__ == "__main__":
    main()