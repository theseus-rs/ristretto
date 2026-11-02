//! Wide branch offsets must reject signed overflow and preserve valid boundaries.
#![expect(
    clippy::panic_in_result_fn,
    reason = "regression tests use assertions with fallible fixture setup"
)]
use ristretto_classfile::Result;
use ristretto_classfile::attributes::Instruction;
use ristretto_classfile::byte_reader::ByteReader;
use std::io::Cursor;

#[test]
fn wide_branch_read_preserves_maximum_displacement() {
    for opcode in [200, 201] {
        let mut bytes = vec![0, opcode];
        bytes.extend_from_slice(&i32::MAX.to_be_bytes());
        let mut reader = ByteReader::new(&bytes);
        reader.set_position(1);
        let instruction = Instruction::from_bytes(&mut reader).unwrap();
        assert_eq!(instruction.branch_offsets().collect::<Vec<_>>(), [i32::MAX]);
    }
}

#[test]
fn wide_branch_write_preserves_minimum_displacement() {
    for instruction in [Instruction::Goto_w(i32::MIN), Instruction::Jsr_w(i32::MIN)] {
        let mut bytes = Cursor::new(vec![0]);
        bytes.set_position(1);
        instruction.to_bytes(&mut bytes).unwrap();
        assert_eq!(
            bytes.get_ref().get(2..),
            Some(i32::MIN.to_be_bytes().as_slice())
        );
    }
}

#[test]
fn wide_branch_signed_boundaries_round_trip() -> Result<()> {
    for instruction in [
        Instruction::Goto_w(i32::MAX),
        Instruction::Jsr_w(i32::MAX),
        Instruction::Goto_w(i32::MIN),
        Instruction::Jsr_w(i32::MIN),
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
