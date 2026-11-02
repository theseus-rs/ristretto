use crate::JAVA_7;
use crate::attributes::Attribute;
use crate::attributes::ExceptionTableEntry;
use crate::attributes::Instruction;
use crate::class_file::ClassFile;
use crate::constant::Constant;
use crate::method::Method;
use crate::method_access_flags::MethodAccessFlags;
use crate::verifiers::error::Result;
use crate::verifiers::error::VerifyError::{InvalidStackFrameOffset, VerificationError};
use crate::{CLASS_INITIALIZER_NAME, FieldType};
use ahash::AHashSet;

/// Verify the `Code` attribute.
///
/// Branches and metadata are validated against JVM bytecode instruction boundaries.
///
/// # Errors
/// Returns `VerificationError` if the code attribute is invalid.
#[expect(clippy::too_many_lines)]
pub(crate) fn verify(
    class_file: &ClassFile<'_>,
    method: &Method,
    _max_stack: u16,
    max_locals: u16,
    code: &[Instruction],
    exception_table: &[ExceptionTableEntry],
    attributes: &[Attribute],
) -> Result<u16> {
    // Verify max_locals
    let descriptor_index = method.descriptor_index;
    if let Some(Constant::Utf8(descriptor)) = class_file.constant_pool.get(descriptor_index)
        && let Ok((parameters, _)) = FieldType::parse_method_descriptor(descriptor)
    {
        let mut required_locals = 0;
        if !method.access_flags.contains(MethodAccessFlags::STATIC)
            && !class_file
                .constant_pool
                .try_get_utf8(method.name_index)
                .is_ok_and(|name| name == CLASS_INITIALIZER_NAME)
        {
            required_locals += 1;
        }
        for param in parameters {
            required_locals += match param {
                FieldType::Base(
                    crate::base_type::BaseType::Double | crate::base_type::BaseType::Long,
                ) => 2,
                _ => 1,
            };
        }
        if max_locals < required_locals {
            return Err(VerificationError {
                context: "Code Attribute".to_string(),
                message: format!(
                    "max_locals ({max_locals}) is less than required locals ({required_locals}) for method parameters"
                ),
            });
        }
    }

    let info = crate::verifiers::bytecode::control_flow::CodeInfo::from_code(code)?;
    let code_length = info.code_length();
    if code_length == 0 {
        return Err(VerificationError {
            context: "Code".to_string(),
            message: "Code must not be empty".to_string(),
        });
    }
    crate::verifiers::bytecode::control_flow::validate_exception_table(exception_table, &info)?;
    let mut jump_target_indices = AHashSet::default();
    for (position, instruction) in Instruction::with_offsets(code) {
        let position = u16::try_from(position)?;
        let mut add = |relative| -> Result<()> {
            jump_target_indices.insert(info.branch_target(position, relative)?);
            Ok(())
        };
        match instruction {
            Instruction::Ifeq(target)
            | Instruction::Ifne(target)
            | Instruction::Iflt(target)
            | Instruction::Ifge(target)
            | Instruction::Ifgt(target)
            | Instruction::Ifle(target)
            | Instruction::If_icmpeq(target)
            | Instruction::If_icmpne(target)
            | Instruction::If_icmplt(target)
            | Instruction::If_icmpge(target)
            | Instruction::If_icmpgt(target)
            | Instruction::If_icmple(target)
            | Instruction::If_acmpeq(target)
            | Instruction::If_acmpne(target)
            | Instruction::Goto(target)
            | Instruction::Jsr(target)
            | Instruction::Ifnull(target)
            | Instruction::Ifnonnull(target) => add(i32::from(*target))?,
            Instruction::Goto_w(target) | Instruction::Jsr_w(target) => add(*target)?,
            Instruction::Tableswitch(table) => {
                add(table.default)?;
                for &relative in &table.offsets {
                    add(relative)?;
                }
            }
            Instruction::Lookupswitch(table) => {
                add(table.default)?;
                for &relative in table.pairs.values() {
                    add(relative)?;
                }
            }
            _ => {}
        }
    }
    for entry in exception_table {
        jump_target_indices.insert(entry.handler_pc);
    }

    // Stack map deltas count bytes from the preceding frame plus one.
    let mut frame_instruction_indices = AHashSet::from_iter([0]);
    let mut has_stack_map = false;
    for attribute in attributes {
        if let Attribute::StackMapTable { frames, .. } = attribute {
            has_stack_map = true;
            let mut current_frame_index = -1i32;
            for (i, frame) in frames.iter().enumerate() {
                frame.validate()?;
                let offset_delta = i32::from(frame.offset_delta());
                let frame_index = if i == 0 {
                    offset_delta
                } else {
                    current_frame_index + offset_delta + 1
                };

                if frame_index < 0
                    || frame_index >= i32::from(code_length)
                    || !info.is_valid_offset(u16::try_from(frame_index)?)
                {
                    return Err(InvalidStackFrameOffset(u16::try_from(frame_index)?));
                }

                let frame_index_u16 = u16::try_from(frame_index)?;
                frame_instruction_indices.insert(frame_index_u16);
                current_frame_index = frame_index;
            }
        }
    }

    // Verify that all jump targets have a corresponding StackFrame for Java 7 and later.
    if class_file.version.major() >= JAVA_7.major() {
        if jump_target_indices.iter().any(|&target| target != 0) && !has_stack_map {
            return Err(VerificationError {
                context: "Code Attribute".to_string(),
                message: format!(
                    "StackMapTable missing for class version >= {}.{}",
                    JAVA_7.major(),
                    JAVA_7.minor()
                ),
            });
        }

        for target_index in &jump_target_indices {
            if !frame_instruction_indices.contains(target_index) {
                return Err(VerificationError {
                    context: "Code Attribute".to_string(),
                    message: format!(
                        "Jump target at byte offset {target_index} does not have a corresponding StackMapFrame"
                    ),
                });
            }
        }
    }

    Ok(code_length)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::attributes::StackFrame;
    use crate::verifiers::VerifyError::InvalidInstructionOffset;
    use crate::version::Version;

    fn get_dummy_method() -> Method {
        Method {
            access_flags: MethodAccessFlags::PUBLIC,
            name_index: 0,
            descriptor_index: 0,
            attributes: vec![],
        }
    }

    #[test]
    fn test_valid_code_with_stack_map() {
        let class_file = ClassFile {
            version: Version::Java7 { minor: 0 },
            ..Default::default()
        };
        let method = get_dummy_method();
        // Branch operands are relative byte offsets; stack maps locate bytecode boundaries.
        let code = vec![
            Instruction::Nop,     // Index 0
            Instruction::Goto(4), // Index 1, jumps to instruction 3
            Instruction::Return,  // Index 2
            Instruction::Return,  // Index 3; target
        ];

        // StackMapTable frame at byte offset 5
        let stack_map_table = Attribute::StackMapTable {
            name_index: 0,
            frames: vec![
                // The first frame is at byte offset 5.
                StackFrame::SameFrame { frame_type: 5 }, // First frame: byte offset 5
            ],
        };
        let attributes = vec![stack_map_table];

        let result = verify(&class_file, &method, 0, 10, &code, &[], &attributes);
        assert!(result.is_ok());
    }

    #[test]
    fn test_valid_code_with_multiple_stack_map_frames() {
        let class_file = ClassFile {
            version: Version::Java7 { minor: 0 },
            ..Default::default()
        };
        let method = get_dummy_method();
        let code = vec![
            Instruction::Nop,
            Instruction::Goto(4),
            Instruction::Return,
            Instruction::Return,
        ];
        let stack_map_table = Attribute::StackMapTable {
            name_index: 0,
            frames: vec![
                StackFrame::SameFrame { frame_type: 1 },
                StackFrame::SameFrame { frame_type: 3 },
            ],
        };

        assert!(verify(&class_file, &method, 0, 10, &code, &[], &[stack_map_table]).is_ok());
    }

    #[test]
    fn test_invalid_jump_target() {
        let class_file = ClassFile::default();
        let method = get_dummy_method();
        let code = vec![
            Instruction::Goto(10), // Jump to instruction index 10 (invalid; only 1 instruction)
        ];

        assert_eq!(
            Err(InvalidInstructionOffset(10)),
            verify(&class_file, &method, 0, 10, &code, &[], &[])
        );
    }

    #[test]
    fn test_missing_stack_map_frame() {
        let class_file = ClassFile {
            version: Version::Java7 { minor: 0 },
            ..Default::default()
        };
        let method = get_dummy_method();
        let code = vec![
            Instruction::Nop,     // Index 0
            Instruction::Goto(4), // Index 1, jumps to instruction 3
            Instruction::Return,  // Index 2
            Instruction::Return,  // Index 3; target
        ];
        // Missing StackMapTable

        let message = verify(&class_file, &method, 0, 10, &code, &[], &[])
            .unwrap_err()
            .to_string();
        assert!(message.contains("StackMapTable missing"));
    }

    #[test]
    fn test_missing_frame_for_target() {
        let class_file = ClassFile {
            version: Version::Java7 { minor: 0 },
            ..Default::default()
        };
        let method = get_dummy_method();
        // Jump to instruction index 3
        let code = vec![
            Instruction::Nop,     // Index 0
            Instruction::Goto(4), // Index 1, jumps to instruction 3
            Instruction::Return,  // Index 2
            Instruction::Return,  // Index 3; target
        ];

        // StackMapTable has frame at instruction index 2, but we jump to instruction 3
        let stack_map_table = Attribute::StackMapTable {
            name_index: 0,
            frames: vec![
                StackFrame::SameFrame { frame_type: 4 }, // Frame at instruction index 2
            ],
        };
        let attributes = vec![stack_map_table];

        let message = verify(&class_file, &method, 0, 10, &code, &[], &attributes)
            .unwrap_err()
            .to_string();
        assert!(
            message.contains(
                "Jump target at byte offset 5 does not have a corresponding StackMapFrame"
            )
        );
    }

    #[test]
    fn test_max_locals_error() {
        let mut constant_pool = crate::ConstantPool::default();
        let descriptor_index = constant_pool.add_utf8("(II)V").unwrap();
        let class_file = ClassFile {
            version: Version::Java7 { minor: 0 },
            constant_pool,
            ..Default::default()
        };
        let method = Method {
            access_flags: MethodAccessFlags::PUBLIC,
            name_index: 0,
            descriptor_index,
            attributes: vec![],
        };
        let code = vec![Instruction::Return];
        // Requires 3 locals (this + int + int)
        // max_locals = 2 -> Error

        let message = verify(&class_file, &method, 0, 2, &code, &[], &[])
            .unwrap_err()
            .to_string();
        assert!(message.contains("max_locals (2) is less than required locals (3)"));
    }

    #[test]
    fn test_max_locals_accepts_wide_parameters() {
        let mut constant_pool = crate::ConstantPool::default();
        let descriptor_index = constant_pool.add_utf8("(JD)V").unwrap();
        let class_file = ClassFile {
            version: Version::Java7 { minor: 0 },
            constant_pool,
            ..Default::default()
        };
        let method = Method {
            access_flags: MethodAccessFlags::PUBLIC,
            name_index: 0,
            descriptor_index,
            attributes: vec![],
        };
        let code = vec![Instruction::Return];

        assert!(verify(&class_file, &method, 0, 5, &code, &[], &[]).is_ok());
    }

    #[test]
    fn test_conditional_jump_variants_are_collected() {
        let class_file = ClassFile {
            version: Version::Java7 { minor: 0 },
            ..Default::default()
        };
        let method = get_dummy_method();
        let code = vec![
            Instruction::Ifeq(55),
            Instruction::Ifne(52),
            Instruction::Iflt(49),
            Instruction::Ifge(46),
            Instruction::Ifgt(43),
            Instruction::Ifle(40),
            Instruction::If_icmpeq(37),
            Instruction::If_icmpne(34),
            Instruction::If_icmplt(31),
            Instruction::If_icmpge(28),
            Instruction::If_icmpgt(25),
            Instruction::If_icmple(22),
            Instruction::If_acmpeq(19),
            Instruction::If_acmpne(16),
            Instruction::Goto(13),
            Instruction::Jsr(10),
            Instruction::Ifnull(7),
            Instruction::Ifnonnull(4),
            Instruction::Nop,
            Instruction::Return,
        ];
        let stack_map_table = Attribute::StackMapTable {
            name_index: 0,
            frames: vec![StackFrame::SameFrame { frame_type: 55 }],
        };

        assert!(verify(&class_file, &method, 0, 10, &code, &[], &[stack_map_table]).is_ok());
    }

    #[test]
    fn test_wide_jump_targets() {
        let class_file = ClassFile::default();
        let method = get_dummy_method();
        let valid_code = vec![
            Instruction::Goto_w(10),
            Instruction::Jsr_w(5),
            Instruction::Return,
        ];
        assert!(verify(&class_file, &method, 0, 10, &valid_code, &[], &[]).is_ok());

        let invalid_code = vec![Instruction::Jsr_w(10)];
        assert_eq!(
            Err(InvalidInstructionOffset(10)),
            verify(&class_file, &method, 0, 10, &invalid_code, &[], &[])
        );
    }

    #[test]
    fn test_invalid_exception_handler_and_stack_frame_offsets() {
        let class_file = ClassFile::default();
        let method = get_dummy_method();
        let code = vec![Instruction::Return];
        let exception_table = vec![ExceptionTableEntry {
            range_pc: 0..1,
            handler_pc: 10,
            catch_type: 0,
        }];
        assert!(verify(&class_file, &method, 0, 10, &code, &exception_table, &[]).is_err());

        let stack_map_table = Attribute::StackMapTable {
            name_index: 0,
            frames: vec![StackFrame::SameFrame { frame_type: 10 }],
        };
        assert_eq!(
            Err(InvalidStackFrameOffset(10)),
            verify(&class_file, &method, 0, 10, &code, &[], &[stack_map_table])
        );
    }

    #[test]
    fn test_valid_exception_handler_is_recorded_as_jump_target() {
        let class_file = ClassFile::default();
        let method = get_dummy_method();
        let code = vec![Instruction::Return];
        let exception_table = vec![ExceptionTableEntry {
            range_pc: 0..1,
            handler_pc: 0,
            catch_type: 0,
        }];

        assert!(verify(&class_file, &method, 0, 10, &code, &exception_table, &[]).is_ok());
    }

    #[test]
    fn test_tableswitch_valid() {
        use crate::attributes::TableSwitch;
        let class_file = ClassFile::default();
        let method = get_dummy_method();
        // Tableswitch at index 0, with targets as relative offsets
        // low=0, high=2 means 3 offsets in the table
        let code = vec![
            Instruction::Tableswitch(Box::new(TableSwitch {
                default: 31, // Relative: 0 + 4 = 4
                low: 0,
                high: 2,
                offsets: vec![28, 29, 30], // Relative: 0+1=1, 0+2=2, 0+3=3
            })), // Index 0
            Instruction::Nop,    // Index 1; target
            Instruction::Nop,    // Index 2; target
            Instruction::Nop,    // Index 3; target
            Instruction::Return, // Index 4; default target
        ];

        let result = verify(&class_file, &method, 0, 10, &code, &[], &[]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_tableswitch_invalid_default() {
        use crate::attributes::TableSwitch;
        let class_file = ClassFile::default();
        let method = get_dummy_method();
        let code = vec![
            Instruction::Tableswitch(Box::new(TableSwitch {
                default: 100, // Invalid: 0 + 100 = 100 (out of bounds)
                low: 0,
                high: 0,
                offsets: vec![20],
            })),
            Instruction::Return,
        ];

        let result = verify(&class_file, &method, 0, 10, &code, &[], &[]);
        assert!(matches!(result, Err(InvalidInstructionOffset(100))));
    }

    #[test]
    fn test_tableswitch_invalid_offset() {
        use crate::attributes::TableSwitch;
        let class_file = ClassFile::default();
        let method = get_dummy_method();
        let code = vec![
            Instruction::Tableswitch(Box::new(TableSwitch {
                default: 20,
                low: 0,
                high: 0,
                offsets: vec![50], // Invalid: 0 + 50 = 50 (out of bounds)
            })),
            Instruction::Return,
        ];

        let result = verify(&class_file, &method, 0, 10, &code, &[], &[]);
        assert!(matches!(result, Err(InvalidInstructionOffset(50))));
    }

    #[test]
    fn test_lookupswitch_valid() {
        use crate::attributes::LookupSwitch;
        use indexmap::IndexMap;
        let class_file = ClassFile::default();
        let method = get_dummy_method();

        let mut pairs = IndexMap::new();
        pairs.insert(1, 28); // match 1 -> byte offset 28
        pairs.insert(2, 29); // match 2 -> byte offset 29

        let code = vec![
            Instruction::Lookupswitch(Box::new(LookupSwitch {
                default: 30, // Default target at byte offset 30
                pairs,
            })), // Index 0
            Instruction::Nop,    // Index 1; target for match 1
            Instruction::Nop,    // Index 2; target for match 2
            Instruction::Return, // Index 3; default target
        ];

        let result = verify(&class_file, &method, 0, 10, &code, &[], &[]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_lookupswitch_invalid_default() {
        use crate::attributes::LookupSwitch;
        use indexmap::IndexMap;
        let class_file = ClassFile::default();
        let method = get_dummy_method();

        let code = vec![
            Instruction::Lookupswitch(Box::new(LookupSwitch {
                default: 100, // Invalid: 0 + 100 = 100 (out of bounds)
                pairs: IndexMap::new(),
            })),
            Instruction::Return,
        ];

        let result = verify(&class_file, &method, 0, 10, &code, &[], &[]);
        assert!(matches!(result, Err(InvalidInstructionOffset(100))));
    }

    #[test]
    fn test_lookupswitch_invalid_pair_offset() {
        use crate::attributes::LookupSwitch;
        use indexmap::IndexMap;
        let class_file = ClassFile::default();
        let method = get_dummy_method();

        let mut pairs = IndexMap::new();
        pairs.insert(1, 50); // Invalid: 0 + 50 = 50 (out of bounds)

        let code = vec![
            Instruction::Lookupswitch(Box::new(LookupSwitch { default: 20, pairs })),
            Instruction::Return,
        ];

        let result = verify(&class_file, &method, 0, 10, &code, &[], &[]);
        assert!(matches!(result, Err(InvalidInstructionOffset(50))));
    }
}
