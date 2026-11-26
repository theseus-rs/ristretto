#!/usr/bin/env python3
"""Verify the VM's exported call fixtures on HotSpot; no timing claims."""
import argparse
import hashlib
import json
from pathlib import Path
import shutil
import subprocess


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--fixtures", type=Path, required=True)
    parser.add_argument("--jdk", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--driver", type=Path,
                        default=Path(__file__).with_name("CheckStaticCalls.java"))
    args = parser.parse_args()
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=False)
    fixtures = sorted(args.fixtures.resolve().glob("*.class"))
    if not fixtures:
        parser.error("no exported class files")
    for path in fixtures:
        shutil.copy2(path, output / path.name)
    source = args.driver.resolve()
    shutil.copy2(source, output / source.name)
    jdk = args.jdk.resolve() / "bin"
    commands = [
        [str(jdk / "java"), "-version"],
        [str(jdk / "javac"), "--release", "8", "-cp", str(output), str(output / source.name)],
        [str(jdk / "java"), "-Xverify:all", "-Xint", "-Xmx32m", "-Xlog:gc",
         "-cp", str(output), source.stem],
    ]
    report = {"qualification": False, "purpose": "bytecode correctness, not GC performance",
              "inputs": {p.name: hashlib.sha256(p.read_bytes()).hexdigest()
                         for p in fixtures + [source]}, "runs": []}
    for index, command in enumerate(commands):
        run = subprocess.run(command, text=True, stdout=subprocess.PIPE,
                             stderr=subprocess.STDOUT, timeout=180)
        (output / f"step-{index}.log").write_text(run.stdout)
        report["runs"].append({"command": command, "exit": run.returncode,
                               "log": f"step-{index}.log"})
        (output / "manifest.json").write_text(json.dumps(report, indent=2) + "\n")
        print(run.stdout, end="")
        run.check_returncode()
    print(f"Preserved {len(fixtures)} exact fixture classes and verification logs in {output}")


if __name__ == "__main__":
    main()
