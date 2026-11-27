import hashlib,io,json,pathlib,shutil,subprocess,sys,tarfile,tempfile,unittest
from canonical_service import digest
from canonical_service_artifacts import verify_build

class ArtifactTests(unittest.TestCase):
    def build(self,root):
        (root/'program').write_bytes(b'finite-test-binary')
        self.archive(root,[('unit.rs',b'fn main() {}')])
        manifest={'unit.rs':hashlib.sha256(b'fn main() {}').hexdigest()}
        (root/'source-manifest.json').write_text(json.dumps(manifest))
        self.receipt(root)
    def archive(self,root,members):
        with tarfile.open(root/'sources.tar.gz','w:gz') as archive:
            for name,data in members:
                member=tarfile.TarInfo(name);member.size=len(data);archive.addfile(member,io.BytesIO(data))
    def receipt(self,root):
        value={'schema_version':1,'source_archive_sha256':digest(root/'sources.tar.gz'),
               'source_manifest_sha256':digest(root/'source-manifest.json'),'binary_sha256':digest(root/'program'),
               'binary_name':'program','target':'native','features':['default'],'command':['cargo','build'],
               'toolchain':'fixture','source_unchanged':True,'qualification':False}
        (root/'build.json').write_text(json.dumps(value))
    def test_archive_content_checks_independent_of_outer_hashes(self):
        for members in [[('unit.rs',b'changed')],[('unit.rs',b'fn main() {}'),('unit.rs',b'fn main() {}')],[],[('../unit.rs',b'fn main() {}')]]:
            with self.subTest(members=members),tempfile.TemporaryDirectory() as tmp:
                root=pathlib.Path(tmp);self.build(root);verify_build(root)
                self.archive(root,members);self.receipt(root)
                with self.assertRaises(ValueError):verify_build(root)
    def test_binary_substitution(self):
        with tempfile.TemporaryDirectory() as tmp:
            root=pathlib.Path(tmp);self.build(root);(root/'program').write_bytes(b'changed')
            with self.assertRaises(ValueError):verify_build(root)
    def test_real_pid_survives_exec(self):
        with tempfile.TemporaryDirectory() as tmp:
            root=pathlib.Path(tmp)
            shutil.copy2('/bin/true',root/'program')
            launcher=pathlib.Path(__file__).with_name('canonical_service_launch.py')
            shutil.copy2(launcher,root/'launcher.py')
            (root/'launch-plan.json').write_text(json.dumps({'run_id':'finite-pid-fixture','mode':'native','command':[str(root/'program')]}))
            process=subprocess.Popen([sys.executable,str(root/'launcher.py'),str(root)])
            self.assertEqual(process.wait(timeout=10),0)
            receipt=json.loads((root/'launch.json').read_text())
            self.assertEqual(receipt['host_process_id'],process.pid)
            self.assertEqual(receipt['host_executable_sha256'],digest(root/'program'))

if __name__=='__main__':unittest.main(verbosity=2)
