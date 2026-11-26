"""Require Cargo to identify both fresh targets for every isolated binary."""
import json
from pathlib import Path
import tempfile
import unittest

from mark_cycles_build import build_controls, compiled_example


class CompilerReceiptTests(unittest.TestCase):
    def test_controls_distinguish_profile_dependencies_and_compiler_features(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            manifest = root / "Cargo.toml"
            lock = root / "Cargo.lock"
            manifest.write_text('[workspace]\nmembers = ["gc"]\n[profile.release]\nlto = true\n')
            lock.write_text('[[package]]\nname = "atom"\nversion = "1"\nsource = "registry+url"\nchecksum = "a"\n')
            item = dict(reason="compiler-artifact", package_id="registry+url#atom@1",
                        target=dict(name="atom", kind=["lib"]), features=["fast"], profile={})

            def fingerprint(**changes):
                return build_controls(json.dumps({**item, **changes}), root, "gc")

            expected = fingerprint()
            manifest.write_text(manifest.read_text().replace('["gc"]', '["gc", "research"]'))
            self.assertEqual(fingerprint(), expected)
            self.assertNotEqual(fingerprint(features=[]), expected)
            self.assertIsNone(fingerprint(package_id="path+file:///outside#dep"))
            original = manifest.read_text()
            manifest.write_text(original.replace("true", "false"))
            self.assertNotEqual(fingerprint(), expected)
            manifest.write_text(original)
            lock.write_text(lock.read_text().replace('checksum = "a"', 'checksum = "b"'))
            self.assertNotEqual(fingerprint(), expected)

    def test_accepts_only_the_fresh_library_and_example_from_one_package(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            library = root / 'ristretto_gc/src/lib.rs'
            fixture = root / 'ristretto_gc/examples/mark_cycles.rs'
            target = root / 'target'
            target.mkdir()
            binary = target / 'mark_cycles'
            binary.touch()

            def message(src, kind, **fields):
                value = dict(reason='compiler-artifact', package_id='gc', fresh=False,
                             target=dict(kind=[kind], src_path=str(src)),
                             executable=str(binary) if kind == 'example' else None)
                return {**value, **fields}

            def select(*messages):
                return compiled_example('\n'.join(map(json.dumps, messages)),
                                        root, fixture, target)

            lib = message(library, 'lib')
            example = message(fixture, 'example')
            self.assertEqual(select(lib, example), (binary, 'gc'))
            for messages in [
                    (example,), (lib,), (lib, example, example),
                    (dict(lib, fresh=True), example), (lib, dict(example, fresh=True)),
                    (message(root / 'other/lib.rs', 'lib'), example),
                    (lib, dict(example, package_id='other')),
                    (lib, message(fixture, 'lib')),
                    (lib, dict(example, executable=str(root)))]:
                with self.subTest(messages=messages), self.assertRaises(ValueError):
                    select(*messages)
