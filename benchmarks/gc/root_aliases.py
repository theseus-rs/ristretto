#!/usr/bin/env python3
"""Evidence boundaries for shared owning-root costs; no pause/JVM qualification."""
MODES = {'legacy', 'canonical-leaf', 'canonical-graph'}
FIELDS = {'version', 'mode', 'copies', 'clone_drop_ns', 'backing_bytes', 'backing_slots',
          'inline_total_bytes', 'charged_bytes', 'drops', 'checksum', 'teardown_bytes',
          'teardown_catalog_bytes'}
COUNTERS = {'allocations', 'bytes', 'releases', 'released_bytes', 'live', 'peak'}
LABELS = ['prepared-before', 'prepared-after', 'prepared-dropped', 'clones-before', 'clones-after']


def validate(row, mode, copies):
    if mode not in MODES or type(copies) is not int or not 1 <= copies <= 10_000_000:
        raise ValueError('invalid root alias workload')
    if not isinstance(row, dict) or set(row) != FIELDS:
        raise ValueError('unexpected root alias schema')
    if any(type(v) is not int or v < 0 for k, v in row.items() if k != 'mode'):
        raise ValueError('root alias measurements must be nonnegative integers')
    expected = dict(version=1, mode=mode, copies=copies, drops=1, checksum=0xa4c97015ee12853b,
                    teardown_bytes=0, teardown_catalog_bytes=0,
                    backing_slots=0 if mode == 'legacy' else 2)
    if any(row[k] != v for k, v in expected.items()) or not row['clone_drop_ns'] or not row['backing_bytes']:
        raise ValueError('root alias integrity or workload mismatch')
    if row['charged_bytes'] != row['inline_total_bytes'] + row['backing_slots'] * row['backing_bytes']:
        raise ValueError('root wrapper storage charge mismatch')
    if (mode == 'legacy') != (row['inline_total_bytes'] == row['charged_bytes'] == 0):
        raise ValueError('missing canonical storage reservation')
    return row


def validate_counts(rows, mode, copies):
    if not isinstance(rows, list) or len(rows) != 6:
        raise ValueError('expected five checkpoints and one report')
    report = validate(rows[-1], mode, copies)
    for label, row in zip(LABELS, rows[:-1], strict=True):
        if not isinstance(row, dict) or set(row) != COUNTERS | {'checkpoint'} or row['checkpoint'] != label:
            raise ValueError('unexpected allocator checkpoint')
        if any(type(row[k]) is not int or row[k] < 0 for k in COUNTERS) or row['peak'] < row['live']:
            raise ValueError('invalid allocator counters')
    before, prepared, dropped, clones_before, clones_after = rows[:-1]
    size = prepared['bytes'] - before['bytes']
    if (size != report['backing_bytes'] or prepared['allocations'] != before['allocations'] + 1
            or prepared['releases'] != before['releases'] or prepared['released_bytes'] != before['released_bytes']
            or prepared['live'] != before['live'] + size):
        raise ValueError('actual wrapper backing differs from reserved layout')
    if (dropped['allocations'] != prepared['allocations'] or dropped['bytes'] != prepared['bytes']
            or dropped['releases'] != prepared['releases'] + 1
            or dropped['released_bytes'] != prepared['released_bytes'] + size
            or dropped['live'] != before['live']):
        raise ValueError('private root wrapper did not release exactly once')
    if any(clones_before[k] != clones_after[k] for k in COUNTERS):
        raise ValueError('clone/drop performed allocator work')
    # Use observed allocation bytes, not only the implementation's layout claim.
    if report['charged_bytes'] != report['inline_total_bytes'] + size * report['backing_slots']:
        raise ValueError('observed root backing was not fully charged')
    return rows
