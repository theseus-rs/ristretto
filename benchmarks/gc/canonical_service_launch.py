"""Write the actual PID/exec receipt, then replace this process with the runtime."""
import hashlib
import json
import os
from pathlib import Path
import sys


def digest(path):
    with Path(path).open("rb") as stream:
        return hashlib.file_digest(stream, "sha256").hexdigest()


def main():
    root = Path(sys.argv[1]).resolve()
    plan = json.loads((root / "launch-plan.json").read_text())
    command = plan["command"]
    binary = root / ("program.wasm" if plan["mode"] == "wasi" else "program")
    receipt = {"schema_version": 1, "run_id": plan["run_id"], "mode": plan["mode"],
               "host_process_id": os.getpid(), "command": command,
               "host_executable_sha256": digest(command[0]), "binary_sha256": digest(binary),
               "launcher_sha256": digest(__file__)}
    with (root / "launch.json").open("x") as stream:
        json.dump(receipt, stream, indent=2)
        stream.write("\n")
    os.execv(command[0], command)


if __name__ == "__main__":
    main()
