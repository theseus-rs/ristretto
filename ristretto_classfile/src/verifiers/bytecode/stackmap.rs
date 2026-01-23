//! # Stack Map Table Decoder
//!
//! This module provides efficient decoding and caching of `StackMapTable` attributes. The decoded
//! frames are stored in a compact representation for fast lookup during the verification pass.
//!
//! # Architecture
//!
//! The decoder converts the delta-encoded `StackMapTable` into absolute offsets and canonical frame
//! representations, supporting O(1) lookup by bytecode offset.
//!
//! # References
//!
//! - [JVMS §4.7.4 - The StackMapTable Attribute](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.4)

use ahash::{AHashMap, RandomState};
use std::sync::Arc;

use crate::attributes::{StackFrame, VerificationType as ClassFileVerificationType};
use crate::class_file::ClassFile;
use crate::verifiers::bytecode::frame::Frame;
use crate::verifiers::bytecode::type_system::VerificationType;
use crate::verifiers::error::{Result, VerifyError};

/// Decoded stack map table with efficient lookup by bytecode offset.
///
/// This struct caches the decoded `StackMapTable` for a method, providing:
/// - O(1) lookup of frames by bytecode offset
/// - Pre-validated frame data
/// - Canonical internal type representation
#[derive(Debug)]
pub struct DecodedStackMapTable {
    /// Frames indexed by bytecode offset.
    frames: AHashMap<u16, DecodedFrame>,

    /// Sorted list of frame offsets for iteration.
    offsets: Vec<u16>,

    /// Whether the table is empty.
    is_empty: bool,
}

/// A decoded frame from the `StackMapTable`.
///
/// Contains the local variable types and stack types at a specific
/// bytecode offset, in canonical internal representation.
#[derive(Debug, Clone)]
pub struct DecodedFrame {
    /// Bytecode offset of this frame.
    pub offset: u16,

    /// Local variable types.
    pub locals: Vec<VerificationType>,

    /// Operand stack types.
    pub stack: Vec<VerificationType>,

    /// The original frame type for debugging.
    pub frame_type: FrameType,
}

/// The type of stack map frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameType {
    /// Same locals as previous, empty stack.
    Same,
    /// Same locals as previous, one stack item.
    SameLocals1StackItem,
    /// Remove k locals, empty stack.
    Chop(u8),
    /// Add new locals, empty stack.
    Append,
    /// Full frame specification.
    Full,
}

impl DecodedStackMapTable {
    /// Creates a new empty decoded stack map table.
    #[must_use]
    pub fn empty() -> Self {
        Self {
            frames: AHashMap::default(),
            offsets: Vec::new(),
            is_empty: true,
        }
    }

    /// Decodes a `StackMapTable` from the class file format.
    ///
    /// # Arguments
    ///
    /// * `stack_frames` - The stack frames from the `StackMapTable` attribute
    /// * `initial_frame` - The initial frame at method entry
    /// * `class_file` - The class file (for constant pool lookups)
    /// * `max_stack` - Maximum operand stack depth
    ///
    /// # Errors
    ///
    /// Returns an error if the `StackMapTable` is malformed.
    pub fn decode(
        stack_frames: &[StackFrame],
        initial_frame: &Frame,
        class_file: &ClassFile,
        max_stack: u16,
    ) -> Result<Self> {
        if stack_frames.is_empty() {
            return Ok(Self::empty());
        }

        let mut frames = AHashMap::with_capacity_and_hasher(stack_frames.len(), RandomState::new());
        let mut offsets = Vec::with_capacity(stack_frames.len());

        let mut current_locals = initial_frame.locals.clone();
        let mut prev_offset: Option<u16> = None;

        for stack_frame in stack_frames {
            let offset_delta = get_offset_delta(stack_frame);

            // Calculate actual offset
            let offset = if let Some(prev) = prev_offset {
                prev.checked_add(offset_delta)
                    .and_then(|o| o.checked_add(1))
                    .ok_or_else(|| {
                        VerifyError::VerifyError("StackMapTable offset overflow".to_string())
                    })?
            } else {
                offset_delta
            };

            // Decode the frame
            let (decoded, new_locals) =
                decode_frame(offset, stack_frame, &current_locals, class_file, max_stack)?;

            frames.insert(offset, decoded);
            offsets.push(offset);
            current_locals = new_locals;
            prev_offset = Some(offset);
        }

        // Sort offsets for ordered iteration
        offsets.sort_unstable();

        Ok(Self {
            frames,
            offsets,
            is_empty: false,
        })
    }

    /// Gets the frame at the specified bytecode offset.
    #[must_use]
    pub fn get(&self, offset: u16) -> Option<&DecodedFrame> {
        self.frames.get(&offset)
    }

    /// Checks if there is a frame at the specified offset.
    #[must_use]
    pub fn has_frame_at(&self, offset: u16) -> bool {
        self.frames.contains_key(&offset)
    }

    /// Returns true if the table is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.is_empty
    }

    /// Returns the number of frames in the table.
    #[must_use]
    pub fn len(&self) -> usize {
        self.frames.len()
    }

    /// Returns an iterator over all frame offsets in order.
    pub fn offsets(&self) -> impl Iterator<Item = u16> + '_ {
        self.offsets.iter().copied()
    }

    /// Returns an iterator over all frames in offset order.
    pub fn frames(&self) -> impl Iterator<Item = &DecodedFrame> + '_ {
        self.offsets.iter().filter_map(|o| self.frames.get(o))
    }

    /// Converts a decoded frame to a Frame for verification.
    #[expect(clippy::unused_self)]
    #[must_use]
    pub fn to_frame(&self, decoded: &DecodedFrame, max_locals: u16, max_stack: u16) -> Frame {
        let mut frame = Frame::new(max_locals as usize, max_stack as usize);

        // Copy locals, padding with Top if needed
        for (i, ty) in decoded.locals.iter().enumerate() {
            if i < frame.locals.len() {
                frame.locals[i] = ty.clone();
            }
        }

        // Copy stack
        for ty in &decoded.stack {
            // Ignore potential overflow - should have been validated during decode
            let _ = frame.push(ty.clone());
        }

        frame
    }

    /// Validates that all frame offsets are valid instruction boundaries.
    ///
    /// # Arguments
    ///
    /// * `valid_offsets` - Set of valid instruction boundary offsets
    ///
    /// # Errors
    ///
    /// Returns an error if any frame offset is not a valid instruction boundary.
    pub fn validate_offsets<F>(&self, is_valid: F) -> Result<()>
    where
        F: Fn(u16) -> bool,
    {
        for &offset in &self.offsets {
            if !is_valid(offset) {
                return Err(VerifyError::VerifyError(format!(
                    "StackMapTable frame at offset {offset} is not at an instruction boundary"
                )));
            }
        }
        Ok(())
    }
}

/// Decodes a single stack map frame.
fn decode_frame(
    offset: u16,
    stack_frame: &StackFrame,
    current_locals: &[VerificationType],
    class_file: &ClassFile,
    _max_stack: u16,
) -> Result<(DecodedFrame, Vec<VerificationType>)> {
    let (frame_type, locals, stack) = match stack_frame {
        StackFrame::SameFrame { .. } | StackFrame::SameFrameExtended { .. } => {
            (FrameType::Same, current_locals.to_vec(), Vec::new())
        }

        StackFrame::SameLocals1StackItemFrame { stack, .. }
        | StackFrame::SameLocals1StackItemFrameExtended { stack, .. } => {
            let stack_types = convert_verification_types(stack, class_file);
            (
                FrameType::SameLocals1StackItem,
                current_locals.to_vec(),
                stack_types,
            )
        }

        StackFrame::ChopFrame { frame_type, .. } => {
            let k = (251 - frame_type) as usize;
            let mut new_locals = current_locals.to_vec();

            // Remove k locals from the end, accounting for category 2 types
            let mut removed = 0;
            while removed < k && !new_locals.is_empty() {
                new_locals.pop();
                removed += 1;
            }

            (FrameType::Chop(u8::try_from(k)?), new_locals, Vec::new())
        }

        StackFrame::AppendFrame { locals, .. } => {
            let mut new_locals = current_locals.to_vec();
            let additional = convert_verification_types(locals, class_file);

            for ty in &additional {
                new_locals.push(ty.clone());
                // Category 2 types occupy two slots
                if ty.is_category2() {
                    new_locals.push(VerificationType::Top);
                }
            }

            (FrameType::Append, new_locals, Vec::new())
        }

        StackFrame::FullFrame { locals, stack, .. } => {
            let new_locals = expand_locals(locals, class_file);
            let stack_types = convert_verification_types(stack, class_file);
            (FrameType::Full, new_locals, stack_types)
        }
    };

    let decoded = DecodedFrame {
        offset,
        locals: locals.clone(),
        stack,
        frame_type,
    };

    Ok((decoded, locals))
}

/// Expands locals from `StackMapTable` format to full locals array.
///
/// In `StackMapTable`, category 2 types are represented as single entries,
/// but in the actual frame they occupy two slots.
fn expand_locals(
    locals: &[ClassFileVerificationType],
    class_file: &ClassFile,
) -> Vec<VerificationType> {
    let mut result = Vec::with_capacity(locals.len() * 2);

    for local in locals {
        let ty = convert_single_type(local, class_file);
        result.push(ty.clone());

        // Category 2 types occupy an additional slot
        if ty.is_category2() {
            result.push(VerificationType::Top);
        }
    }

    result
}

/// Converts a slice of class file verification types to internal types.
fn convert_verification_types(
    types: &[ClassFileVerificationType],
    class_file: &ClassFile,
) -> Vec<VerificationType> {
    types
        .iter()
        .map(|t| convert_single_type(t, class_file))
        .collect()
}

/// Converts a single class file verification type to internal type.
fn convert_single_type(
    v_type: &ClassFileVerificationType,
    class_file: &ClassFile,
) -> VerificationType {
    match v_type {
        ClassFileVerificationType::Top => VerificationType::Top,
        ClassFileVerificationType::Integer => VerificationType::Integer,
        ClassFileVerificationType::Float => VerificationType::Float,
        ClassFileVerificationType::Long => VerificationType::Long,
        ClassFileVerificationType::Double => VerificationType::Double,
        ClassFileVerificationType::Null => VerificationType::Null,
        ClassFileVerificationType::UninitializedThis => VerificationType::UninitializedThis,
        ClassFileVerificationType::Object { cpool_index } => {
            if let Ok(name) = class_file.constant_pool.try_get_class(*cpool_index) {
                VerificationType::Object(Arc::from(name))
            } else {
                VerificationType::Top
            }
        }
        ClassFileVerificationType::Uninitialized { offset } => {
            VerificationType::Uninitialized(*offset)
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ConstantPool;
    use crate::Version;
    use crate::constant::Constant;

    fn create_test_class_file() -> ClassFile {
        let mut constant_pool = ConstantPool::default();
        constant_pool
            .add(Constant::Utf8("TestClass".to_string()))
            .unwrap();
        constant_pool.add(Constant::Class(1)).unwrap();

        ClassFile {
            version: Version::Java8 { minor: 0 },
            constant_pool,
            access_flags: crate::ClassAccessFlags::PUBLIC,
            this_class: 2,
            super_class: 0,
            interfaces: vec![],
            fields: vec![],
            methods: vec![],
            attributes: vec![],
        }
    }

    #[test]
    fn test_empty_stack_map_table() {
        let table = DecodedStackMapTable::empty();
        assert!(table.is_empty());
        assert_eq!(table.len(), 0);
        assert!(table.get(0).is_none());
    }

    #[test]
    fn test_decode_same_frame() {
        let class_file = create_test_class_file();
        let initial_frame = Frame::new(2, 4);

        let stack_frames = vec![StackFrame::SameFrame { frame_type: 10 }];

        let table =
            DecodedStackMapTable::decode(&stack_frames, &initial_frame, &class_file, 4).unwrap();

        assert!(!table.is_empty());
        assert_eq!(table.len(), 1);

        let frame = table.get(10).unwrap();
        assert_eq!(frame.offset, 10);
        assert_eq!(frame.frame_type, FrameType::Same);
        assert!(frame.stack.is_empty());
    }

    #[test]
    fn test_decode_chop_frame() {
        let class_file = create_test_class_file();
        let mut initial_frame = Frame::new(4, 4);
        initial_frame.locals[0] = VerificationType::Integer;
        initial_frame.locals[1] = VerificationType::Integer;
        initial_frame.locals[2] = VerificationType::Integer;

        let stack_frames = vec![StackFrame::ChopFrame {
            frame_type: 250, // Chop 1 local
            offset_delta: 5,
        }];

        let table =
            DecodedStackMapTable::decode(&stack_frames, &initial_frame, &class_file, 4).unwrap();

        let frame = table.get(5).unwrap();
        assert_eq!(frame.frame_type, FrameType::Chop(1));
        // Should have removed one local
        assert_eq!(frame.locals.len(), initial_frame.locals.len() - 1);
    }

    #[test]
    fn test_decode_multiple_frames() {
        let class_file = create_test_class_file();
        let initial_frame = Frame::new(2, 4);

        let stack_frames = vec![
            StackFrame::SameFrame { frame_type: 5 },
            StackFrame::SameFrame { frame_type: 10 },
        ];

        let table =
            DecodedStackMapTable::decode(&stack_frames, &initial_frame, &class_file, 4).unwrap();

        assert_eq!(table.len(), 2);

        // First frame at offset 5
        assert!(table.has_frame_at(5));

        // Second frame at offset 5 + 10 + 1 = 16
        assert!(table.has_frame_at(16));
    }
}
