import csv
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest
import xml.etree.ElementTree as ET

from report import expected, fork_rows


HERE = Path(__file__).resolve().parent
VECTORS = HERE.parents[2] / "ristretto_vm/tests/fixtures/canonical/ring-oracle.tsv"
NS = {"svg": "http://www.w3.org/2000/svg"}


def capture_for(forks, times=(1000, 2000, 6000, 11000)):
    checksum = 619975554  # small workload, one step
    return {
        "schema": 1, "workload": "small", "steps": 1, "expected": checksum,
        "processes": [
            {"fork": fork, "collector": name,
             "samples": [{"checksum": checksum, "elapsed_ns": t} for t in times]}
            for fork, names in enumerate(forks) for name in names
        ],
    }


class ReportTests(unittest.TestCase):
    def test_integer_model(self):
        for workload, steps, checksum in csv.reader(VECTORS.read_text().splitlines(), delimiter="\t"):
            with self.subTest(workload=workload, steps=steps):
                if int(steps) < 0:
                    with self.assertRaises(ValueError):
                        expected(workload, int(steps))
                else:
                    self.assertEqual(expected(workload, int(steps)), int(checksum))

        class Steps(int):
            pass

        self.assertEqual(expected("small", Steps(1)), 619975554)
        for invalid in (True, 1.0, 100001):
            with self.assertRaises(ValueError):
                expected("small", invalid)

    def test_quartiles_and_process_validation(self):
        row, = fork_rows(capture_for([["Ristretto"]], times=(3000,)))
        self.assertEqual(row, (0, "Ristretto", 3, 3, 3))
        for groups in ([["G1", "G1"]], [["G1", "ZGC"], ["G1"]],
                       [["G1", "ZGC"], ["G1", "SerialGC"]], [[], ["ZGC"]]):
            with self.subTest(groups=groups), self.assertRaises(ValueError):
                fork_rows(capture_for(groups))
        capture = capture_for([["SerialGC"]])
        capture["processes"][0]["samples"][0]["checksum"] = 0
        with self.assertRaises(ValueError):
            fork_rows(capture)
        capture["expected"] = 0
        with self.assertRaisesRegex(ValueError, "integer model"):
            fork_rows(capture)

    def test_standalone_cli(self):
        names = ["G1", "Ristretto & GC", "ZGC"]
        capture = capture_for([names[::-1], names])
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            # Copy only the script. Isolate Python and remove external commands from PATH.
            script = root / "report.py"
            script.write_bytes((HERE / "report.py").read_bytes())
            source, table, figure = (root / name for name in ("capture.json", "forks.csv", "forks.svg"))
            source.write_text(json.dumps(capture), encoding="utf-8")
            command = [sys.executable, "-I", "-S", str(script), str(source), str(table), str(figure)]
            env = dict(os.environ, PATH="")
            subprocess.run(command, env=env, check=True)
            with table.open(encoding="utf-8", newline="") as stream:
                rows = list(csv.DictReader(stream))
            self.assertEqual([(int(r["fork"]), r["collector"]) for r in rows],
                             [(fork, name) for fork in (0, 1) for name in names])
            for row in rows:
                self.assertEqual([float(row[k]) for k in ("p25_call_us", "median_call_us", "p75_call_us")],
                                 [1.75, 4, 7.25])

            chart = ET.parse(figure)
            groups = chart.findall("svg:g", NS)
            self.assertEqual(len(groups), 6)
            for start in (0, 3):
                bars = [g.find("svg:line", NS) for g in groups[start:start + 3]]
                self.assertEqual(len({bar.get("x1") for bar in bars}), 3)
                for group, bar in zip(groups[start:start + 3], bars):
                    p25_y, p75_y = float(bar.get("y1")), float(bar.get("y2"))
                    self.assertGreater(p25_y, p75_y)
                    marker = group.find("svg:path", NS)
                    x, y = map(float, marker.get("transform")[len("translate("):-1].split())
                    self.assertEqual(x, float(bar.get("x1")))
                    self.assertLess(p75_y, y)
                    self.assertLess(y, p25_y)
            labels = [text.text for text in chart.findall("svg:text", NS)]
            self.assertTrue(set(names).issubset(labels))

            outputs = table.read_bytes(), figure.read_bytes()
            table.unlink()
            blocked = subprocess.run(command, env=env, capture_output=True)
            self.assertNotEqual(blocked.returncode, 0)
            self.assertFalse(table.exists())
            self.assertEqual(figure.read_bytes(), outputs[1])
            for flag in ("-f", "--overwrite"):
                table.write_bytes(b"previous CSV")
                figure.write_bytes(b"previous SVG")
                subprocess.run([*command, flag], env=env, check=True)
                self.assertEqual((table.read_bytes(), figure.read_bytes()), outputs)


if __name__ == "__main__":
    unittest.main()
