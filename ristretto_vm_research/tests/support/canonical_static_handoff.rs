use ristretto_classfile::ClassFile;
use ristretto_gc::canonical::Configuration;
use ristretto_vm_research::canonical::{Error, Program};
pub const CLASS: &str = "CanonicalCompiledStaticHandoff";
pub const ORACLE: &str = include_str!(
    "../../../ristretto_vm/tests/fixtures/canonical/compiled-static-handoff-oracle.tsv"
);
pub fn program() -> Result<Program, Error> {
    Program::from_classfiles([
        ClassFile::from_bytes(include_bytes!(
            "../../../ristretto_vm/tests/fixtures/canonical/CanonicalCompiledStaticHandoff.class"
        ))?,
        ClassFile::from_bytes(include_bytes!(
            "../../../ristretto_vm/tests/fixtures/canonical/CanonicalCompiledStaticHandoff$Node.class"
        ))?,
        ClassFile::from_bytes(include_bytes!(
            "../../../ristretto_vm/tests/fixtures/canonical/CanonicalCompiledStaticHandoff$Target.class"
        ))?,
    ])
}
pub fn configuration() -> Configuration {
    Configuration {
        objects: 32,
        roots: 64,
        mutators: 3,
        helpers: 3,
        edges_per_object: 1,
    }
}
