#!/usr/bin/env python3
"""Freeze freshly built workload binaries and their source evidence together."""
import argparse
import json
from pathlib import Path
import shutil
import sys

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "benchmarks/gc"))
from run import capture, digest, freeze_sources


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", nargs="+", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=False)
    sources = freeze_sources(output)
    for original in args.binary:
        binary = output / original.name
        shutil.copy2(original, binary)
        data = {"binary_sha256": digest(binary), "source_sha256": sources,
                "source_archive_sha256": digest(output / "sources.tar.gz"),
                "rustc": capture(["rustc", "--version"]), "git_head": capture(["git", "rev-parse", "HEAD"])}
        (output / f"{binary.name}.provenance.json").write_text(json.dumps(data, indent=2) + "\n")


if __name__ == "__main__":
    main()
