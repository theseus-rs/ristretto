#!/usr/bin/env python3
"""Reproduce valid javac goto_w classfiles and their HotSpot oracle."""
import argparse
from pathlib import Path
import subprocess
import tempfile

parser = argparse.ArgumentParser()
parser.add_argument('--check', action='store_true')
args = parser.parse_args()
fixture = Path(__file__).resolve().parent
with tempfile.TemporaryDirectory() as directory:
    output = Path(directory)
    helper = ('public final class CanonicalScalarWideJump {\n'
              '  public static int wide(int value) {\n'
              '    if (value == 0) {\n'
              + '      value++;\n' * 11_000
              + '    }\n    return value;\n  }\n}\n')
    caller = ('public final class CanonicalScalarCall {\n'
              '  static int value = CanonicalScalarWideJump.wide(1);\n'
              '  public static int run(int input) { return value + input; }\n}\n')
    oracle = ('public final class CanonicalScalarCallOracle {\n'
              '  public static void main(String[] args) {\n'
              '    if (CanonicalScalarWideJump.wide(0) != 11000) throw new AssertionError();\n'
              '    if (CanonicalScalarCall.run(0) != 1) throw new AssertionError();\n'
              '    System.out.println("wide 0 11000\\nwide 1 1\\nrun 0 1");\n  }\n}\n')
    for name, source in [('CanonicalScalarWideJump', helper), ('CanonicalScalarCall', caller), ('CanonicalScalarCallOracle', oracle)]:
        (output / (name + '.java')).write_text(source)
    subprocess.run(['javac', '--release', '17', '-g', *map(str, output.glob('*.java'))], check=True)
    result = subprocess.run(['java', '-Xverify:all', '-cp', str(output), 'CanonicalScalarCallOracle'], check=True, capture_output=True)
    for name in ['CanonicalScalarWideJump.class', 'CanonicalScalarCall.class']:
        data = (output / name).read_bytes()
        if args.check:
            if (fixture / name).read_bytes() != data:
                raise SystemExit('classfile differs: ' + name)
        else:
            (fixture / name).write_bytes(data)
    if args.check:
        if (fixture / 'oracle.txt').read_bytes() != result.stdout:
            raise SystemExit('oracle differs')
    else:
        (fixture / 'oracle.txt').write_bytes(result.stdout)
