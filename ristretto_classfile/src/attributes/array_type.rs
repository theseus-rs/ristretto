use crate::Error::InvalidArrayTypeCode;
use crate::error::Result;
use byteorder::{ReadBytesExt, WriteBytesExt};
use std::fmt;
use std::io::Cursor;

/// Represents the primitive type of an array, used by the `newarray` JVM instruction.
///
/// Each variant corresponds to a specific primitive type that an array can hold in Java. The
/// `newarray` instruction takes an `atype` operand, which is represented by this enum.
///
/// See the [JVMS on `newarray`](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.newarray)
/// for more details on the instruction and type codes.
///
/// # Examples
///
/// Basic usage and obtaining the type code:
///
/// ```rust
/// use ristretto_classfile::attributes::ArrayType;
/// use std::io::Cursor;
///
/// let int_array_type = ArrayType::Int;
/// assert_eq!(int_array_type.code(), 10);
/// assert_eq!(int_array_type.to_string(), "int");
///
/// let bool_array_type = ArrayType::Boolean;
/// assert_eq!(bool_array_type.code(), 4);
/// assert_eq!(bool_array_type.to_string(), "boolean");
///
/// // Serialize to bytes
/// let mut bytes_cursor = Cursor::new(Vec::new());
/// int_array_type.to_bytes(&mut bytes_cursor)?;
/// assert_eq!(bytes_cursor.into_inner(), vec![10]);
///
/// // Deserialize from bytes
/// let mut read_cursor = Cursor::new(vec![4]);
/// let deserialized_type = ArrayType::from_bytes(&mut read_cursor)?;
/// assert_eq!(deserialized_type, ArrayType::Boolean);
/// # Ok::<(), ristretto_classfile::Error>(())
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ArrayType {
    /// Represents a `boolean` array (`T_BOOLEAN = 4`).
    Boolean,
    /// Represents a `char` array (`T_CHAR = 5`).
    Char,
    /// Represents a `float` array (`T_FLOAT = 6`).
    Float,
    /// Represents a `double` array (`T_DOUBLE = 7`).
    Double,
    /// Represents a `byte` array (`T_BYTE = 8`).
    Byte,
    /// Represents a `short` array (`T_SHORT = 9`).
    Short,
    /// Represents an `int` array (`T_INT = 10`).
    Int,
    /// Represents a `long` array (`T_LONG = 11`).
    Long,
}

impl ArrayType {
    /// Returns the numeric code for the array type, as defined in the JVM specification
    /// for the `newarray` instruction.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::ArrayType;
    ///
    /// assert_eq!(ArrayType::Int.code(), 10);
    /// assert_eq!(ArrayType::Byte.code(), 8);
    /// ```
    #[must_use]
    pub fn code(&self) -> u8 {
        match self {
            ArrayType::Boolean => 4,
            ArrayType::Char => 5,
            ArrayType::Float => 6,
            ArrayType::Double => 7,
            ArrayType::Byte => 8,
            ArrayType::Short => 9,
            ArrayType::Int => 10,
            ArrayType::Long => 11,
        }
    }

    /// Deserializes an `ArrayType` from a byte stream.
    ///
    /// Reads a single byte from the cursor, which is expected to be a valid array type code.
    ///
    /// # Errors
    ///
    /// Returns `Error::InvalidArrayTypeCode` if the byte read from the cursor does not
    /// correspond to one of the defined array type codes (4-11).
    /// Propagates I/O errors if reading the byte fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::ArrayType;
    /// use ristretto_classfile::Error;
    /// use std::io::Cursor;
    ///
    /// let mut cursor = Cursor::new(vec![10]); // Code for Int
    /// let array_type = ArrayType::from_bytes(&mut cursor)?;
    /// assert_eq!(array_type, ArrayType::Int);
    ///
    /// let mut invalid_cursor = Cursor::new(vec![0]); // Invalid code
    /// match ArrayType::from_bytes(&mut invalid_cursor) {
    ///     Err(Error::InvalidArrayTypeCode(0)) => { /* Expected error */ }
    ///     _ => panic!("Expected InvalidArrayTypeCode error"),
    /// }
    /// # Ok::<(), Error>(())
    /// ```
    pub fn from_bytes(bytes: &mut Cursor<Vec<u8>>) -> Result<ArrayType> {
        let code = bytes.read_u8()?;

        let array_type = match code {
            4 => ArrayType::Boolean,
            5 => ArrayType::Char,
            6 => ArrayType::Float,
            7 => ArrayType::Double,
            8 => ArrayType::Byte,
            9 => ArrayType::Short,
            10 => ArrayType::Int,
            11 => ArrayType::Long,
            _ => return Err(InvalidArrayTypeCode(code)),
        };
        Ok(array_type)
    }

    /// Serializes the `ArrayType` to a byte stream.
    ///
    /// Writes the numeric code of the array type as a single byte to the cursor.
    ///
    /// # Errors
    ///
    /// Propagates I/O errors if writing the byte fails. Currently, this method itself
    /// does not produce other errors, but the signature includes `Result` for consistency
    /// and potential future extensions.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::ArrayType;
    /// use std::io::Cursor;
    ///
    /// let array_type = ArrayType::Float;
    /// let mut cursor = Cursor::new(Vec::new());
    /// array_type.to_bytes(&mut cursor)?;
    ///
    /// assert_eq!(cursor.into_inner(), vec![6]); // Code for Float
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    pub fn to_bytes(&self, bytes: &mut Cursor<Vec<u8>>) -> Result<()> {
        bytes.write_u8(self.code())?;
        Ok(())
    }
}

impl fmt::Display for ArrayType {
    /// Implements the `Display` trait for `ArrayType`, allowing the enum to be formatted as a string.
    ///
    /// This implementation converts each array type variant to its corresponding Java type name as a string.
    /// This is useful for human-readable output, debugging, and generating descriptors.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::ArrayType;
    ///
    /// let output = ArrayType::Int.to_string();
    /// assert_eq!(output, "int");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ArrayType::Boolean => write!(f, "boolean"),
            ArrayType::Char => write!(f, "char"),
            ArrayType::Float => write!(f, "float"),
            ArrayType::Double => write!(f, "double"),
            ArrayType::Byte => write!(f, "byte"),
            ArrayType::Short => write!(f, "short"),
            ArrayType::Int => write!(f, "int"),
            ArrayType::Long => write!(f, "long"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::Read;

    #[test]
    fn test_invalid_code() {
        let mut bytes = Cursor::new(vec![0]);
        assert_eq!(
            Err(InvalidArrayTypeCode(0)),
            ArrayType::from_bytes(&mut bytes)
        );
    }

    fn test_array_type(array_type: &ArrayType, code: u8) -> Result<()> {
        assert_eq!(code, array_type.code());
        let expected_bytes = [code];

        let mut buffer = Cursor::new(Vec::new());
        array_type.to_bytes(&mut buffer)?;
        let mut bytes = Vec::new();
        buffer.set_position(0);
        buffer.read_to_end(&mut bytes)?;
        assert_eq!(expected_bytes, bytes.as_slice());
        let mut bytes = Cursor::new(expected_bytes.to_vec());
        assert_eq!(*array_type, ArrayType::from_bytes(&mut bytes)?);
        Ok(())
    }

    #[test]
    fn test_boolean() -> Result<()> {
        assert_eq!("boolean", ArrayType::Boolean.to_string());
        test_array_type(&ArrayType::Boolean, 4)
    }

    #[test]
    fn test_char() -> Result<()> {
        assert_eq!("char", ArrayType::Char.to_string());
        test_array_type(&ArrayType::Char, 5)
    }

    #[test]
    fn test_float() -> Result<()> {
        assert_eq!("float", ArrayType::Float.to_string());
        test_array_type(&ArrayType::Float, 6)
    }

    #[test]
    fn test_double() -> Result<()> {
        assert_eq!("double", ArrayType::Double.to_string());
        test_array_type(&ArrayType::Double, 7)
    }

    #[test]
    fn test_byte() -> Result<()> {
        assert_eq!("byte", ArrayType::Byte.to_string());
        test_array_type(&ArrayType::Byte, 8)
    }

    #[test]
    fn test_short() -> Result<()> {
        assert_eq!("short", ArrayType::Short.to_string());
        test_array_type(&ArrayType::Short, 9)
    }

    #[test]
    fn test_int() -> Result<()> {
        assert_eq!("int", ArrayType::Int.to_string());
        test_array_type(&ArrayType::Int, 10)
    }

    #[test]
    fn test_long() -> Result<()> {
        assert_eq!("long", ArrayType::Long.to_string());
        test_array_type(&ArrayType::Long, 11)
    }
}
