use crate::error::Result;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fmt;
use std::io::Cursor;

/// Implementation of `CodeException`.
///
/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.7.3>
#[derive(Clone, Debug, PartialEq)]
pub struct CodeException {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: u16,
}

impl CodeException {
    /// Deserialize the code exception from bytes.
    ///
    /// # Errors
    /// Should not occur; reserved for future use.
    pub fn from_bytes(bytes: &mut Cursor<Vec<u8>>) -> Result<CodeException> {
        let start_pc = bytes.read_u16::<BigEndian>()?;
        let end_pc = bytes.read_u16::<BigEndian>()?;
        let handler_pc = bytes.read_u16::<BigEndian>()?;
        let catch_type = bytes.read_u16::<BigEndian>()?;
        let code_exception = CodeException {
            start_pc,
            end_pc,
            handler_pc,
            catch_type,
        };
        Ok(code_exception)
    }

    /// Serialize the code exception to bytes.
    ///
    /// # Errors
    /// Should not occur; reserved for future use.
    pub fn to_bytes(&self, bytes: &mut Vec<u8>) -> Result<()> {
        bytes.write_u16::<BigEndian>(self.start_pc)?;
        bytes.write_u16::<BigEndian>(self.end_pc)?;
        bytes.write_u16::<BigEndian>(self.handler_pc)?;
        bytes.write_u16::<BigEndian>(self.catch_type)?;
        Ok(())
    }
}

impl fmt::Display for CodeException {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "start_pc: {}, end_pc: {}, handler_pc: {}, catch_type: {}",
            self.start_pc, self.end_pc, self.handler_pc, self.catch_type
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_to_string() {
        let code_exception = CodeException {
            start_pc: 1,
            end_pc: 2,
            handler_pc: 3,
            catch_type: 4,
        };
        assert_eq!(
            "start_pc: 1, end_pc: 2, handler_pc: 3, catch_type: 4",
            code_exception.to_string()
        );
    }

    #[test]
    fn test_serialization() -> Result<()> {
        let bootstrap_method = CodeException {
            start_pc: 1,
            end_pc: 2,
            handler_pc: 3,
            catch_type: 4,
        };
        let expected_value = [0, 1, 0, 2, 0, 3, 0, 4];

        let mut bytes = Vec::new();
        bootstrap_method.clone().to_bytes(&mut bytes)?;
        assert_eq!(expected_value, &bytes[..]);

        let mut bytes = Cursor::new(expected_value.to_vec());
        assert_eq!(bootstrap_method, CodeException::from_bytes(&mut bytes)?);
        Ok(())
    }
}
