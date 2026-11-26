#!/usr/bin/env python3
"""Finite HotSpot semantic oracle. No timing/allocation measurements."""
import json
from pathlib import Path
import subprocess
import sys
root = Path(__file__).resolve().parents[3]
java = Path(sys.argv[1]) if len(sys.argv)>1 else Path.home()/'.ristretto/linux-x64/25.0.5.0.10/bin/java'
cases = [("CanonicalStaticRootsSmall", "run", n) for n in (-3,0,7)]
cases += [("CanonicalStaticRootsSmall", "prepared", 0), ("CanonicalStaticRootsFull", "run", 2), ("CanonicalStaticRootsFull", "prepared", 0)]
cases += [("CanonicalStaticRootsSmall", "referenceOverwrite", 22)]
cases += [("CanonicalStaticProtocol", method, 0) for method in ("recursive","newTrigger","getTrigger","putTrigger","callTrigger","noTrigger","fieldless","failure","nestedFailure")]
rows=[]
for owner, method, argument in cases:
    cmd=[str(java),'-Xverify:all','-cp',str(root/'ristretto_vm/tests/fixtures/canonical')+':'+str(Path(__file__).parent),'CanonicalStaticOracle',owner,method,str(argument)]
    result=subprocess.run(cmd,capture_output=True,text=True,check=True,timeout=30)
    rows.append(result.stdout.strip())
    print(json.dumps({'command':cmd,'stdout':result.stdout,'stderr':result.stderr}))
(root/'ristretto_vm/tests/fixtures/canonical/static-oracle.tsv').write_text('\n'.join(rows)+'\n')
