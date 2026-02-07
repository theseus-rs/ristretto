use crate::error::Result;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fmt;
use std::io::Cursor;
use std::ops::Range;

/// Represents an entry in the exception table of a `Code` attribute.
///
/// Each entry defines a range of bytecode instructions within which an exception handler is active.
/// If an exception is thrown while the program counter (PC) is within this range (inclusive start,
/// exclusive end) and the thrown exception is an instance of the `catch_type` (or a subclass),
/// control is transferred to the `handler_pc`.
///
/// **Note on PC representation:** In this implementation, `start_pc`, `end_pc`, and `handler_pc`
/// represent indices into the logical sequence of `Instruction`s within the `Code` attribute,
/// rather than raw byte offsets as in the class file format. This abstraction simplifies
/// instruction manipulation and analysis.
///
/// See the [JVMS §4.7.3](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.3)
/// for more details (specifically, the `exception_table` within the `Code` attribute).
///
/// # Examples
///
/// ```rust
/// use ristretto_classfile::attributes::ExceptionTableEntry;
/// use std::io::Cursor;
///
/// // Create an ExceptionTableEntry
/// // This handler covers instructions from index 5 up to (but not including) index 10.
/// // If an exception of type indicated by constant pool index 7 occurs in this range,
/// // execution jumps to instruction index 12.
/// let entry = ExceptionTableEntry {
///     range_pc: 5..10,      // start_pc = 5, end_pc = 10
///     handler_pc: 12,
///     catch_type: 7,        // Index to a CONSTANT_Class_info for the exception type
/// };
///
/// // Serialize to bytes (assuming these are byte offsets for serialization context)
/// let mut bytes = Vec::new();
/// entry.to_bytes(&mut bytes)?;
///
/// // Deserialize from bytes
/// let mut cursor = Cursor::new(bytes);
/// let deserialized_entry = ExceptionTableEntry::from_bytes(&mut cursor)?;
///
/// assert_eq!(entry, deserialized_entry);
///
/// // Check PC range
/// assert!(entry.range_pc.contains(&5));
/// assert!(entry.range_pc.contains(&9));
/// assert!(!entry.range_pc.contains(&10));
/// # Ok::<(), ristretto_classfile::Error>(())
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExceptionTableEntry {
    pub range_pc: Range<u16>,
    pub handler_pc: u16,
    pub catch_type: u16,
}

impl ExceptionTableEntry {
    /// Deserializes an `ExceptionTableEntry` from a byte stream.
    ///
    /// Reads `start_pc`, `end_pc`, `handler_pc`, and `catch_type` from the stream. Note that these
    /// are read as raw PC values (typically byte offsets in a class file) and may need further
    /// mapping to logical instruction indices depending on the context (e.g., when parsing a
    /// `Code` attribute).
    ///
    /// # Errors
    ///
    /// Returns an error if reading from the byte stream fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::ExceptionTableEntry;
    /// use std::io::Cursor;
    ///
    /// // Byte data for an exception table entry:
    /// // start_pc = 1, end_pc = 5, handler_pc = 10, catch_type = 2
    /// let data = vec![0, 1, 0, 5, 0, 10, 0, 2];
    /// let mut cursor = Cursor::new(data);
    ///
    /// let entry = ExceptionTableEntry::from_bytes(&mut cursor)?;
    ///
    /// assert_eq!(entry.range_pc.start, 1);
    /// assert_eq!(entry.range_pc.end, 5);
    /// assert_eq!(entry.handler_pc, 10);
    /// assert_eq!(entry.catch_type, 2);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    pub fn from_bytes(bytes: &mut Cursor<impl AsRef<[u8]>>) -> Result<ExceptionTableEntry> {
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

    /// Serializes the `ExceptionTableEntry` to a byte vector.
    ///
    /// Writes `range_pc.start`, `range_pc.end`, `handler_pc`, and `catch_type` to the vector. Note
    /// that these PC values are written directly; if they represent logical instruction indices,
    /// they must be converted to byte offsets before serialization in the context of a `Code`
    /// attribute.
    ///
    /// # Errors
    ///
    /// Returns an error if writing to the byte vector fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::ExceptionTableEntry;
    ///
    /// let entry = ExceptionTableEntry {
    ///     range_pc: 0..100,  // start_pc=0, end_pc=100
    ///     handler_pc: 150,
    ///     catch_type: 0,    // Catch any exception (finally block)
    /// };
    ///
    /// let mut bytes = Vec::new();
    /// entry.to_bytes(&mut bytes)?;
    ///
    /// // Expected: start_pc (0x0000), end_pc (0x0064), handler_pc (0x0096), catch_type (0x0000)
    /// assert_eq!(bytes, vec![0, 0, 0, 100, 0, 150, 0, 0]);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    pub fn to_bytes(&self, bytes: &mut Vec<u8>) -> Result<()> {
        bytes.write_u16::<BigEndian>(self.range_pc.start)?;
        bytes.write_u16::<BigEndian>(self.range_pc.end)?;
        bytes.write_u16::<BigEndian>(self.handler_pc)?;
        bytes.write_u16::<BigEndian>(self.catch_type)?;
        Ok(())
    }
}

impl fmt::Display for ExceptionTableEntry {
    /// Formats the `ExceptionTableEntry` as a human-readable string.
    ///
    /// This implementation displays all fields of the entry in a clear format, making it useful for
    /// debugging and logging purposes.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::ExceptionTableEntry;
    ///
    /// let entry = ExceptionTableEntry {
    ///     range_pc: 10..20,
    ///     handler_pc: 50,
    ///     catch_type: 15,
    /// };
    ///
    /// let output = entry.to_string();
    /// assert_eq!(output, "start_pc: 10, end_pc: 20, handler_pc: 50, catch_type: 15");
    /// ```
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
