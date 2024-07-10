use crate::error::Result;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fmt;
use std::io::Cursor;

/// Implementation of `LineNumber`.
///
/// See: <https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.7.12>
#[derive(Clone, Debug, PartialEq)]
pub struct LineNumber {
    pub start_pc: u16,
    pub line_number: u16,
}

impl fmt::Display for LineNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.start_pc, self.line_number)
    }
}

impl LineNumber {
    /// Deserialize the line number from bytes.
    ///
    /// # Errors
    /// Should not occur; reserved for future use.
    pub fn from_bytes(bytes: &mut Cursor<Vec<u8>>) -> Result<LineNumber> {
        let line_number = LineNumber {
            start_pc: bytes.read_u16::<BigEndian>()?,
            line_number: bytes.read_u16::<BigEndian>()?,
        };
        Ok(line_number)
    }

    /// Serialize the line number to bytes.
    ///
    /// # Errors
    /// Should not occur; reserved for future use.
    pub fn to_bytes(&self, bytes: &mut Vec<u8>) -> Result<()> {
        bytes.write_u16::<BigEndian>(self.start_pc)?;
        bytes.write_u16::<BigEndian>(self.line_number)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_serialization() -> Result<()> {
        let line_number = LineNumber {
            start_pc: 1,
            line_number: 42,
        };
        let expected_value = [0, 1, 0, 42];
        let mut bytes = Vec::new();

        assert_eq!("1: 42", line_number.to_string());

        line_number.clone().to_bytes(&mut bytes)?;
        assert_eq!(expected_value, &bytes[..]);

        let mut bytes = Cursor::new(expected_value.to_vec());
        assert_eq!(line_number, LineNumber::from_bytes(&mut bytes)?);
        Ok(())
    }
}
