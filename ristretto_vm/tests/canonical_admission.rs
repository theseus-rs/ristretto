#![cfg(feature = "experimental-canonical-runtime")]
#![expect(
    clippy::panic_in_result_fn,
    reason = "test assertions report mismatches"
)]
use ristretto_classfile::{
    ClassFile,
    attributes::{Attribute, Instruction},
};
use ristretto_vm::canonical::Program;

const SCALAR: &[u8] = include_bytes!("fixtures/canonical/CanonicalScalarEntry.class");

#[test]
fn closed_scalar_entries() -> Result<(), Box<dyn std::error::Error>> {
    let file = ClassFile::from_bytes(SCALAR)?;
    let program = Program::from_classfiles([file.clone()])?;
    assert_eq!(
        i32::MIN,
        program.run_scalar("CanonicalScalarEntry", "identity", i32::MIN)?
    );
    assert_eq!(64, program.run_scalar("CanonicalScalarEntry", "answer", 0)?);
    assert!(program.run_scalar("Missing", "answer", 0).is_err());
    assert!(Program::from_classfiles([file.clone(), file]).is_err());
    Ok(())
}

#[test]
fn reject_out_of_scope_and_invalid_bytecode_at_admission() -> Result<(), Box<dyn std::error::Error>>
{
    let mut file = ClassFile::from_bytes(SCALAR)?;
    file.super_class = file.constant_pool.add_class("Missing")?;
    assert!(Program::from_classfiles([file]).is_err());
    for replacement in [Instruction::Iadd, Instruction::Aload_0] {
        let mut file = ClassFile::from_bytes(SCALAR)?;
        let method = file
            .methods
            .iter_mut()
            .find(|method| {
                file.constant_pool
                    .try_get_utf8(method.name_index)
                    .is_ok_and(|name| name == "identity")
            })
            .ok_or("missing fixture method")?;
        let code = method
            .attributes
            .iter_mut()
            .find_map(|attr| match attr {
                Attribute::Code { code, .. } => Some(code),
                _ => None,
            })
            .ok_or("missing code")?;
        *code.first_mut().ok_or("empty code")? = replacement;
        assert!(Program::from_classfiles([file]).is_err());
    }
    Ok(())
}

#[test]
fn strict_verifier_rejects_a_scalar_entry_with_no_operand_stack()
-> Result<(), Box<dyn std::error::Error>> {
    let mut file = ClassFile::from_bytes(SCALAR)?;
    for method in &mut file.methods {
        if file.constant_pool.try_get_utf8(method.name_index)? == "identity" {
            for attribute in &mut method.attributes {
                if let Attribute::Code { max_stack, .. } = attribute {
                    *max_stack = 0;
                }
            }
        }
    }
    assert!(Program::from_classfiles([file]).is_err());
    Ok(())
}

#[test]
fn loop_matches_java_and_obeys_instruction_budget() -> Result<(), Box<dyn std::error::Error>> {
    let file = ClassFile::from_bytes(include_bytes!(
        "fixtures/canonical/CanonicalScalarLoop.class"
    ))?;
    let program = Program::from_classfiles([file])?;
    for line in include_str!("fixtures/canonical/scalar-loop-oracle.txt").lines() {
        let (rounds, expected) = line.split_once(' ').ok_or("invalid oracle row")?;
        assert_eq!(
            program.run_scalar("CanonicalScalarLoop", "calculate", rounds.parse()?)?,
            expected.parse::<i32>()?
        );
    }
    assert!(matches!(
        program.run_scalar_with_limit("CanonicalScalarLoop", "calculate", i32::MAX, 80),
        Err(ristretto_vm::canonical::Error::Limit("instructions"))
    ));
    Ok(())
}

#[test]
fn typed_scalar_results_and_category_two_locals() -> Result<(), Box<dyn std::error::Error>> {
    use ristretto_vm::canonical::ScalarValue::{Double, Float, Int, Long};
    let file = ClassFile::from_bytes(include_bytes!(
        "fixtures/canonical/CanonicalWideScalar.class"
    ))?;
    let program = Program::from_classfiles([file])?;
    for (method, descriptor, input, expected) in [
        ("wide", "(I)J", i32::MIN, Long(-6_442_450_943_999_999_983)),
        ("single", "(I)F", 16_777_217, Float(16_777_216.0)),
        ("fraction", "(I)D", -7, Double(-2.75)),
        ("narrow", "(I)B", 255, Int(-1)),
        ("unsigned", "(I)C", -1, Int(65535)),
        ("saturate", "(I)J", 1, Long(i64::MAX)),
        ("saturate", "(I)J", -1, Long(i64::MIN)),
    ] {
        assert_eq!(
            expected,
            program.run_primitive("CanonicalWideScalar", method, descriptor, input, 100)?
        );
    }
    Ok(())
}
