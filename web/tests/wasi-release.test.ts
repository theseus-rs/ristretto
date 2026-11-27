import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { mkdirSync, mkdtempSync, realpathSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { test } from 'node:test';
import { wasiReleaseDirectory } from '../scripts/wasi-release.mjs';

test('uses the same WASI release directory as Cargo with config and environment overrides', (t) => {
  const root = realpathSync(mkdtempSync(join(tmpdir(), 'ristretto-wasi-')));
  t.after(() => rmSync(root, { recursive: true, force: true }));
  mkdirSync(join(root, 'src'));
  writeFileSync(join(root, 'src/lib.rs'), '');
  writeFileSync(
    join(root, 'Cargo.toml'),
    '[package]\nname = "probe"\nversion = "0.1.0"\nedition = "2021"\n',
  );
  execFileSync('cargo', ['generate-lockfile', '--offline'], { cwd: root });
  mkdirSync(join(root, '.cargo'));
  writeFileSync(join(root, '.cargo/config.toml'), '[build]\ntarget-dir = "configured"\n');
  for (const key of ['CARGO_TARGET_DIR', 'CARGO_BUILD_TARGET_DIR']) {
    const previous = process.env[key];
    t.after(() => {
      if (previous === undefined) delete process.env[key];
      else process.env[key] = previous;
    });
    delete process.env[key];
  }
  assert.equal(wasiReleaseDirectory(root), join(root, 'configured', 'wasm32-wasip2', 'release'));
  process.env.CARGO_TARGET_DIR = join(root, 'custom');
  assert.equal(wasiReleaseDirectory(root), join(root, 'custom', 'wasm32-wasip2', 'release'));
});
