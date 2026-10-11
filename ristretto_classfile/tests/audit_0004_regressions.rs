//! Regression coverage added while reviewing the classfile patch series.
#![expect(
    clippy::panic_in_result_fn,
    reason = "regression tests use assertions with fallible fixture setup"
)]
use ristretto_classfile::attributes::Instruction;
use ristretto_classfile::byte_reader::ByteReader;
use ristretto_classfile::{Error, Result};
use std::io::Cursor;

#[test]
fn wide_branch_read_rejects_signed_overflow() {
    for opcode in [200, 201] {
        let mut bytes = vec![0, opcode];
        bytes.extend_from_slice(&i32::MAX.to_be_bytes());
        let mut reader = ByteReader::new(&bytes);
        reader.set_position(1);
        assert!(matches!(
            Instruction::from_bytes(&mut reader),
            Err(Error::InvalidInstruction(code)) if code == opcode
        ));
    }
}

#[test]
fn wide_branch_write_rejects_signed_overflow() {
    for instruction in [Instruction::Goto_w(i32::MIN), Instruction::Jsr_w(i32::MIN)] {
        let mut bytes = Cursor::new(vec![0]);
        bytes.set_position(1);
        assert!(matches!(
            instruction.to_bytes(&mut bytes),
            Err(Error::InvalidInstruction(_))
        ));
    }
}

#[test]
fn wide_branch_signed_boundaries_round_trip() -> Result<()> {
    for instruction in [
        Instruction::Goto_w(i32::MAX),
        Instruction::Jsr_w(i32::MAX),
        Instruction::Goto_w(i32::MIN + 1),
        Instruction::Jsr_w(i32::MIN + 1),
    ] {
        let mut bytes = Cursor::new(vec![0]);
        bytes.set_position(1);
        instruction.to_bytes(&mut bytes)?;
        let mut reader = ByteReader::new(bytes.get_ref());
        reader.set_position(1);
        assert_eq!(Instruction::from_bytes(&mut reader)?, instruction);
    }
    Ok(())
}
