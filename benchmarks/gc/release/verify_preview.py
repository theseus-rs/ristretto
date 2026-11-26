#!/usr/bin/env python3
"""Build the embedded Java preview and check its report from this source tree."""

import argparse
import json
import math
import os
from pathlib import Path
import shutil
import time
import signal
import subprocess

TIERS = ("native-interpreted", "native-compiled", "wasi-interpreted")


def unique_fields(pairs):
    fields = {}
    for key, value in pairs:
        if key in fields:
            raise ValueError(f"duplicate report field {key}")
        fields[key] = value
    return fields


def check_report(output, compiled):
    fields = json.loads(output, object_pairs_hook=unique_fields)
    expected = {"result": 64, "cleared_witnesses": 2,
                "interpreted_entries": 0 if compiled else 25,
                "compiled_entries": 25 if compiled else 0,
                "compiled_backedges": 8 if compiled else 0}
    if not isinstance(fields, dict):
        raise ValueError("missing JVM report")
    for key, value in expected.items():
        if type(fields.get(key)) is not int or fields[key] != value:
            raise ValueError(f"incorrect {key} in JVM report")
    for key in ("collections_completed", "bytes_freed"):
        if type(fields.get(key)) is not int or fields[key] <= 0:
            raise ValueError(f"no {key} in JVM report")
    return fields


def command_for(tier):
    if tier not in TIERS:
        raise ValueError(f"unsupported tier {tier!r}")
    features = "execution-profile"
    if tier == "native-compiled":
        features += ",experimental-reference-dispatch"
    command = ["cargo", "build", "--locked", "-p", "ristretto_vm", "--no-default-features",
               "--features", features, "--example", "gc_preview", "--message-format=json-render-diagnostics"]
    if tier == "wasi-interpreted":
        command.extend(["--target", "wasm32-wasip2"])
    return command


def run(command, root, timeout):
    """Keep Cargo diagnostics on stderr; bound the complete build/run process group."""
    process = subprocess.Popen(command, cwd=root, text=True, stdout=subprocess.PIPE,
                               start_new_session=os.name == "posix")
    try:
        stdout, _ = process.communicate(timeout=timeout)
    except subprocess.TimeoutExpired:
        try:
            if os.name == "posix":
                os.killpg(process.pid, signal.SIGKILL)
            else:
                process.kill()
        except ProcessLookupError:
            pass  # The process exited between the timeout and group signal.
        process.communicate()
        raise RuntimeError(f"preview build/run exceeded {timeout:g} seconds") from None
    if process.returncode != 0:
        raise RuntimeError(f"preview build/run exited with status {process.returncode}")
    return stdout


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("tier", choices=TIERS)
    parser.add_argument("--timeout", type=float, default=1200,
                        help="complete build/run watchdog in seconds (default: 1200)")
    parser.add_argument("--receipt", type=Path, help="new JSON result file outside the source tree")
    args = parser.parse_args()
    if not math.isfinite(args.timeout) or args.timeout <= 0:
        parser.error("timeout must be finite and positive")
    root = Path(__file__).resolve().parents[3]
    if args.receipt is not None:
        args.receipt = args.receipt.resolve()
        if args.receipt.exists():
            parser.error("receipt path already exists")
        if args.receipt.is_relative_to(root):
            parser.error("receipt must be outside the source tree")
        if not args.receipt.parent.is_dir():
            parser.error("receipt parent directory does not exist")
    home = os.environ.get("JAVA_HOME")
    java = shutil.which("java")
    if home is None and java is None:
        parser.error("set JAVA_HOME to a JDK or install java on PATH")
    home = Path(home).resolve() if home else Path(java).resolve().parents[1]
    classes = root / "ristretto_vm/tests/fixtures/canonical"
    command = command_for(args.tier)
    deadline = time.monotonic() + args.timeout
    try:
        output = run(command, root, args.timeout)
        records = [json.loads(line) for line in output.splitlines()]
        binaries = [r["executable"] for r in records if r.get("reason") == "compiler-artifact"
                    and r["target"]["name"] == "gc_preview" and r["executable"]]
        if len(binaries) != 1:
            raise ValueError("expected exactly one Cargo preview executable")
        execute = [binaries[0], str(home), str(classes)]
        if args.tier == "native-compiled":
            execute.append("--compiled")
        if args.tier == "wasi-interpreted":
            execute = [os.environ.get("WASMTIME", "wasmtime"), "run", "-W", "max-wasm-stack=8388608",
                       "-S", "http", "--dir", f"{home}::/jdk", "--dir", f"{classes}::/classes",
                       binaries[0], "/jdk", "/classes"]
        fields = check_report(run(execute, root, max(0, deadline-time.monotonic())), args.tier == "native-compiled")
    except (OSError, ValueError, RuntimeError) as error:
        parser.exit(1, f"preview verification failed: {error}\n")
    receipt = {"schema_version": 2, "tier": args.tier, "source_root": str(root),
               "command": command, "execute": execute, "report": fields, "scope": "finite functional example",
               "performance_qualified": False}
    if args.receipt is not None:
        try:
            with args.receipt.open("x", encoding="utf-8") as destination:
                json.dump(receipt, destination, indent=2)
                destination.write("\n")
        except OSError as error:
            parser.exit(1, f"cannot save preview receipt: {error}\n")
    print(f"{args.tier}: result=64 cleared_witnesses=2")


if __name__ == "__main__":
    main()
