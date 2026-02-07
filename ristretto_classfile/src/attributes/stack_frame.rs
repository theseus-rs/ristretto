use crate::attributes::VerificationType;
use crate::error::Error::InvalidStackFrameType;
use crate::error::Result;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fmt;
use std::io::Cursor;

/// Represents a stack map frame in the `StackMapTable` attribute of a Java class file.
///
/// Stack frames are used for bytecode verification in the JVM. Each frame describes the state of
/// the local variable array and the operand stack at a specific point in the bytecode. The JVM uses
/// these frames to verify that bytecode is type-safe without having to simulate the entire program.
///
/// The JVM specification defines several types of stack frames to efficiently encode different
/// verification scenarios:
///
/// - `SameFrame`: The locals are the same as the previous frame and the stack is empty
/// - `SameLocals1StackItemFrame`: The locals are the same as the previous frame with one stack item
/// - `SameLocals1StackItemFrameExtended`: Same as above, but with a larger offset delta
/// - `ChopFrame`: The same as the previous frame except that the last k locals are absent
/// - `SameFrameExtended`: The same as the previous frame but with a larger offset delta
/// - `AppendFrame`: The same as the previous frame except with k additional locals
/// - `FullFrame`: A complete frame that explicitly defines all locals and stack items
///
/// Each stack frame variant has a specific `frame_type` range as defined in the JVM specification.
/// The offset delta of a frame is the difference in bytecode offset from the previous frame.
///
/// # Examples
///
/// ```rust
/// use ristretto_classfile::attributes::{StackFrame, VerificationType};
/// use ristretto_classfile::Result;
/// use std::io::Cursor;
///
/// // Example: SameFrame
/// let same_frame = StackFrame::SameFrame { frame_type: 63 };
/// let mut bytes = Vec::new();
/// same_frame.to_bytes(&mut bytes)?;
/// let mut cursor = Cursor::new(bytes.clone());
/// let deserialized_same_frame = StackFrame::from_bytes(&mut cursor)?;
/// assert_eq!(same_frame, deserialized_same_frame);
/// assert_eq!(same_frame.frame_type(), 63);
/// assert_eq!(same_frame.offset_delta(), 63);
///
/// // Example: FullFrame
/// let full_frame = StackFrame::FullFrame {
///     frame_type: 255,
///     offset_delta: 100,
///     locals: vec![VerificationType::Integer, VerificationType::Float],
///     stack: vec![VerificationType::Double],
/// };
/// bytes.clear();
/// full_frame.to_bytes(&mut bytes)?;
/// let mut cursor = Cursor::new(bytes);
/// let deserialized_full_frame = StackFrame::from_bytes(&mut cursor)?;
/// assert_eq!(full_frame, deserialized_full_frame);
/// assert_eq!(full_frame.frame_type(), 255);
/// assert_eq!(full_frame.offset_delta(), 100);
/// # Ok::<(), ristretto_classfile::Error>(())
/// ```
///
/// # References
///
/// - [JVMS §4.7.4](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.4)
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StackFrame {
    /// Represents a frame with the same locals as the previous frame and an empty stack.
    ///
    /// Valid `frame_type` values are 0-63. The `offset_delta` is the value of `frame_type`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::StackFrame;
    ///
    /// let frame = StackFrame::SameFrame { frame_type: 25 };
    /// assert_eq!(frame.offset_delta(), 25);
    /// ```
    SameFrame { frame_type: u8 },

    /// Represents a frame with the same locals as the previous frame and a single one-item stack.
    ///
    /// Valid `frame_type` values are 64-127. The `offset_delta` is `frame_type - 64`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::{StackFrame, VerificationType};
    ///
    /// let frame = StackFrame::SameLocals1StackItemFrame {
    ///     frame_type: 70,
    ///     stack: vec![VerificationType::Integer]
    /// };
    /// assert_eq!(frame.offset_delta(), 6); // 70 - 64 = 6
    /// ```
    SameLocals1StackItemFrame {
        frame_type: u8,
        stack: Vec<VerificationType>,
    },

    /// Represents a frame with the same locals as the previous frame and a single one-item stack,
    /// with an explicit offset delta.
    ///
    /// The `frame_type` must be 247.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::{StackFrame, VerificationType};
    ///
    /// let frame = StackFrame::SameLocals1StackItemFrameExtended {
    ///     frame_type: 247,
    ///     offset_delta: 1000,
    ///     stack: vec![VerificationType::Double]
    /// };
    /// assert_eq!(frame.offset_delta(), 1000);
    /// ```
    SameLocals1StackItemFrameExtended {
        frame_type: u8,
        offset_delta: u16,
        stack: Vec<VerificationType>,
    },

    /// Represents a frame with the same locals as the previous frame, except that the last `k` are
    /// absent, where `k` is specified by `frame_type - 251`. The stack is empty.
    ///
    /// Valid `frame_type` values are 248-250.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::StackFrame;
    ///
    /// let frame = StackFrame::ChopFrame {
    ///     frame_type: 249,
    ///     offset_delta: 300
    /// };
    /// // This frame chops 2 local variables (249 - 251 = -2)
    /// assert_eq!(frame.offset_delta(), 300);
    /// ```
    ChopFrame { frame_type: u8, offset_delta: u16 },

    /// Represents a frame with the same locals as the previous frame and an empty stack, with an
    /// explicit offset delta.
    ///
    /// The `frame_type` must be 251.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::StackFrame;
    ///
    /// let frame = StackFrame::SameFrameExtended {
    ///     frame_type: 251,
    ///     offset_delta: 500
    /// };
    /// assert_eq!(frame.offset_delta(), 500);
    /// ```
    SameFrameExtended { frame_type: u8, offset_delta: u16 },

    /// Represents a frame with the same locals as the previous frame, except that`k` additional
    /// locals are defined, where `k` is specified by `frame_type - 251`. The stack is empty.
    ///
    /// Valid `frame_type` values are 252-254.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::{StackFrame, VerificationType};
    ///
    /// let frame = StackFrame::AppendFrame {
    ///     frame_type: 253,
    ///     offset_delta: 400,
    ///     locals: vec![VerificationType::Integer, VerificationType::Float]
    /// };
    /// // This frame appends 2 local variables (253 - 251 = 2)
    /// assert_eq!(frame.offset_delta(), 400);
    /// ```
    AppendFrame {
        frame_type: u8,
        offset_delta: u16,
        locals: Vec<VerificationType>,
    },

    /// Represents a complete frame that explicitly defines all locals and the entire stack.
    ///
    /// The `frame_type` must be 255.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::{StackFrame, VerificationType};
    ///
    /// let frame = StackFrame::FullFrame {
    ///     frame_type: 255,
    ///     offset_delta: 200,
    ///     locals: vec![VerificationType::Integer, VerificationType::Top],
    ///     stack: vec![VerificationType::Long]
    /// };
    /// assert_eq!(frame.offset_delta(), 200);
    /// ```
    FullFrame {
        frame_type: u8,
        offset_delta: u16,
        locals: Vec<VerificationType>,
        stack: Vec<VerificationType>,
    },
}

impl StackFrame {
    /// Get the frame type.
    ///
    /// The frame type is a byte that identifies the type of stack frame and determines how to
    /// interpret the remaining data.
    ///
    /// # Frame type ranges
    ///
    /// - 0-63: `SameFrame`
    /// - 64-127: `SameLocals1StackItemFrame`
    /// - 247: `SameLocals1StackItemFrameExtended`
    /// - 248-250: `ChopFrame`
    /// - 251: `SameFrameExtended`
    /// - 252-254: `AppendFrame`
    /// - 255: `FullFrame`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::StackFrame;
    ///
    /// let frame = StackFrame::SameFrame { frame_type: 42 };
    /// assert_eq!(frame.frame_type(), 42);
    /// ```
    #[must_use]
    pub fn frame_type(&self) -> u8 {
        match self {
            StackFrame::SameFrame { frame_type }
            | StackFrame::SameLocals1StackItemFrame { frame_type, .. }
            | StackFrame::SameLocals1StackItemFrameExtended { frame_type, .. }
            | StackFrame::ChopFrame { frame_type, .. }
            | StackFrame::SameFrameExtended { frame_type, .. }
            | StackFrame::AppendFrame { frame_type, .. }
            | StackFrame::FullFrame { frame_type, .. } => *frame_type,
        }
    }

    /// Get the offset delta.
    ///
    /// The offset delta is used to calculate the actual bytecode offset of this frame. It
    /// represents the difference in bytecode offset from the previous frame.
    ///
    /// For different frame types, the offset delta is determined differently:
    /// - For `SameFrame`: equal to the `frame_type` value (0-63)
    /// - For `SameLocals1StackItemFrame`: equal to `frame_type` - 64 (for values 64-127)
    /// - For all other frames: explicitly stored in the `offset_delta` field
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::{StackFrame, VerificationType};
    ///
    /// // For SameFrame, offset_delta is the frame_type value
    /// let frame1 = StackFrame::SameFrame { frame_type: 30 };
    /// assert_eq!(frame1.offset_delta(), 30);
    ///
    /// // For SameLocals1StackItemFrame, offset_delta is frame_type - 64
    /// let frame2 = StackFrame::SameLocals1StackItemFrame {
    ///     frame_type: 75,
    ///     stack: vec![VerificationType::Integer]
    /// };
    /// assert_eq!(frame2.offset_delta(), 11); // 75 - 64 = 11
    ///
    /// // For FullFrame, offset_delta is explicitly stored
    /// let frame3 = StackFrame::FullFrame {
    ///     frame_type: 255,
    ///     offset_delta: 1000,
    ///     locals: vec![],
    ///     stack: vec![]
    /// };
    /// assert_eq!(frame3.offset_delta(), 1000);
    /// ```
    #[must_use]
    pub fn offset_delta(&self) -> u16 {
        match self {
            StackFrame::SameFrame { frame_type } => u16::from(*frame_type),
            StackFrame::SameLocals1StackItemFrame { frame_type, .. } => {
                let frame_type = u16::from(*frame_type);
                frame_type.saturating_sub(64)
            }
            StackFrame::AppendFrame { offset_delta, .. }
            | StackFrame::ChopFrame { offset_delta, .. }
            | StackFrame::FullFrame { offset_delta, .. }
            | StackFrame::SameFrameExtended { offset_delta, .. }
            | StackFrame::SameLocals1StackItemFrameExtended { offset_delta, .. } => *offset_delta,
        }
    }

    /// Deserialize the stack frame from bytes.
    ///
    /// This method reads a stack frame from a byte stream according to the JVM specification. The
    /// frame type is first read, and then the rest of the data is interpreted based on the frame
    /// type value.
    ///
    /// # Errors
    ///
    /// Returns an error if the frame type is invalid or if reading from the byte stream fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::{StackFrame, VerificationType};
    /// use std::io::Cursor;
    /// use ristretto_classfile::Result;
    ///
    /// // Create a byte array with a SameFrame (frame_type = 10)
    /// let bytes = vec![10];
    /// let mut cursor = Cursor::new(bytes);
    ///
    /// // Parse the bytes into a StackFrame
    /// let frame = StackFrame::from_bytes(&mut cursor)?;
    ///
    /// // Verify it's the expected frame type
    /// assert!(matches!(frame, StackFrame::SameFrame { frame_type: 10 }));
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    pub fn from_bytes(bytes: &mut Cursor<impl AsRef<[u8]>>) -> Result<StackFrame> {
        let frame_type = bytes.read_u8()?;
        let frame = match frame_type {
            0..=63 => StackFrame::SameFrame { frame_type },
            64..=127 => {
                let verification_type = VerificationType::from_bytes(bytes)?;
                let stack = vec![verification_type];
                StackFrame::SameLocals1StackItemFrame { frame_type, stack }
            }
            247 => {
                let offset_delta = bytes.read_u16::<BigEndian>()?;
                let verification_type = VerificationType::from_bytes(bytes)?;
                let stack = vec![verification_type];
                StackFrame::SameLocals1StackItemFrameExtended {
                    frame_type,
                    offset_delta,
                    stack,
                }
            }
            248..=250 => {
                let offset_delta = bytes.read_u16::<BigEndian>()?;
                StackFrame::ChopFrame {
                    frame_type,
                    offset_delta,
                }
            }
            251 => {
                let offset_delta = bytes.read_u16::<BigEndian>()?;
                StackFrame::SameFrameExtended {
                    frame_type,
                    offset_delta,
                }
            }
            252..=254 => {
                let offset_delta = bytes.read_u16::<BigEndian>()?;
                let mut locals = Vec::with_capacity((frame_type - 251) as usize);
                for _ in 0..(frame_type - 251) {
                    let verification_type = VerificationType::from_bytes(bytes)?;
                    locals.push(verification_type);
                }
                StackFrame::AppendFrame {
                    frame_type,
                    offset_delta,
                    locals,
                }
            }
            255 => {
                let offset_delta = bytes.read_u16::<BigEndian>()?;
                let number_of_locals = bytes.read_u16::<BigEndian>()?;
                let mut locals = Vec::with_capacity(number_of_locals as usize);
                for _ in 0..number_of_locals {
                    let verification_type = VerificationType::from_bytes(bytes)?;
                    locals.push(verification_type);
                }
                let number_of_stack_items = bytes.read_u16::<BigEndian>()?;
                let mut stack = Vec::with_capacity(number_of_stack_items as usize);
                for _ in 0..number_of_stack_items {
                    let verification_type = VerificationType::from_bytes(bytes)?;
                    stack.push(verification_type);
                }
                StackFrame::FullFrame {
                    frame_type,
                    offset_delta,
                    locals,
                    stack,
                }
            }
            _ => return Err(InvalidStackFrameType(frame_type)),
        };
        Ok(frame)
    }

    /// Serialize the stack frame to bytes.
    ///
    /// This method writes the stack frame to a byte stream according to the JVM specification.
    /// The format of the bytes depends on the specific variant of the stack frame.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::{StackFrame, VerificationType};
    /// use ristretto_classfile::Result;
    ///
    /// // Create a SameFrame
    /// let frame = StackFrame::SameFrame { frame_type: 25 };
    ///
    /// // Serialize to bytes
    /// let mut bytes = Vec::new();
    /// frame.to_bytes(&mut bytes)?;
    ///
    /// // Check the serialized bytes
    /// assert_eq!(bytes, vec![25]);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    ///
    /// # Errors
    ///
    /// - If the number of locals or stack items exceeds `u16::MAX`.
    /// - If a stack frame fails to serialize.
    /// - If writing to the byte stream fails.
    #[expect(clippy::match_same_arms)]
    pub fn to_bytes(&self, bytes: &mut Vec<u8>) -> Result<()> {
        match self {
            StackFrame::SameFrame { frame_type } => {
                bytes.write_u8(*frame_type)?;
            }
            StackFrame::SameLocals1StackItemFrame { frame_type, stack } => {
                bytes.write_u8(*frame_type)?;
                stack[0].to_bytes(bytes)?;
            }
            StackFrame::SameLocals1StackItemFrameExtended {
                frame_type,
                offset_delta,
                stack,
            } => {
                bytes.write_u8(*frame_type)?;
                bytes.write_u16::<BigEndian>(*offset_delta)?;
                stack[0].to_bytes(bytes)?;
            }
            StackFrame::ChopFrame {
                frame_type,
                offset_delta,
            } => {
                bytes.write_u8(*frame_type)?;
                bytes.write_u16::<BigEndian>(*offset_delta)?;
            }
            StackFrame::SameFrameExtended {
                frame_type,
                offset_delta,
            } => {
                bytes.write_u8(*frame_type)?;
                bytes.write_u16::<BigEndian>(*offset_delta)?;
            }
            StackFrame::AppendFrame {
                frame_type,
                offset_delta,
                locals,
            } => {
                bytes.write_u8(*frame_type)?;
                bytes.write_u16::<BigEndian>(*offset_delta)?;
                for verification_type in locals {
                    verification_type.to_bytes(bytes)?;
                }
            }
            StackFrame::FullFrame {
                frame_type,
                offset_delta,
                locals,
                stack,
            } => {
                bytes.write_u8(*frame_type)?;
                bytes.write_u16::<BigEndian>(*offset_delta)?;

                let locales_length = u16::try_from(locals.len())?;
                bytes.write_u16::<BigEndian>(locales_length)?;
                for verification_type in locals {
                    verification_type.to_bytes(bytes)?;
                }

                let stack_length = u16::try_from(stack.len())?;
                bytes.write_u16::<BigEndian>(stack_length)?;
                for verification_type in stack {
                    verification_type.to_bytes(bytes)?;
                }
            }
        }
        Ok(())
    }
}

impl fmt::Display for StackFrame {
    /// Formats the `StackFrame` for display purposes.
    ///
    /// This implementation provides a human-readable representation of a stack frame, showing its
    /// frame type, offset delta, and any associated stack or local variables. Each variant is
    /// formatted differently to clearly show its structure.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::{StackFrame, VerificationType};
    ///
    /// let same_frame = StackFrame::SameFrame { frame_type: 42 };
    ///
    /// let output = same_frame.to_string();
    /// assert_eq!(output, "frame_type = 42 /* same */");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StackFrame::SameFrame { frame_type } => {
                write!(f, "frame_type = {frame_type} /* same */")
            }
            StackFrame::SameLocals1StackItemFrame { frame_type, stack } => {
                writeln!(
                    f,
                    "frame_type = {frame_type} /* same_locals_1_stack_item */"
                )?;
                let stack = stack
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "  stack = [ {stack} ]")
            }
            StackFrame::SameLocals1StackItemFrameExtended {
                frame_type,
                offset_delta,
                stack,
            } => {
                writeln!(
                    f,
                    "frame_type = {frame_type} /* same_locals_1_stack_item_frame_extended */"
                )?;
                writeln!(f, "  offset_delta = {offset_delta}")?;
                let stack = stack
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "  stack = [ {stack} ]")
            }
            StackFrame::ChopFrame {
                frame_type,
                offset_delta,
            } => {
                writeln!(f, "frame_type = {frame_type} /* chop */")?;
                write!(f, "  offset_delta = {offset_delta}")
            }
            StackFrame::SameFrameExtended {
                frame_type,
                offset_delta,
            } => {
                writeln!(f, "frame_type = {frame_type} /* same_frame_extended */")?;
                write!(f, "  offset_delta = {offset_delta}")
            }
            StackFrame::AppendFrame {
                frame_type,
                offset_delta,
                locals,
            } => {
                writeln!(f, "frame_type = {frame_type} /* append */")?;
                writeln!(f, "  offset_delta = {offset_delta}")?;
                let locals = locals
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "  locals = [ {locals} ]")
            }
            StackFrame::FullFrame {
                frame_type,
                offset_delta,
                locals,
                stack,
            } => {
                writeln!(f, "frame_type = {frame_type} /* full_frame */")?;
                writeln!(f, "  offset_delta = {offset_delta}")?;
                let locals = locals
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<String>>()
                    .join(", ");
                writeln!(f, "  locals = [ {locals} ]")?;
                let stack = stack
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "  stack = [ {stack} ]")
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_invalid_stack_frame() -> Result<()> {
        let mut bytes = Vec::new();
        let frame_type = 128;
        bytes.write_u8(frame_type)?;
        let mut bytes = Cursor::new(bytes);

        assert_eq!(
            Err(InvalidStackFrameType(frame_type)),
            StackFrame::from_bytes(&mut bytes)
        );
        Ok(())
    }

    fn test_stack_frame(stack_frame: &StackFrame, expected_bytes: &[u8]) -> Result<()> {
        let mut bytes = Vec::new();
        stack_frame.to_bytes(&mut bytes)?;
        assert_eq!(expected_bytes, &bytes[..]);
        let mut bytes = Cursor::new(expected_bytes.to_vec());
        assert_eq!(*stack_frame, StackFrame::from_bytes(&mut bytes)?);
        Ok(())
    }

    #[test]
    fn test_same_frame() -> Result<()> {
        let frame_type = 42;
        let stack_frame = StackFrame::SameFrame { frame_type: 42 };
        let expected_bytes = [42];

        assert_eq!(frame_type, stack_frame.frame_type());
        assert_eq!(42, stack_frame.offset_delta());
        assert_eq!(
            indoc! {"frame_type = 42 /* same */"},
            stack_frame.to_string()
        );
        test_stack_frame(&stack_frame, &expected_bytes)
    }

    #[test]
    fn test_same_locales_1_stack_item_frame() -> Result<()> {
        let frame_type = 65;
        let stack_frame = StackFrame::SameLocals1StackItemFrame {
            frame_type,
            stack: vec![VerificationType::Null],
        };
        let expected_bytes = [65, 5];

        assert_eq!(frame_type, stack_frame.frame_type());
        assert_eq!(1, stack_frame.offset_delta());
        assert_eq!(
            indoc! {"
            frame_type = 65 /* same_locals_1_stack_item */
              stack = [ null ]"},
            stack_frame.to_string()
        );
        test_stack_frame(&stack_frame, &expected_bytes)
    }

    #[test]
    fn test_same_locales_1_stack_item_frame_extended() -> Result<()> {
        let frame_type = 247;
        let stack_frame = StackFrame::SameLocals1StackItemFrameExtended {
            frame_type,
            offset_delta: 42,
            stack: vec![VerificationType::Null],
        };
        let expected_bytes = [247, 0, 42, 5];

        assert_eq!(frame_type, stack_frame.frame_type());
        assert_eq!(42, stack_frame.offset_delta());
        assert_eq!(
            indoc! {"
            frame_type = 247 /* same_locals_1_stack_item_frame_extended */
              offset_delta = 42
              stack = [ null ]"},
            stack_frame.to_string()
        );
        test_stack_frame(&stack_frame, &expected_bytes)
    }

    #[test]
    fn test_chop_frame() -> Result<()> {
        let frame_type = 248;
        let stack_frame = StackFrame::ChopFrame {
            frame_type,
            offset_delta: 42,
        };
        let expected_bytes = [248, 0, 42];

        assert_eq!(frame_type, stack_frame.frame_type());
        assert_eq!(42, stack_frame.offset_delta());
        assert_eq!(
            indoc! {"
            frame_type = 248 /* chop */
              offset_delta = 42"},
            stack_frame.to_string()
        );
        test_stack_frame(&stack_frame, &expected_bytes)
    }

    #[test]
    fn test_same_frame_extended() -> Result<()> {
        let frame_type = 251;
        let stack_frame = StackFrame::SameFrameExtended {
            frame_type,
            offset_delta: 42,
        };
        let expected_bytes = [251, 0, 42];

        assert_eq!(frame_type, stack_frame.frame_type());
        assert_eq!(42, stack_frame.offset_delta());
        assert_eq!(
            indoc! {"
            frame_type = 251 /* same_frame_extended */
              offset_delta = 42"},
            stack_frame.to_string()
        );
        test_stack_frame(&stack_frame, &expected_bytes)
    }

    #[test]
    fn test_append_frame() -> Result<()> {
        let frame_type = 252;
        let stack_frame = StackFrame::AppendFrame {
            frame_type,
            offset_delta: 42,
            locals: vec![VerificationType::Null],
        };
        let expected_bytes = [252, 0, 42, 5];

        assert_eq!(frame_type, stack_frame.frame_type());
        assert_eq!(42, stack_frame.offset_delta());
        assert_eq!(
            indoc! {"
            frame_type = 252 /* append */
              offset_delta = 42
              locals = [ null ]"},
            stack_frame.to_string()
        );
        test_stack_frame(&stack_frame, &expected_bytes)
    }

    #[test]
    fn test_full_frame() -> Result<()> {
        let frame_type = 255;
        let stack_frame = StackFrame::FullFrame {
            frame_type,
            offset_delta: 42,
            locals: vec![VerificationType::Null],
            stack: vec![VerificationType::Integer],
        };
        let expected_bytes = [255, 0, 42, 0, 1, 5, 0, 1, 1];

        assert_eq!(frame_type, stack_frame.frame_type());
        assert_eq!(42, stack_frame.offset_delta());
        assert_eq!(
            indoc! {"\
            frame_type = 255 /* full_frame */
              offset_delta = 42
              locals = [ null ]
              stack = [ int ]"},
            stack_frame.to_string()
        );
        test_stack_frame(&stack_frame, &expected_bytes)
    }
}
