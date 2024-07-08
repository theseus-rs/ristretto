use crate::error::Result;
use crate::Error::InvalidArrayTypeCode;
use byteorder::{ReadBytesExt, WriteBytesExt};
use std::io::Cursor;

/// Implementation of `ArrayType`.
///
/// See: <https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-6.html#jvms-6.5.newarray>
#[derive(Clone, Debug, PartialEq)]
pub enum ArrayType {
    Boolean,
    Char,
    Float,
    Double,
    Byte,
    Short,
    Int,
    Long,
}

impl ArrayType {
    /// Return the code for the array type.
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

    /// Deserialize the array type from bytes.
    ///
    /// # Errors
    /// If the code is not a valid array type code.
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

    /// Serialize the array type to bytes.
    ///
    /// # Errors
    /// Should not occur; reserved for future use.
    pub fn to_bytes(&self, bytes: &mut Cursor<Vec<u8>>) -> Result<()> {
        bytes.write_u8(self.code())?;
        Ok(())
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
        test_array_type(&ArrayType::Boolean, 4)
    }

    #[test]
    fn test_char() -> Result<()> {
        test_array_type(&ArrayType::Char, 5)
    }

    #[test]
    fn test_float() -> Result<()> {
        test_array_type(&ArrayType::Float, 6)
    }

    #[test]
    fn test_double() -> Result<()> {
        test_array_type(&ArrayType::Double, 7)
    }

    #[test]
    fn test_byte() -> Result<()> {
        test_array_type(&ArrayType::Byte, 8)
    }

    #[test]
    fn test_short() -> Result<()> {
        test_array_type(&ArrayType::Short, 9)
    }

    #[test]
    fn test_int() -> Result<()> {
        test_array_type(&ArrayType::Int, 10)
    }

    #[test]
    fn test_long() -> Result<()> {
        test_array_type(&ArrayType::Long, 11)
    }
}
