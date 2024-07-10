use crate::error::Result;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fmt;
use std::io::Cursor;

/// Implementation of `LocalVariableTarget`.
///
/// See: <https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.7.20>
#[derive(Clone, Debug, PartialEq)]
pub struct LocalVariableTarget {
    pub start_pc: u16,
    pub length: u16,
    pub index: u16,
}

impl LocalVariableTarget {
    /// Deserialize the local variable target from bytes.
    ///
    /// # Errors
    /// Should not occur; reserved for future use.
    pub fn from_bytes(bytes: &mut Cursor<Vec<u8>>) -> Result<LocalVariableTarget> {
        let start_pc = bytes.read_u16::<BigEndian>()?;
        let length = bytes.read_u16::<BigEndian>()?;
        let index = bytes.read_u16::<BigEndian>()?;

        let local_variable_target = LocalVariableTarget {
            start_pc,
            length,
            index,
        };
        Ok(local_variable_target)
    }

    /// Serialize the local variable target to bytes.
    ///
    /// # Errors
    /// Should not occur; reserved for future use.
    pub fn to_bytes(&self, bytes: &mut Vec<u8>) -> Result<()> {
        bytes.write_u16::<BigEndian>(self.start_pc)?;
        bytes.write_u16::<BigEndian>(self.length)?;
        bytes.write_u16::<BigEndian>(self.index)?;
        Ok(())
    }
}

impl fmt::Display for LocalVariableTarget {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "start_pc: {}, length: {}, index: {}",
            self.start_pc, self.length, self.index
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_display() {
        let local_variable_target = LocalVariableTarget {
            start_pc: 1,
            length: 2,
            index: 3,
        };
        assert_eq!(
            "start_pc: 1, length: 2, index: 3",
            local_variable_target.to_string()
        );
    }

    #[test]
    fn test_serialization() -> Result<()> {
        let local_variable_target = LocalVariableTarget {
            start_pc: 1,
            length: 2,
            index: 3,
        };
        let expected_value = [0, 1, 0, 2, 0, 3];
        let mut bytes = Vec::new();
        local_variable_target.clone().to_bytes(&mut bytes)?;
        assert_eq!(expected_value, &bytes[..]);

        let mut bytes = Cursor::new(expected_value.to_vec());
        assert_eq!(
            local_variable_target,
            LocalVariableTarget::from_bytes(&mut bytes)?
        );
        Ok(())
    }
}
