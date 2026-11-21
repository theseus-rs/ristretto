#!/usr/bin/env python3
"""Independent integer/index model of the bounded retained-ring workload."""
import argparse
from pathlib import Path

CONFIGURATIONS = {'small': (8, 64, 63), 'medium': (32, 1024, 255), 'large': (128, 4096, 1023)}
STEPS = (-1, 0, 1, 7, 17, 65, 257)

def signed(value, bits):
    value &= (1 << bits) - 1
    return value - (1 << bits) if value >= (1 << (bits - 1)) else value

def inspect(generation, base, mask, salt):
    length = base + (generation & mask)
    return signed(salt * 31 + length + signed(generation, 8) * 7 + signed(generation * 17 + 3, 8), 32)

def expected(workload, steps):
    if steps < 0:
        return -1
    retained, base, mask = CONFIGURATIONS[workload]
    # Live sequence only: removed self-cycles have no effect on later values.
    ring = list(range(retained))
    head = 0
    cursor = 0
    checksum = 1
    for step in range(steps):
        old_index = (ring.index(cursor) + 1) % retained
        removed = ring[old_index]
        replacement = step + retained
        ring[old_index] = replacement
        if head == removed:
            head = replacement
        checksum = inspect(removed, base, mask, signed(checksum + removed, 32))
        cursor = ring[(old_index + 1) % retained]
    start = ring.index(head)
    for offset in range(retained):
        generation = ring[(start + offset) % retained]
        checksum = inspect(generation, base, mask, signed(checksum + generation, 32))
    return checksum

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('oracle', type=Path)
    args = parser.parse_args()
    rows = []
    for line in args.oracle.read_text().splitlines():
        if line.startswith('#'):
            continue
        workload, steps, value = line.split('\t')
        rows.append((workload, int(steps), int(value)))
    wanted = [(workload, steps, expected(workload, steps)) for workload in CONFIGURATIONS for steps in STEPS]
    if rows != wanted:
        raise SystemExit('oracle mismatch, missing, duplicate or reordered cases')
    print(f'{len(rows)} HotSpot cases match independent ring model; no performance qualification')

if __name__ == '__main__':
    main()
