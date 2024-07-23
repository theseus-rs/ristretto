use crate::error::Error::InvalidConstantTag;
use crate::error::Result;
use crate::mutf8;
use crate::reference_kind::ReferenceKind;
use crate::version::Version;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fmt;
use std::io::{Cursor, Read};

const VERSION_45_0: Version = Version::Java1_0_2 { minor: 0 };
const VERSION_45_3: Version = Version::Java1_0_2 { minor: 3 };
const VERSION_51_0: Version = Version::Java7 { minor: 0 };
const VERSION_53_0: Version = Version::Java9 { minor: 0 };
const VERSION_55_0: Version = Version::Java11 { minor: 0 };

/// Constant
///
/// See: <https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.4>
#[derive(Clone, Debug, PartialEq)]
pub enum Constant {
    Utf8(String),
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    Class(u16),  // Name index (Utf8)
    String(u16), // String index (Utf8)
    FieldRef {
        class_index: u16,
        name_and_type_index: u16,
    },
    MethodRef {
        class_index: u16,
        name_and_type_index: u16,
    },
    InterfaceMethodRef {
        class_index: u16,
        name_and_type_index: u16,
    },
    NameAndType {
        name_index: u16,
        descriptor_index: u16,
    },
    MethodHandle {
        reference_kind: ReferenceKind,
        reference_index: u16,
    },
    MethodType(u16), // Descriptor index (NameAndType)
    Dynamic {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    },
    InvokeDynamic {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    },
    Module(u16),  // Name index (Utf8)
    Package(u16), // Name index (Utf8)
}

impl Constant {
    /// Get the tag of the `Constant`.
    #[must_use]
    pub fn tag(&self) -> u8 {
        match self {
            Constant::Utf8(_) => 1,
            Constant::Integer(_) => 3,
            Constant::Float(_) => 4,
            Constant::Long(_) => 5,
            Constant::Double(_) => 6,
            Constant::Class { .. } => 7,
            Constant::String { .. } => 8,
            Constant::FieldRef { .. } => 9,
            Constant::MethodRef { .. } => 10,
            Constant::InterfaceMethodRef { .. } => 11,
            Constant::NameAndType { .. } => 12,
            Constant::MethodHandle { .. } => 15,
            Constant::MethodType { .. } => 16,
            Constant::Dynamic { .. } => 17,
            Constant::InvokeDynamic { .. } => 18,
            Constant::Module { .. } => 19,
            Constant::Package { .. } => 20,
        }
    }

    /// Check if the `Constant` is valid for the given version.
    #[must_use]
    pub fn valid_for_version(&self, version: &Version) -> bool {
        match self {
            Constant::MethodHandle { .. }
            | Constant::MethodType { .. }
            | Constant::InvokeDynamic { .. } => *version >= VERSION_51_0,
            Constant::Module { .. } | Constant::Package { .. } => *version >= VERSION_53_0,
            Constant::Dynamic { .. } => *version >= VERSION_55_0,
            _ => *version >= VERSION_45_3,
        }
    }

    /// Deserialize the Constant from bytes.
    ///
    /// # Errors
    /// Returns an error if the tag is invalid.
    pub fn from_bytes(bytes: &mut Cursor<Vec<u8>>) -> Result<Constant> {
        let tag = bytes.read_u8()?;
        let constant = match tag {
            1 => {
                let length = bytes.read_u16::<BigEndian>()? as usize;
                let mut utf8_bytes = vec![0; length];
                bytes.read_exact(&mut utf8_bytes)?;
                let string = mutf8::from_bytes(utf8_bytes.as_slice())?;
                Constant::Utf8(string)
            }
            3 => Constant::Integer(bytes.read_i32::<BigEndian>()?),
            4 => Constant::Float(bytes.read_f32::<BigEndian>()?),
            5 => Constant::Long(bytes.read_i64::<BigEndian>()?),
            6 => Constant::Double(bytes.read_f64::<BigEndian>()?),
            7 => Constant::Class(bytes.read_u16::<BigEndian>()?),
            8 => Constant::String(bytes.read_u16::<BigEndian>()?),
            9 => Constant::FieldRef {
                class_index: bytes.read_u16::<BigEndian>()?,
                name_and_type_index: bytes.read_u16::<BigEndian>()?,
            },
            10 => Constant::MethodRef {
                class_index: bytes.read_u16::<BigEndian>()?,
                name_and_type_index: bytes.read_u16::<BigEndian>()?,
            },
            11 => Constant::InterfaceMethodRef {
                class_index: bytes.read_u16::<BigEndian>()?,
                name_and_type_index: bytes.read_u16::<BigEndian>()?,
            },
            12 => Constant::NameAndType {
                name_index: bytes.read_u16::<BigEndian>()?,
                descriptor_index: bytes.read_u16::<BigEndian>()?,
            },
            15 => Constant::MethodHandle {
                reference_kind: ReferenceKind::from_bytes(bytes)?,
                reference_index: bytes.read_u16::<BigEndian>()?,
            },
            16 => Constant::MethodType(bytes.read_u16::<BigEndian>()?),
            17 => Constant::Dynamic {
                bootstrap_method_attr_index: bytes.read_u16::<BigEndian>()?,
                name_and_type_index: bytes.read_u16::<BigEndian>()?,
            },
            18 => Constant::InvokeDynamic {
                bootstrap_method_attr_index: bytes.read_u16::<BigEndian>()?,
                name_and_type_index: bytes.read_u16::<BigEndian>()?,
            },
            19 => Constant::Module(bytes.read_u16::<BigEndian>()?),
            20 => Constant::Package(bytes.read_u16::<BigEndian>()?),
            _ => return Err(InvalidConstantTag(tag)),
        };
        Ok(constant)
    }

    /// Serialize the `Constant` to bytes.
    ///
    /// # Errors
    /// If a UTF-8 string is more than 65535 bytes long.
    #[allow(clippy::match_same_arms)]
    pub fn to_bytes(&self, bytes: &mut Vec<u8>) -> Result<()> {
        bytes.write_u8(self.tag())?;

        match self {
            Constant::Utf8(value) => {
                let utf8_value = mutf8::to_bytes(value)?;
                let length = u16::try_from(utf8_value.len())?;
                bytes.write_u16::<BigEndian>(length)?;
                bytes.extend_from_slice(utf8_value.as_ref());
            }
            Constant::Integer(value) => bytes.write_i32::<BigEndian>(*value)?,
            Constant::Float(value) => bytes.write_f32::<BigEndian>(*value)?,
            Constant::Long(value) => bytes.write_i64::<BigEndian>(*value)?,
            Constant::Double(value) => bytes.write_f64::<BigEndian>(*value)?,
            Constant::Class(name_index) => bytes.write_u16::<BigEndian>(*name_index)?,
            Constant::String(string_index) => bytes.write_u16::<BigEndian>(*string_index)?,
            Constant::FieldRef {
                class_index,
                name_and_type_index,
            } => {
                bytes.write_u16::<BigEndian>(*class_index)?;
                bytes.write_u16::<BigEndian>(*name_and_type_index)?;
            }
            Constant::MethodRef {
                class_index,
                name_and_type_index,
            } => {
                bytes.write_u16::<BigEndian>(*class_index)?;
                bytes.write_u16::<BigEndian>(*name_and_type_index)?;
            }
            Constant::InterfaceMethodRef {
                class_index,
                name_and_type_index,
            } => {
                bytes.write_u16::<BigEndian>(*class_index)?;
                bytes.write_u16::<BigEndian>(*name_and_type_index)?;
            }
            Constant::NameAndType {
                name_index,
                descriptor_index,
            } => {
                bytes.write_u16::<BigEndian>(*name_index)?;
                bytes.write_u16::<BigEndian>(*descriptor_index)?;
            }
            Constant::MethodHandle {
                reference_kind,
                reference_index,
            } => {
                reference_kind.to_bytes(bytes)?;
                bytes.write_u16::<BigEndian>(*reference_index)?;
            }
            Constant::MethodType(descriptor_index) => {
                bytes.write_u16::<BigEndian>(*descriptor_index)?;
            }
            Constant::Dynamic {
                bootstrap_method_attr_index,
                name_and_type_index,
            } => {
                bytes.write_u16::<BigEndian>(*bootstrap_method_attr_index)?;
                bytes.write_u16::<BigEndian>(*name_and_type_index)?;
            }
            Constant::InvokeDynamic {
                bootstrap_method_attr_index,
                name_and_type_index,
            } => {
                bytes.write_u16::<BigEndian>(*bootstrap_method_attr_index)?;
                bytes.write_u16::<BigEndian>(*name_and_type_index)?;
            }
            Constant::Module(name_index) => bytes.write_u16::<BigEndian>(*name_index)?,
            Constant::Package(name_index) => bytes.write_u16::<BigEndian>(*name_index)?,
        }

        Ok(())
    }
}

impl fmt::Display for Constant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Constant::Utf8(value) => write!(f, "Utf8 {value}"),
            Constant::Integer(value) => write!(f, "Integer {value}"),
            Constant::Float(value) => write!(f, "Float {value}"),
            Constant::Long(value) => write!(f, "Long {value}"),
            Constant::Double(value) => write!(f, "Double {value}"),
            Constant::Class(name_index) => write!(f, "Class #{name_index}"),
            Constant::String(string_index) => write!(f, "String #{string_index}"),
            Constant::FieldRef {
                class_index,
                name_and_type_index,
            } => write!(f, "Fieldref #{class_index}.#{name_and_type_index}"),
            Constant::MethodRef {
                class_index,
                name_and_type_index,
            } => write!(f, "Methodref #{class_index}.#{name_and_type_index}"),
            Constant::InterfaceMethodRef {
                class_index,
                name_and_type_index,
            } => write!(
                f,
                "InterfaceMethodRef #{class_index}.#{name_and_type_index}"
            ),
            Constant::NameAndType {
                name_index,
                descriptor_index,
            } => write!(f, "NameAndType #{name_index}:#{descriptor_index}"),
            Constant::MethodHandle {
                reference_kind,
                reference_index,
            } => write!(f, "MethodHandle {reference_kind}.#{reference_index}"),
            Constant::MethodType(descriptor_index) => {
                write!(f, "MethodType #{descriptor_index}")
            }
            Constant::Dynamic {
                bootstrap_method_attr_index,
                name_and_type_index,
            } => write!(
                f,
                "Dynamic #{bootstrap_method_attr_index}.#{name_and_type_index}"
            ),
            Constant::InvokeDynamic {
                bootstrap_method_attr_index,
                name_and_type_index,
            } => write!(
                f,
                "InvokeDynamic #{bootstrap_method_attr_index}.#{name_and_type_index}"
            ),
            Constant::Module(name_index) => write!(f, "Module #{name_index}"),
            Constant::Package(name_index) => write!(f, "Package #{name_index}"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_invalid_tag() {
        let mut bytes = Cursor::new(vec![0]);
        assert_eq!(Err(InvalidConstantTag(0)), Constant::from_bytes(&mut bytes));
    }

    fn test_constant(
        constant: &Constant,
        expected_bytes: &[u8],
        tag: u8,
        supported_versions: &Version,
    ) -> Result<()> {
        assert_eq!(tag, constant.tag());
        assert!(constant.valid_for_version(supported_versions));
        assert!(!constant.valid_for_version(&VERSION_45_0));

        let mut bytes = Vec::new();
        constant.to_bytes(&mut bytes)?;
        assert_eq!(expected_bytes, &bytes[..]);
        let mut bytes = Cursor::new(expected_bytes.to_vec());
        assert_eq!(*constant, Constant::from_bytes(&mut bytes)?);
        Ok(())
    }

    #[test]
    fn test_utf8() -> Result<()> {
        let constant = Constant::Utf8("foo".to_string());
        let expected_bytes = [1, 0, 3, 102, 111, 111];

        assert_eq!("Utf8 foo", constant.to_string());
        test_constant(&constant, &expected_bytes, 1, &VERSION_45_3)
    }

    #[test]
    fn test_integer() -> Result<()> {
        let constant = Constant::Integer(42);
        let expected_bytes = [3, 0, 0, 0, 42];

        assert_eq!("Integer 42", constant.to_string());
        test_constant(&constant, &expected_bytes, 3, &VERSION_45_3)
    }

    #[test]
    fn test_float() -> Result<()> {
        let constant = Constant::Float(std::f32::consts::PI);
        let expected_bytes = [4, 64, 73, 15, 219];

        assert_eq!("Float 3.1415927", constant.to_string());
        test_constant(&constant, &expected_bytes, 4, &VERSION_45_3)
    }

    #[test]
    fn test_long() -> Result<()> {
        let constant = Constant::Long(1_234_567_890);
        let expected_bytes = [5, 0, 0, 0, 0, 73, 150, 2, 210];

        assert_eq!("Long 1234567890", constant.to_string());
        test_constant(&constant, &expected_bytes, 5, &VERSION_45_3)
    }

    #[test]
    fn test_double() -> Result<()> {
        let constant = Constant::Double(std::f64::consts::PI);
        let expected_bytes = [6, 64, 9, 33, 251, 84, 68, 45, 24];

        assert_eq!("Double 3.141592653589793", constant.to_string());
        test_constant(&constant, &expected_bytes, 6, &VERSION_45_3)
    }

    #[test]
    fn test_class() -> Result<()> {
        let constant = Constant::Class(1);
        let expected_bytes = [7, 0, 1];

        assert_eq!("Class #1", constant.to_string());
        test_constant(&constant, &expected_bytes, 7, &VERSION_45_3)
    }

    #[test]
    fn test_string() -> Result<()> {
        let constant = Constant::String(1);
        let expected_bytes = [8, 0, 1];

        assert_eq!("String #1", constant.to_string());
        test_constant(&constant, &expected_bytes, 8, &VERSION_45_3)
    }

    #[test]
    fn test_field_ref() -> Result<()> {
        let constant = Constant::FieldRef {
            class_index: 1,
            name_and_type_index: 2,
        };
        let expected_bytes = [9, 0, 1, 0, 2];

        assert_eq!("Fieldref #1.#2", constant.to_string());
        test_constant(&constant, &expected_bytes, 9, &VERSION_45_3)
    }

    #[test]
    fn test_method_ref() -> Result<()> {
        let constant = Constant::MethodRef {
            class_index: 1,
            name_and_type_index: 2,
        };
        let expected_bytes = [10, 0, 1, 0, 2];

        assert_eq!("Methodref #1.#2", constant.to_string());
        test_constant(&constant, &expected_bytes, 10, &VERSION_45_3)
    }

    #[test]
    fn test_interface_method_ref() -> Result<()> {
        let constant = Constant::InterfaceMethodRef {
            class_index: 1,
            name_and_type_index: 2,
        };
        let expected_bytes = [11, 0, 1, 0, 2];

        assert_eq!("InterfaceMethodRef #1.#2", constant.to_string());
        test_constant(&constant, &expected_bytes, 11, &VERSION_45_3)
    }

    #[test]
    fn test_name_and_type() -> Result<()> {
        let constant = Constant::NameAndType {
            name_index: 1,
            descriptor_index: 2,
        };
        let expected_bytes = [12, 0, 1, 0, 2];

        assert_eq!("NameAndType #1:#2", constant.to_string());
        test_constant(&constant, &expected_bytes, 12, &VERSION_45_3)
    }

    #[test]
    fn test_method_handle() -> Result<()> {
        let constant = Constant::MethodHandle {
            reference_kind: ReferenceKind::GetField,
            reference_index: 2,
        };
        let expected_bytes = [15, 1, 0, 2];

        assert_eq!("MethodHandle GetField.#2", constant.to_string());
        test_constant(&constant, &expected_bytes, 15, &VERSION_51_0)
    }

    #[test]
    fn test_method_type() -> Result<()> {
        let constant = Constant::MethodType(1);
        let expected_bytes = [16, 0, 1];

        assert_eq!("MethodType #1", constant.to_string());
        test_constant(&constant, &expected_bytes, 16, &VERSION_51_0)
    }

    #[test]
    fn test_dynamic() -> Result<()> {
        let constant = Constant::Dynamic {
            bootstrap_method_attr_index: 1,
            name_and_type_index: 2,
        };
        let expected_bytes = [17, 0, 1, 0, 2];

        assert_eq!("Dynamic #1.#2", constant.to_string());
        test_constant(&constant, &expected_bytes, 17, &VERSION_55_0)
    }

    #[test]
    fn test_invoke_dynamic() -> Result<()> {
        let constant = Constant::InvokeDynamic {
            bootstrap_method_attr_index: 1,
            name_and_type_index: 2,
        };
        let expected_bytes = [18, 0, 1, 0, 2];

        assert_eq!("InvokeDynamic #1.#2", constant.to_string());
        test_constant(&constant, &expected_bytes, 18, &VERSION_51_0)
    }

    #[test]
    fn test_module() -> Result<()> {
        let constant = Constant::Module(1);
        let expected_bytes = [19, 0, 1];

        assert_eq!("Module #1", constant.to_string());
        test_constant(&constant, &expected_bytes, 19, &VERSION_55_0)
    }

    #[test]
    fn test_package() -> Result<()> {
        let constant = Constant::Package(1);
        let expected_bytes = [20, 0, 1];

        assert_eq!("Package #1", constant.to_string());
        test_constant(&constant, &expected_bytes, 20, &VERSION_55_0)
    }
}
