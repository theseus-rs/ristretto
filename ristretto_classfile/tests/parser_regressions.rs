//! Boundary, malformed-input, and lossless round-trip regressions.
#![expect(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "test fixtures and byte-level mutations"
)]
use ristretto_classfile::attributes::{AnnotationElement, Attribute, Instruction};
use ristretto_classfile::byte_reader::ByteReader;
use ristretto_classfile::{Constant, ConstantPool};

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

#[test]
fn deeply_nested_annotations_return_an_error() {
    let mut bytes = Vec::new();
    for _ in 0..30000 {
        bytes.extend_from_slice(&[b'[', 0, 1]);
    }
    bytes.extend_from_slice(&[b'I', 0, 1]);
    assert!(AnnotationElement::from_bytes(&mut ByteReader::new(&bytes)).is_err());
}

#[test]
fn constant_pool_count_and_slot_width_are_checked() {
    let bytes = [0, 2, 5, 0, 0, 0, 0, 0, 0, 0, 1];
    assert!(ConstantPool::from_bytes(&mut ByteReader::new(&bytes)).is_err());
    let mut pool = ConstantPool::new();
    let index = pool.add_long(42).unwrap();
    let other = pool.add_integer(7).unwrap();
    assert!(pool.set(index, Constant::Integer(1)).is_err());
    assert_eq!(pool.try_get(other).unwrap(), &Constant::Integer(7));
    let mut pool = ConstantPool::new();
    for _ in 0..65534 {
        pool.add_integer(0).unwrap();
    }
    assert!(pool.add_integer(1).is_err());
    let mut bytes = Vec::new();
    pool.to_bytes(&mut bytes).unwrap();
    assert_eq!(
        ConstantPool::from_bytes(&mut ByteReader::new(&bytes))
            .unwrap()
            .len(),
        65534
    );
    pool.push(Constant::Integer(1));
    assert!(pool.to_bytes(&mut Vec::new()).is_err());
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
