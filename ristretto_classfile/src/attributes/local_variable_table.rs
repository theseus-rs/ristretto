use crate::error::Result;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fmt;
use std::io::Cursor;

/// Implementation of `LocalVariableTable`.
///
/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.7.13>
#[derive(Clone, Debug, PartialEq)]
pub struct LocalVariableTable {
    pub start_pc: u16,
    pub length: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub index: u16,
}

impl LocalVariableTable {
    /// Deserialize the local variable table from bytes.
    ///
    /// # Errors
    /// Should not occur; reserved for future use.
    pub fn from_bytes(bytes: &mut Cursor<Vec<u8>>) -> Result<LocalVariableTable> {
        let start_pc = bytes.read_u16::<BigEndian>()?;
        let length = bytes.read_u16::<BigEndian>()?;
        let name_index = bytes.read_u16::<BigEndian>()?;
        let descriptor_index = bytes.read_u16::<BigEndian>()?;
        let index = bytes.read_u16::<BigEndian>()?;

        let local_variable_target = LocalVariableTable {
            start_pc,
            length,
            name_index,
            descriptor_index,
            index,
        };
        Ok(local_variable_target)
    }

    /// Serialize the local variable table to bytes.
    ///
    /// # Errors
    /// Should not occur; reserved for future use.
    pub fn to_bytes(&self, bytes: &mut Vec<u8>) -> Result<()> {
        bytes.write_u16::<BigEndian>(self.start_pc)?;
        bytes.write_u16::<BigEndian>(self.length)?;
        bytes.write_u16::<BigEndian>(self.name_index)?;
        bytes.write_u16::<BigEndian>(self.descriptor_index)?;
        bytes.write_u16::<BigEndian>(self.index)?;
        Ok(())
    }
}

impl fmt::Display for LocalVariableTable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "start_pc: {}, length: {}, name_index: {}, descriptor_index: {}, index: {}",
            self.start_pc, self.length, self.name_index, self.descriptor_index, self.index
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_to_string() {
        let local_variable_table = LocalVariableTable {
            start_pc: 1,
            length: 2,
            name_index: 3,
            descriptor_index: 4,
            index: 5,
        };

        assert_eq!(
            "start_pc: 1, length: 2, name_index: 3, descriptor_index: 4, index: 5",
            local_variable_table.to_string()
        );
    }

    #[test]
    fn test_serialization() -> Result<()> {
        let local_variable_table = LocalVariableTable {
            start_pc: 1,
            length: 2,
            name_index: 3,
            descriptor_index: 4,
            index: 5,
        };
        let expected_value = [0, 1, 0, 2, 0, 3, 0, 4, 0, 5];
        let mut bytes = Vec::new();
        local_variable_table.clone().to_bytes(&mut bytes)?;
        assert_eq!(expected_value, &bytes[..]);

        let mut bytes = Cursor::new(expected_value.to_vec());
        assert_eq!(
            local_variable_table,
            LocalVariableTable::from_bytes(&mut bytes)?
        );
        Ok(())
    }
}
