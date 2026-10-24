import { test } from 'node:test';
import assert from 'node:assert/strict';
import { cpSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { toolchainLicenses } from '../scripts/licenses.mjs';

const directory = fileURLToPath(new URL('../licenses', import.meta.url));
const manifest = JSON.parse(readFileSync(join(directory, 'toolchain.json'), 'utf8'));
const compilerVersion = `rustc ${manifest.rust.release}
commit-hash: ${manifest.rust.commit}
release: ${manifest.rust.release}
`;

test('includes Rust and WASI notices with upstream revisions and complete license texts', () => {
  const notices = toolchainLicenses(directory, compilerVersion);
  assert.match(notices, /Copyright \(c\) The Rust Project Contributors/);
  assert.match(notices, /wasi-libc as a whole is multi-licensed/);
  assert.match(notices, /libc-bottom-half\/cloudlibc\/LICENSE/);
  assert.match(notices, /Copyright \(c\) 2015-2017 Nuxi/);
  assert.match(notices, /musl as a whole is licensed/);
  assert.match(notices, /Doug Lea and released to the public domain/);
  assert.match(notices, /fts\/musl-fts\/COPYING/);
  assert.match(notices, /compiler-rt\/LICENSE.TXT/);
  assert.match(notices, /libunwind\/LICENSE.TXT/);
  for (const component of manifest.components) {
    assert.ok(notices.includes(`Source: ${component.repository}/tree/${component.revision}`));
    for (const { path } of component.files) {
      assert.ok(notices.includes(readFileSync(join(directory, component.directory, path), 'utf8')));
    }
  }
});

test('notices track the repository toolchain pin', () => {
  const toolchain = readFileSync(new URL('../../rust-toolchain.toml', import.meta.url), 'utf8');
  assert.equal(toolchain.match(/^channel = "(.+)"$/m)?.[1], manifest.rust.release);
});

test('rejects a different Rust release, commit, or unrecognized version output', () => {
  for (const version of [
    compilerVersion.replace(`release: ${manifest.rust.release}`, 'release: 0.0.0'),
    compilerVersion.replace(manifest.rust.commit, '0'.repeat(40)),
    'rustc unknown',
  ]) {
    assert.throws(() => toolchainLicenses(directory, version), /Refresh web\/licenses/);
  }
});

test('fails when a required notice is missing or modified', (t) => {
  const temporary = mkdtempSync(join(tmpdir(), 'ristretto-licenses-'));
  t.after(() => rmSync(temporary, { recursive: true, force: true }));
  cpSync(directory, temporary, { recursive: true });
  const file = join(temporary, 'wasi-libc/libc-top-half/musl/COPYRIGHT');
  writeFileSync(file, 'Incomplete notice\n');
  assert.throws(() => toolchainLicenses(temporary, compilerVersion), /checksum mismatch/);
  rmSync(file);
  assert.throws(() => toolchainLicenses(temporary, compilerVersion), /ENOENT/);
});
