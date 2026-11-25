"""Create a valid package-private class variant and verify HotSpot access behavior."""
from pathlib import Path
import hashlib, json, shutil, subprocess, sys

source, output, jdk = map(Path, sys.argv[1:])
output.mkdir(exist_ok=False)
fixture = source / 'ristretto_vm/tests/fixtures/canonical_access'
commands = []
for variant in ['public', 'package']:
    classes = output / variant
    shutil.copytree(fixture, classes)
    target = classes / 'casttarget/CanonicalCastAccessTarget.class'
    data = bytearray(target.read_bytes())
    count = int.from_bytes(data[8:10], 'big')
    index, offset = 1, 10
    while index < count:
        tag = data[offset]; offset += 1
        if tag == 1:
            size = int.from_bytes(data[offset:offset+2], 'big'); offset += 2 + size
        elif tag in [3,4,9,10,11,12,17,18]: offset += 4
        elif tag in [5,6]: offset += 8; index += 1
        elif tag in [7,8,16,19,20]: offset += 2
        elif tag == 15: offset += 3
        else: raise ValueError(tag)
        index += 1
    flags = int.from_bytes(data[offset:offset+2], 'big')
    assert flags & 1
    if variant == 'package':
        data[offset:offset+2] = (flags & ~1).to_bytes(2, 'big')
        target.write_bytes(data)
    command = [str(jdk/'javac'), '--release', '17', '-g', '-d', str(classes), str(source/'benchmarks/gc/canonical-cast-oracle/CanonicalCastAccessOracle.java')]
    commands.append(command); subprocess.run(command, check=True)
    command = [str(jdk/'java'), '-Xverify:all', '-cp', str(classes), 'CanonicalCastAccessOracle', variant]
    commands.append(command)
    with (output/(variant+'.tsv')).open('w') as stream:
        subprocess.run(command, check=True, stdout=stream, stderr=subprocess.PIPE)
rows = []
for variant in ['public','package']:
    for line in (output/(variant+'.tsv')).read_text().splitlines():
        mode, caller, method, result = line.split('\t')
        if caller.startswith('casttarget.'):
            expected = '3'
        elif mode == 'package':
            expected = 'java.lang.IllegalAccessError'
        elif method in ['classCast','arrayCast']:
            expected = 'java.lang.ClassCastException'
        else:
            expected = '3'
        assert result == expected, (mode, caller, method, result, expected)
        rows.append(line)
assert len(rows) == 16
(output/'receipt.json').write_text(json.dumps({'commands':commands,'rows':rows,'variant':'Only ACC_PUBLIC cleared from target ClassFile; canonical admission rejects cross-package use instead of executing unsupported linking errors.','digests':{str(p.relative_to(output)):hashlib.sha256(p.read_bytes()).hexdigest() for p in output.rglob('*.class')}},indent=2)+'\n')
print('16 HotSpot class-access rows match public/package policy')
