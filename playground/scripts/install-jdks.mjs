import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, renameSync } from 'node:fs';
import { homedir, tmpdir } from 'node:os';
import { join } from 'node:path';
import jdks from '../jdks.json' with { type: 'json' };

const root = process.env.PLAYGROUND_JDK_ROOT ?? join(homedir(), '.ristretto', 'linux-x64');
mkdirSync(root, { recursive: true });
for (const { major, version, url, sha256 } of jdks) {
  const destination = join(root, version);
  const versionFile = join(destination, major === 8 ? 'version.txt' : 'release');
  if (existsSync(versionFile) && readFileSync(versionFile, 'utf8').includes(version)) continue;
  const temporary = mkdtempSync(join(tmpdir(), 'playground-jdk-'));
  const staging = mkdtempSync(join(root, '.install-'));
  try {
    const archive = join(temporary, 'jdk.tar.gz');
    execFileSync(
      'curl',
      ['--proto', '=https', '--tlsv1.2', '-fLsS', '--retry', '3', url, '-o', archive],
      { stdio: 'inherit' },
    );
    if (createHash('sha256').update(readFileSync(archive)).digest('hex') !== sha256)
      throw new Error(`Checksum mismatch for Java ${major}`);
    execFileSync('tar', ['-xzf', archive, '--strip-components=1', '-C', staging]);
    // An incomplete existing download can be replaced only after verification and extraction.
    rmSync(destination, { recursive: true, force: true });
    renameSync(staging, destination);
    console.log(`Installed Corretto ${version}`);
  } finally {
    rmSync(temporary, { recursive: true, force: true });
    rmSync(staging, { recursive: true, force: true });
  }
}
