//! Boundary, malformed-input, and lossless round-trip regressions.
#![expect(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "test fixtures and byte-level mutations"
)]
use ristretto_classfile::ConstantPool;
use ristretto_classfile::attributes::{Attribute, Instruction};
use ristretto_classfile::byte_reader::ByteReader;

#[test]
fn byte_reader_arithmetic_is_checked() {
    let mut reader = ByteReader::new(&[1, 2]);
    reader.set_position(usize::MAX);
    assert!(reader.read_u16().is_err());
    assert!(reader.read_u8().is_err());
    assert!(reader.skip(1).is_err());
    reader.set_position(1);
    assert!(reader.read_bytes(usize::MAX).is_err());
    assert_eq!(reader.position(), 1);
}

fn code(pool: &mut ConstantPool<'_>, instructions: Vec<Instruction>) -> Attribute {
    Attribute::Code {
        name_index: pool.add_utf8("Code").unwrap(),
        max_stack: 2,
        max_locals: 1,
        code: instructions,
        exception_table: vec![],
        attributes: vec![],
    }
}

#[test]
fn attribute_payload_lengths_are_enforced() {
    let mut pool = ConstantPool::new();
    let attr = code(&mut pool, vec![Instruction::Return]);
    let mut bytes = Vec::new();
    attr.to_bytes(&mut bytes).unwrap();
    for length in [0_u32, 1, 12, 14, u32::MAX] {
        bytes[2..6].copy_from_slice(&length.to_be_bytes());
        assert!(Attribute::from_bytes(&pool, &mut ByteReader::new(&bytes)).is_err());
    }
    let bytes = [
        0, 1, 0, 0, 0, 14, 0, 0, 0, 0, 0, 0, 0, 1, 177, 0, 0, 0, 0, 0,
    ];
    assert!(Attribute::from_bytes(&pool, &mut ByteReader::new(&bytes)).is_err());
}
