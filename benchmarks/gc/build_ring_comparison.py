"""Freeze clean source and build a release ring comparator; execution is separate."""
import argparse
import json
import os
from pathlib import Path
import shutil
import subprocess
import tarfile
from canonical_service import digest


def source_files(root):
    command = ["rg", "--files", "--hidden", "--no-ignore", "-g", "!**/.git/**",
               "-g", "!**/target/**", "-g", "!**/__pycache__/**", "-g", "!**/*.pyc"]
    names = sorted(subprocess.check_output(command, cwd=root, text=True).splitlines())
    for name in names:
        if (root / name).is_symlink():
            raise ValueError("source symlinks are not supported by this freeze protocol")
    return names


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--target-dir", type=Path, required=True)
    parser.add_argument("--target", choices=["native", "wasm32-wasip2"], required=True)
    parser.add_argument("--compiled", action="store_true")
    parser.add_argument("--profile", choices=["release", "dev"], default="release")
    args = parser.parse_args()
    root, output, target_dir = args.source.resolve(), args.output.resolve(), args.target_dir.resolve()
    if output.is_relative_to(root) or target_dir.is_relative_to(root):
        raise ValueError("source snapshots require build/evidence outside the source tree")
    if args.target == "wasm32-wasip2" and args.compiled:
        raise ValueError("compiled canonical WASI unsupported")
    target_dir.mkdir(parents=True, exist_ok=True)
    marker = target_dir / "canonical-ring-source-root.json"
    if marker.exists():
        if json.loads(marker.read_text()) != {"source": str(root)}:
            raise ValueError("Cargo target belongs to another source root")
    else:
        marker.write_text(json.dumps({"source": str(root)}) + "\n")
    output.mkdir(parents=True, exist_ok=False)
    manifest_path = root / "benchmarks/gc/ring_compare/Cargo.toml"
    env = {key: value for key, value in os.environ.items() if key not in {"RUSTFLAGS", "CARGO_ENCODED_RUSTFLAGS"} and not key.startswith("CARGO_PROFILE_")}
    env.update(CARGO_BUILD_JOBS="1", CARGO_TARGET_DIR=str(target_dir))
    with (output / "lock.log").open("w") as log:
        subprocess.run(["cargo", "generate-lockfile", "--offline", "--manifest-path", str(manifest_path)],
                       cwd=root, env=env, stdout=log, stderr=subprocess.STDOUT, check=True)
    toolchain = subprocess.check_output(["rustc", "-vV"], cwd=root, text=True)
    target = args.target
    if target == "native":
        target = next(line.removeprefix("host: ") for line in toolchain.splitlines() if line.startswith("host: "))
    files = source_files(root)
    manifest = {name: digest(root / name) for name in files}
    (output / "source-manifest.json").write_text(json.dumps(manifest, indent=2) + "\n")
    with tarfile.open(output / "sources.tar.gz", "w:gz") as archive:
        for name in files:
            archive.add(root / name, arcname=name, recursive=False)
    command = ["cargo", "build", "--locked", "--offline", "--manifest-path", str(manifest_path),
               "--target", target, "--message-format=json"]
    if args.profile == "release":
        command += ["--release"]
    features = ["compiled"] if args.compiled else ["default"]
    if args.compiled:
        command += ["--features", "compiled"]
    with (output / "build.log").open("w") as log:
        subprocess.run(command, cwd=root, env=env, stdout=log, stderr=subprocess.STDOUT, check=True)
    if source_files(root) != files or any(digest(root / name) != value for name, value in manifest.items()):
        raise ValueError("source changed during build; no binding receipt will be issued")
    artifacts = [json.loads(line) for line in (output / "build.log").read_text().splitlines() if line.startswith("{")]
    binaries = [item["executable"] for item in artifacts if item.get("reason") == "compiler-artifact"
                and item["target"]["name"] == "canonical-ring-compare" and item.get("executable")]
    if len(binaries) != 1:
        raise ValueError("expected one compiler-reported capture executable")
    binary_name = "program.wasm" if target == "wasm32-wasip2" else "program"
    shutil.copy2(binaries[0], output / binary_name)
    receipt = {"schema_version": 1, "source_archive_sha256": digest(output / "sources.tar.gz"),
               "source_manifest_sha256": digest(output / "source-manifest.json"),
               "binary_sha256": digest(output / binary_name), "binary_name": binary_name,
               "target": target, "features": features, "command": command, "toolchain": toolchain,
               "source_unchanged": True, "qualification": False}
    (output / "build.json").write_text(json.dumps(receipt, indent=2) + "\n")
    print(json.dumps(receipt, indent=2))


if __name__ == "__main__":
    main()
