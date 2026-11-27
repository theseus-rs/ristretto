"""Verify the source and Cargo dependency identity of the unified collector."""
import subprocess

RETIRED = {"ristretto_vm_research", "ristretto_gc_catalog"}


# Historical Rust module paths in the two shipping packages. Java fixtures
# under tests/fixtures/canonical still exercise the ordinary JVM.
RETIRED_MODULES = {
    "ristretto_gc": ("canonical_catalog", "canonical_pool", "canonical_diagnostics", "canonical_adapter"),
    "ristretto_vm": ("canonical", "canonical_heap", "canonical_cycle_windows",
                     "canonical_recording", "canonical_statics", "canonical_telemetry"),
}


def check_shipping_source(package, root):
    """Catch retired heap or interpreter sources even without a Cargo dependency edge."""
    for module in RETIRED_MODULES.get(package, ()):
        source = root / "src" / module
        if source.with_suffix(".rs").is_file() or any(source.rglob("*.rs")):
            raise ValueError(f"retired {package} module in {root}: {module}")
    if package == "ristretto_gc":
        prototypes = root / "validation/catalog"
        if any(prototypes.rglob("Cargo.toml")) or any(prototypes.rglob("*.rs")):
            raise ValueError(f"retired standalone catalog implementation in {prototypes}")


def check_tree(output, expected):
    collectors = set()
    for line in output.splitlines():
        package_id, features = line.removesuffix(" (*)").rsplit("|", 1)
        name = package_id.split()[0]
        if name in RETIRED:
            raise ValueError(f"retired research dependency {name}")
        if name in {"ristretto_gc", "ristretto_vm"}:
            if any(feature.startswith("experimental-canonical") for feature in features.split(",")):
                raise ValueError(f"retired research feature in {name}")
            if name == "ristretto_gc":
                collectors.add(package_id)
    if collectors != {expected}:
        raise ValueError(f"expected {expected}, found {sorted(collectors)}")


def verify_tree(manifest, expected, *, target=None, package=None, all_features=False, build_deps=True,
                cargo_cwd=None):
    args = ["cargo", "tree", "--locked", "--manifest-path", manifest,
            "--edges", "normal,build" if build_deps else "normal",
            "--prefix", "none", "--format", "{p}|{f}"]
    if package:
        args.extend(["--package", package])
    if all_features:
        args.append("--all-features")
    if target:
        args.extend(["--target", target])
    check_tree(subprocess.check_output(args, cwd=cargo_cwd or manifest.parent, text=True), expected)
