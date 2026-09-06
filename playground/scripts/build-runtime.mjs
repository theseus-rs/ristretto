import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import {
  existsSync,
  mkdirSync,
  readFileSync,
  readdirSync,
  rmSync,
  statSync,
  writeFileSync,
} from 'node:fs';
import { homedir } from 'node:os';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { transpileBytes } from '@bytecodealliance/jco-transpile';
import { zipSync } from 'fflate';
import { licenses } from './licenses.mjs';
import jdks from '../jdks.json' with { type: 'json' };

const playground = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const root = resolve(playground, '..');
const generated = join(playground, 'generated');
const assets = join(playground, 'public', 'runtime');
const run = (command, args, options = {}) =>
  execFileSync(command, args, { cwd: root, stdio: 'inherit', ...options });
if (!process.argv.includes('--skip-rust')) {
  run(
    'cargo',
    ['build', '--locked', '-p', 'ristretto_playground', '--target', 'wasm32-wasip2', '--release'],
    {
      env: {
        ...process.env,
        RUSTC_BOOTSTRAP: '1',
        CARGO_TARGET_WASM32_WASIP2_RUSTFLAGS: '-C link-arg=-zstack-size=8388608',
      },
    },
  );
}
rmSync(generated, { recursive: true, force: true });
rmSync(assets, { recursive: true, force: true });
mkdirSync(generated, { recursive: true });
mkdirSync(assets, { recursive: true });
const { files } = await transpileBytes(
  readFileSync(join(root, 'target/wasm32-wasip2/release/ristretto_playground.wasm')),
  {
    name: 'runner',
    instantiation: 'async',
    asyncMode: 'sync',
    wasiShim: false,
    nodejsCompat: false,
    base64Cutoff: 0,
    emitTypescriptDeclarations: true,
  },
);
const manifest = {
  jdks: {},
  files: {},
};
function asset(logicalName, bytes) {
  const hash = createHash('sha256').update(bytes).digest('hex');
  const file = `${hash.slice(0, 16)}-${logicalName}`;
  writeFileSync(join(assets, file), bytes);
  return { file, sha256: hash, size: bytes.length };
}
for (const [name, bytes] of Object.entries(files)) {
  if (name.endsWith('.wasm')) manifest.files[name] = asset(name, bytes);
  else {
    const path = join(generated, name);
    mkdirSync(dirname(path), { recursive: true });
    writeFileSync(path, bytes);
  }
}
const notices = [];
for (const { major, version } of jdks) {
  const jdk = join(
    process.env.PLAYGROUND_JDK_ROOT ?? join(homedir(), '.ristretto', 'linux-x64'),
    version,
  );
  const versionFile = join(jdk, major === 8 ? 'version.txt' : 'release');
  if (!existsSync(versionFile) || !readFileSync(versionFile, 'utf8').includes(version))
    throw new Error(`Run npm run install:jdks to install Linux Corretto ${version}.`);
  let imageDir = jdk;
  if (major !== 8) {
    const macJlink = join(
      homedir(),
      '.ristretto',
      `macos-${process.arch === 'arm64' ? 'aarch64' : 'x64'}`,
      version,
      'Contents',
      'Home',
      'bin',
      'jlink',
    );
    const jlink =
      process.env[`PLAYGROUND_JLINK_${major}`] ??
      (process.platform === 'darwin' ? macJlink : join(jdk, 'bin', 'jlink'));
    imageDir = join(root, 'target', `playground-jdk-${major}`);
    rmSync(imageDir, { recursive: true, force: true });
    run(jlink, [
      '--module-path',
      join(jdk, 'jmods'),
      '--add-modules',
      'java.base,java.compiler,jdk.compiler,jdk.zipfs',
      // Ristretto reads uncompressed jimage entries. Compress the transport archive instead.
      '--endian',
      'little',
      '--disable-plugin',
      'compress',
      '--strip-debug',
      '--no-header-files',
      '--no-man-pages',
      '--output',
      imageDir,
    ]);
  }
  const runtimeFiles = {};
  function include(name) {
    const path = join(imageDir, name);
    if (!existsSync(path)) return;
    if (statSync(path).isDirectory()) {
      for (const child of readdirSync(path).sort()) include(`${name}/${child}`);
    } else {
      // Dereference legal-notice symlinks; archives contain regular files only.
      runtimeFiles[name] = [
        new Uint8Array(readFileSync(path)),
        { mtime: new Date('2026-01-01T00:00:00Z') },
      ];
    }
  }
  const names =
    major === 8
      ? [
          'version.txt',
          'lib/tools.jar',
          'jre/lib/rt.jar',
          'jre/lib/jsse.jar',
          'jre/lib/jce.jar',
          'jre/lib/charsets.jar',
          'jre/lib/resources.jar',
          'jre/lib/tzdb.dat',
          'jre/lib/currency.data',
          'jre/lib/security',
          'jre/lib/ext',
          'LICENSE',
          'ASSEMBLY_EXCEPTION',
          'THIRD_PARTY_README',
        ]
      : ['release', 'lib/modules', 'lib/tzdb.dat', 'conf', 'legal'];
  for (const name of names) include(name);
  manifest.jdks[major] = {
    version,
    ...asset(`jdk-${major}.zip`, zipSync(runtimeFiles, { level: 6 })),
  };
  notices.push(
    `\n=== Amazon Corretto ${version} ===\nSource: https://github.com/corretto/corretto-${major}/tree/${version}`,
  );
  for (const [path, [bytes]] of Object.entries(runtimeFiles)) {
    if (path.startsWith('legal/') || /^(LICENSE|ASSEMBLY_EXCEPTION|THIRD_PARTY_README)$/.test(path))
      notices.push(`\n--- ${path} ---\n${Buffer.from(bytes).toString('utf8')}`);
  }
}
writeFileSync(
  join(assets, 'THIRD_PARTY_LICENSES.txt'),
  licenses(root, playground) + notices.join('\n\n') + '\n',
);
writeFileSync(join(generated, 'runtime-manifest.json'), JSON.stringify(manifest, null, 2) + '\n');
for (const item of [...Object.values(manifest.files), ...Object.values(manifest.jdks)]) {
  if (item.size >= 100 * 1024 * 1024)
    throw new Error(`Asset exceeds GitHub's file limit: ${item.file}`);
}
console.log(
  `Runtime assets: ${([...Object.values(manifest.files), ...Object.values(manifest.jdks)].reduce((sum, item) => sum + item.size, 0) / 1024 / 1024).toFixed(1)} MiB`,
);
