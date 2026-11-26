//! Reproducer for a scalar-only opcode called from an object/initializer frame.
#![cfg(feature = "experimental-canonical-runtime")]
#![expect(clippy::panic_in_result_fn, reason = "boundary assertions")]
use ristretto_classfile::{
    ClassFile,
    attributes::{Attribute, Instruction},
};
use ristretto_vm_research::canonical::{Error, Program};

#[test]
fn scalar_only_call_is_rejected_without_restricting_standalone_scalar_entries() -> Result<(), Error>
{
    let helper = ClassFile::from_bytes(include_bytes!(
        "../../ristretto_vm/tests/fixtures/canonical_call_boundary/CanonicalScalarWideJump.class"
    ))?;
    assert!(helper.methods.iter().flat_map(|method| &method.attributes).any(|attribute| matches!(attribute, Attribute::Code { code, .. } if code.iter().any(|op| matches!(op, Instruction::Goto_w(_))))));
    let standalone = Program::from_classfiles([helper.clone()])?;
    assert_eq!(
        standalone.run_scalar("CanonicalScalarWideJump", "wide", 0)?,
        11_000
    );
    assert_eq!(
        standalone.run_scalar("CanonicalScalarWideJump", "wide", 1)?,
        1
    );
    let caller = ClassFile::from_bytes(include_bytes!(
        "../../ristretto_vm/tests/fixtures/canonical_call_boundary/CanonicalScalarCall.class"
    ))?;
    assert!(
        matches!(Program::from_classfiles([helper, caller]), Err(Error::Unsupported(name)) if name.contains("Goto_w"))
    );
    Ok(())
}
