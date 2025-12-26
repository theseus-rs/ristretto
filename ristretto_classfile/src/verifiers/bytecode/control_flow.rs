//! # Control Flow Analysis for Bytecode Verification
//!
//! This module provides control flow analysis utilities including:
//! - Basic block identification
//! - Control flow graph construction
//! - `StackMapTable` validation
//! - Instruction boundary validation
//!
//! # References
//!
//! - [JVMS §4.9 - Constraints on Java Virtual Machine Code](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.9)
//! - [JVMS §4.10.1 - Verification by Type Checking](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.10.1)

use std::collections::HashSet;

use crate::attributes::{ExceptionTableEntry, Instruction, StackFrame};
use crate::verifiers::error::{Result, VerifyError};

/// Information about the bytecode for control flow analysis.
///
/// This struct holds precomputed information about instruction boundaries
/// and provides efficient lookups for validation.
#[derive(Debug)]
pub struct CodeInfo {
    /// Mapping from instruction index to bytecode offset.
    instruction_offsets: Vec<u16>,

    /// Set of valid instruction start offsets for O(1) lookup.
    valid_offsets: HashSet<u16>,

    /// Total length of the code array in bytes.
    code_length: u16,

    /// Number of instructions.
    instruction_count: usize,
}

impl CodeInfo {
    /// Creates a new `CodeInfo` from instruction offsets.
    ///
    /// # Arguments
    ///
    /// * `instruction_offsets` - Vector of bytecode offsets for each instruction
    /// * `code_length` - Total length of the code array
    #[must_use]
    pub fn new(instruction_offsets: Vec<u16>, code_length: u16) -> Self {
        let valid_offsets: HashSet<u16> = instruction_offsets.iter().copied().collect();
        let instruction_count = instruction_offsets.len();
        Self {
            instruction_offsets,
            valid_offsets,
            code_length,
            instruction_count,
        }
    }

    /// Returns the bytecode offset for an instruction index.
    ///
    /// # Arguments
    ///
    /// * `index` - The instruction index
    ///
    /// # Returns
    ///
    /// The bytecode offset, or `None` if out of bounds.
    #[inline]
    #[must_use]
    pub fn offset_at(&self, index: usize) -> Option<u16> {
        self.instruction_offsets.get(index).copied()
    }

    /// Returns the instruction index for a bytecode offset.
    ///
    /// # Arguments
    ///
    /// * `offset` - The bytecode offset
    ///
    /// # Returns
    ///
    /// The instruction index, or `None` if not a valid instruction boundary.
    #[must_use]
    pub fn index_at(&self, offset: u16) -> Option<usize> {
        self.instruction_offsets.iter().position(|&o| o == offset)
    }

    /// Checks if an offset is a valid instruction boundary.
    ///
    /// # Arguments
    ///
    /// * `offset` - The bytecode offset to check
    #[inline]
    #[must_use]
    pub fn is_valid_offset(&self, offset: u16) -> bool {
        self.valid_offsets.contains(&offset)
    }

    /// Returns the total code length.
    #[inline]
    #[must_use]
    pub fn code_length(&self) -> u16 {
        self.code_length
    }

    /// Returns the number of instructions.
    #[inline]
    #[must_use]
    pub fn instruction_count(&self) -> usize {
        self.instruction_count
    }

    /// Returns a slice of all instruction offsets.
    #[inline]
    #[must_use]
    pub fn offsets(&self) -> &[u16] {
        &self.instruction_offsets
    }

    /// Validates that an offset is within code bounds and on an instruction boundary.
    ///
    /// # Arguments
    ///
    /// * `offset` - The offset to validate
    /// * `context` - Description of where the offset comes from (for error messages)
    ///
    /// # Errors
    ///
    /// Returns an error if the offset is invalid.
    pub fn validate_offset(&self, offset: u16, context: &str) -> Result<()> {
        if offset >= self.code_length {
            return Err(VerifyError::VerifyError(format!(
                "{context}: offset {offset} exceeds code length {}",
                self.code_length
            )));
        }
        if !self.is_valid_offset(offset) {
            return Err(VerifyError::VerifyError(format!(
                "{context}: offset {offset} is not an instruction boundary"
            )));
        }
        Ok(())
    }
}

/// Validates exception table entries.
///
/// Checks that all exception handler ranges and targets are valid according
/// to [JVMS §4.9.1](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.9.1).
///
/// # Arguments
///
/// * `exception_table` - The exception table to validate
/// * `code_info` - Information about instruction boundaries
///
/// # Errors
///
/// Returns an error if any exception table entry is invalid.
///
/// # JVMS Reference
///
/// - `start_pc` and `end_pc` must be valid instruction boundaries
/// - `start_pc` must be less than `end_pc`
/// - `handler_pc` must be a valid instruction boundary
///
/// # Errors
///
/// Returns a `VerifyError` if the exception table is malformed.
pub fn validate_exception_table(
    exception_table: &[ExceptionTableEntry],
    code_info: &CodeInfo,
) -> Result<()> {
    for (i, handler) in exception_table.iter().enumerate() {
        // Get start and end from range
        let start_pc = handler.range_pc.start;
        let end_pc = handler.range_pc.end;

        // start_pc must be a valid instruction boundary
        code_info.validate_offset(start_pc, &format!("Exception handler {i} start_pc"))?;

        // end_pc can be code_length (exclusive end) or an instruction boundary
        if end_pc != code_info.code_length() && !code_info.is_valid_offset(end_pc) {
            return Err(VerifyError::VerifyError(format!(
                "Exception handler {i} end_pc {end_pc} is not valid"
            )));
        }

        // start_pc must be less than end_pc
        if start_pc >= end_pc {
            return Err(VerifyError::VerifyError(format!(
                "Exception handler {i}: start_pc ({start_pc}) must be < end_pc ({end_pc})"
            )));
        }

        // handler_pc must be a valid instruction boundary
        code_info.validate_offset(
            handler.handler_pc,
            &format!("Exception handler {i} handler_pc"),
        )?;
    }

    Ok(())
}

/// Computes the successor offsets for an instruction.
///
/// This determines the possible next instructions considering:
/// - Fallthrough for sequential instructions
/// - Branch targets for conditional/unconditional jumps
/// - Switch targets
///
/// # Arguments
///
/// * `offset` - Current instruction offset
/// * `instruction` - The instruction to analyze
/// * `next_offset` - The next instruction offset for fallthrough
/// * `code_info` - Code boundary information
///
/// # Returns
///
/// A tuple of (successors, `falls_through`) where:
/// - successors: List of possible next instruction offsets
/// - `falls_through`: Whether the instruction falls through to the next
///
/// # Errors
///
/// Returns an error if branch targets are invalid.
pub fn compute_successors(
    offset: u16,
    instruction: &Instruction,
    next_offset: u16,
    code_info: &CodeInfo,
) -> Result<(Vec<u16>, bool)> {
    let mut successors = Vec::new();
    let mut falls_through = true;

    match instruction {
        // Unconditional branches
        Instruction::Goto(target) => {
            code_info.validate_offset(*target, "Goto target")?;
            successors.push(*target);
            falls_through = false;
        }
        Instruction::Goto_w(target) => {
            let target_u16 = compute_relative_target(offset, *target)?;
            code_info.validate_offset(target_u16, "Goto_w target")?;
            successors.push(target_u16);
            falls_through = false;
        }

        // Returns and athrow don't have successors
        Instruction::Return
        | Instruction::Ireturn
        | Instruction::Lreturn
        | Instruction::Freturn
        | Instruction::Dreturn
        | Instruction::Areturn
        | Instruction::Athrow => {
            falls_through = false;
        }

        // Conditional branches
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
        | Instruction::Ifnull(target)
        | Instruction::Ifnonnull(target) => {
            code_info.validate_offset(*target, "Conditional branch target")?;
            successors.push(*target);
            // Also falls through
        }

        // Table switch
        Instruction::Tableswitch(table) => {
            // Default target
            let default_target = compute_relative_target(offset, table.default)?;
            code_info.validate_offset(default_target, "tableswitch default")?;
            successors.push(default_target);

            // All case targets
            for (i, &off) in table.offsets.iter().enumerate() {
                let case_target = compute_relative_target(offset, off)?;
                let case_index = i32::try_from(i).unwrap_or(0);
                code_info.validate_offset(
                    case_target,
                    &format!("tableswitch case {}", table.low + case_index),
                )?;
                successors.push(case_target);
            }
            falls_through = false;
        }

        // Lookup switch
        Instruction::Lookupswitch(lookup) => {
            // Default target
            let default_target = compute_relative_target(offset, lookup.default)?;
            code_info.validate_offset(default_target, "lookupswitch default")?;
            successors.push(default_target);

            // All case targets
            for (&key, &off) in &lookup.pairs {
                let case_target = compute_relative_target(offset, off)?;
                code_info.validate_offset(case_target, &format!("lookupswitch case {key}"))?;
                successors.push(case_target);
            }
            falls_through = false;
        }

        // JSR/RET are rejected for class file version >= 51.0
        Instruction::Jsr(_)
        | Instruction::Jsr_w(_)
        | Instruction::Ret(_)
        | Instruction::Ret_w(_) => {
            return Err(VerifyError::VerifyError(
                "jsr/ret instructions are not allowed in class files version 51.0 or later"
                    .to_string(),
            ));
        }

        // All other instructions fall through
        _ => {}
    }

    // Add fallthrough successor if applicable
    if falls_through {
        if next_offset > code_info.code_length() {
            return Err(VerifyError::VerifyError(format!(
                "Instruction at offset {offset} falls off the end of code"
            )));
        }
        successors.push(next_offset);
    }

    Ok((successors, falls_through))
}

/// Computes an absolute target from a relative offset.
fn compute_relative_target(current_offset: u16, relative: i32) -> Result<u16> {
    let current = i32::from(current_offset);
    let target = current + relative;
    if target < 0 || target > i32::from(u16::MAX) {
        return Err(VerifyError::VerifyError(format!(
            "Branch target {target} out of range"
        )));
    }
    Ok(u16::try_from(target)?)
}

/// Decodes stack map table entries and returns frame information at each offset.
///
/// The `StackMapTable` attribute contains delta-encoded frames. This function
/// decodes them into absolute offsets.
///
/// # Arguments
///
/// * `stack_frames` - The stack frames from the `StackMapTable` attribute
/// * `code_info` - Code boundary information
///
/// # Returns
///
/// A vector of (offset, `stack_frame`) pairs.
///
/// # Errors
///
/// Returns an error if the `StackMapTable` contains invalid offsets.
pub fn decode_stack_map_table<'a>(
    stack_frames: &'a [StackFrame],
    code_info: &CodeInfo,
) -> Result<Vec<(u16, &'a StackFrame)>> {
    let mut result = Vec::with_capacity(stack_frames.len());
    let mut current_offset: i32 = -1; // Previous offset (starts before first instruction)

    for frame in stack_frames {
        let offset_delta = get_offset_delta(frame);

        // Calculate the actual offset
        // First frame: offset = offset_delta
        // Subsequent frames: offset = previous_offset + offset_delta + 1
        let offset = if current_offset < 0 {
            offset_delta
        } else {
            let prev = u16::try_from(current_offset)?;
            prev.checked_add(offset_delta)
                .and_then(|o| o.checked_add(1))
                .ok_or_else(|| {
                    VerifyError::VerifyError("StackMapTable offset overflow".to_string())
                })?
        };

        // Validate the offset is a valid instruction boundary
        if !code_info.is_valid_offset(offset) {
            return Err(VerifyError::VerifyError(format!(
                "StackMapTable frame offset {offset} is not a valid instruction boundary"
            )));
        }

        result.push((offset, frame));
        current_offset = i32::from(offset);
    }

    Ok(result)
}

/// Extracts the offset delta from a stack frame.
fn get_offset_delta(frame: &StackFrame) -> u16 {
    match frame {
        StackFrame::SameFrame { frame_type } => u16::from(*frame_type),
        StackFrame::SameLocals1StackItemFrame { frame_type, .. } => u16::from(*frame_type) - 64,
        StackFrame::SameLocals1StackItemFrameExtended { offset_delta, .. }
        | StackFrame::ChopFrame { offset_delta, .. }
        | StackFrame::SameFrameExtended { offset_delta, .. }
        | StackFrame::AppendFrame { offset_delta, .. }
        | StackFrame::FullFrame { offset_delta, .. } => *offset_delta,
    }
}

/// Worklist-based algorithm state for dataflow analysis.
///
/// Uses a bitset for efficient tracking of which instructions need (re)processing.
#[derive(Debug)]
pub struct Worklist {
    /// Bitset tracking which instruction indices are in the worklist.
    in_worklist: Vec<bool>,

    /// Queue of instruction indices to process.
    queue: Vec<usize>,
}

impl Worklist {
    /// Creates a new worklist for the given number of instructions.
    #[must_use]
    pub fn new(instruction_count: usize) -> Self {
        Self {
            in_worklist: vec![false; instruction_count],
            queue: Vec::with_capacity(instruction_count),
        }
    }

    /// Adds an instruction index to the worklist if not already present.
    pub fn add(&mut self, index: usize) {
        if index < self.in_worklist.len() && !self.in_worklist[index] {
            self.in_worklist[index] = true;
            self.queue.push(index);
        }
    }

    /// Removes and returns the next instruction index to process.
    #[must_use]
    pub fn pop(&mut self) -> Option<usize> {
        let idx = self.queue.pop()?;
        self.in_worklist[idx] = false;
        Some(idx)
    }

    /// Checks if the worklist is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_code_info() -> CodeInfo {
        // Simulates: offset 0, 1, 3, 6, 10 (instructions of various sizes)
        CodeInfo::new(vec![0, 1, 3, 6, 10], 12)
    }

    #[test]
    fn test_code_info_basics() {
        let info = make_code_info();

        assert_eq!(info.code_length(), 12);
        assert_eq!(info.instruction_count(), 5);

        assert!(info.is_valid_offset(0));
        assert!(info.is_valid_offset(3));
        assert!(!info.is_valid_offset(2));
        assert!(!info.is_valid_offset(100));
    }

    #[test]
    fn test_code_info_index_lookup() {
        let info = make_code_info();

        assert_eq!(info.index_at(0), Some(0));
        assert_eq!(info.index_at(3), Some(2));
        assert_eq!(info.index_at(10), Some(4));
        assert_eq!(info.index_at(2), None);
    }

    #[test]
    fn test_code_info_offset_lookup() {
        let info = make_code_info();

        assert_eq!(info.offset_at(0), Some(0));
        assert_eq!(info.offset_at(2), Some(3));
        assert_eq!(info.offset_at(10), None);
    }

    #[test]
    fn test_validate_offset() {
        let info = make_code_info();

        assert!(info.validate_offset(0, "test").is_ok());
        assert!(info.validate_offset(6, "test").is_ok());

        // Out of bounds
        assert!(info.validate_offset(100, "test").is_err());

        // Not an instruction boundary
        assert!(info.validate_offset(2, "test").is_err());
    }

    #[test]
    fn test_validate_exception_table_valid() {
        let info = make_code_info();

        let handlers = vec![ExceptionTableEntry {
            range_pc: 0..6,
            handler_pc: 10,
            catch_type: 0,
        }];

        assert!(validate_exception_table(&handlers, &info).is_ok());
    }

    #[test]
    fn test_validate_exception_table_invalid_start() {
        let info = make_code_info();

        let handlers = vec![ExceptionTableEntry {
            range_pc: 2..6, // 2 is not valid
            handler_pc: 10,
            catch_type: 0,
        }];

        assert!(validate_exception_table(&handlers, &info).is_err());
    }

    #[test]
    fn test_validate_exception_table_invalid_range() {
        let info = make_code_info();

        #[expect(clippy::reversed_empty_ranges)]
        let handlers = vec![ExceptionTableEntry {
            range_pc: 6..3, // start >= end (empty range)
            handler_pc: 10,
            catch_type: 0,
        }];

        assert!(validate_exception_table(&handlers, &info).is_err());
    }

    #[test]
    fn test_worklist() {
        let mut worklist = Worklist::new(10);

        assert!(worklist.is_empty());

        worklist.add(5);
        worklist.add(3);
        worklist.add(5); // Duplicate, should be ignored

        assert!(!worklist.is_empty());

        let first = worklist.pop();
        let second = worklist.pop();

        // Order is LIFO
        assert!(first == Some(3) || first == Some(5));
        assert!(second == Some(3) || second == Some(5));
        assert_ne!(first, second);

        assert!(worklist.is_empty());
        assert_eq!(worklist.pop(), None);
    }

    #[test]
    fn test_compute_relative_target() {
        assert_eq!(compute_relative_target(10, 5).unwrap(), 15);
        assert_eq!(compute_relative_target(10, -5).unwrap(), 5);

        // Overflow
        assert!(compute_relative_target(0, -1).is_err());
    }
}
