import { execFileSync } from 'node:child_process';
import { join } from 'node:path';

export function wasiReleaseDirectory(root) {
  const metadata = execFileSync(
    'cargo',
    ['metadata', '--locked', '--format-version', '1', '--no-deps'],
    {
      cwd: root,
      encoding: 'utf8',
    },
  );
  return join(JSON.parse(metadata).target_directory, 'wasm32-wasip2', 'release');
}
