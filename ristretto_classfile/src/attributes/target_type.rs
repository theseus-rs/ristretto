use crate::Error::InvalidTargetTypeCode;
use crate::attributes::LocalVariableTarget;
use crate::error::Result;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fmt;
use std::io::Cursor;

/// Implementation of `TargetType`.
///
/// The `target_info` union in a `type_annotation` structure specifies the particular
/// entity that is annotated. The `target_type` item, which is part of various `target_info`
/// variants, indicates the kind of target.
///
/// Different kinds of targets are represented by different variants of this enum.
/// Each variant corresponds to a specific `target_type` value as defined in the
/// Java Virtual Machine Specification.
///
/// # References
///
/// - [JVMS §4.7.20.1](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.20.1)
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TargetType {
    /// `target_type` value `0x00`.
    /// Identifies a type parameter declaration of a generic class or interface.
    ///
    /// - `type_parameter_index`: The index of the type parameter.
    TypeParameter {
        target_type: u8,
        type_parameter_index: u8,
    },
    /// `target_type` value `0x10`.
    /// Identifies a type in the `extends` or `implements` clause of a class or interface declaration.
    ///
    /// - `supertype_index`: An index into the `interfaces` array of the `ClassFile` structure,
    ///   or `0xFFFF` if the supertype is the `extends` clause.
    SuperType {
        target_type: u8,
        supertype_index: u16,
    },
    /// `target_type` values `0x11` or `0x12`.
    /// Identifies a type in a bound of a type parameter declaration of a generic class, interface, method, or constructor.
    ///
    /// - `type_parameter_index`: The index of the type parameter.
    /// - `bound_index`: The index of the bound.
    TypeParameterBound {
        target_type: u8,
        type_parameter_index: u8,
        bound_index: u8,
    },
    /// `target_type` values `0x13`, `0x14`, or `0x15`.
    /// Identifies a type in a field declaration, a return type of a method, or a receiver type.
    /// This target type does not have additional data.
    Empty { target_type: u8 },
    /// `target_type` value `0x16`.
    /// Identifies a type in a formal parameter declaration of a method or constructor.
    ///
    /// - `formal_parameter_index`: The index of the formal parameter.
    FormalParameter {
        target_type: u8,
        formal_parameter_index: u8,
    },
    /// `target_type` value `0x17`.
    /// Identifies a type in the `throws` clause of a method or constructor.
    ///
    /// - `throws_type_index`: An index into the `exception_table` of the `Code` attribute,
    ///   or an index into the `exception_index_table` of the `Exceptions` attribute.
    Throws {
        target_type: u8,
        throws_type_index: u16,
    },
    /// `target_type` values `0x40` or `0x41`.
    /// Identifies a type in a local variable declaration.
    ///
    /// - `local_variable_targets`: A table of `LocalVariableTarget` entries.
    LocalVar {
        target_type: u8,
        local_variable_targets: Vec<LocalVariableTarget>,
    },
    /// `target_type` value `0x42`.
    /// Identifies a type in an exception parameter declaration.
    ///
    /// - `exception_table_index`: An index into the `exception_table` of the `Code` attribute.
    Catch {
        target_type: u8,
        exception_table_index: u16,
    },
    /// `target_type` values `0x43`, `0x44`, `0x45`, or `0x46`.
    /// Identifies a type in an `instanceof` expression, a `new` expression, a method reference expression (`::new`),
    /// or a method reference expression (`::identifier`).
    ///
    /// - `offset`: The bytecode offset of the instruction.
    Offset { target_type: u8, offset: u16 },
    /// `target_type` values `0x47`, `0x48`, `0x49`, `0x4A`, or `0x4B`.
    /// Identifies a type argument in a cast expression, a type argument of a generic method invocation,
    /// a type argument in a `new` expression, a type argument in a method reference expression (`::new`),
    /// or a type argument in a method reference expression (`::identifier`).
    ///
    /// - `offset`: The bytecode offset of the instruction.
    /// - `type_argument_index`: The index of the type argument.
    TypeArgument {
        target_type: u8,
        offset: u16,
        type_argument_index: u8,
    },
}

impl TargetType {
    /// Return the raw `target_type` byte value.
    ///
    /// This value indicates the kind of target being annotated, as defined in the JVM specification.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::TargetType;
    ///
    /// let target = TargetType::TypeParameter { target_type: 0x00, type_parameter_index: 1 };
    /// assert_eq!(target.target_type(), 0x00);
    ///
    /// let target_empty = TargetType::Empty { target_type: 0x13 };
    /// assert_eq!(target_empty.target_type(), 0x13);
    /// ```
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

    /// Deserialize the `TargetType` from a byte stream.
    ///
    /// Reads the `target_type` byte and then, based on its value, reads the
    /// subsequent bytes to construct the appropriate `TargetType` variant.
    ///
    /// # Errors
    ///
    /// Returns `Err(Error::InvalidTargetTypeCode(code))` if the read `target_type`
    /// byte does not correspond to any known target type.
    /// Returns `Err(Error::Io(io_error))` if an I/O error occurs during reading.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::TargetType;
    /// use std::io::Cursor;
    ///
    /// // Bytes for a TypeParameter target_type = 0x00, type_parameter_index = 5
    /// let mut bytes = Cursor::new(vec![0x00, 0x05]);
    /// let target_type = TargetType::from_bytes(&mut bytes)?;
    /// assert_eq!(target_type, TargetType::TypeParameter { target_type: 0x00, type_parameter_index: 5 });
    ///
    /// // Bytes for an Empty target_type = 0x13
    /// let mut bytes_empty = Cursor::new(vec![0x13]);
    /// let target_type_empty = TargetType::from_bytes(&mut bytes_empty)?;
    /// assert_eq!(target_type_empty, TargetType::Empty { target_type: 0x13 });
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    pub fn from_bytes(bytes: &mut Cursor<impl AsRef<[u8]>>) -> Result<TargetType> {
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

    /// Serialize the `TargetType` to a byte vector.
    ///
    /// Writes the `target_type` byte and then, based on the variant, writes
    /// the subsequent bytes representing the specific target information.
    ///
    /// # Errors
    ///
    /// Returns `Err(Error::Io(io_error))` if an I/O error occurs during writing.
    /// Can also return `Err(Error::TryFromIntError)` if a length (e.g., number of local variable targets)
    /// exceeds `u16::MAX`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::TargetType;
    /// use ristretto_classfile::attributes::LocalVariableTarget;
    ///
    /// let target = TargetType::SuperType { target_type: 0x10, supertype_index: 123 };
    /// let mut bytes = Vec::new();
    /// target.to_bytes(&mut bytes)?;
    /// assert_eq!(bytes, vec![0x10, 0x00, 0x7B]);
    ///
    /// let local_var_target = TargetType::LocalVar {
    ///     target_type: 0x40,
    ///     local_variable_targets: vec![
    ///         LocalVariableTarget { start_pc: 10, length: 5, index: 1 }
    ///     ]
    /// };
    /// let mut bytes_local_var = Vec::new();
    /// local_var_target.to_bytes(&mut bytes_local_var)?;
    /// assert_eq!(bytes_local_var, vec![
    ///     0x40, // target_type
    ///     0x00, 0x01, // table_length = 1
    ///     0x00, 0x0A, // start_pc = 10
    ///     0x00, 0x05, // length = 5
    ///     0x00, 0x01  // index = 1
    /// ]);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
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
    /// Implements the `Display` trait for `TargetType`.
    ///
    /// Provides a human-readable string representation of a `TargetType` value, showing the variant
    /// type and all of its fields.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::TargetType;
    ///
    /// let target = TargetType::SuperType { target_type: 0x10, supertype_index: 5 };
    ///
    /// let output = target.to_string();
    /// assert_eq!(
    ///     output,
    ///     "SuperType[target_type=16, supertype_index=5]"
    /// );
    /// ```
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
            } => write!(f, "Offset[target_type={target_type}, offset={offset}]",),
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

        assert_eq!(
            "LocalVar[target_type=64, local_variable_targets=[LocalVariableTarget { start_pc: 1, length: 2, index: 3 }]]",
            target_type.to_string()
        );
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
