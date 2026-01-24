use crate::FieldType;
use crate::JAVA_6;
use crate::attributes::Attribute;
use crate::attributes::ExceptionTableEntry;
use crate::attributes::Instruction;
use crate::class_file::ClassFile;
use crate::constant::Constant;
use crate::method::Method;
use crate::method_access_flags::MethodAccessFlags;
use crate::verifiers::error::Result;
use crate::verifiers::error::VerifyError::{
    InvalidInstructionOffset, InvalidStackFrameOffset, VerificationError,
};
use ahash::AHashSet;
use std::io::Cursor;

/// Verify the `Code` attribute.
///
/// Note: After parsing, both branch instruction targets and `StackMapTable` frame offsets
/// are stored as **instruction indices**. This function verifies them as such.
///
/// # Errors
/// Returns `VerificationError` if the code attribute is invalid.
#[expect(clippy::too_many_lines)]
pub(crate) fn verify(
    class_file: &ClassFile,
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
        if !method.access_flags.contains(MethodAccessFlags::STATIC) {
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

    // Calculate code byte length (needed for return value)
    // We need to serialize to compute the actual byte length
    let mut cursor = Cursor::new(Vec::new());
    for instruction in code {
        instruction.to_bytes(&mut cursor)?;
    }
    let code_length = u16::try_from(cursor.position())?;

    let num_instructions = code.len();
    let num_inst_u16 = u16::try_from(num_instructions)?;

    // 1. Identify jump targets (these are instruction indices after parsing)
    let mut jump_target_indices = AHashSet::default();
    for instruction in code {
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
            | Instruction::Ifnonnull(target) => {
                // target is an instruction index
                let target_index = *target;
                if target_index >= num_inst_u16 {
                    return Err(InvalidInstructionOffset(u32::from(target_index)));
                }
                jump_target_indices.insert(target_index);
            }
            Instruction::Goto_w(target) | Instruction::Jsr_w(target) => {
                // target is an instruction index
                let target_index = u16::try_from(*target)?;
                if target_index >= num_inst_u16 {
                    return Err(InvalidInstructionOffset(u32::from(target_index)));
                }
                jump_target_indices.insert(target_index);
            }
            Instruction::Tableswitch(switch) => {
                // After parsing, switch offsets are relative instruction offsets from current instruction
                let current_index = code
                    .iter()
                    .position(|i| std::ptr::eq(i, instruction))
                    .unwrap_or(0);
                let current_index_i32 = i32::try_from(current_index)?;
                let num_instr_i32 = i32::try_from(num_instructions)?;

                // Validate default target
                let default_target = current_index_i32 + switch.default;
                if default_target < 0 || default_target >= num_instr_i32 {
                    return Err(InvalidInstructionOffset(u32::try_from(default_target)?));
                }

                jump_target_indices.insert(u16::try_from(default_target)?);

                // Validate all jump table offsets
                for offset in &switch.offsets {
                    let target = current_index_i32 + offset;
                    if target < 0 || target >= num_instr_i32 {
                        return Err(InvalidInstructionOffset(u32::try_from(target)?));
                    }
                    jump_target_indices.insert(u16::try_from(target)?);
                }
            }
            Instruction::Lookupswitch(switch) => {
                // After parsing, switch offsets are relative instruction offsets from current instruction
                let current_index = code
                    .iter()
                    .position(|i| std::ptr::eq(i, instruction))
                    .unwrap_or(0);
                let current_index_i32 = i32::try_from(current_index)?;
                let num_instr_i32 = i32::try_from(num_instructions)?;

                // Validate default target
                let default_target = current_index_i32 + switch.default;
                if default_target < 0 || default_target >= num_instr_i32 {
                    return Err(InvalidInstructionOffset(u32::try_from(default_target)?));
                }
                jump_target_indices.insert(u16::try_from(default_target)?);

                // Validate all match-offset pairs
                for (_, offset) in &switch.pairs {
                    let target = current_index_i32 + offset;
                    if target < 0 || target >= num_instr_i32 {
                        return Err(InvalidInstructionOffset(u32::try_from(target)?));
                    }
                    jump_target_indices.insert(u16::try_from(target)?);
                }
            }
            _ => {}
        }
    }

    // Exception table handlers - already instruction indices after parsing
    for entry in exception_table {
        if entry.handler_pc >= num_inst_u16 {
            return Err(InvalidInstructionOffset(u32::from(entry.handler_pc)));
        }
        jump_target_indices.insert(entry.handler_pc);
    }

    // 2. Verify StackMapTable (frame offsets are instruction indices after parsing)
    let mut frame_instruction_indices = AHashSet::default();
    let mut has_stack_map = false;
    for attribute in attributes {
        if let Attribute::StackMapTable { frames, .. } = attribute {
            has_stack_map = true;
            let mut current_frame_index = -1i32;
            for (i, frame) in frames.iter().enumerate() {
                // After parsing, offset_delta is in instruction indices, not byte offsets
                let offset_delta = i32::from(frame.offset_delta());
                let frame_index = if i == 0 {
                    offset_delta
                } else {
                    current_frame_index + offset_delta + 1
                };

                if frame_index < 0 || frame_index >= i32::from(num_inst_u16) {
                    return Err(InvalidStackFrameOffset(u16::try_from(frame_index)?));
                }

                let frame_index_u16 = u16::try_from(frame_index)?;
                frame_instruction_indices.insert(frame_index_u16);
                current_frame_index = frame_index;
            }
        }
    }

    // Verify that all jump targets have a corresponding StackFrame (if StackMapTable is present and version >= 50)
    if class_file.version >= JAVA_6 {
        if !jump_target_indices.is_empty() && !has_stack_map {
            return Err(VerificationError {
                context: "Code Attribute".to_string(),
                message: "StackMapTable missing for class version >= 50.0".to_string(),
            });
        }

        for target_index in &jump_target_indices {
            if !frame_instruction_indices.contains(target_index) {
                return Err(VerificationError {
                    context: "Code Attribute".to_string(),
                    message: format!(
                        "Jump target at instruction index {target_index} does not have a corresponding StackMapFrame"
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
        // After parsing, Goto targets AND StackMapTable frame offsets are instruction indices
        let code = vec![
            Instruction::Nop,     // Index 0
            Instruction::Goto(3), // Index 1, jumps to instruction 3
            Instruction::Return,  // Index 2
            Instruction::Return,  // Index 3 - target
        ];

        // StackMapTable frame at instruction index 3
        let stack_map_table = Attribute::StackMapTable {
            name_index: 0,
            frames: vec![
                // After parsing, frame_type is instruction index delta, not byte offset
                StackFrame::SameFrame { frame_type: 3 }, // First frame: instruction index 3
            ],
        };
        let attributes = vec![stack_map_table];

        let result = verify(&class_file, &method, 0, 10, &code, &[], &attributes);
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_jump_target() {
        let class_file = ClassFile::default();
        let method = get_dummy_method();
        let code = vec![
            Instruction::Goto(10), // Jump to instruction index 10 (invalid - only 1 instruction)
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
            Instruction::Goto(3), // Index 1, jumps to instruction 3
            Instruction::Return,  // Index 2
            Instruction::Return,  // Index 3 - target
        ];
        // Missing StackMapTable

        match verify(&class_file, &method, 0, 10, &code, &[], &[]) {
            Err(VerificationError { message, .. }) => {
                assert!(message.contains("StackMapTable missing"));
            }
            _ => panic!("Expected VerificationError"),
        }
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
            Instruction::Goto(3), // Index 1, jumps to instruction 3
            Instruction::Return,  // Index 2
            Instruction::Return,  // Index 3 - target
        ];

        // StackMapTable has frame at instruction index 2, but we jump to instruction 3
        let stack_map_table = Attribute::StackMapTable {
            name_index: 0,
            frames: vec![
                StackFrame::SameFrame { frame_type: 2 }, // Frame at instruction index 2
            ],
        };
        let attributes = vec![stack_map_table];

        match verify(&class_file, &method, 0, 10, &code, &[], &attributes) {
            Err(VerificationError { message, .. }) => {
                assert!(message.contains(
                    "Jump target at instruction index 3 does not have a corresponding StackMapFrame"
                ));
            }
            _ => panic!("Expected VerificationError"),
        }
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

        match verify(&class_file, &method, 0, 2, &code, &[], &[]) {
            Err(VerificationError { message, .. }) => {
                assert!(message.contains("max_locals (2) is less than required locals (3)"));
            }
            _ => panic!("Expected VerificationError"),
        }
    }

    #[test]
    fn test_tableswitch_valid() {
        use crate::attributes::TableSwitch;
        let class_file = ClassFile::default();
        let method = get_dummy_method();
        // Tableswitch at index 0, with targets as relative offsets
        // low=0, high=2 means 3 offsets in the table
        let code = vec![
            Instruction::Tableswitch(TableSwitch {
                default: 4, // Relative: 0 + 4 = 4
                low: 0,
                high: 2,
                offsets: vec![1, 2, 3], // Relative: 0+1=1, 0+2=2, 0+3=3
            }), // Index 0
            Instruction::Nop,    // Index 1 - target
            Instruction::Nop,    // Index 2 - target
            Instruction::Nop,    // Index 3 - target
            Instruction::Return, // Index 4 - default target
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
            Instruction::Tableswitch(TableSwitch {
                default: 100, // Invalid: 0 + 100 = 100 (out of bounds)
                low: 0,
                high: 0,
                offsets: vec![1],
            }),
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
            Instruction::Tableswitch(TableSwitch {
                default: 1,
                low: 0,
                high: 0,
                offsets: vec![50], // Invalid: 0 + 50 = 50 (out of bounds)
            }),
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
        pairs.insert(1, 1); // match 1 -> offset 1 (relative from instruction 0)
        pairs.insert(2, 2); // match 2 -> offset 2

        let code = vec![
            Instruction::Lookupswitch(LookupSwitch {
                default: 3, // Relative: 0 + 3 = 3
                pairs,
            }), // Index 0
            Instruction::Nop,    // Index 1 - target for match 1
            Instruction::Nop,    // Index 2 - target for match 2
            Instruction::Return, // Index 3 - default target
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
            Instruction::Lookupswitch(LookupSwitch {
                default: 100, // Invalid: 0 + 100 = 100 (out of bounds)
                pairs: IndexMap::new(),
            }),
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
            Instruction::Lookupswitch(LookupSwitch { default: 1, pairs }),
            Instruction::Return,
        ];

        let result = verify(&class_file, &method, 0, 10, &code, &[], &[]);
        assert!(matches!(result, Err(InvalidInstructionOffset(50))));
    }
}
