"""Source-bind the native diagnostic gc_compare artifact; executes no workload."""
import argparse
import json
import os
from pathlib import Path
import shutil
import subprocess
import tarfile
from canonical_service import digest


def inventory(root):
    names = subprocess.check_output(["git", "ls-files", "--cached", "--others", "--exclude-standard", "-z"], cwd=root).decode().split("\0")
    result = {}
    for name in sorted(set(names) - {""}):
        path = root / name
        if path.is_symlink() or not path.is_file():
            raise ValueError(f"unsupported source member: {name}")
        result[name] = digest(path)
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--target-dir", type=Path, required=True)
    args = parser.parse_args()
    source, output, target = args.source.resolve(), args.output.resolve(), args.target_dir.resolve()
    if output.is_relative_to(source) or target.is_relative_to(source):
        raise ValueError("evidence and source-specific Cargo target must be outside the source tree")
    output.mkdir(parents=True, exist_ok=False)
    before = inventory(source)
    (output / "source-manifest.json").write_text(json.dumps(before, indent=2) + "\n")
    with tarfile.open(output / "sources.tar.gz", "w:gz") as archive:
        for name in before:
            archive.add(source / name, arcname=name, recursive=False)
    command = ["cargo", "build", "--locked", "-p", "ristretto_vm", "--example", "gc_compare",
               "--no-default-features", "--features", "experimental-execution-measurement", "--message-format=json"]
    environment = dict(os.environ, CARGO_BUILD_JOBS="1", CARGO_PROFILE_DEV_DEBUG="0",
                       CARGO_PROFILE_TEST_DEBUG="0", CARGO_INCREMENTAL="0", CARGO_TARGET_DIR=str(target))
    toolchain = subprocess.check_output(["rustc", "-Vv"], cwd=source, text=True)
    with (output / "cargo.jsonl").open("w") as stdout, (output / "cargo.stderr").open("w") as stderr:
        result = subprocess.run(command, cwd=source, env=environment, stdout=stdout, stderr=stderr)
    if result.returncode or inventory(source) != before:
        raise ValueError("build failed or source changed; raw output preserved")
    executables = [row["executable"] for line in (output / "cargo.jsonl").read_text().splitlines()
                   if (row := json.loads(line)).get("reason") == "compiler-artifact"
                   and row.get("target", {}).get("name") == "gc_compare" and row.get("executable")]
    if len(executables) != 1:
        raise ValueError("compiler did not identify exactly one gc_compare executable")
    shutil.copy2(executables[0], output / "program")
    receipt = {"schema_version": 1, "source_archive_sha256": digest(output / "sources.tar.gz"),
               "source_manifest_sha256": digest(output / "source-manifest.json"),
               "binary_sha256": digest(output / "program"), "binary_name": "program", "target": "native",
               "features": ["experimental-execution-measurement"], "command": command, "toolchain": toolchain,
               "source_unchanged": True, "qualification": False}
    (output / "build.json").write_text(json.dumps(receipt, indent=2) + "\n")
    print(json.dumps(receipt, indent=2))


if __name__ == "__main__":
    main()
