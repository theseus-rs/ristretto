use crate::attributes::LocalVariableTarget;
use crate::error::Result;
use crate::Error::InvalidTargetTypeCode;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fmt;
use std::io::Cursor;

/// Implementation of `TargetType`.
///
/// See: <https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.7.20>
#[derive(Clone, Debug, PartialEq)]
pub enum TargetType {
    TypeParameter {
        target_type: u8,
        type_parameter_index: u8,
    },
    SuperType {
        target_type: u8,
        supertype_index: u16,
    },
    TypeParameterBound {
        target_type: u8,
        type_parameter_index: u8,
        bound_index: u8,
    },
    Empty {
        target_type: u8,
    },
    FormalParameter {
        target_type: u8,
        formal_parameter_index: u8,
    },
    Throws {
        target_type: u8,
        throws_type_index: u16,
    },
    LocalVar {
        target_type: u8,
        local_variable_targets: Vec<LocalVariableTarget>,
    },
    Catch {
        target_type: u8,
        exception_table_index: u16,
    },
    Offset {
        target_type: u8,
        offset: u16,
    },
    TypeArgument {
        target_type: u8,
        offset: u16,
        type_argument_index: u8,
    },
}

impl TargetType {
    /// Return the target type.
    #[must_use]
    pub fn target_type(&self) -> u8 {
        match self {
            TargetType::TypeParameter { target_type, .. }
            | TargetType::SuperType { target_type, .. }
            | TargetType::TypeParameterBound { target_type, .. }
            | TargetType::Empty { target_type }
            | TargetType::FormalParameter { target_type, .. }
            | TargetType::Throws { target_type, .. }
            | TargetType::LocalVar { target_type, .. }
            | TargetType::Catch { target_type, .. }
            | TargetType::Offset { target_type, .. }
            | TargetType::TypeArgument { target_type, .. } => *target_type,
        }
    }

    /// Deserialize the `TargetType` from bytes.
    ///
    /// # Errors
    /// If the target type is not a valid `TargetType`.
    pub fn from_bytes(bytes: &mut Cursor<Vec<u8>>) -> Result<TargetType> {
        let target_type = bytes.read_u8()?;

        let array_type = match target_type {
            0..=1 => {
                // 0x00 | 0x01
                let type_parameter_index = bytes.read_u8()?;
                TargetType::TypeParameter {
                    target_type,
                    type_parameter_index,
                }
            }
            16 => {
                // 0x10
                let supertype_index = bytes.read_u16::<BigEndian>()?;
                TargetType::SuperType {
                    target_type,
                    supertype_index,
                }
            }
            17..=18 => {
                // 0x11 | 0x12
                let type_parameter_index = bytes.read_u8()?;
                let bound_index = bytes.read_u8()?;
                TargetType::TypeParameterBound {
                    target_type,
                    type_parameter_index,
                    bound_index,
                }
            }
            19..=21 => TargetType::Empty { target_type }, // 0x13 | 0x14 | 0x15
            22 => {
                // 0x16
                let formal_parameter_index = bytes.read_u8()?;
                TargetType::FormalParameter {
                    target_type,
                    formal_parameter_index,
                }
            }
            23 => {
                // 0x17
                let throws_type_index = bytes.read_u16::<BigEndian>()?;
                TargetType::Throws {
                    target_type,
                    throws_type_index,
                }
            }
            64..=65 => {
                // 0x40 | 0x41
                let targets_count = bytes.read_u16::<BigEndian>()? as usize;
                let mut targets = Vec::with_capacity(targets_count);
                for _ in 0..targets_count {
                    let target = LocalVariableTarget::from_bytes(bytes)?;
                    targets.push(target);
                }

                TargetType::LocalVar {
                    target_type,
                    local_variable_targets: targets,
                }
            }
            66 => {
                // 0x42
                let exception_table_index = bytes.read_u16::<BigEndian>()?;
                TargetType::Catch {
                    target_type,
                    exception_table_index,
                }
            }
            67..=70 => {
                // 0x43 | 0x44 | 0x45 | 0x46
                let offset = bytes.read_u16::<BigEndian>()?;
                TargetType::Offset {
                    target_type,
                    offset,
                }
            }
            71..=75 => {
                // 0x47 | 0x48 | 0x49 | 0x4A | 0x4B
                let offset = bytes.read_u16::<BigEndian>()?;
                let type_argument_index = bytes.read_u8()?;
                TargetType::TypeArgument {
                    target_type,
                    offset,
                    type_argument_index,
                }
            }
            _ => return Err(InvalidTargetTypeCode(target_type)),
        };
        Ok(array_type)
    }

    /// Serialize the `TargetType` to bytes.
    ///
    /// # Errors
    /// Should not occur; reserved for future use.
    pub fn to_bytes(&self, bytes: &mut Vec<u8>) -> Result<()> {
        match self {
            TargetType::TypeParameter {
                target_type,
                type_parameter_index,
            } => {
                bytes.write_u8(*target_type)?;
                bytes.write_u8(*type_parameter_index)?;
            }
            TargetType::SuperType {
                target_type,
                supertype_index,
            } => {
                bytes.write_u8(*target_type)?;
                bytes.write_u16::<BigEndian>(*supertype_index)?;
            }
            TargetType::TypeParameterBound {
                target_type,
                type_parameter_index,
                bound_index,
            } => {
                bytes.write_u8(*target_type)?;
                bytes.write_u8(*type_parameter_index)?;
                bytes.write_u8(*bound_index)?;
            }
            TargetType::Empty { target_type } => {
                bytes.write_u8(*target_type)?;
            }
            TargetType::FormalParameter {
                target_type,
                formal_parameter_index,
            } => {
                bytes.write_u8(*target_type)?;
                bytes.write_u8(*formal_parameter_index)?;
            }
            TargetType::Throws {
                target_type,
                throws_type_index,
            } => {
                bytes.write_u8(*target_type)?;
                bytes.write_u16::<BigEndian>(*throws_type_index)?;
            }
            TargetType::LocalVar {
                target_type,
                local_variable_targets,
            } => {
                bytes.write_u8(*target_type)?;
                let targets_count = u16::try_from(local_variable_targets.len())?;
                bytes.write_u16::<BigEndian>(targets_count)?;
                for target in local_variable_targets {
                    target.to_bytes(bytes)?;
                }
            }
            TargetType::Catch {
                target_type,
                exception_table_index,
            } => {
                bytes.write_u8(*target_type)?;
                bytes.write_u16::<BigEndian>(*exception_table_index)?;
            }
            TargetType::Offset {
                target_type,
                offset,
            } => {
                bytes.write_u8(*target_type)?;
                bytes.write_u16::<BigEndian>(*offset)?;
            }
            TargetType::TypeArgument {
                target_type,
                offset,
                type_argument_index,
            } => {
                bytes.write_u8(*target_type)?;
                bytes.write_u16::<BigEndian>(*offset)?;
                bytes.write_u8(*type_argument_index)?;
            }
        }
        Ok(())
    }
}

impl fmt::Display for TargetType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TargetType::TypeParameter {
                target_type,
                type_parameter_index,
            } => write!(
                f,
                "TypeParameter[target_type={target_type}, type_parameter_index={type_parameter_index}]",
            ),
            TargetType::SuperType {
                target_type,
                supertype_index,
            } => write!(
                f,
                "SuperType[target_type={target_type}, supertype_index={supertype_index}]",
            ),
            TargetType::TypeParameterBound {
                target_type,
                type_parameter_index,
                bound_index,
            } => write!(
                f,
                "TypeParameterBound[target_type={target_type}, type_parameter_index={type_parameter_index}, bound_index={bound_index}]",
            ),
            TargetType::Empty { target_type } => {
                write!(f, "Empty[target_type={target_type}]")
            }
            TargetType::FormalParameter {
                target_type,
                formal_parameter_index,
            } => write!(
                f,
                "FormalParameter[target_type={target_type}, formal_parameter_index={formal_parameter_index}]",
            ),
            TargetType::Throws {
                target_type,
                throws_type_index,
            } => write!(
                f,
                "Throws[target_type={target_type}, throws_type_index={throws_type_index}]",
            ),
            TargetType::LocalVar {
                target_type,
                local_variable_targets,
            } => write!(
                f,
                "LocalVar[target_type={target_type}, local_variable_targets={local_variable_targets:?}]",
            ),
            TargetType::Catch {
                target_type,
                exception_table_index,
            } => write!(
                f,
                "Catch[target_type={target_type}, exception_table_index={exception_table_index}]",
            ),
            TargetType::Offset {
                target_type,
                offset,
            } => write!(
                f,
                "Offset[target_type={target_type}, offset={offset}]",
            ),
            TargetType::TypeArgument {
                target_type,
                offset,
                type_argument_index,
            } => write!(
                f,
                "TypeArgument[target_type={target_type}, offset={offset}, type_argument_index={type_argument_index}]",
            ),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_invalid_code() {
        let mut bytes = Cursor::new(vec![255]);
        assert_eq!(
            Err(InvalidTargetTypeCode(255)),
            TargetType::from_bytes(&mut bytes)
        );
    }

    fn test_array_type(target_type: &TargetType, expected_bytes: &[u8]) -> Result<()> {
        assert_eq!(target_type.target_type(), expected_bytes[0]);
        let mut bytes = Vec::new();
        target_type.to_bytes(&mut bytes)?;
        assert_eq!(expected_bytes, &bytes[..]);
        let mut bytes = Cursor::new(expected_bytes.to_vec());
        assert_eq!(*target_type, TargetType::from_bytes(&mut bytes)?);
        Ok(())
    }

    #[test]
    fn test_type_parameter() -> Result<()> {
        let target_type = TargetType::TypeParameter {
            target_type: 0,
            type_parameter_index: 42,
        };

        assert_eq!(
            "TypeParameter[target_type=0, type_parameter_index=42]",
            target_type.to_string()
        );
        test_array_type(&target_type, &[0, 42])
    }

    #[test]
    fn test_super_type() -> Result<()> {
        let target_type = TargetType::SuperType {
            target_type: 16,
            supertype_index: 42,
        };

        assert_eq!(
            "SuperType[target_type=16, supertype_index=42]",
            target_type.to_string()
        );
        test_array_type(&target_type, &[16, 0, 42])
    }

    #[test]
    fn test_type_parameter_bound() -> Result<()> {
        let target_type = TargetType::TypeParameterBound {
            target_type: 17,
            type_parameter_index: 1,
            bound_index: 42,
        };

        assert_eq!(
            "TypeParameterBound[target_type=17, type_parameter_index=1, bound_index=42]",
            target_type.to_string()
        );
        test_array_type(&target_type, &[17, 1, 42])
    }

    #[test]
    fn test_empty() -> Result<()> {
        let target_type = TargetType::Empty { target_type: 19 };

        assert_eq!("Empty[target_type=19]", target_type.to_string());
        test_array_type(&target_type, &[19])
    }

    #[test]
    fn test_formal_parameter() -> Result<()> {
        let target_type = TargetType::FormalParameter {
            target_type: 22,
            formal_parameter_index: 42,
        };

        assert_eq!(
            "FormalParameter[target_type=22, formal_parameter_index=42]",
            target_type.to_string()
        );
        test_array_type(&target_type, &[22, 42])
    }

    #[test]
    fn test_throws() -> Result<()> {
        let target_type = TargetType::Throws {
            target_type: 23,
            throws_type_index: 42,
        };

        assert_eq!(
            "Throws[target_type=23, throws_type_index=42]",
            target_type.to_string()
        );
        test_array_type(&target_type, &[23, 0, 42])
    }

    #[test]
    fn test_local_var() -> Result<()> {
        let local_variable_targets = vec![LocalVariableTarget {
            start_pc: 1,
            length: 2,
            index: 3,
        }];
        let target_type = TargetType::LocalVar {
            target_type: 64,
            local_variable_targets,
        };

        assert_eq!("LocalVar[target_type=64, local_variable_targets=[LocalVariableTarget { start_pc: 1, length: 2, index: 3 }]]", target_type.to_string());
        test_array_type(&target_type, &[64, 0, 1, 0, 1, 0, 2, 0, 3])
    }

    #[test]
    fn test_catch() -> Result<()> {
        let target_type = TargetType::Catch {
            target_type: 66,
            exception_table_index: 42,
        };

        assert_eq!(
            "Catch[target_type=66, exception_table_index=42]",
            target_type.to_string()
        );
        test_array_type(&target_type, &[66, 0, 42])
    }

    #[test]
    fn test_offset() -> Result<()> {
        let target_type = TargetType::Offset {
            target_type: 67,
            offset: 42,
        };

        assert_eq!("Offset[target_type=67, offset=42]", target_type.to_string());
        test_array_type(&target_type, &[67, 0, 42])
    }

    #[test]
    fn test_type_argument() -> Result<()> {
        let target_type = TargetType::TypeArgument {
            target_type: 71,
            offset: 1,
            type_argument_index: 42,
        };

        assert_eq!(
            "TypeArgument[target_type=71, offset=1, type_argument_index=42]",
            target_type.to_string()
        );
        test_array_type(&target_type, &[71, 0, 1, 42])
    }
}
