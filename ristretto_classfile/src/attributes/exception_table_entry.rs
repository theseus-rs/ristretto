use crate::error::Result;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fmt;
use std::io::Cursor;
use std::ops::Range;

/// Implementation of an `ExceptionTable` entry.
///
/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.7.3>
#[derive(Clone, Debug, PartialEq)]
pub struct ExceptionTableEntry {
    pub range_pc: Range<u16>,
    pub handler_pc: u16,
    pub catch_type: u16,
}

impl ExceptionTableEntry {
    /// Deserialize the code exception from bytes.
    ///
    /// # Errors
    /// Should not occur; reserved for future use.
    pub fn from_bytes(bytes: &mut Cursor<Vec<u8>>) -> Result<ExceptionTableEntry> {
        let start_pc = bytes.read_u16::<BigEndian>()?;
        let end_pc = bytes.read_u16::<BigEndian>()?;
        let range_pc = start_pc..end_pc;
        let handler_pc = bytes.read_u16::<BigEndian>()?;
        let catch_type = bytes.read_u16::<BigEndian>()?;
        let exception_table_entry = ExceptionTableEntry {
            range_pc,
            handler_pc,
            catch_type,
        };
        Ok(exception_table_entry)
    }

    /// Serialize the code exception to bytes.
    ///
    /// # Errors
    /// Should not occur; reserved for future use.
    pub fn to_bytes(&self, bytes: &mut Vec<u8>) -> Result<()> {
        bytes.write_u16::<BigEndian>(self.range_pc.start)?;
        bytes.write_u16::<BigEndian>(self.range_pc.end)?;
        bytes.write_u16::<BigEndian>(self.handler_pc)?;
        bytes.write_u16::<BigEndian>(self.catch_type)?;
        Ok(())
    }
}

impl fmt::Display for ExceptionTableEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "start_pc: {}, end_pc: {}, handler_pc: {}, catch_type: {}",
            self.range_pc.start, self.range_pc.end, self.handler_pc, self.catch_type
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_to_string() {
        let exception_table_entry = ExceptionTableEntry {
            range_pc: 1..2,
            handler_pc: 3,
            catch_type: 4,
        };
        assert_eq!(
            "start_pc: 1, end_pc: 2, handler_pc: 3, catch_type: 4",
            exception_table_entry.to_string()
        );
    }

    #[test]
    fn test_serialization() -> Result<()> {
        let exception_table_entry = ExceptionTableEntry {
            range_pc: 1..2,
            handler_pc: 3,
            catch_type: 4,
        };
        let expected_value = [0, 1, 0, 2, 0, 3, 0, 4];

        let mut bytes = Vec::new();
        exception_table_entry.clone().to_bytes(&mut bytes)?;
        assert_eq!(expected_value, &bytes[..]);

        let mut bytes = Cursor::new(expected_value.to_vec());
        assert_eq!(
            exception_table_entry,
            ExceptionTableEntry::from_bytes(&mut bytes)?
        );
        Ok(())
    }

    /// This test is largely unnecessary as it is mostly testing the `Range` type.  However, it does
    /// ensure that the range start is inclusive and the end is exclusive per the JVM specification.
    #[test]
    fn test_range() {
        let exception_table_entry = ExceptionTableEntry {
            range_pc: 1..3,
            handler_pc: 8,
            catch_type: 42,
        };
        assert!(!exception_table_entry.range_pc.contains(&0));
        assert!(exception_table_entry.range_pc.contains(&1));
        assert!(exception_table_entry.range_pc.contains(&2));
        assert!(!exception_table_entry.range_pc.contains(&3));
    }
}
