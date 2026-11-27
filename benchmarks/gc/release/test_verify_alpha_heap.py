import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[3] / ".github/scripts"))
from collector_graph import check_collector_source, check_tree, verify_tree


class CollectorIdentityTests(unittest.TestCase):
    workspace_gc = "ristretto_gc v0.35.0 (/workspace/ristretto_gc)"
    registry_gc = "ristretto_gc v0.35.0"
    other_checkout = "ristretto_gc v0.35.0 (/other/ristretto_gc)"

    def test_multiple_edges_to_the_workspace_collector(self):
        check_tree(f"{self.workspace_gc}|\n{self.workspace_gc}| (*)", self.workspace_gc)

    def test_graph_must_resolve_exactly_the_workspace_collector(self):
        for graph in ("ristretto_vm v0.35.0|", self.registry_gc + "|",
                      self.other_checkout + "|",
                      f"{self.workspace_gc}|\n{self.registry_gc}|",
                      f"{self.workspace_gc}|\nristretto_gc_catalog v0.35.0|",
                      f"{self.workspace_gc}|\nristretto_vm v0.35.0|experimental-canonical-heap"):
            with self.subTest(graph=graph), self.assertRaises(ValueError):
                check_tree(graph, self.workspace_gc)

    def test_retired_sources_with_no_cargo_dependency(self):
        for path in ("src/canonical_catalog.rs", "src/canonical_pool/state.rs",
                     "src/canonical_diagnostics/leaf.rs", "src/canonical_adapter.rs",
                     "validation/catalog/prototype/src/lib.rs", "validation/catalog/old/Cargo.toml"):
            with self.subTest(path=path), tempfile.TemporaryDirectory() as directory:
                root = Path(directory)
                source = root / path
                source.parent.mkdir(parents=True)
                source.touch()
                with self.assertRaisesRegex(ValueError, "retired"):
                    check_collector_source(root)

    def test_archived_notes_and_java_fixture_names_are_allowed(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            for path in ("docs/CANONICAL_CATALOG_PORT.md", "tests/canonical/Graph.java"):
                file = root / path
                file.parent.mkdir(parents=True)
                file.touch()
            check_collector_source(root)

    def test_wasi_consumer_cannot_link_a_second_collector(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory).resolve()
            for member, name, version in [
                ("archive", "ristretto_gc", "1.0.0"), ("other", "ristretto_gc", "0.9.0"),
                ("consumer", "consumer", "1.0.0"),
            ]:
                package = root / member
                (package / "src").mkdir(parents=True)
                (package / "src/lib.rs").touch()
                (package / "Cargo.toml").write_text(
                    f'[package]\nname = "{name}"\nversion = "{version}"\nedition = "2024"\n')
            manifest = root / "consumer/Cargo.toml"
            with manifest.open("a") as stream:
                stream.write('[dependencies]\nristretto_gc = { path = "../archive" }\n'
                             "[target.'cfg(target_family = \"wasm\")'.dependencies]\n"
                             'old = { package = "ristretto_gc", path = "../other" }\n')
            subprocess.run(["cargo", "generate-lockfile", "--offline"], cwd=manifest.parent,
                           check=True, capture_output=True)
            expected = f"ristretto_gc v1.0.0 ({root / 'archive'})"
            verify_tree(manifest, expected, target="x86_64-unknown-linux-gnu")
            with self.assertRaisesRegex(ValueError, "ristretto_gc v0.9.0"):
                verify_tree(manifest, expected, target="wasm32-wasip2")
            # The external consumer build also inherits the caller's Cargo config.
            config = root / "driver/.cargo"
            config.mkdir(parents=True)
            (config / "config.toml").write_text('[build]\ntarget = "wasm32-wasip2"\n')
            with self.assertRaisesRegex(ValueError, "ristretto_gc v0.9.0"):
                verify_tree(manifest, expected, cargo_cwd=config.parent)
