use crate::error::Error::InvalidVerificationTypeTag;
use crate::error::Result;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fmt;
use std::io::Cursor;

/// Represents a verification type used in the Java Virtual Machine's type checking system.
///
/// Verification types are used during bytecode verification to track the types of values on the
/// operand stack and in local variables. They are primarily used in the `StackMapTable` attribute,
/// which helps the JVM perform type checking during class loading.
///
/// # Examples
///
/// ```rust
/// use ristretto_classfile::attributes::VerificationType;
///
/// // Create primitive verification types
/// let top_type = VerificationType::Top;
/// let int_type = VerificationType::Integer;
/// let float_type = VerificationType::Float;
/// let double_type = VerificationType::Double;
/// let long_type = VerificationType::Long;
/// let null_type = VerificationType::Null;
/// let uninit_this = VerificationType::UninitializedThis;
///
/// // Create reference verification types
/// let object_type = VerificationType::Object { cpool_index: 15 };
/// let uninit_type = VerificationType::Uninitialized { offset: 42 };
/// ```
///
/// Serializing and deserializing verification types:
///
/// ```rust
/// use ristretto_classfile::attributes::VerificationType;
/// use std::io::Cursor;
///
/// // Serialize an Object verification type
/// let object_type = VerificationType::Object { cpool_index: 15 };
/// let mut bytes = Vec::new();
/// object_type.to_bytes(&mut bytes)?;
/// assert_eq!(bytes, vec![0x07, 0x00, 0x0F]); // Tag 7 + index 15 (big-endian)
///
/// // Deserialize back from bytes
/// let mut cursor = Cursor::new(bytes);
/// let deserialized = VerificationType::from_bytes(&mut cursor)?;
/// assert_eq!(deserialized, object_type);
/// # Ok::<(), ristretto_classfile::Error>(())
/// ```
///
/// # References
///
/// - [JVM Specification §4.7.4](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.4)
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum VerificationType {
    Top,
    Integer,
    Float,
    Double,
    Long,
    Null,
    UninitializedThis,
    Object { cpool_index: u16 },
    Uninitialized { offset: u16 },
}

impl VerificationType {
    /// Return the tag for the verification type.
    ///
    /// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-VerificationTypeInfo>
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::VerificationType;
    ///
    /// let top_type = VerificationType::Top;
    /// assert_eq!(top_type.tag(), 0);
    ///
    /// let int_type = VerificationType::Integer;
    /// assert_eq!(int_type.tag(), 1);
    /// ```
    #[must_use]
    pub fn tag(&self) -> u8 {
        match self {
            VerificationType::Top => 0,
            VerificationType::Integer => 1,
            VerificationType::Float => 2,
            VerificationType::Double => 3,
            VerificationType::Long => 4,
            VerificationType::Null => 5,
            VerificationType::UninitializedThis => 6,
            VerificationType::Object { .. } => 7,
            VerificationType::Uninitialized { .. } => 8,
        }
    }

    /// Deserialize the verification type from bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if the tag is invalid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::VerificationType;
    /// use std::io::Cursor;
    ///
    /// let bytes = vec![0x07, 0x00, 0x0A]; // Object type with cpool_index 10
    /// let mut cursor = Cursor::new(bytes);
    /// let verification_type = VerificationType::from_bytes(&mut cursor)?;
    /// assert_eq!(verification_type, VerificationType::Object { cpool_index: 10 });
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    pub fn from_bytes(bytes: &mut Cursor<Vec<u8>>) -> Result<VerificationType> {
        let tag = bytes.read_u8()?;

        let verification_type = match tag {
            0 => VerificationType::Top,
            1 => VerificationType::Integer,
            2 => VerificationType::Float,
            3 => VerificationType::Double,
            4 => VerificationType::Long,
            5 => VerificationType::Null,
            6 => VerificationType::UninitializedThis,
            7 => VerificationType::Object {
                cpool_index: bytes.read_u16::<BigEndian>()?,
            },
            8 => VerificationType::Uninitialized {
                offset: bytes.read_u16::<BigEndian>()?,
            },
            _ => return Err(InvalidVerificationTypeTag(tag)),
        };
        Ok(verification_type)
    }

    /// Serialize the verification type to bytes.
    ///
    /// # Errors
    ///
    /// Should not occur; reserved for future use.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::VerificationType;
    ///
    /// let object_type = VerificationType::Object { cpool_index: 10 };
    /// let mut bytes = Vec::new();
    /// object_type.to_bytes(&mut bytes)?;
    /// assert_eq!(bytes, vec![0x07, 0x00, 0x0A]);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    pub fn to_bytes(&self, bytes: &mut Vec<u8>) -> Result<()> {
        bytes.write_u8(self.tag())?;
        match self {
            VerificationType::Object { cpool_index } => {
                bytes.write_u16::<BigEndian>(*cpool_index)?;
            }
            VerificationType::Uninitialized { offset } => bytes.write_u16::<BigEndian>(*offset)?,
            _ => {}
        }
        Ok(())
    }
}

impl fmt::Display for VerificationType {
    /// Implements the `Display` trait for `VerificationType`, providing a human-readable string
    /// representation of each verification type.
    ///
    /// # Examples
    ///
    /// Using the `Display` trait to format verification types as strings:
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::VerificationType;
    ///
    /// let output = VerificationType::Top.to_string();
    /// assert_eq!(output, "top");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VerificationType::Top => write!(f, "top"),
            VerificationType::Integer => write!(f, "int"),
            VerificationType::Float => write!(f, "float"),
            VerificationType::Double => write!(f, "double"),
            VerificationType::Long => write!(f, "long"),
            VerificationType::Null => write!(f, "null"),
            VerificationType::UninitializedThis => write!(f, "uninitialized this"),
            VerificationType::Object { cpool_index } => write!(f, "object #{cpool_index}"),
            VerificationType::Uninitialized { offset } => write!(f, "uninitialized {offset}"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_invalid_verification_type() -> Result<()> {
        let mut bytes = Vec::new();
        let tag = u8::MAX;
        bytes.write_u8(tag)?;

        assert_eq!(
            Err(InvalidVerificationTypeTag(tag)),
            VerificationType::from_bytes(&mut Cursor::new(bytes))
        );
        Ok(())
    }

    fn test_verification_type(
        verification_type: &VerificationType,
        expected_bytes: &[u8],
        tag: u8,
    ) -> Result<()> {
        assert_eq!(tag, verification_type.tag());

        let mut bytes = Vec::new();
        verification_type.to_bytes(&mut bytes)?;
        assert_eq!(expected_bytes, &bytes[..]);
        let mut bytes = Cursor::new(expected_bytes.to_vec());
        assert_eq!(
            *verification_type,
            VerificationType::from_bytes(&mut bytes)?
        );
        Ok(())
    }

    #[test]
    fn test_top() -> Result<()> {
        let verification_type = VerificationType::Top;
        let tag = 0;
        let expected_bytes = [tag];

        assert_eq!("top", verification_type.to_string());
        test_verification_type(&verification_type, &expected_bytes, tag)
    }

    #[test]
    fn test_integer() -> Result<()> {
        let verification_type = VerificationType::Integer;
        let tag = 1;
        let expected_bytes = [tag];

        assert_eq!("int", verification_type.to_string());
        test_verification_type(&verification_type, &expected_bytes, tag)
    }

    #[test]
    fn test_float() -> Result<()> {
        let verification_type = VerificationType::Float;
        let tag = 2;
        let expected_bytes = [tag];

        assert_eq!("float", verification_type.to_string());
        test_verification_type(&verification_type, &expected_bytes, tag)
    }

    #[test]
    fn test_double() -> Result<()> {
        let verification_type = VerificationType::Double;
        let tag = 3;
        let expected_bytes = [tag];

        assert_eq!("double", verification_type.to_string());
        test_verification_type(&verification_type, &expected_bytes, tag)
    }

    #[test]
    fn test_long() -> Result<()> {
        let verification_type = VerificationType::Long;
        let tag = 4;
        let expected_bytes = [tag];

        assert_eq!("long", verification_type.to_string());
        test_verification_type(&verification_type, &expected_bytes, tag)
    }

    #[test]
    fn test_null() -> Result<()> {
        let verification_type = VerificationType::Null;
        let tag = 5;
        let expected_bytes = [tag];

        assert_eq!("null", verification_type.to_string());
        test_verification_type(&verification_type, &expected_bytes, tag)
    }

    #[test]
    fn test_uninitialized_this() -> Result<()> {
        let verification_type = VerificationType::UninitializedThis;
        let tag = 6;
        let expected_bytes = [tag];

        assert_eq!("uninitialized this", verification_type.to_string());
        test_verification_type(&verification_type, &expected_bytes, tag)
    }

    #[test]
    fn test_object() -> Result<()> {
        let verification_type = VerificationType::Object { cpool_index: 42 };
        let tag = 7;
        let expected_bytes = [tag, 0, 42];

        assert_eq!("object #42", verification_type.to_string());
        test_verification_type(&verification_type, &expected_bytes, tag)
    }

    #[test]
    fn test_uninitialized() -> Result<()> {
        let verification_type = VerificationType::Uninitialized { offset: 42 };
        let tag = 8;
        let expected_bytes = [tag, 0, 42];

        assert_eq!("uninitialized 42", verification_type.to_string());
        test_verification_type(&verification_type, &expected_bytes, tag)
    }
}
