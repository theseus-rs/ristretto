#!/usr/bin/env python3
"""Final shared-root backing must be freed before its catalog credit is refunded."""
FIELDS = {'version', 'mode', 'checkpoints', 'refunds', 'last_free', 'last_refund',
          'frees', 'refunded_bytes', 'before_charge', 'after_charge', 'valid'}


def validate(rows):
    if not isinstance(rows, list) or len(rows) != 2:
        raise ValueError('expected leaf and graph final-owner audit rows')
    for mode, row in zip(['leaf', 'graph'], rows, strict=True):
        if not isinstance(row, dict) or set(row) != FIELDS:
            raise ValueError('unexpected final-owner audit schema')
        if any(type(v) is not int or v < 0 for k, v in row.items() if k not in ['mode', 'valid']):
            raise ValueError('final-owner counters must be nonnegative integers')
        if row['version'] != 1 or row['mode'] != mode or row['checkpoints'] != 2:
            raise ValueError('final-owner audit identity or checkpoints differ')
        if (row['refunds'] != 1 or not row['before_charge'] or row['after_charge'] != 0
                or row['refunded_bytes'] != row['before_charge']):
            raise ValueError('final-owner credit refund differs')
        if not row['frees'] or not row['last_free'] or row['last_refund'] <= row['last_free']:
            raise ValueError('shared root backing freed after credit refund')
        if row['valid'] is not True:
            raise ValueError('final-owner audit reported a failure')
    if rows[1]['last_free'] <= rows[0]['last_refund']:
        raise ValueError('final-owner audit sequence did not advance')
    return rows
