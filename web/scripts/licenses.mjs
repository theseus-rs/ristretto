import { execFileSync } from 'node:child_process';
import { existsSync, readFileSync, readdirSync, statSync } from 'node:fs';
import { dirname, join } from 'node:path';

/** Collect the actual installed licenses alongside the distributed binaries. */
export function licenses(root, playground) {
  const sections = ['Ristretto Java playground — dependency licenses and notices'];
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
