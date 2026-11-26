"""Compile only the peer observer; copy the exact shared ring classfiles."""
import argparse
import json
from pathlib import Path
import shutil
import subprocess
from canonical_service import digest, workload_digest
from canonical_service_artifacts import verify_build


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("--source", type=Path, required=True)
    p.add_argument("--source-build", type=Path, required=True)
    p.add_argument("--jdk", type=Path, required=True)
    p.add_argument("--output", type=Path, required=True)
    args = p.parse_args()
    build = verify_build(args.source_build)
    manifest = json.loads((args.source_build / "source-manifest.json").read_text())
    relative = "benchmarks/gc/ring_compare/CanonicalRingPeer.java"
    if digest(args.source / relative) != manifest[relative]:
        raise ValueError("peer observer source differs from frozen build")
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=False)
    classes = output / "classes"
    classes.mkdir()
    shared = output / "shared-classes"
    shared.mkdir()
    for name in ("CanonicalRingWorkload.class", "CanonicalRingWorkload$Node.class"):
        relative_class = "ristretto_vm/tests/fixtures/canonical/" + name
        if digest(args.source / relative_class) != manifest[relative_class]:
            raise ValueError("shared ring bytecode differs from frozen source")
        shutil.copy2(args.source / relative_class, classes / name)
        shutil.copy2(args.source / relative_class, shared / name)
    command = [str(args.jdk.resolve() / "bin/javac"), "--release", "17", "-g", "-cp", str(classes),
               "-d", str(classes), str(args.source.resolve() / relative)]
    with (output / "javac.log").open("w") as log:
        subprocess.run(command, stdout=log, stderr=subprocess.STDOUT, check=True)
    if digest(args.source / relative) != manifest[relative]:
        raise ValueError("peer source changed while compiling")
    for name in ("CanonicalRingWorkload.class", "CanonicalRingWorkload$Node.class"):
        if digest(classes / name) != manifest["ristretto_vm/tests/fixtures/canonical/" + name]:
            raise ValueError("shared bytecode changed during peer compilation")
    if {path.name for path in classes.glob("*.class")} != {"CanonicalRingWorkload.class", "CanonicalRingWorkload$Node.class", "CanonicalRingPeer.class"}:
        raise ValueError("unexpected peer compilation products")
    for name in ("sources.tar.gz", "source-manifest.json"):
        shutil.copy2(args.source_build / name, output / name)
    runtime = output / "jdk-artifacts"
    runtime.mkdir()
    files = ["bin/java", "bin/javac", "lib/server/libjvm.so", "lib/modules", "release"]
    hashes = {}
    for name in files:
        source = args.jdk.resolve() / name
        dest = runtime / name
        dest.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(source, dest)
        hashes[name] = digest(dest)
    receipt = {"schema_version": 1, "jdk": str(args.jdk.resolve()), "jdk_sha256": hashes,
               "command": command, "peer_source_sha256": manifest[relative],
               "peer_class_sha256": digest(classes / "CanonicalRingPeer.class"),
               "shared_workload_sha256": workload_digest(shared),
               "source_archive_sha256": build["source_archive_sha256"],
               "source_manifest_sha256": build["source_manifest_sha256"], "qualification": False}
    (output / "peer-build.json").write_text(json.dumps(receipt, indent=2) + "\n")
    print(json.dumps(receipt, indent=2))


if __name__ == "__main__":
    main()
