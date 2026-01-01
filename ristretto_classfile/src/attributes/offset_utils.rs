//! This module provides utility functions for converting between physical byte offsets and logical
//! instruction offsets in JVM bytecode.
//!
//! When bytecode is read from a .class file, jump instructions (like `goto`, `ifeq`) use byte
//! offsets relative to the start of the instruction. For easier manipulation and analysis, it's
//! often beneficial to convert these to logical offsets, i.e., the index of the target instruction
//! in a sequence of instructions. This module handles this two-way conversion.
//!
//! The `instructions_from_bytes` function reads bytecode and performs the conversionfrom byte
//! offsets to logical instruction indices.
//!
//! The `instructions_to_bytes` function takes a list of instructions with logical offsets and
//! converts them back to bytecode with byte offsets.

use crate::Error::InvalidInstructionOffset;
use crate::Result;
use crate::attributes::Instruction;
use std::collections::HashMap;
use std::io::Cursor;

/// Converts a byte stream representing JVM bytecode into a vector of `Instruction`s.
///
/// This function parses the raw bytes, identifies each instruction, and crucially, converts any
/// jump/branch offsets from physical byte offsets (relative to the start of the current
/// instruction) to logical instruction offsets (i.e., the index of the target instruction in the
/// resulting `Vec<Instruction>`).
///
/// It also returns a map (`HashMap<u16, u16>`) that translates original byte offsets to the new
/// logical instruction indices. This can be useful for debugging or cross-referencing with raw
/// bytecode.
///
/// # Errors
///
/// - returns `Error::InvalidInstructionOffset` if a jump instruction's target offset (after being
///   resolved to an absolute byte position) does not correspond to the start of a valid
///   instruction.
/// - returns `Error::Io` if there is an issue reading from the byte cursor.
/// - returns `Error::TryFromIntError` if a conversion from a numeric type fails.
pub(crate) fn instructions_from_bytes(
    bytes: &mut Cursor<Vec<u8>>,
) -> Result<(HashMap<u16, u16>, Vec<Instruction>)> {
    let mut instructions = Vec::new();
    let mut byte_to_instruction_map = HashMap::new();
    let mut instruction_to_byte_map = HashMap::new();
    while bytes.position() < bytes.get_ref().len() as u64 {
        let byte_position = u16::try_from(bytes.position())?;
        let instruction_position = u16::try_from(instructions.len())?;
        byte_to_instruction_map.insert(byte_position, instruction_position);
        instruction_to_byte_map.insert(instruction_position, byte_position);
        let instruction = Instruction::from_bytes(bytes)?;
        instructions.push(instruction);
    }

    for (index, instruction) in instructions.iter_mut().enumerate() {
        match instruction {
            Instruction::Ifeq(offset)
            | Instruction::Ifne(offset)
            | Instruction::Iflt(offset)
            | Instruction::Ifge(offset)
            | Instruction::Ifgt(offset)
            | Instruction::Ifle(offset)
            | Instruction::If_icmpeq(offset)
            | Instruction::If_icmpne(offset)
            | Instruction::If_icmplt(offset)
            | Instruction::If_icmpge(offset)
            | Instruction::If_icmpgt(offset)
            | Instruction::If_icmple(offset)
            | Instruction::If_acmpeq(offset)
            | Instruction::If_acmpne(offset)
            | Instruction::Goto(offset)
            | Instruction::Jsr(offset)
            | Instruction::Ifnull(offset)
            | Instruction::Ifnonnull(offset) => {
                *offset = *byte_to_instruction_map
                    .get(offset)
                    .ok_or(InvalidInstructionOffset(u32::from(*offset)))?;
            }
            Instruction::Goto_w(offset) | Instruction::Jsr_w(offset) => {
                // Note the map may need to be updated to use 32-bit offsets if/when the JVM spec
                // is updated to support 32-bit offsets for goto_w and jsr_w.
                // See: https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.goto_w
                let map_offset = u16::try_from(*offset)?;
                *offset = i32::from(
                    *byte_to_instruction_map
                        .get(&map_offset)
                        .ok_or(InvalidInstructionOffset(u32::from(map_offset)))?,
                );
            }
            Instruction::Tableswitch(table_switch) => {
                let position = instruction_to_byte_map
                    .get(&u16::try_from(index)?)
                    .expect("instruction byte");
                let position = u32::from(*position);
                let default_offset = position + u32::try_from(table_switch.default)?;
                let instruction_default = byte_to_instruction_map
                    .get(&u16::try_from(default_offset)?)
                    .ok_or(InvalidInstructionOffset(default_offset))?
                    - u16::try_from(index)?;
                table_switch.default = i32::from(instruction_default);

                for offset in &mut table_switch.offsets {
                    let byte_offset = position + u32::try_from(*offset)?;
                    let instruction_offset = byte_to_instruction_map
                        .get(&u16::try_from(byte_offset)?)
                        .ok_or(InvalidInstructionOffset(byte_offset))?
                        - u16::try_from(index)?;
                    *offset = i32::from(instruction_offset);
                }
            }
            Instruction::Lookupswitch(lookup_switch) => {
                let position = instruction_to_byte_map
                    .get(&u16::try_from(index)?)
                    .expect("instruction byte");
                let position = u32::from(*position);
                let default_offset =
                    u32::try_from(i64::from(position) + i64::from(lookup_switch.default))?;
                let instruction_default = i32::from(
                    *byte_to_instruction_map
                        .get(&u16::try_from(default_offset)?)
                        .ok_or(InvalidInstructionOffset(default_offset))?,
                ) - i32::try_from(index)?;
                lookup_switch.default = instruction_default;

                for (_match, offset) in &mut lookup_switch.pairs {
                    let byte_offset = u32::try_from(i64::from(position) + i64::from(*offset))?;

                    let instruction_offset = i32::from(
                        *byte_to_instruction_map
                            .get(&u16::try_from(byte_offset)?)
                            .ok_or(InvalidInstructionOffset(byte_offset))?,
                    ) - i32::try_from(index)?;

                    *offset = instruction_offset;
                }
            }
            _ => {}
        }
    }
    Ok((byte_to_instruction_map, instructions))
}

/// Converts a vector of `Instruction`s (with logical offsets) into a raw byte vector.
///
/// This function takes a sequence of `Instruction`s, where jump/branch offsets are represented as
/// logical instruction indices (relative to the start of the instruction list for simple jumps, or
/// relative to the current instruction for `tableswitch` and `lookupswitch`), and serializes them
/// into a `Vec<u8>` of JVM bytecode. During this process, all logical offsets are converted back to
/// physical byte offsets.
///
/// It also returns a map (`HashMap<u16, u16>`) that translates logical instruction indices to their
/// corresponding byte offsets in the generated bytecode.
///
/// # Errors
///
/// - returns `Error::InvalidInstructionOffset` if a logical instruction offset is out of bounds
///   (e.g., refers to an index beyond the length of the `instructions` slice).
/// - returns `Error::Io` if there is an issue writing to the internal byte buffer.
/// - returns `Error::TryFromIntError` if a conversion from a numeric type fails.
pub(crate) fn instructions_to_bytes(
    instructions: &[Instruction],
) -> Result<(HashMap<u16, u16>, Vec<u8>)> {
    let mut bytes = Cursor::new(Vec::new());
    let mut instruction_to_byte_map = HashMap::new();
    for (index, instruction) in instructions.iter().enumerate() {
        let byte_position = u16::try_from(bytes.position())?;
        let instruction_position = u16::try_from(index)?;
        instruction_to_byte_map.insert(instruction_position, byte_position);
        instruction.to_bytes(&mut bytes)?;
    }

    let mut bytes = Cursor::new(Vec::new());
    let mut instructions = instructions.to_owned();
    for (index, instruction) in instructions.iter_mut().enumerate() {
        match instruction {
            Instruction::Ifeq(offset)
            | Instruction::Ifne(offset)
            | Instruction::Iflt(offset)
            | Instruction::Ifge(offset)
            | Instruction::Ifgt(offset)
            | Instruction::Ifle(offset)
            | Instruction::If_icmpeq(offset)
            | Instruction::If_icmpne(offset)
            | Instruction::If_icmplt(offset)
            | Instruction::If_icmpge(offset)
            | Instruction::If_icmpgt(offset)
            | Instruction::If_icmple(offset)
            | Instruction::If_acmpeq(offset)
            | Instruction::If_acmpne(offset)
            | Instruction::Goto(offset)
            | Instruction::Jsr(offset)
            | Instruction::Ifnull(offset)
            | Instruction::Ifnonnull(offset) => {
                *offset = *instruction_to_byte_map
                    .get(offset)
                    .ok_or(InvalidInstructionOffset(u32::from(*offset)))?;
            }
            Instruction::Goto_w(offset) | Instruction::Jsr_w(offset) => {
                // Note the map may need to be updated to use 32-bit offsets if/when the JVM spec
                // is updated to support 32-bit offsets for goto_w and jsr_w.
                // See: https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.goto_w
                let map_offset = u16::try_from(*offset)?;
                *offset = i32::from(
                    *instruction_to_byte_map
                        .get(&map_offset)
                        .ok_or(InvalidInstructionOffset(u32::from(map_offset)))?,
                );
            }
            Instruction::Tableswitch(table_switch) => {
                let position = u32::try_from(index)?;
                let position_byte = i32::from(
                    *instruction_to_byte_map
                        .get(&u16::try_from(position)?)
                        .expect("instruction byte"),
                );
                let default_offset = position + u32::try_from(table_switch.default)?;
                let default_byte = *instruction_to_byte_map
                    .get(&u16::try_from(default_offset)?)
                    .ok_or(InvalidInstructionOffset(default_offset))?;
                table_switch.default = i32::from(default_byte) - position_byte;

                for offset in &mut table_switch.offsets {
                    let instruction_offset = position + u32::try_from(*offset)?;
                    let offset_byte = instruction_to_byte_map
                        .get(&u16::try_from(instruction_offset)?)
                        .ok_or(InvalidInstructionOffset(instruction_offset))?;
                    *offset = i32::from(*offset_byte) - position_byte;
                }
            }
            Instruction::Lookupswitch(lookup_switch) => {
                let position = u32::try_from(index)?;
                let position_byte = i32::from(
                    *instruction_to_byte_map
                        .get(&u16::try_from(position)?)
                        .expect("instruction byte"),
                );
                let default_offset = position + u32::try_from(lookup_switch.default)?;
                let default_byte = *instruction_to_byte_map
                    .get(&u16::try_from(default_offset)?)
                    .ok_or(InvalidInstructionOffset(default_offset))?;
                lookup_switch.default = i32::from(default_byte) - position_byte;

                for (_match, offset) in &mut lookup_switch.pairs {
                    let instruction_offset = u32::try_from(i64::from(position) + i64::from(*offset))?;
                    let offset_byte = instruction_to_byte_map
                        .get(&u16::try_from(instruction_offset)?)
                        .ok_or(InvalidInstructionOffset(instruction_offset))?;
                    *offset = i32::from(*offset_byte) - position_byte;
                }
            }
            _ => {}
        }

        instruction.to_bytes(&mut bytes)?;
    }
    Ok((instruction_to_byte_map, bytes.into_inner()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::attributes::instruction::{LookupSwitch, TableSwitch};
    use indexmap::IndexMap;

    #[test]
    fn test_to_bytes() -> Result<()> {
        let instructions = vec![
            Instruction::Iconst_0,
            Instruction::Istore_0,
            Instruction::Iload_0,
            Instruction::Iconst_1,
            Instruction::Iadd,
            Instruction::Ireturn,
        ];
        let (_instruction_to_byte_map, bytes) = instructions_to_bytes(&instructions)?;
        let mut cursor = Cursor::new(bytes);
        let (_byte_to_instruction_map, result) = instructions_from_bytes(&mut cursor)?;

        assert_eq!(instructions, result);

        Ok(())
    }

    #[test]
    fn test_to_bytes_invalid() {
        let instructions = vec![Instruction::Iflt(42)];
        let result = instructions_to_bytes(&instructions);
        assert!(matches!(result, Err(InvalidInstructionOffset(_))));
    }

    #[test]
    fn test_to_bytes_invalid_table_switch_default_offset() {
        let instructions = vec![Instruction::Tableswitch(TableSwitch {
            default: 42,
            low: 0,
            high: 0,
            offsets: vec![],
        })];
        let result = instructions_to_bytes(&instructions);
        assert!(matches!(result, Err(InvalidInstructionOffset(_))));
    }

    #[test]
    fn test_to_bytes_invalid_table_switch_offset() {
        let instructions = vec![
            Instruction::Nop,
            Instruction::Tableswitch(TableSwitch {
                default: 0,
                low: 0,
                high: 0,
                offsets: vec![42],
            }),
        ];
        let result = instructions_to_bytes(&instructions);
        assert!(matches!(result, Err(InvalidInstructionOffset(_))));
    }

    #[test]
    fn test_to_bytes_invalid_lookup_switch_default_offset() {
        let instructions = vec![Instruction::Lookupswitch(LookupSwitch {
            default: 42,
            pairs: IndexMap::new(),
        })];
        let result = instructions_to_bytes(&instructions);
        assert!(matches!(result, Err(InvalidInstructionOffset(_))));
    }

    #[test]
    fn test_to_bytes_invalid_lookup_switch_pairs_offset() {
        let instructions = vec![
            Instruction::Nop,
            Instruction::Lookupswitch(LookupSwitch {
                default: 0,
                pairs: IndexMap::from([(0, 42)]),
            }),
        ];
        let result = instructions_to_bytes(&instructions);
        assert!(matches!(result, Err(InvalidInstructionOffset(_))));
    }

    #[test]
    fn test_from_bytes() -> Result<()> {
        let instructions = vec![
            Instruction::Iconst_0,
            Instruction::Istore_0,
            Instruction::Iload_0,
            Instruction::Iconst_1,
            Instruction::Iadd,
            Instruction::Ireturn,
        ];
        let bytes = instructions
            .iter()
            .map(Instruction::code)
            .collect::<Vec<u8>>();
        let mut cursor = Cursor::new(bytes);
        let (_byte_to_instruction_map, result) = instructions_from_bytes(&mut cursor)?;

        assert_eq!(instructions, result);

        Ok(())
    }

    #[test]
    fn test_from_bytes_invalid() {
        let bytes = vec![155, 0, 42];
        let mut cursor = Cursor::new(bytes);
        let result = instructions_from_bytes(&mut cursor);
        assert!(matches!(result, Err(InvalidInstructionOffset(_))));
    }

    #[test]
    fn test_from_bytes_invalid_table_switch_default_offset() {
        let bytes = vec![
            170, 0, 0, 0, 0, 0, 0, 42, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4,
        ];
        let mut cursor = Cursor::new(bytes);
        let result = instructions_from_bytes(&mut cursor);
        assert!(matches!(result, Err(InvalidInstructionOffset(_))));
    }

    #[test]
    fn test_from_bytes_invalid_table_switch_offset() {
        let bytes = vec![
            0, 170, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4,
        ];
        let mut cursor = Cursor::new(bytes);
        let result = instructions_from_bytes(&mut cursor);
        assert!(matches!(result, Err(InvalidInstructionOffset(_))));
    }

    #[test]
    fn test_from_bytes_invalid_lookup_switch_default_offset() {
        let bytes = vec![
            171, 0, 0, 0, 0, 0, 0, 42, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 2,
        ];
        let mut cursor = Cursor::new(bytes);
        let result = instructions_from_bytes(&mut cursor);
        assert!(matches!(result, Err(InvalidInstructionOffset(_))));
    }

    #[test]
    fn test_from_bytes_invalid_lookup_switch_offset() {
        let bytes = vec![0, 171, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 2];
        let mut cursor = Cursor::new(bytes);
        let result = instructions_from_bytes(&mut cursor);
        assert!(matches!(result, Err(InvalidInstructionOffset(_))));
    }

    fn test_instruction(instruction: Instruction) -> Result<()> {
        let expected_bytes = [Instruction::Nop.code(), instruction.code(), 255, 255];
        let instructions = [Instruction::Nop, instruction];

        let (_instruction_to_byte_map, bytes) = instructions_to_bytes(instructions.as_slice())?;
        assert_eq!(expected_bytes, bytes.as_slice());

        let (_byte_to_instruction_map, instructions_from_bytes) =
            instructions_from_bytes(&mut Cursor::new(bytes))?;
        assert_eq!(instructions, instructions_from_bytes.as_slice());
        Ok(())
    }

    #[test]
    fn test_ifeq() -> Result<()> {
        test_instruction(Instruction::Ifeq(0))
    }

    #[test]
    fn test_ifne() -> Result<()> {
        test_instruction(Instruction::Ifne(0))
    }

    #[test]
    fn test_iflt() -> Result<()> {
        test_instruction(Instruction::Iflt(0))
    }

    #[test]
    fn test_ifge() -> Result<()> {
        test_instruction(Instruction::Ifge(0))
    }

    #[test]
    fn test_ifgt() -> Result<()> {
        test_instruction(Instruction::Ifgt(0))
    }

    #[test]
    fn test_ifle() -> Result<()> {
        test_instruction(Instruction::Ifle(0))
    }

    #[test]
    fn test_if_icmpeq() -> Result<()> {
        test_instruction(Instruction::If_icmpeq(0))
    }

    #[test]
    fn test_if_icmpne() -> Result<()> {
        test_instruction(Instruction::If_icmpne(0))
    }

    #[test]
    fn test_if_icmplt() -> Result<()> {
        test_instruction(Instruction::If_icmplt(0))
    }

    #[test]
    fn test_if_icmpge() -> Result<()> {
        test_instruction(Instruction::If_icmpge(0))
    }

    #[test]
    fn test_if_icmpgt() -> Result<()> {
        test_instruction(Instruction::If_icmpgt(0))
    }

    #[test]
    fn test_if_icmple() -> Result<()> {
        test_instruction(Instruction::If_icmple(0))
    }

    #[test]
    fn test_if_acmpeq() -> Result<()> {
        test_instruction(Instruction::If_acmpeq(0))
    }

    #[test]
    fn test_if_acmpne() -> Result<()> {
        test_instruction(Instruction::If_acmpne(0))
    }

    #[test]
    fn test_goto() -> Result<()> {
        test_instruction(Instruction::Goto(0))
    }

    #[test]
    fn test_jsr() -> Result<()> {
        test_instruction(Instruction::Jsr(0))
    }

    #[test]
    fn test_ifnull() -> Result<()> {
        test_instruction(Instruction::Ifnull(0))
    }

    #[test]
    fn test_ifnonnull() -> Result<()> {
        test_instruction(Instruction::Ifnonnull(0))
    }

    #[test]
    fn test_goto_w() -> Result<()> {
        let instruction = Instruction::Goto_w(1);
        let expected_bytes = [instruction.code(), 0, 0, 0, 5, Instruction::Nop.code()];
        let instructions = [instruction, Instruction::Nop];

        let (_instruction_to_byte_map, bytes) = instructions_to_bytes(instructions.as_slice())?;
        assert_eq!(expected_bytes, bytes.as_slice());

        let (_byte_to_instruction_map, instructions_from_bytes) =
            instructions_from_bytes(&mut Cursor::new(bytes))?;
        assert_eq!(instructions, instructions_from_bytes.as_slice());
        Ok(())
    }

    #[test]
    fn test_jsr_w() -> Result<()> {
        let instruction = Instruction::Jsr_w(1);
        let expected_bytes = [instruction.code(), 0, 0, 0, 5, Instruction::Nop.code()];
        let instructions = [instruction, Instruction::Nop];

        let (_instruction_to_byte_map, bytes) = instructions_to_bytes(instructions.as_slice())?;
        assert_eq!(expected_bytes, bytes.as_slice());

        let (_byte_to_instruction_map, instructions_from_bytes) =
            instructions_from_bytes(&mut Cursor::new(bytes))?;
        assert_eq!(instructions, instructions_from_bytes.as_slice());
        Ok(())
    }

    #[test]
    fn test_tableswitch() -> Result<()> {
        let instruction = Instruction::Tableswitch(TableSwitch {
            default: 3,
            low: 3,
            high: 4,
            offsets: vec![1, 2],
        });
        let expected_bytes = [
            instruction.code(),
            0,
            0,
            0,
            0,
            0,
            0,
            26,
            0,
            0,
            0,
            3,
            0,
            0,
            0,
            4,
            0,
            0,
            0,
            24,
            0,
            0,
            0,
            25,
            Instruction::Nop.code(),
            Instruction::Nop.code(),
            Instruction::Nop.code(),
        ];
        let instructions = [
            instruction,
            Instruction::Nop,
            Instruction::Nop,
            Instruction::Nop,
        ];

        let (_instruction_to_byte_map, bytes) = instructions_to_bytes(instructions.as_slice())?;
        assert_eq!(expected_bytes, bytes.as_slice());

        let (_byte_to_instruction_map, instructions_from_bytes) =
            instructions_from_bytes(&mut Cursor::new(bytes))?;
        assert_eq!(instructions, instructions_from_bytes.as_slice());
        Ok(())
    }

    #[test]
    fn test_lookupswitch() -> Result<()> {
        let instruction = Instruction::Lookupswitch(LookupSwitch {
            default: 3,
            pairs: IndexMap::from([(1, 2)]),
        });
        let expected_bytes = [
            instruction.code(),
            0,
            0,
            0,
            0,
            0,
            0,
            22,
            0,
            0,
            0,
            1,
            0,
            0,
            0,
            1,
            0,
            0,
            0,
            21,
            Instruction::Nop.code(),
            Instruction::Nop.code(),
            Instruction::Nop.code(),
        ];
        let instructions = [
            instruction,
            Instruction::Nop,
            Instruction::Nop,
            Instruction::Nop,
        ];

        let (_instruction_to_byte_map, bytes) = instructions_to_bytes(instructions.as_slice())?;
        assert_eq!(expected_bytes, bytes.as_slice());

        let (_byte_to_instruction_map, instructions_from_bytes) =
            instructions_from_bytes(&mut Cursor::new(bytes))?;
        assert_eq!(instructions, instructions_from_bytes.as_slice());
        Ok(())
    }
}
