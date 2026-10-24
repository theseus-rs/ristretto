import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { existsSync, readFileSync, readdirSync, statSync } from 'node:fs';
import { dirname, join } from 'node:path';

/** Notices for libraries bundled with Rust's WASI target, outside Cargo metadata. */
export function toolchainLicenses(directory, compilerVersion) {
  const manifest = JSON.parse(readFileSync(join(directory, 'toolchain.json'), 'utf8'));
  const release = compilerVersion.match(/^release: (.+)$/m)?.[1];
  const commit = compilerVersion.match(/^commit-hash: (.+)$/m)?.[1];
  if (release !== manifest.rust.release || commit !== manifest.rust.commit) {
    throw new Error(
      `Toolchain notices cover rustc ${manifest.rust.release} (${manifest.rust.commit}), ` +
        `but the build uses ${release ?? 'unknown'} (${commit ?? 'unknown'}). ` +
        'Refresh web/licenses as described in web/licenses/README.md.',
    );
  }
  const sections = [];
  for (const component of manifest.components) {
    sections.push(
      `\n=== ${component.name} ===\nLicense: ${component.license}\n` +
        `Source: ${component.repository}/tree/${component.revision}`,
    );
    for (const { path, sha256 } of component.files) {
      const file = join(directory, component.directory, path);
      const bytes = readFileSync(file);
      if (createHash('sha256').update(bytes).digest('hex') !== sha256) {
        throw new Error(`Toolchain notice checksum mismatch: ${file}`);
      }
      sections.push(`\n--- ${component.directory}/${path} ---\n${bytes.toString('utf8')}`);
    }
  }
  return sections.join('\n\n') + '\n';
}

/** Collect the actual installed licenses alongside the distributed binaries. */
export function licenses(root, playground) {
  const sections = ['Ristretto Java playground — dependency licenses and notices'];
  sections.push(
    toolchainLicenses(
      join(playground, 'licenses'),
      execFileSync(process.env.RUSTC ?? 'rustc', ['--version', '--verbose'], {
        cwd: root,
        encoding: 'utf8',
      }),
    ),
  );
  function include(name, directory, license, repository, explicit) {
    const files = new Set();
    function scan(path, depth = 0) {
      for (const entry of readdirSync(path)) {
        if (!/^(licen[sc]e|copying|notice)([._-]|$)/i.test(entry)) continue;
        const file = join(path, entry);
        if (statSync(file).isFile()) files.add(file);
        else if (depth < 2) scan(file, depth + 1);
      }
    }
    scan(directory);
    if (explicit && existsSync(explicit)) files.add(explicit);
    sections.push(
      `\n=== ${name} ===\nLicense: ${license ?? 'See upstream'}\nSource: ${repository ?? ''}`,
    );
    for (const file of files) sections.push(readFileSync(file, 'utf8'));
  }
  include('Ristretto', root, 'Apache-2.0 OR MIT', 'https://github.com/theseus-rs/ristretto');
  const metadata = JSON.parse(
    execFileSync(
      'cargo',
      ['metadata', '--locked', '--format-version', '1', '--filter-platform', 'wasm32-wasip2'],
      { cwd: root, encoding: 'utf8', maxBuffer: 16 * 1024 * 1024 },
    ),
  );
  const packages = new Map(metadata.packages.map((pkg) => [pkg.id, pkg]));
  const nodes = new Map(metadata.resolve.nodes.map((node) => [node.id, node]));
  const seen = new Set();
  function visit(id) {
    if (seen.has(id)) return;
    seen.add(id);
    const pkg = packages.get(id);
    include(
      `${pkg.name} ${pkg.version}`,
      dirname(pkg.manifest_path),
      pkg.license,
      pkg.repository,
      pkg.license_file,
    );
    for (const dependency of nodes.get(id).dependencies) visit(dependency);
  }
  visit(metadata.packages.find((pkg) => pkg.name === 'ristretto_playground').id);
  const lock = JSON.parse(readFileSync(join(playground, 'package-lock.json'), 'utf8'));
  for (const [path, pkg] of Object.entries(lock.packages)) {
    if (!path || (pkg.dev && !path.endsWith('/jco-transpile'))) continue;
    const directory = join(playground, path);
    if (!existsSync(join(directory, 'package.json'))) continue; // Optional packages for other hosts.
    const info = JSON.parse(readFileSync(join(directory, 'package.json'), 'utf8'));
    include(
      `${info.name} ${pkg.version}`,
      directory,
      info.license,
      typeof info.repository === 'string' ? info.repository : info.repository?.url,
    );
  }
  return sections.join('\n\n') + '\n';
}
