use crate::error::Result;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fmt;
use std::io::Cursor;

/// Represents an entry in the `LocalVariableTable` attribute, describing a single local variable.
///
/// The `LocalVariableTable` attribute is an optional attribute in the `Code` attribute of a method.
/// It is used by debuggers to determine the name and type of a local variable at a given point
/// in the method's execution.
///
/// **Note on PC representation:** In this implementation, `start_pc` and `length` define a range
/// of instruction indices. `start_pc` is the first instruction index where the variable is in scope,
/// and `start_pc + length` is the first instruction index where it is no longer in scope. This
/// differs from the raw byte offsets in the class file format.
///
/// See the [JVMS §4.7.13](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.13)
/// for more details.
///
/// # Fields
///
/// - `start_pc`: The instruction index (program counter) from which the local variable is in scope.
/// - `length`: The number of subsequent instruction indices for which the variable remains in scope.
///   The variable is in scope from `start_pc` to `start_pc + length - 1` inclusive.
/// - `name_index`: An index into the `constant_pool` table. The entry at this index must be a
///   `CONSTANT_Utf8_info` structure representing the name of the local variable.
/// - `descriptor_index`: An index into the `constant_pool` table. The entry at this index must be a
///   `CONSTANT_Utf8_info` structure representing a field descriptor encoding the type of the
///   local variable.
/// - `index`: The local variable's index in the current frame's local variable array.
///   If the local variable is of type `long` or `double`, it occupies `index` and `index + 1`.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// use ristretto_classfile::attributes::LocalVariableTable;
/// use std::io::Cursor;
///
/// // Example: A local variable "myVar" of type int, in slot 1,
/// //          scoped from instruction index 5 for 10 instructions.
/// let lv_entry = LocalVariableTable {
///     start_pc: 5,         // Scope starts at instruction index 5
///     length: 10,          // In scope for 10 instructions (indices 5-14)
///     name_index: 100,     // CP index for Utf8 "myVar"
///     descriptor_index: 101, // CP index for Utf8 "I" (int descriptor)
///     index: 1,            // Local variable array slot 1
/// };
///
/// // Serialize the entry
/// let mut bytes = Vec::new();
/// lv_entry.to_bytes(&mut bytes)?;
///
/// // Deserialize the entry
/// let mut cursor = Cursor::new(bytes);
/// let deserialized_entry = LocalVariableTable::from_bytes(&mut cursor)?;
///
/// assert_eq!(lv_entry, deserialized_entry);
/// assert_eq!(lv_entry.to_string(), "start_pc: 5, length: 10, name_index: 100, descriptor_index: 101, index: 1");
/// # Ok::<(), ristretto_classfile::Error>(())
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalVariableTable {
    pub start_pc: u16,
    pub length: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub index: u16,
}

impl LocalVariableTable {
    /// Deserializes a `LocalVariableTable` entry from a byte stream.
    ///
    /// Reads `start_pc`, `length`, `name_index`, `descriptor_index`, and `index` from the stream.
    /// Note that `start_pc` and `length` are read as raw PC values/lengths (typically byte offsets
    /// in a class file) and may need further mapping to logical instruction indices/counts
    /// depending on the context.
    ///
    /// # Errors
    ///
    /// Returns an error if reading from the byte stream fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::LocalVariableTable;
    /// use std::io::Cursor;
    ///
    /// // Byte data for a LocalVariableTable entry:
    /// // start_pc=10, length=50, name_idx=1, desc_idx=2, index=3
    /// let data = vec![0, 10, 0, 50, 0, 1, 0, 2, 0, 3];
    /// let mut cursor = Cursor::new(data);
    ///
    /// let lv_entry = LocalVariableTable::from_bytes(&mut cursor)?;
    ///
    /// assert_eq!(lv_entry.start_pc, 10);
    /// assert_eq!(lv_entry.length, 50);
    /// assert_eq!(lv_entry.name_index, 1);
    /// assert_eq!(lv_entry.descriptor_index, 2);
    /// assert_eq!(lv_entry.index, 3);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    pub fn from_bytes(bytes: &mut Cursor<impl AsRef<[u8]>>) -> Result<LocalVariableTable> {
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

    /// Serializes the `LocalVariableTable` entry to a byte vector.
    ///
    /// Writes `start_pc`, `length`, `name_index`, `descriptor_index`, and `index` to the vector.
    /// Note that `start_pc` and `length` are written directly; if they represent logical
    /// instruction indices/counts, they must be converted to byte offsets/lengths before
    /// serialization in the context of a `Code` attribute.
    ///
    /// # Errors
    ///
    /// Returns an error if writing to the byte vector fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::LocalVariableTable;
    ///
    /// let lv_entry = LocalVariableTable {
    ///     start_pc: 0,
    ///     length: 100,
    ///     name_index: 5,
    ///     descriptor_index: 6,
    ///     index: 0,
    /// };
    ///
    /// let mut bytes = Vec::new();
    /// lv_entry.to_bytes(&mut bytes)?;
    ///
    /// // Expected: start_pc(0), length(100), name(5), desc(6), index(0)
    /// assert_eq!(bytes, vec![0, 0, 0, 100, 0, 5, 0, 6, 0, 0]);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
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
    /// Formats the `LocalVariableTable` entry as a human-readable string.
    ///
    /// This implementation provides a simple comma-separated representation of all fields in the
    /// `LocalVariableTable` entry, making it useful for debugging and logging.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::LocalVariableTable;
    ///
    /// let lv_entry = LocalVariableTable {
    ///     start_pc: 5,
    ///     length: 10,
    ///     name_index: 15,
    ///     descriptor_index: 20,
    ///     index: 2,
    /// };
    ///
    /// // Using to_string() to get the formatted string
    /// let output = lv_entry.to_string();
    /// assert_eq!(
    ///     output,
    ///     "start_pc: 5, length: 10, name_index: 15, descriptor_index: 20, index: 2"
    /// );
    /// ```
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
