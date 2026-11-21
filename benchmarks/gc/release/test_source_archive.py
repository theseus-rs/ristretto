import hashlib
import os
from pathlib import Path
import subprocess
import sys
import tarfile
import tempfile
import unittest

ARCHIVER = Path(__file__).with_name('source_archive.py').resolve()


class SourceArchiveChecks(unittest.TestCase):
    def test_conflicting_git_modes_preserve_identical_bytes_and_executables(self):
        with tempfile.TemporaryDirectory() as temporary:
            directory = Path(temporary)
            repository = directory / 'source'
            repository.mkdir()
            subprocess.run(['git', 'init', '--quiet', str(repository)], check=True)
            for key, value in [('user.name', 'Archive test'), ('user.email', 'archive@example.invalid')]:
                subprocess.run(['git', 'config', key, value], cwd=repository, check=True)
            (repository / 'data.txt').write_bytes(b'committed bytes\n')
            (repository / 'run.sh').write_bytes(b'#!/bin/sh\nexit 0\n')
            (repository / 'run.sh').chmod(0o755)
            subprocess.run(['git', 'add', 'data.txt', 'run.sh'], cwd=repository, check=True)
            subprocess.run(['git', '-c', 'commit.gpgsign=false', 'commit', '--quiet', '-m', 'fixture'],
                           cwd=repository, check=True)
            (repository / 'untracked.txt').write_bytes(b'must stay outside the archive\n')
            archives = []
            for name, mask in [('first', '0000'), ('second', '0077')]:
                environment = dict(os.environ, GIT_CONFIG_COUNT='1',
                                   GIT_CONFIG_KEY_0='tar.umask', GIT_CONFIG_VALUE_0=mask)
                destination = directory / name
                subprocess.run([sys.executable, str(ARCHIVER), 'HEAD', str(destination)],
                               cwd=repository, env=environment, check=True, capture_output=True)
                archive, = destination.glob('*.tar.gz')
                archives.append(archive.read_bytes())
                expected = hashlib.sha256(archive.read_bytes()).hexdigest() + '  ' + archive.name + '\n'
                self.assertEqual((destination / 'SHA256SUMS').read_text(), expected)
                with tarfile.open(archive) as bundle:
                    files = {Path(item.name).name: item for item in bundle if item.isfile()}
                    self.assertEqual(set(files), {'data.txt', 'run.sh'})
                    self.assertEqual(files['data.txt'].mode, 0o644)
                    self.assertEqual(files['run.sh'].mode, 0o755)
                    self.assertEqual(bundle.extractfile(files['data.txt']).read(), b'committed bytes\n')
                repeated = subprocess.run([sys.executable, str(ARCHIVER), 'HEAD', str(destination)],
                                          cwd=repository, capture_output=True, text=True)
                self.assertNotEqual(repeated.returncode, 0)
                self.assertIn('output directory already exists', repeated.stderr)
            self.assertEqual(*archives)


if __name__ == '__main__':
    unittest.main()
