import { test } from 'node:test';
import assert from 'node:assert/strict';
import { restoreState } from '../playground/state.ts';
import { executionTarget } from '../shared/protocol.ts';

test('migrates the existing Java draft and selected JDK', () => {
  const saved = restoreState(
    null,
    JSON.stringify({ source: 'class Saved {}', className: 'Saved', javaVersion: 11 }),
  );
  assert.equal(saved.language, 'java');
  assert.equal(saved.javaVersion, 11);
  assert.deepEqual(saved.drafts.java, {
    source: 'class Saved {}',
    className: 'Saved',
    example: 'hello',
  });
});

test('restores separate Scala drafts without replacing the Java selection', () => {
  const saved = restoreState(
    JSON.stringify({
      language: 'scala',
      scalaVersion: '2.13',
      javaVersion: 8,
      drafts: {
        scala2: { source: 'println("two")', className: 'Main', example: 'collections' },
        scala3: { source: 'println("three")', className: 'Main', example: 'hello' },
      },
    }),
    '{',
  );
  assert.equal(saved.javaVersion, 8);
  assert.equal(executionTarget(saved.language, saved.scalaVersion), 'scala2');
  assert.equal(saved.drafts.scala3?.source, 'println("three")');
});

test('ignores malformed storage, invalid choices, and invalid drafts', () => {
  assert.equal(restoreState('{', null).language, 'java');
  const saved = restoreState(
    JSON.stringify({
      language: 'unknown',
      scalaVersion: 'bad',
      javaVersion: 9,
      drafts: { kotlin: { source: 42 } },
    }),
    null,
  );
  assert.deepEqual(saved, { language: 'java', scalaVersion: '3', javaVersion: 25, drafts: {} });
});
