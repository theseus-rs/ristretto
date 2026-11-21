from pathlib import Path
import sys

def expected(method, value):
    if method == 'nullable': return '0'
    if method == 'booleans': return str(value & 1)
    if method == 'bytes': return str((value + 128) % 256 - 128)
    if method == 'chars': return str(value & 65535)
    if method == 'shorts': return str((value + 32768) % 65536 - 32768)
    if method == 'failArrayStore': return 'java.lang.ArrayStoreException'
    if method == 'failArrayBounds': return 'java.lang.ArrayIndexOutOfBoundsException'
    if method == 'failNullArray': return 'java.lang.NullPointerException'
    if method.startswith('fail'): return 'java.lang.ClassCastException'
    if method in {'objects','ints','longs','floats','doubles','referenceArrays','objectArrays'}: return str(value)
    raise ValueError(method)

rows=[line.split('\t') for line in Path(sys.argv[1]).read_text().splitlines()]
assert len(rows)==85 and len({(m,v) for m,v,_ in rows})==85
for method,value,result in rows: assert expected(method,int(value))==result,(method,value,result)
print('85 HotSpot rows match independent scalar/fault oracle')
