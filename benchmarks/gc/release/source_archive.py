#!/usr/bin/env python3
"""Build a reproducible GC preview source archive from one Git commit."""
import argparse
import gzip
import hashlib
from pathlib import Path
import shutil
import subprocess
import tempfile


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("revision", help="Git commit, tag or branch")
    parser.add_argument("directory", type=Path, help="new output directory")
    args = parser.parse_args()
    root = Path(subprocess.check_output(["git", "rev-parse", "--show-toplevel"], text=True).strip())
    commit = subprocess.check_output(["git", "rev-parse", "--verify", "--end-of-options",
                                      args.revision + "^{commit}"], cwd=root, text=True).strip()
    # These names can be used for release attachments or extracted into a fresh directory.
    prefix = f"ristretto-gc-preview-{commit[:12]}"
    filename = prefix + ".tar.gz"
    destination = args.directory.resolve()
    if destination.exists():
        parser.error("output directory already exists")
    with tempfile.TemporaryDirectory(dir=destination.parent) as temporary:
        staging = Path(temporary) / "artifacts"
        staging.mkdir()
        archive = staging / filename
        with archive.open("wb") as raw, gzip.GzipFile(filename="", fileobj=raw, mode="wb",
                                                       compresslevel=9, mtime=0) as compressed:
            with subprocess.Popen(["git", "archive", "--format=tar", "--prefix=" + prefix + "/", commit],
                                  cwd=root, stdout=subprocess.PIPE) as export:
                shutil.copyfileobj(export.stdout, compressed)
                if export.wait():
                    raise SystemExit("git archive failed")
        with archive.open("rb") as raw:
            digest = hashlib.file_digest(raw, "sha256").hexdigest()
        (staging / "SHA256SUMS").write_text(f"{digest}  {filename}\n", encoding="ascii")
        staging.rename(destination)
    print(f"{commit}  {destination / filename}")


if __name__ == "__main__":
    main()
