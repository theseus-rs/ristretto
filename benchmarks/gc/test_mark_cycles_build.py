"""Require Cargo to identify both fresh targets for every isolated binary."""
import json
from pathlib import Path
import tempfile
import unittest

from mark_cycles_build import compiled_example


class CompilerReceiptTests(unittest.TestCase):
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
