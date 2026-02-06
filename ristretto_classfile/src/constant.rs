use crate::error::Error::InvalidConstantTag;
use crate::error::Result;
use crate::reference_kind::ReferenceKind;
use crate::version::Version;
use crate::{JAVA_7, JAVA_9, JAVA_11, mutf8};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fmt;
use std::io::{Cursor, Read};

const VERSION_45_3: Version = Version::Java1_0_2 { minor: 3 };
const VERSION_51_0: Version = JAVA_7;
const VERSION_53_0: Version = JAVA_9;
const VERSION_55_0: Version = JAVA_11;

/// Constant
///
/// Represents the different types of constants in the Java class file constant pool. The constant
/// pool is a table of structures representing various string constants, class and interface names,
/// field names, and other constants that are referred to within the class file structure and its
/// substructures.
///
/// # Examples
///
/// Creating different types of constants and working with them:
///
/// ```rust
/// use ristretto_classfile::{Constant, ReferenceKind, Version, JAVA_11, JAVA_7};
/// use std::io::Cursor;
///
/// // Create some constants
/// let utf8_constant = Constant::Utf8("java/lang/Object".to_string());
/// let class_constant = Constant::Class(1); // Points to the UTF-8 constant
/// let string_const = Constant::String(1);
/// let method_reference = Constant::MethodRef {
///     class_index: 2,          // Points to a Class constant
///     name_and_type_index: 3,  // Points to a NameAndType constant
/// };
///
/// // Serialize constants to bytes
/// let mut buffer = Vec::new();
/// utf8_constant.to_bytes(&mut buffer)?;
///
/// assert!(utf8_constant.valid_for_version(&JAVA_7));
///
/// let dynamic_constant = Constant::Dynamic {
///     bootstrap_method_attr_index: 1,
///     name_and_type_index: 2,
/// };
/// assert!(!dynamic_constant.valid_for_version(&JAVA_7)); // Dynamic was added in Java 11
/// assert!(dynamic_constant.valid_for_version(&JAVA_11));
///
/// // Deserialize a constant from bytes
/// // For example, deserializing an Integer constant:
/// // Tag 3 (Integer) followed by a 4-byte integer value
/// let int_bytes = vec![3, 0, 0, 0, 42];
/// let mut cursor = Cursor::new(int_bytes);
/// let deserialized = Constant::from_bytes(&mut cursor)?;
/// assert_eq!(deserialized, Constant::Integer(42));
///
/// // Display constants in human-readable format
/// println!("{utf8_constant}");    // Outputs: "Utf8 java/lang/Object"
/// println!("{class_constant}");   // Outputs: "Class #1"
/// println!("{method_reference}");    // Outputs: "Methodref #2.#3"
/// # Ok::<(), ristretto_classfile::Error>(())
/// ```
///
///  # References
///
/// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4>
#[derive(Clone, Debug)]
pub enum Constant {
    /// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.7>
    Utf8(String),
    /// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.4>
    Integer(i32),
    /// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.4>
    Float(f32),
    /// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.5>
    Long(i64),
    /// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.5>
    Double(f64),
    /// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.1>
    Class(u16), // Name index (Utf8)
    /// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.3>
    String(u16), // String index (Utf8)
    /// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.2>
    FieldRef {
        class_index: u16,
        name_and_type_index: u16,
    },
    /// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.2>
    MethodRef {
        class_index: u16,
        name_and_type_index: u16,
    },
    /// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.2>
    InterfaceMethodRef {
        class_index: u16,
        name_and_type_index: u16,
    },
    /// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.6>
    NameAndType {
        name_index: u16,
        descriptor_index: u16,
    },
    /// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.8>
    MethodHandle {
        reference_kind: ReferenceKind,
        reference_index: u16,
    },
    /// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.9>
    MethodType(u16), // Descriptor index (NameAndType)
    /// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.10>
    Dynamic {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    },
    /// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.10>
    InvokeDynamic {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    },
    /// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.11>
    Module(u16), // Name index (Utf8)
    /// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.12>
    Package(u16), // Name index (Utf8)
}

impl PartialEq for Constant {
    #[expect(clippy::match_same_arms)]
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Constant::Utf8(a), Constant::Utf8(b)) => a == b,
            (Constant::Integer(a), Constant::Integer(b)) => a == b,
            (Constant::Float(a), Constant::Float(b)) => a.to_bits() == b.to_bits(),
            (Constant::Long(a), Constant::Long(b)) => a == b,
            (Constant::Double(a), Constant::Double(b)) => a.to_bits() == b.to_bits(),
            (Constant::Class(a), Constant::Class(b)) => a == b,
            (Constant::String(a), Constant::String(b)) => a == b,
            (
                Constant::FieldRef {
                    class_index: a1,
                    name_and_type_index: a2,
                },
                Constant::FieldRef {
                    class_index: b1,
                    name_and_type_index: b2,
                },
            ) => a1 == b1 && a2 == b2,
            (
                Constant::MethodRef {
                    class_index: a1,
                    name_and_type_index: a2,
                },
                Constant::MethodRef {
                    class_index: b1,
                    name_and_type_index: b2,
                },
            ) => a1 == b1 && a2 == b2,
            (
                Constant::InterfaceMethodRef {
                    class_index: a1,
                    name_and_type_index: a2,
                },
                Constant::InterfaceMethodRef {
                    class_index: b1,
                    name_and_type_index: b2,
                },
            ) => a1 == b1 && a2 == b2,
            (
                Constant::NameAndType {
                    name_index: a1,
                    descriptor_index: a2,
                },
                Constant::NameAndType {
                    name_index: b1,
                    descriptor_index: b2,
                },
            ) => a1 == b1 && a2 == b2,
            (
                Constant::MethodHandle {
                    reference_kind: a1,
                    reference_index: a2,
                },
                Constant::MethodHandle {
                    reference_kind: b1,
                    reference_index: b2,
                },
            ) => a1 == b1 && a2 == b2,
            (Constant::MethodType(a), Constant::MethodType(b)) => a == b,
            (
                Constant::Dynamic {
                    bootstrap_method_attr_index: a1,
                    name_and_type_index: a2,
                },
                Constant::Dynamic {
                    bootstrap_method_attr_index: b1,
                    name_and_type_index: b2,
                },
            ) => a1 == b1 && a2 == b2,
            (
                Constant::InvokeDynamic {
                    bootstrap_method_attr_index: a1,
                    name_and_type_index: a2,
                },
                Constant::InvokeDynamic {
                    bootstrap_method_attr_index: b1,
                    name_and_type_index: b2,
                },
            ) => a1 == b1 && a2 == b2,
            (Constant::Module(a), Constant::Module(b)) => a == b,
            (Constant::Package(a), Constant::Package(b)) => a == b,
            _ => false,
        }
    }
}

impl Eq for Constant {}

impl Constant {
    /// Returns the tag value for this constant type.
    ///
    /// The tag is a single-byte identifier used to differentiate between different constant types
    /// in a class file.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::Constant;
    ///
    /// let constant = Constant::Utf8("Hello, world!".to_string());
    /// assert_eq!(constant.tag(), 1);
    ///
    /// let constant = Constant::Integer(42);
    /// assert_eq!(constant.tag(), 3);
    /// ```
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

    /// Checks if the constant is valid for the specified Java version.
    ///
    /// Different constant types were introduced in different Java versions. This method verifies
    /// whether the constant is supported in the given version.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::{Constant, ReferenceKind, Version, JAVA_6, JAVA_7};
    ///
    /// let constant = Constant::Utf8("Hello, world!".to_string());
    /// assert!(constant.valid_for_version(&JAVA_7));
    ///
    /// let constant = Constant::MethodHandle {
    ///     reference_kind: ReferenceKind::GetField,
    ///     reference_index: 1,
    /// };
    /// assert!(!constant.valid_for_version(&JAVA_6)); // MethodHandle was introduced in Java 7
    /// ```
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

    /// Deserializes a `Constant` from bytes.
    ///
    /// Reads a constant pool entry from the provided byte buffer according to the Java class file
    /// format specification.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The tag is invalid
    /// - There's not enough data to read
    /// - The data is malformed
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::Constant;
    /// use std::io::Cursor;
    ///
    /// // Create a buffer with a UTF-8 constant (tag 1)
    /// // Format: [tag(1), length(2 bytes), data(length bytes)]
    /// let buffer = vec![1, 0, 5, 72, 101, 108, 108, 111]; // "Hello"
    /// let mut cursor = Cursor::new(buffer);
    ///
    /// let constant = Constant::from_bytes(&mut cursor)?;
    /// assert_eq!(constant, Constant::Utf8("Hello".to_string()));
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    pub fn from_bytes(bytes: &mut Cursor<impl AsRef<[u8]>>) -> Result<Constant> {
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

    /// Serializes the `Constant` to bytes.
    ///
    /// Writes the constant pool entry to the provided byte buffer according to the Java class file
    /// format specification.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::Constant;
    ///
    /// let constant = Constant::Integer(42);
    /// let mut buffer = Vec::new();
    ///
    /// constant.to_bytes(&mut buffer)?;
    /// assert_eq!(buffer, vec![3, 0, 0, 0, 42]);
    /// // Tag 3 (Integer) followed by the 4-byte integer value in big-endian format
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - A UTF-8 string is more than 65535 bytes long
    /// - Writing to the buffer fails
    #[expect(clippy::match_same_arms)]
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
    /// Formats the `Constant` for display purposes.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::Constant;
    /// use std::fmt::Display;
    ///
    /// // Create some constants
    /// let utf8 = Constant::Utf8("Hello, world!".to_string());
    /// let class = Constant::Class(5);
    /// let method_reference = Constant::MethodRef {
    ///     class_index: 3,
    ///     name_and_type_index: 7,
    /// };
    ///
    /// // Display them as strings
    /// assert_eq!(utf8.to_string(), "Utf8 Hello, world!");
    /// assert_eq!(class.to_string(), "Class #5");
    /// assert_eq!(method_reference.to_string(), "Methodref #3.#7");
    ///
    /// // Using format! with Display trait
    /// let formatted = format!("Constants: {utf8}, {class}, {method_reference}");
    /// assert_eq!(
    ///     formatted,
    ///     "Constants: Utf8 Hello, world!, Class #5, Methodref #3.#7"
    /// );
    /// ```
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
    use crate::JAVA_1_0_2;

    const VERSION_45_0: Version = JAVA_1_0_2;

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

    #[test]
    fn test_eq_utf8_equal() {
        let a = Constant::Utf8("hello".to_string());
        let b = Constant::Utf8("hello".to_string());
        assert_eq!(a, b);
    }

    #[test]
    fn test_eq_integer_equal() {
        let a = Constant::Integer(42);
        let b = Constant::Integer(42);
        assert_eq!(a, b);
    }

    #[test]
    fn test_eq_float_equal() {
        let a = Constant::Float(1.5);
        let b = Constant::Float(1.5);
        assert_eq!(a, b);
    }

    #[test]
    fn test_eq_float_nan_equal() {
        let a = Constant::Float(f32::NAN);
        let b = Constant::Float(f32::NAN);
        assert_eq!(a, b); // NaN equals NaN using to_bits()
    }

    #[test]
    fn test_eq_float_negative_zero_not_equal_positive_zero() {
        let a = Constant::Float(-0.0);
        let b = Constant::Float(0.0);
        assert_ne!(a, b); // -0.0 and 0.0 have different bit patterns
    }

    #[test]
    fn test_eq_long_equal() {
        let a = Constant::Long(1_234_567_890);
        let b = Constant::Long(1_234_567_890);
        assert_eq!(a, b);
    }

    #[test]
    fn test_eq_double_equal() {
        let a = Constant::Double(std::f64::consts::PI);
        let b = Constant::Double(std::f64::consts::PI);
        assert_eq!(a, b);
    }

    #[test]
    fn test_eq_double_nan_equal() {
        let a = Constant::Double(f64::NAN);
        let b = Constant::Double(f64::NAN);
        assert_eq!(a, b); // NaN equals NaN using to_bits()
    }

    #[test]
    fn test_eq_double_negative_zero_not_equal_positive_zero() {
        let a = Constant::Double(-0.0);
        let b = Constant::Double(0.0);
        assert_ne!(a, b); // -0.0 and 0.0 have different bit patterns
    }

    #[test]
    fn test_eq_class_equal() {
        let a = Constant::Class(1);
        let b = Constant::Class(1);
        assert_eq!(a, b);
    }

    #[test]
    fn test_eq_string_equal() {
        let a = Constant::String(1);
        let b = Constant::String(1);
        assert_eq!(a, b);
    }

    #[test]
    fn test_eq_field_ref_equal() {
        let a = Constant::FieldRef {
            class_index: 1,
            name_and_type_index: 2,
        };
        let b = Constant::FieldRef {
            class_index: 1,
            name_and_type_index: 2,
        };
        assert_eq!(a, b);
    }

    #[test]
    fn test_eq_method_ref_equal() {
        let a = Constant::MethodRef {
            class_index: 1,
            name_and_type_index: 2,
        };
        let b = Constant::MethodRef {
            class_index: 1,
            name_and_type_index: 2,
        };
        assert_eq!(a, b);
    }

    #[test]
    fn test_eq_interface_method_ref_equal() {
        let a = Constant::InterfaceMethodRef {
            class_index: 1,
            name_and_type_index: 2,
        };
        let b = Constant::InterfaceMethodRef {
            class_index: 1,
            name_and_type_index: 2,
        };
        assert_eq!(a, b);
    }

    #[test]
    fn test_eq_name_and_type_equal() {
        let a = Constant::NameAndType {
            name_index: 1,
            descriptor_index: 2,
        };
        let b = Constant::NameAndType {
            name_index: 1,
            descriptor_index: 2,
        };
        assert_eq!(a, b);
    }

    #[test]
    fn test_eq_method_handle_equal() {
        let a = Constant::MethodHandle {
            reference_kind: ReferenceKind::GetField,
            reference_index: 1,
        };
        let b = Constant::MethodHandle {
            reference_kind: ReferenceKind::GetField,
            reference_index: 1,
        };
        assert_eq!(a, b);
    }

    #[test]
    fn test_eq_method_type_equal() {
        let a = Constant::MethodType(1);
        let b = Constant::MethodType(1);
        assert_eq!(a, b);
    }

    #[test]
    fn test_eq_dynamic_equal() {
        let a = Constant::Dynamic {
            bootstrap_method_attr_index: 1,
            name_and_type_index: 2,
        };
        let b = Constant::Dynamic {
            bootstrap_method_attr_index: 1,
            name_and_type_index: 2,
        };
        assert_eq!(a, b);
    }

    #[test]
    fn test_eq_invoke_dynamic_equal() {
        let a = Constant::InvokeDynamic {
            bootstrap_method_attr_index: 1,
            name_and_type_index: 2,
        };
        let b = Constant::InvokeDynamic {
            bootstrap_method_attr_index: 1,
            name_and_type_index: 2,
        };
        assert_eq!(a, b);
    }

    #[test]
    fn test_eq_module_equal() {
        let a = Constant::Module(1);
        let b = Constant::Module(1);
        assert_eq!(a, b);
    }

    #[test]
    fn test_eq_package_equal() {
        let a = Constant::Package(1);
        let b = Constant::Package(1);
        assert_eq!(a, b);
    }

    #[test]
    fn test_eq_utf8_not_equal() {
        let a = Constant::Utf8("hello".to_string());
        let b = Constant::Utf8("world".to_string());
        assert_ne!(a, b);
    }

    #[test]
    fn test_eq_integer_not_equal() {
        let a = Constant::Integer(42);
        let b = Constant::Integer(43);
        assert_ne!(a, b);
    }

    #[test]
    fn test_eq_float_not_equal() {
        let a = Constant::Float(1.5);
        let b = Constant::Float(2.5);
        assert_ne!(a, b);
    }

    #[test]
    fn test_eq_long_not_equal() {
        let a = Constant::Long(1_234_567_890);
        let b = Constant::Long(9_876_543_210);
        assert_ne!(a, b);
    }

    #[test]
    fn test_eq_double_not_equal() {
        let a = Constant::Double(std::f64::consts::PI);
        let b = Constant::Double(std::f64::consts::E);
        assert_ne!(a, b);
    }

    #[test]
    fn test_eq_class_not_equal() {
        let a = Constant::Class(1);
        let b = Constant::Class(2);
        assert_ne!(a, b);
    }

    #[test]
    fn test_eq_string_not_equal() {
        let a = Constant::String(1);
        let b = Constant::String(2);
        assert_ne!(a, b);
    }

    #[test]
    fn test_eq_field_ref_not_equal_class_index() {
        let a = Constant::FieldRef {
            class_index: 1,
            name_and_type_index: 2,
        };
        let b = Constant::FieldRef {
            class_index: 3,
            name_and_type_index: 2,
        };
        assert_ne!(a, b);
    }

    #[test]
    fn test_eq_field_ref_not_equal_name_and_type_index() {
        let a = Constant::FieldRef {
            class_index: 1,
            name_and_type_index: 2,
        };
        let b = Constant::FieldRef {
            class_index: 1,
            name_and_type_index: 3,
        };
        assert_ne!(a, b);
    }

    #[test]
    fn test_eq_method_ref_not_equal() {
        let a = Constant::MethodRef {
            class_index: 1,
            name_and_type_index: 2,
        };
        let b = Constant::MethodRef {
            class_index: 3,
            name_and_type_index: 4,
        };
        assert_ne!(a, b);
    }

    #[test]
    fn test_eq_interface_method_ref_not_equal() {
        let a = Constant::InterfaceMethodRef {
            class_index: 1,
            name_and_type_index: 2,
        };
        let b = Constant::InterfaceMethodRef {
            class_index: 3,
            name_and_type_index: 4,
        };
        assert_ne!(a, b);
    }

    #[test]
    fn test_eq_name_and_type_not_equal() {
        let a = Constant::NameAndType {
            name_index: 1,
            descriptor_index: 2,
        };
        let b = Constant::NameAndType {
            name_index: 3,
            descriptor_index: 4,
        };
        assert_ne!(a, b);
    }

    #[test]
    fn test_eq_method_handle_not_equal_reference_kind() {
        let a = Constant::MethodHandle {
            reference_kind: ReferenceKind::GetField,
            reference_index: 1,
        };
        let b = Constant::MethodHandle {
            reference_kind: ReferenceKind::GetStatic,
            reference_index: 1,
        };
        assert_ne!(a, b);
    }

    #[test]
    fn test_eq_method_handle_not_equal_reference_index() {
        let a = Constant::MethodHandle {
            reference_kind: ReferenceKind::GetField,
            reference_index: 1,
        };
        let b = Constant::MethodHandle {
            reference_kind: ReferenceKind::GetField,
            reference_index: 2,
        };
        assert_ne!(a, b);
    }

    #[test]
    fn test_eq_method_type_not_equal() {
        let a = Constant::MethodType(1);
        let b = Constant::MethodType(2);
        assert_ne!(a, b);
    }

    #[test]
    fn test_eq_dynamic_not_equal() {
        let a = Constant::Dynamic {
            bootstrap_method_attr_index: 1,
            name_and_type_index: 2,
        };
        let b = Constant::Dynamic {
            bootstrap_method_attr_index: 3,
            name_and_type_index: 4,
        };
        assert_ne!(a, b);
    }

    #[test]
    fn test_eq_invoke_dynamic_not_equal() {
        let a = Constant::InvokeDynamic {
            bootstrap_method_attr_index: 1,
            name_and_type_index: 2,
        };
        let b = Constant::InvokeDynamic {
            bootstrap_method_attr_index: 3,
            name_and_type_index: 4,
        };
        assert_ne!(a, b);
    }

    #[test]
    fn test_eq_module_not_equal() {
        let a = Constant::Module(1);
        let b = Constant::Module(2);
        assert_ne!(a, b);
    }

    #[test]
    fn test_eq_package_not_equal() {
        let a = Constant::Package(1);
        let b = Constant::Package(2);
        assert_ne!(a, b);
    }

    #[test]
    fn test_eq_different_variants_not_equal() {
        let utf8 = Constant::Utf8("1".to_string());
        let integer = Constant::Integer(1);
        let float = Constant::Float(1.0);
        let long = Constant::Long(1);
        let double = Constant::Double(1.0);
        let class = Constant::Class(1);
        let string = Constant::String(1);
        let field_ref = Constant::FieldRef {
            class_index: 1,
            name_and_type_index: 1,
        };
        let method_ref = Constant::MethodRef {
            class_index: 1,
            name_and_type_index: 1,
        };
        let interface_method_ref = Constant::InterfaceMethodRef {
            class_index: 1,
            name_and_type_index: 1,
        };
        let name_and_type = Constant::NameAndType {
            name_index: 1,
            descriptor_index: 1,
        };
        let method_handle = Constant::MethodHandle {
            reference_kind: ReferenceKind::GetField,
            reference_index: 1,
        };
        let method_type = Constant::MethodType(1);
        let dynamic = Constant::Dynamic {
            bootstrap_method_attr_index: 1,
            name_and_type_index: 1,
        };
        let invoke_dynamic = Constant::InvokeDynamic {
            bootstrap_method_attr_index: 1,
            name_and_type_index: 1,
        };
        let module = Constant::Module(1);
        let package = Constant::Package(1);

        // All different variants should not be equal to each other
        assert_ne!(utf8, integer);
        assert_ne!(integer, float);
        assert_ne!(float, long);
        assert_ne!(long, double);
        assert_ne!(double, class);
        assert_ne!(class, string);
        assert_ne!(string, field_ref);
        assert_ne!(field_ref, method_ref);
        assert_ne!(method_ref, interface_method_ref);
        assert_ne!(interface_method_ref, name_and_type);
        assert_ne!(name_and_type, method_handle);
        assert_ne!(method_handle, method_type);
        assert_ne!(method_type, dynamic);
        assert_ne!(dynamic, invoke_dynamic);
        assert_ne!(invoke_dynamic, module);
        assert_ne!(module, package);
        assert_ne!(package, utf8);
    }
}
