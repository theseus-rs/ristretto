#!/usr/bin/env python3
"""Build the embedded Java preview and check its report from this source tree."""

import argparse
import json
import math
import os
from pathlib import Path
import re
import signal
import subprocess

TIERS = ("native-interpreted", "native-compiled", "wasi-interpreted")
TOKEN = re.compile(r"\s*(-?\d+|[A-Za-z_][A-Za-z_0-9]*|[{}:,])")


def parse_report(line):
    """Parse the example's integer/bool/struct Debug output without flattening it."""
    tokens = []
    position = 0
    while position < len(line):
        match = TOKEN.match(line, position)
        if match is None:
            raise ValueError(f"invalid report syntax at character {position}")
        tokens.append(match.group(1))
        position = match.end()
    cursor = 0

    def take(expected=None):
        nonlocal cursor
        if cursor >= len(tokens):
            raise ValueError("truncated report")
        token = tokens[cursor]
        cursor += 1
        if expected is not None and token != expected:
            raise ValueError(f"expected {expected!r}, found {token!r}")
        return token

    def structure(name):
        take(name)
        take("{")
        fields = {}
        while cursor < len(tokens) and tokens[cursor] != "}":
            field = take()
            if not re.fullmatch(r"[a-z_][a-z_0-9]*", field):
                raise ValueError(f"invalid report field {field!r}")
            if field in fields:
                raise ValueError(f"duplicate report field {field!r}")
            take(":")
            if cursor >= len(tokens):
                raise ValueError(f"missing value for {field}")
            value = tokens[cursor]
            if re.fullmatch(r"-?\d+", value):
                fields[field] = int(take())
            elif value in ("true", "false"):
                fields[field] = take() == "true"
            else:
                fields[field] = structure(value)
            if cursor < len(tokens) and tokens[cursor] != "}":
                take(",")
        take("}")
        return fields

    report = structure("Report")
    if cursor != len(tokens):
        raise ValueError("trailing content after report")
    return report


def check_report(output, compiled):
    lines = [line.strip() for line in output.splitlines() if line.strip()]
    if len(lines) != 1:
        raise ValueError(f"expected one report and no other stdout, found {len(lines)} lines")
    fields = parse_report(lines[0])
    expected = {"result": 64, "allocations": 16, "released_objects": 16,
                "payload_bytes_after_cleanup": 0, "catalog_bytes_after_drop": 0,
                "compiled_entries": 25 if compiled else 0,
                "compiled_backedges": 8 if compiled else 0}
    for field, value in expected.items():
        observed = fields.get(field)
        if type(observed) is not int or observed != value:
            raise ValueError(f"expected top-level {field}={value}, found {observed!r}")
    if fields.get("instruction_count_available") is not (not compiled):
        raise ValueError("instruction availability does not match the selected tier")
    return fields


def command_for(tier):
    if tier not in TIERS:
        raise ValueError(f"unsupported tier {tier!r}")
    feature = "experimental-canonical-jit" if tier == "native-compiled" else "experimental-canonical-runtime"
    command = ["cargo", "run", "--locked", "-p", "ristretto_vm_research", "--no-default-features",
               "--features", feature, "--example", "canonical_java"]
    if tier == "wasi-interpreted":
        command.extend(["--target", "wasm32-wasip2"])
    if tier == "native-compiled":
        command.extend(["--", "--compiled"])
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
    command = command_for(args.tier)
    try:
        output = run(command, root, args.timeout)
        fields = check_report(output, args.tier == "native-compiled")
    except (OSError, ValueError, RuntimeError) as error:
        parser.exit(1, f"preview verification failed: {error}\n")
    receipt = {"schema_version": 1, "tier": args.tier, "source_root": str(root),
               "command": command, "report": fields, "scope": "finite functional example",
               "performance_qualified": False}
    if args.receipt is not None:
        try:
            with args.receipt.open("x", encoding="utf-8") as destination:
                json.dump(receipt, destination, indent=2)
                destination.write("\n")
        except OSError as error:
            parser.exit(1, f"cannot save preview receipt: {error}\n")
    print(f"{args.tier}: result=64 allocations=16 released=16 payload=0 catalog=0")


if __name__ == "__main__":
    main()
