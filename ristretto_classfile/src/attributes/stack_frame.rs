use crate::attributes::VerificationType;
use crate::error::Error::InvalidStackFrameType;
use crate::error::Result;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fmt;
use std::io::Cursor;

/// Implementation of `StackFrame`.
///
/// See: <https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.7.4>
#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq)]
pub enum StackFrame {
    SameFrame {
        frame_type: u8,
    },
    SameLocals1StackItemFrame {
        frame_type: u8,
        stack: Vec<VerificationType>,
    },
    SameLocals1StackItemFrameExtended {
        frame_type: u8,
        offset_delta: u16,
        stack: Vec<VerificationType>,
    },
    ChopFrame {
        frame_type: u8,
        offset_delta: u16,
    },
    SameFrameExtended {
        frame_type: u8,
        offset_delta: u16,
    },
    AppendFrame {
        frame_type: u8,
        offset_delta: u16,
        locals: Vec<VerificationType>,
    },
    FullFrame {
        frame_type: u8,
        offset_delta: u16,
        locals: Vec<VerificationType>,
        stack: Vec<VerificationType>,
    },
}

impl StackFrame {
    /// Deserialize the stack frame from bytes.
    ///
    /// # Errors
    /// Returns an error if the frame type is invalid.
    pub fn from_bytes(bytes: &mut Cursor<Vec<u8>>) -> Result<StackFrame> {
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
    /// # Errors
    /// - If the number of locals or stack items exceeds 65,534.
    /// - If a stack frame fails to serialize.
    #[allow(clippy::match_same_arms)]
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StackFrame::SameFrame { frame_type } => {
                write!(f, "SameFrame[frame_type={frame_type}]")
            }
            StackFrame::SameLocals1StackItemFrame { frame_type, stack } => {
                write!(
                    f,
                    "SameLocals1StackItemFrame[frame_type={frame_type}, stack={stack:?}]",
                )
            }
            StackFrame::SameLocals1StackItemFrameExtended {
                frame_type,
                offset_delta,
                stack,
            } => {
                write!(
                    f,
                    "SameLocals1StackItemFrameExtended[frame_type={frame_type}, offset_delta={offset_delta}, stack={stack:?}]",
                )
            }
            StackFrame::ChopFrame {
                frame_type,
                offset_delta,
            } => {
                write!(
                    f,
                    "ChopFrame[frame_type={frame_type}, offset_delta={offset_delta}]",
                )
            }
            StackFrame::SameFrameExtended {
                frame_type,
                offset_delta,
            } => {
                write!(
                    f,
                    "SameFrameExtended[frame_type={frame_type}, offset_delta={offset_delta}]",
                )
            }
            StackFrame::AppendFrame {
                frame_type,
                offset_delta,
                locals,
            } => {
                write!(
                    f,
                    "AppendFrame[frame_type={frame_type}, offset_delta={offset_delta}, locals={locals:?}]",
                )
            }
            StackFrame::FullFrame {
                frame_type,
                offset_delta,
                locals,
                stack,
            } => {
                write!(
                    f,
                    "FullFrame[frame_type={frame_type}, offset_delta={offset_delta}, locals={locals:?}, stack={stack:?}]",
                )
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

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
        let stack_frame = StackFrame::SameFrame { frame_type: 0 };
        let expected_bytes = [0];

        assert_eq!("SameFrame[frame_type=0]", stack_frame.to_string());
        test_stack_frame(&stack_frame, &expected_bytes)
    }

    #[test]
    fn test_same_locales_1_stack_item_frame() -> Result<()> {
        let stack_frame = StackFrame::SameLocals1StackItemFrame {
            frame_type: 64,
            stack: vec![VerificationType::Null],
        };
        let expected_bytes = [64, 5];

        assert_eq!(
            "SameLocals1StackItemFrame[frame_type=64, stack=[Null]]",
            stack_frame.to_string()
        );
        test_stack_frame(&stack_frame, &expected_bytes)
    }

    #[test]
    fn test_same_locales_1_stack_item_frame_extended() -> Result<()> {
        let stack_frame = StackFrame::SameLocals1StackItemFrameExtended {
            frame_type: 247,
            offset_delta: 42,
            stack: vec![VerificationType::Null],
        };
        let expected_bytes = [247, 0, 42, 5];

        assert_eq!(
            "SameLocals1StackItemFrameExtended[frame_type=247, offset_delta=42, stack=[Null]]",
            stack_frame.to_string()
        );
        test_stack_frame(&stack_frame, &expected_bytes)
    }

    #[test]
    fn test_chop_frame() -> Result<()> {
        let stack_frame = StackFrame::ChopFrame {
            frame_type: 248,
            offset_delta: 42,
        };
        let expected_bytes = [248, 0, 42];

        assert_eq!(
            "ChopFrame[frame_type=248, offset_delta=42]",
            stack_frame.to_string()
        );
        test_stack_frame(&stack_frame, &expected_bytes)
    }

    #[test]
    fn test_same_frame_extended() -> Result<()> {
        let stack_frame = StackFrame::SameFrameExtended {
            frame_type: 251,
            offset_delta: 42,
        };
        let expected_bytes = [251, 0, 42];

        assert_eq!(
            "SameFrameExtended[frame_type=251, offset_delta=42]",
            stack_frame.to_string()
        );
        test_stack_frame(&stack_frame, &expected_bytes)
    }

    #[test]
    fn test_append_frame() -> Result<()> {
        let stack_frame = StackFrame::AppendFrame {
            frame_type: 252,
            offset_delta: 42,
            locals: vec![VerificationType::Null],
        };
        let expected_bytes = [252, 0, 42, 5];

        assert_eq!(
            "AppendFrame[frame_type=252, offset_delta=42, locals=[Null]]",
            stack_frame.to_string()
        );
        test_stack_frame(&stack_frame, &expected_bytes)
    }

    #[test]
    fn test_full_frame() -> Result<()> {
        let stack_frame = StackFrame::FullFrame {
            frame_type: 255,
            offset_delta: 42,
            locals: vec![VerificationType::Null],
            stack: vec![VerificationType::Integer],
        };
        let expected_bytes = [255, 0, 42, 0, 1, 5, 0, 1, 1];

        assert_eq!(
            "FullFrame[frame_type=255, offset_delta=42, locals=[Null], stack=[Integer]]",
            stack_frame.to_string()
        );
        test_stack_frame(&stack_frame, &expected_bytes)
    }
}
