"""Keep native/Wasm mechanism comparisons in separate execution lanes."""
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

from mechanisms import validate_target


class TargetTests(unittest.TestCase):
    def test_mixed_targets_and_missing_or_unnecessary_runtime_are_rejected(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            native, wasm = root / "native", root / "wasm"
            native.write_bytes(b"\x7fELF")
            wasm.write_bytes(b"\0asm\1\0\0\0")
            self.assertEqual(validate_target(native, native, None), "native")
            self.assertEqual(validate_target(wasm, wasm, root / "wasmtime"), "WASI")
            for before, after, runtime in [(native, wasm, None), (wasm, native, root / "wasmtime"),
                                           (wasm, wasm, None), (native, native, root / "wasmtime")]:
                with self.subTest(before=before.name, after=after.name, runtime=runtime):
                    with self.assertRaises(ValueError):
                        validate_target(before, after, runtime)

    def test_wasm_cli_runs_through_the_frozen_runtime_and_preserves_raw_failures(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            wasm = root / "module.wasm"
            wasm.write_bytes(b"\0asm\1\0\0\0")
            runner = root / "fake-wasmtime"
            runner.write_text(f"#!{sys.executable}\n" + '''import json, pathlib, sys
if sys.argv[1:] == ['--version']:
    print('fixture-runtime 1')
    raise SystemExit(0)
assert sys.argv[1] == 'run'
assert sys.argv[2:6] == ['-W', 'max-wasm-stack=8388608', '--argv0', 'mechanisms']
assert pathlib.Path(sys.argv[6]).read_bytes().startswith(b'\\0asm')
work, policy, operations, diagnostics = sys.argv[7:]
operations = int(operations)
if policy == 'counted':
    print('preserved fixture failure', file=sys.stderr)
    raise SystemExit(7)
print(json.dumps(dict(workload=work, policy=policy, diagnostics=diagnostics == 'true',
                     operations=operations, checksum=42*operations, operation_ns=100,
                     wall_ns=200, teardown_live=0)))
''')
            runner.chmod(0o755)
            output = root / "results"
            result = subprocess.run([sys.executable, str(Path(__file__).with_name("mechanisms.py")),
                                     "--before", str(wasm), "--after", str(wasm),
                                     "--wasmtime", str(runner), "--output", str(output),
                                     "--cpu", str(min(os.sched_getaffinity(0))), "--repeats", "2",
                                     "--operations", "1", "--workloads", "control", "leaf_allocation"],
                                    capture_output=True, text=True, timeout=30)
            self.assertEqual(result.returncode, 1, result.stderr)
            metadata = json.loads((output / "metadata.json").read_text())
            self.assertEqual(metadata["target"], "WASI")
            self.assertEqual(metadata["runner"]["version"], "fixture-runtime 1")
            self.assertEqual(Path(metadata["runner"]["path"]), output / "wasmtime")
            rows = [json.loads(line) for line in (output / "raw.jsonl").read_text().splitlines()]
            self.assertEqual(len(rows), 80)
            failures = [row for row in rows if row["status"] == "failed"]
            self.assertEqual(len(failures), 16)
            self.assertTrue(all(row["exit_code"] == 7 and "preserved fixture failure" in row["stderr"] for row in failures))
            self.assertTrue(all(row["command"][:2] == [str(output / "wasmtime"), "run"] for row in rows))
            summary = json.loads((output / "summary.json").read_text())
            self.assertTrue(summary["inputs_unchanged"])
            self.assertFalse(summary["qualification"])
            self.assertEqual(len(summary["failures"]), 16)


class MechanismInvocationTests(unittest.TestCase):
    def test_fixed_native_argv0(self):
        import shutil
        from mechanisms import invocation, INVOCATION_NAME
        shell = shutil.which("sh")
        if not shell:
            self.skipTest("native shell required")
        with tempfile.TemporaryDirectory() as directory:
            for name in ["a", "long-name-" * 12]:
                binary = Path(directory) / name
                binary.symlink_to(shell)
                command, executable = invocation(binary, ["-c", 'printf "%s" "$0"'])
                result = subprocess.run(command, executable=executable, capture_output=True, text=True, check=True)
                self.assertEqual(result.stdout, INVOCATION_NAME)

    def test_duplicate_policy_rows_are_rejected(self):
        result = subprocess.run([sys.executable, str(Path(__file__).with_name("mechanisms.py")),
                                 "--before", "unused", "--after", "unused", "--output", "unused",
                                 "--cpu", str(min(os.sched_getaffinity(0))),
                                 "--policies", "baseline", "baseline"],
                                capture_output=True, text=True, check=False)
        self.assertEqual(result.returncode, 2)
        self.assertIn("must be distinct", result.stderr)

    def test_fixed_wasm_argv0(self):
        from mechanisms import invocation, INVOCATION_NAME
        command, executable = invocation("input.wasm", ["allocation"], "wasmtime")
        self.assertIsNone(executable)
        self.assertEqual(command[command.index("--argv0") + 1], INVOCATION_NAME)
        self.assertEqual(command[-2:], ["input.wasm", "allocation"])

if __name__ == "__main__":
    unittest.main()
