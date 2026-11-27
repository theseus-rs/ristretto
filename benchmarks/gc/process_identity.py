"""Optional diagnostic launcher; the receipt PID survives exec into the VM.

Executed under the same affinity/cgroup/time resource accounting as the VM.
The launcher adds startup cost; it does not run in the workload measured loop.
"""
import json
import os
from pathlib import Path
import sys


def main():
    path, *command = sys.argv[1:]
    with Path(path).open("x") as stream:
        json.dump({"process_id": os.getpid(),
                   "run_id": os.environ["RISTRETTO_QUALIFICATION_RUN_ID"]}, stream)
        stream.write("\n")
    os.execvpe(command[0], command, os.environ)


if __name__ == "__main__":
    main()
