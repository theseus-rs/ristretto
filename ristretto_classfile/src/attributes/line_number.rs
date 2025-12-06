use crate::error::Result;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fmt;
use std::io::Cursor;

/// Represents an entry in the `LineNumberTable` attribute, mapping a bytecode instruction start PC
/// (program counter) to a source file line number.
///
/// The `LineNumberTable` attribute is an optional attribute in the `Code` attribute of a method. It
/// is used by debuggers to determine the source line corresponding to a given bytecode instruction.
///
/// **Note on PC representation:** In this implementation, `start_pc` represents an index into the
/// logical sequence of `Instruction`s within the `Code` attribute, rather than a raw byte offset as
/// in the class file format. This abstraction simplifies instruction manipulation.
///
/// See the [JVM Specification §4.7.12](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.12)
/// for more details.
///
/// # Fields
///
/// - `start_pc`: The instruction index (program counter) at which the new line number begins.
/// - `line_number`: The corresponding line number in the original source file.
///
/// # Examples
///
/// ```rust
/// use ristretto_classfile::attributes::LineNumber;
/// use std::io::Cursor;
///
/// // Create a LineNumber entry: instruction at index 10 corresponds to source line 42
/// let ln_entry = LineNumber {
///     start_pc: 10,    // Instruction index
///     line_number: 42, // Source line number
/// };
///
/// // Serialize the entry
/// let mut bytes = Vec::new();
/// ln_entry.to_bytes(&mut bytes)?;
///
/// // Deserialize the entry
/// let mut cursor = Cursor::new(bytes);
/// let deserialized_entry = LineNumber::from_bytes(&mut cursor)?;
///
/// assert_eq!(ln_entry, deserialized_entry);
/// assert_eq!(ln_entry.to_string(), "10: 42");
/// # Ok::<(), ristretto_classfile::Error>(())
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LineNumber {
    pub start_pc: u16,
    pub line_number: u16,
}

impl fmt::Display for LineNumber {
    /// Formats the `LineNumber` entry as a string.
    ///
    /// This implementation produces a string in the format `"start_pc: line_number"`, which
    /// concisely represents the line number mapping for debugging or display purposes.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::LineNumber;
    ///
    /// let ln_entry = LineNumber {
    ///     start_pc: 5,
    ///     line_number: 42,
    /// };
    ///
    /// assert_eq!(ln_entry.to_string(), "5: 42");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.start_pc, self.line_number)
    }
}

impl LineNumber {
    /// Deserializes a `LineNumber` entry from a byte stream.
    ///
    /// Reads `start_pc` and `line_number` from the stream. Note that `start_pc` is read as a raw PC
    /// value (typically a byte offset in a class file) and may need further mapping to a logical
    /// instruction index depending on the context (e.g., when parsing a `Code` attribute).
    ///
    /// # Errors
    ///
    /// Returns an error if reading from the byte stream fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::LineNumber;
    /// use std::io::Cursor;
    ///
    /// // Byte data for a LineNumber entry: start_pc = 5, line_number = 20
    /// let data = vec![0, 5, 0, 20];
    /// let mut cursor = Cursor::new(data);
    ///
    /// let ln_entry = LineNumber::from_bytes(&mut cursor)?;
    ///
    /// assert_eq!(ln_entry.start_pc, 5);
    /// assert_eq!(ln_entry.line_number, 20);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    pub fn from_bytes(bytes: &mut Cursor<Vec<u8>>) -> Result<LineNumber> {
        let line_number = LineNumber {
            start_pc: bytes.read_u16::<BigEndian>()?,
            line_number: bytes.read_u16::<BigEndian>()?,
        };
        Ok(line_number)
    }

    /// Serializes the `LineNumber` entry to a byte vector.
    ///
    /// Writes `start_pc` and `line_number` to the vector. Note that `start_pc` is written directly;
    /// if it represents a logical instruction index, it must be converted to a byte offset before
    /// serialization in the context of a `Code` attribute.
    ///
    /// # Errors
    ///
    /// Returns an error if writing to the byte vector fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::LineNumber;
    ///
    /// let ln_entry = LineNumber {
    ///     start_pc: 100,   // Byte offset or instruction index
    ///     line_number: 55,
    /// };
    ///
    /// let mut bytes = Vec::new();
    /// ln_entry.to_bytes(&mut bytes)?;
    ///
    /// // Expected: start_pc (0x0064), line_number (0x0037)
    /// assert_eq!(bytes, vec![0, 100, 0, 55]);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
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
