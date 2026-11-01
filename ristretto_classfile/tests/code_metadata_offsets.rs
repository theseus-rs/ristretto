//! Code metadata retains byte offsets while requiring instruction boundaries.
#![expect(clippy::panic_in_result_fn, reason = "fallible regression fixtures")]

use ristretto_classfile::attributes::{
    Attribute, Instruction, LineNumber, LocalVariableTable, LocalVariableTarget,
    LocalVariableTypeTable, StackFrame, TargetType, TypeAnnotation, VerificationType,
};
use ristretto_classfile::verifiers::bytecode::{VerifierConfig, verify_class};
use ristretto_classfile::verifiers::context::VerificationContext;
use ristretto_classfile::verifiers::error::Result as VerifyResult;
use ristretto_classfile::{ClassAccessFlags, ClassFile, JAVA_8, Method, MethodAccessFlags, Result};

struct Context;

impl VerificationContext for Context {
    fn is_subclass(&self, source: &str, target: &str) -> VerifyResult<bool> {
        self.is_assignable(target, source)
    }

    fn is_assignable(&self, target: &str, source: &str) -> VerifyResult<bool> {
        Ok(source == target || target == "java/lang/Object")
    }

    fn common_superclass(&self, a: &str, b: &str) -> VerifyResult<String> {
        Ok(if a == b { a } else { "java/lang/Object" }.to_string())
    }
}

fn class() -> Result<ClassFile<'static>> {
    let mut class = ClassFile {
        version: JAVA_8,
        access_flags: ClassAccessFlags::PUBLIC,
        ..ClassFile::default()
    };
    class.this_class = class.constant_pool.add_class("MetadataOffsets")?;
    class.super_class = class.constant_pool.add_class("java/lang/Object")?;
    Ok(class)
}

fn check_metadata(class: ClassFile<'_>, metadata: Attribute, valid: bool) -> Result<()> {
    // Instruction starts at 0, 3 and 4; code_length is 5.
    let code = vec![
        Instruction::Sipush(1),
        Instruction::Pop,
        Instruction::Return,
    ];
    check_code_metadata(class, metadata, code, valid)
}

fn check_code_metadata(
    mut class: ClassFile<'_>,
    metadata: Attribute,
    code: Vec<Instruction>,
    valid: bool,
) -> Result<()> {
    class.methods.push(Method {
        access_flags: MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
        name_index: class.constant_pool.add_utf8("test")?,
        descriptor_index: class.constant_pool.add_utf8("()V")?,
        attributes: vec![Attribute::Code {
            name_index: class.constant_pool.add_utf8("Code")?,
            max_stack: 1,
            max_locals: 1,
            code,
            exception_table: vec![],
            attributes: vec![metadata],
        }],
    });
    let mut bytes = Vec::new();
    class.to_bytes(&mut bytes)?;
    let parsed = ClassFile::from_bytes(&bytes);
    assert_eq!(parsed.is_ok(), valid, "parser: {parsed:?}");
    if let Ok(parsed) = parsed {
        let mut encoded = Vec::new();
        parsed.to_bytes(&mut encoded)?;
        assert_eq!(encoded, bytes, "metadata byte offsets must be preserved");
    }
    let verified = verify_class(&class, &Context, &VerifierConfig::strict());
    assert_eq!(verified.is_ok(), valid, "verifier: {verified:?}");
    Ok(())
}

#[test]
fn line_numbers_require_instruction_starts() -> Result<()> {
    for offset in [0, 1, 2, 3, 4, 5, 6, u16::MAX] {
        let mut class = class()?;
        let metadata = Attribute::LineNumberTable {
            name_index: class.constant_pool.add_utf8("LineNumberTable")?,
            line_numbers: vec![LineNumber {
                start_pc: offset,
                line_number: 1,
            }],
        };
        check_metadata(class, metadata, matches!(offset, 0 | 3 | 4))?;
    }
    Ok(())
}

fn annotation(
    class: &mut ClassFile<'_>,
    target_type: TargetType,
    visible: bool,
) -> Result<Attribute> {
    let type_annotations = vec![TypeAnnotation {
        target_type,
        type_path: vec![],
        type_index: class.constant_pool.add_utf8("LAnnotation;")?,
        elements: vec![],
    }];
    Ok(if visible {
        Attribute::RuntimeVisibleTypeAnnotations {
            name_index: class
                .constant_pool
                .add_utf8("RuntimeVisibleTypeAnnotations")?,
            type_annotations,
        }
    } else {
        Attribute::RuntimeInvisibleTypeAnnotations {
            name_index: class
                .constant_pool
                .add_utf8("RuntimeInvisibleTypeAnnotations")?,
            type_annotations,
        }
    })
}

#[test]
fn type_annotation_offsets_require_instruction_starts() -> Result<()> {
    for visible in [false, true] {
        for valid_offset in [4, 9] {
            for offset in [valid_offset, 1, 2, 5, 6, 10, 11, 14, 15, u16::MAX] {
                let target_type = if valid_offset == 4 {
                    TargetType::Offset {
                        target_type: 0x44,
                        offset,
                    }
                } else {
                    TargetType::TypeArgument {
                        target_type: 0x47,
                        offset,
                        type_argument_index: 0,
                    }
                };
                let mut class = class()?;
                let metadata = annotation(&mut class, target_type, visible)?;
                // Valid annotations target new at byte 4 and checkcast at byte 9.
                let code = vec![
                    Instruction::Sipush(1),
                    Instruction::Pop,
                    Instruction::New(class.super_class),
                    Instruction::Pop,
                    Instruction::Aconst_null,
                    Instruction::Checkcast(class.super_class),
                    Instruction::Pop,
                    Instruction::Return,
                ];
                check_code_metadata(class, metadata, code, offset == valid_offset)?;
            }
        }
    }
    Ok(())
}

#[test]
fn local_scopes_require_boundaries_and_allow_code_length_as_the_end() -> Result<()> {
    for (start_pc, length, valid) in [
        (0, 3, true),
        (3, 2, true),
        (4, 0, true),
        (1, 2, false),
        (0, 1, false),
        (0, 2, false),
        (3, 3, false),
        (5, 0, false),
        (6, 0, false),
        (3, u16::MAX, false),
    ] {
        let mut class = class()?;
        let name_index = class.constant_pool.add_utf8("local")?;
        let descriptor_index = class.constant_pool.add_utf8("I")?;
        let signature_index = class.constant_pool.add_utf8("Ljava/lang/Object;")?;
        let variables = Attribute::LocalVariableTable {
            name_index: class.constant_pool.add_utf8("LocalVariableTable")?,
            variables: vec![LocalVariableTable {
                start_pc,
                length,
                name_index,
                descriptor_index,
                index: 0,
            }],
        };
        let types = Attribute::LocalVariableTypeTable {
            name_index: class.constant_pool.add_utf8("LocalVariableTypeTable")?,
            variable_types: vec![LocalVariableTypeTable {
                start_pc,
                length,
                name_index,
                signature_index,
                index: 0,
            }],
        };
        check_metadata(class.clone(), variables, valid)?;
        check_metadata(class.clone(), types, valid)?;
        for visible in [false, true] {
            for target_type in [0x40, 0x41] {
                let metadata = annotation(
                    &mut class,
                    TargetType::LocalVar {
                        target_type,
                        local_variable_targets: vec![LocalVariableTarget {
                            start_pc,
                            length,
                            index: 0,
                        }],
                    },
                    visible,
                )?;
                check_metadata(class.clone(), metadata, valid)?;
            }
        }
    }
    Ok(())
}

#[test]
fn stack_map_deltas_require_instruction_starts() -> Result<()> {
    for offset in [1, 2, 5, 6, u16::MAX] {
        let mut class = class()?;
        let metadata = Attribute::StackMapTable {
            name_index: class.constant_pool.add_utf8("StackMapTable")?,
            frames: vec![StackFrame::SameFrameExtended {
                frame_type: 251,
                offset_delta: offset,
            }],
        };
        check_metadata(class, metadata, false)?;
    }
    let mut class = class()?;
    let metadata = Attribute::StackMapTable {
        name_index: class.constant_pool.add_utf8("StackMapTable")?,
        frames: vec![
            StackFrame::SameFrame { frame_type: 0 },
            StackFrame::SameFrame { frame_type: 3 }, // 0 + 3 + 1 = 4
        ],
    };
    check_metadata(class, metadata, true)
}

#[test]
fn stack_map_uninitialized_offsets_require_instruction_starts() -> Result<()> {
    for offset in [1, 2, 5, 6, u16::MAX] {
        let mut class = class()?;
        let metadata = Attribute::StackMapTable {
            name_index: class.constant_pool.add_utf8("StackMapTable")?,
            frames: vec![StackFrame::FullFrame {
                frame_type: 255,
                offset_delta: 0,
                locals: vec![VerificationType::Uninitialized { offset }],
                stack: vec![],
            }],
        };
        check_metadata(class, metadata, false)?;
    }
    Ok(())
}
