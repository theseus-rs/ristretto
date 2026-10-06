//! Boundary, malformed-input, and lossless round-trip regressions.
#![expect(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "test fixtures and byte-level mutations"
)]
use ristretto_classfile::attributes::{
    AnnotationElement, Attribute, ExceptionTableEntry, Instruction, LocalVariableTable, StackFrame,
    VerificationType,
};
use ristretto_classfile::byte_reader::ByteReader;
use ristretto_classfile::{ClassFile, Constant, ConstantPool};

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

#[test]
fn switches_reject_invalid_ranges_counts_and_key_order() {
    let cases: &[&[u8]] = &[
        &[171, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255],
        &[170, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 1],
        &[
            171, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0,
        ],
        &[
            171, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0,
        ],
    ];
    for bytes in cases {
        assert!(Instruction::from_bytes(&mut ByteReader::new(bytes)).is_err());
    }
    let mut reader = ByteReader::new(&[0, 200, 127, 255, 255, 255]);
    reader.read_u8().unwrap();
    assert!(Instruction::from_bytes(&mut reader).is_err());
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

#[test]
fn trailing_class_data_is_rejected_by_both_parsers() {
    let mut bytes = include_bytes!("../../classes/Minimum.class").to_vec();
    bytes.push(0);
    assert!(ClassFile::from_bytes(&bytes).is_err());
    assert!(ClassFile::from_slice(&bytes).is_err());
}

#[test]
fn unknown_attribute_context_and_version_preserve_payload() {
    let mut class = ClassFile::from_bytes(include_bytes!("../../classes/Minimum.class")).unwrap();
    class.version = ristretto_classfile::JAVA_8;
    let index = class.constant_pool.add_utf8("Code").unwrap();
    class.attributes.push(Attribute::Unknown {
        name_index: index,
        info: vec![1, 2],
    });
    let future = class.constant_pool.add_utf8("Record").unwrap();
    class.attributes.push(Attribute::Unknown {
        name_index: future,
        info: vec![255],
    });
    let mut bytes = Vec::new();
    class.to_bytes(&mut bytes).unwrap();
    let parsed = ClassFile::from_bytes(&bytes).unwrap();
    assert!(
        matches!(parsed.attributes.last(), Some(Attribute::Unknown { info, .. }) if info == &[255])
    );
    let mut result = Vec::new();
    parsed.to_bytes(&mut result).unwrap();
    assert_eq!(bytes, result);
}

#[test]
fn source_debug_extension_preserves_unpaired_surrogates() {
    let mut pool = ConstantPool::new();
    pool.add_utf8("SourceDebugExtension").unwrap();
    let bytes = [0, 1, 0, 0, 0, 3, 0xed, 0xa0, 0x80];
    let attr = Attribute::from_bytes(&pool, &mut ByteReader::new(&bytes)).unwrap();
    let mut out = Vec::new();
    attr.to_bytes(&mut out).unwrap();
    assert_eq!(out, bytes);
    assert!(ristretto_classfile::mutf8::from_bytes(&[0]).is_err());
    assert!(ristretto_classfile::mutf8::from_bytes_cow(&[0]).is_err());
}

#[test]
fn code_metadata_relocates_after_instruction_width_changes() {
    let mut pool = ConstantPool::new();
    let local_name = pool.add_utf8("x").unwrap();
    let descriptor = pool.add_utf8("I").unwrap();
    let table_name = pool.add_utf8("LocalVariableTable").unwrap();
    let frames_name = pool.add_utf8("StackMapTable").unwrap();
    let class_index = pool.add_class("java/lang/Object").unwrap();
    let mut attr = code(
        &mut pool,
        vec![
            Instruction::Bipush(0),
            Instruction::Pop,
            Instruction::New(class_index),
            Instruction::Pop,
            Instruction::Return,
        ],
    );
    if let Attribute::Code {
        exception_table,
        attributes,
        ..
    } = &mut attr
    {
        exception_table.push(ExceptionTableEntry {
            range_pc: 0..5,
            handler_pc: 4,
            catch_type: 0,
        });
        attributes.push(Attribute::LocalVariableTable {
            name_index: table_name,
            variables: vec![LocalVariableTable {
                start_pc: 1,
                length: 4,
                name_index: local_name,
                descriptor_index: descriptor,
                index: 0,
            }],
        });
        attributes.push(Attribute::StackMapTable {
            name_index: frames_name,
            frames: vec![StackFrame::FullFrame {
                frame_type: 255,
                offset_delta: 3,
                locals: vec![],
                stack: vec![VerificationType::Uninitialized { offset: 2 }],
            }],
        });
    }
    // Inspect nested attributes outside Code parsing so offsets remain raw byte offsets.
    // A round trip alone cannot detect a parser and writer sharing the same wrong units.
    let check_encoded_offsets = |bytes: &[u8], local_start: u16, allocation: u16| {
        let mut reader = ByteReader::new(bytes);
        reader.skip(10).unwrap();
        let code_length = usize::try_from(reader.read_u32().unwrap()).unwrap();
        reader.skip(code_length).unwrap();
        let handlers = usize::from(reader.read_u16().unwrap());
        reader.skip(handlers * 8).unwrap();
        assert_eq!(reader.read_u16().unwrap(), 2);
        assert_eq!(
            Attribute::from_bytes(&pool, &mut reader).unwrap(),
            Attribute::LocalVariableTable {
                name_index: table_name,
                variables: vec![LocalVariableTable {
                    start_pc: local_start,
                    length: 6,
                    name_index: local_name,
                    descriptor_index: descriptor,
                    index: 0,
                }],
            }
        );
        assert_eq!(
            Attribute::from_bytes(&pool, &mut reader).unwrap(),
            Attribute::StackMapTable {
                name_index: frames_name,
                frames: vec![StackFrame::FullFrame {
                    frame_type: 255,
                    offset_delta: allocation + 3,
                    locals: vec![],
                    stack: vec![VerificationType::Uninitialized { offset: allocation }],
                }],
            }
        );
    };
    let mut bytes = Vec::new();
    attr.to_bytes(&mut bytes).unwrap();
    check_encoded_offsets(&bytes, 2, 3);
    let mut parsed = Attribute::from_bytes(&pool, &mut ByteReader::new(&bytes)).unwrap();
    assert_eq!(parsed, attr);
    if let Attribute::Code { code, .. } = &mut parsed {
        code[0] = Instruction::Sipush(0);
    }
    let mut changed = Vec::new();
    parsed.to_bytes(&mut changed).unwrap();
    check_encoded_offsets(&changed, 3, 4);
    assert_eq!(changed.len(), bytes.len() + 1);
    assert_eq!(
        Attribute::from_bytes(&pool, &mut ByteReader::new(&changed)).unwrap(),
        parsed
    );
}

#[test]
fn malformed_constructed_stack_frames_are_not_serialized() {
    for frame in [
        StackFrame::SameFrame { frame_type: 64 },
        StackFrame::SameLocals1StackItemFrame {
            frame_type: 64,
            stack: vec![VerificationType::Integer, VerificationType::Integer],
        },
        StackFrame::AppendFrame {
            frame_type: 252,
            offset_delta: 0,
            locals: vec![],
        },
    ] {
        assert!(frame.to_bytes(&mut Vec::new()).is_err());
    }
    let mut pool = ConstantPool::new();
    assert!(
        code(&mut pool, vec![Instruction::Nop; 65536])
            .to_bytes(&mut Vec::new())
            .is_err()
    );
}

#[test]
fn signature_checks_accept_context_and_recursive_bounds() {
    use ristretto_classfile::verifiers::signature::{
        verify_class_signature, verify_method_signature_with_context,
    };
    assert!(verify_class_signature("<T:Ljava/lang/Comparable<TT;>;>Ljava/lang/Object;").is_ok());
    assert!(verify_class_signature("<T:TU;U:Ljava/lang/Object;>Ljava/lang/Object;").is_ok());
    assert!(verify_class_signature("Lfoo-bar;").is_ok());
    assert!(verify_method_signature_with_context("(TT;)TT;", &["T".into()]).is_ok());
    assert!(verify_method_signature_with_context("(TU;)TT;", &["T".into()]).is_err());
    let signature = format!("LBox<{}I>;", "[".repeat(30000));
    assert!(verify_class_signature(&signature).is_err());
}
