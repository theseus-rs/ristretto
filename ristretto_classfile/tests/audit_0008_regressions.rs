//! Exception-table boundaries must identify exact instruction boundaries.
#![expect(
    clippy::panic_in_result_fn,
    reason = "regression tests use assertions with fallible fixture setup"
)]
use ristretto_classfile::attributes::{Attribute, ExceptionTableEntry, Instruction};
use ristretto_classfile::byte_reader::ByteReader;
use ristretto_classfile::{ConstantPool, Result};

#[test]
fn exception_range_can_end_at_code_length() -> Result<()> {
    let mut pool = ConstantPool::new();
    let attribute = Attribute::Code {
        name_index: pool.add_utf8("Code")?,
        max_stack: 1,
        max_locals: 0,
        code: vec![
            Instruction::Sipush(1),
            Instruction::Pop,
            Instruction::Return,
        ],
        exception_table: vec![ExceptionTableEntry {
            range_pc: 0..3,
            handler_pc: 2,
            catch_type: 0,
        }],
        attributes: vec![],
    };
    let mut bytes = Vec::new();
    attribute.to_bytes(&mut bytes)?;
    let parsed = Attribute::from_bytes(&pool, &mut ByteReader::new(&bytes))?;
    assert_eq!(parsed, attribute);
    Ok(())
}

#[test]
fn exception_range_rejects_end_inside_instruction() -> Result<()> {
    let mut pool = ConstantPool::new();
    let name = pool.add_utf8("Code")?;
    for end in [1_u16, 2] {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&name.to_be_bytes());
        bytes.extend_from_slice(&25_u32.to_be_bytes());
        bytes.extend_from_slice(&[0, 1, 0, 0, 0, 0, 0, 5]);
        bytes.extend_from_slice(&[17, 0, 1, 87, 177]);
        bytes.extend_from_slice(&[0, 1, 0, 0]);
        bytes.extend_from_slice(&end.to_be_bytes());
        bytes.extend_from_slice(&[0, 4, 0, 0, 0, 0]);
        assert!(Attribute::from_bytes(&pool, &mut ByteReader::new(&bytes)).is_err());
    }
    Ok(())
}
