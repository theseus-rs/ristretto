import { test } from 'node:test';
import assert from 'node:assert/strict';
import { EventDecoder, type Event } from '../src/protocol.ts';

test('decodes fragmented records and UTF-8 independently across stdout and stderr', () => {
  const events: Event[] = [];
  const decoder = new EventDecoder((event) => events.push(event));
  const data = [
    { id: 1, type: 'output', stream: 'stdout', bytes: [0xe2] },
    { id: 1, type: 'output', stream: 'stderr', bytes: [65] },
    { id: 1, type: 'output', stream: 'stdout', bytes: [0x98, 0x95] },
    { id: 1, type: 'done' },
  ]
    .map((event) => JSON.stringify(event) + '\n')
    .join('');
  for (const byte of new TextEncoder().encode(data)) decoder.push(new Uint8Array([byte]));
  assert.deepEqual(events, [
    { id: 1, type: 'output', stream: 'stderr', text: 'A' },
    { id: 1, type: 'output', stream: 'stdout', text: '☕' },
    { id: 1, type: 'done' },
  ]);
});

test('flushes incomplete output at termination instead of losing it', () => {
  const events: Event[] = [];
  const decoder = new EventDecoder((event) => events.push(event));
  decoder.push(
    new TextEncoder().encode(
      '{"id":2,"type":"output","stream":"stdout","bytes":[226]}\n{"id":2,"type":"error","message":"stopped"}\n',
    ),
  );
  assert.deepEqual(events, [
    { id: 2, type: 'output', stream: 'stdout', text: '�' },
    { id: 2, type: 'error', message: 'stopped' },
  ]);
});
