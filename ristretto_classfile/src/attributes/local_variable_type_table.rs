use crate::error::Result;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fmt;
use std::io::Cursor;

/// Represents an entry in the `LocalVariableTypeTable` attribute, describing a single local
/// variable that has a generic type signature.
///
/// The `LocalVariableTypeTable` attribute is an optional attribute in the `Code` attribute of a
/// method. It is an extension of the `LocalVariableTable` attribute and is used by debuggers to
/// determine the generic type of a local variable, in addition to its name and descriptor.
///
/// **Note on PC representation:** Similar to `LocalVariableTable`, `start_pc` and `length` define
/// a range of instruction indices.
///
/// See the [JVM specification for the LocalVariableTypeTable attribute](https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.7.14)
/// for more details.
///
/// # Fields
///
/// - `start_pc`: The instruction index from which the local variable is in scope.
/// - `length`: The number of subsequent instruction indices for which the variable remains in scope.
/// - `name_index`: An index into the `constant_pool` for the `CONSTANT_Utf8_info` of the variable's name.
/// - `signature_index`: An index into the `constant_pool` for the `CONSTANT_Utf8_info` of the variable's
///   generic type signature (a field signature, not a field descriptor).
/// - `index`: The local variable's index in the current frame's local variable array.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// use ristretto_classfile::attributes::LocalVariableTypeTable;
/// use std::io::Cursor;
///
/// // Example: A local variable "myList" of generic type List<String>,
/// //          in slot 2, scoped from instruction index 8 for 15 instructions.
/// let lvt_entry = LocalVariableTypeTable {
///     start_pc: 8,         // Scope starts at instruction index 8
///     length: 15,          // In scope for 15 instructions (indices 8-22)
///     name_index: 200,     // CP index for Utf8 "myList"
///     signature_index: 201, // CP index for Utf8 "Ljava/util/List<Ljava/lang/String;>;"
///     index: 2,            // Local variable array slot 2
/// };
///
/// // Serialize the entry
/// let mut bytes = Vec::new();
/// lvt_entry.to_bytes(&mut bytes)?;
///
/// // Deserialize the entry
/// let mut cursor = Cursor::new(bytes);
/// let deserialized_entry = LocalVariableTypeTable::from_bytes(&mut cursor)?;
///
/// assert_eq!(lvt_entry, deserialized_entry);
/// assert_eq!(lvt_entry.to_string(), "start_pc: 8, length: 15, name_index: 200, signature_index: 201, index: 2");
/// # Ok::<(), ristretto_classfile::Error>(())
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct LocalVariableTypeTable {
    pub start_pc: u16,
    pub length: u16,
    pub name_index: u16,
    pub signature_index: u16,
    pub index: u16,
}

impl LocalVariableTypeTable {
    /// Deserializes a `LocalVariableTypeTable` entry from a byte stream.
    ///
    /// Reads all fields from the stream. `start_pc` and `length` may need mapping from byte offsets
    /// to logical instruction indices/counts.
    ///
    /// # Errors
    ///
    /// Returns an error if reading from the byte stream fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::LocalVariableTypeTable;
    /// use std::io::Cursor;
    ///
    /// // Byte data for an entry: start_pc=2, len=10, name=3, sig=4, idx=1
    /// let data = vec![0, 2, 0, 10, 0, 3, 0, 4, 0, 1];
    /// let mut cursor = Cursor::new(data);
    ///
    /// let lvt_entry = LocalVariableTypeTable::from_bytes(&mut cursor)?;
    ///
    /// assert_eq!(lvt_entry.start_pc, 2);
    /// assert_eq!(lvt_entry.length, 10);
    /// assert_eq!(lvt_entry.name_index, 3);
    /// assert_eq!(lvt_entry.signature_index, 4);
    /// assert_eq!(lvt_entry.index, 1);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    pub fn from_bytes(bytes: &mut Cursor<Vec<u8>>) -> Result<LocalVariableTypeTable> {
        let start_pc = bytes.read_u16::<BigEndian>()?;
        let length = bytes.read_u16::<BigEndian>()?;
        let name_index = bytes.read_u16::<BigEndian>()?;
        let signature_index = bytes.read_u16::<BigEndian>()?;
        let index = bytes.read_u16::<BigEndian>()?;

        let inner_class = LocalVariableTypeTable {
            start_pc,
            length,
            name_index,
            signature_index,
            index,
        };
        Ok(inner_class)
    }

    /// Serializes the `LocalVariableTypeTable` entry to a byte vector.
    ///
    /// Writes all fields to the vector. `start_pc` and `length` are written directly; conversion to
    /// byte offsets may be needed prior to calling this.
    ///
    /// # Errors
    ///
    /// Returns an error if writing to the byte vector fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::LocalVariableTypeTable;
    ///
    /// let lvt_entry = LocalVariableTypeTable {
    ///     start_pc: 50,
    ///     length: 20,
    ///     name_index: 8,
    ///     signature_index: 9,
    ///     index: 4,
    /// };
    ///
    /// let mut bytes = Vec::new();
    /// lvt_entry.to_bytes(&mut bytes)?;
    ///
    /// // Expected: start_pc(50), length(20), name(8), sig(9), index(4)
    /// assert_eq!(bytes, vec![0, 50, 0, 20, 0, 8, 0, 9, 0, 4]);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    pub fn to_bytes(&self, bytes: &mut Vec<u8>) -> Result<()> {
        bytes.write_u16::<BigEndian>(self.start_pc)?;
        bytes.write_u16::<BigEndian>(self.length)?;
        bytes.write_u16::<BigEndian>(self.name_index)?;
        bytes.write_u16::<BigEndian>(self.signature_index)?;
        bytes.write_u16::<BigEndian>(self.index)?;
        Ok(())
    }
}

impl fmt::Display for LocalVariableTypeTable {
    /// Implements the `Display` trait for `LocalVariableTypeTable`.
    ///
    /// This implementation provides a human-readable string representation of a
    /// `LocalVariableTypeTable` entry, showing all fields in a comma-separated format.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::LocalVariableTypeTable;
    /// use std::fmt::Display;
    ///
    /// let entry = LocalVariableTypeTable {
    ///     start_pc: 10,
    ///     length: 25,
    ///     name_index: 42,
    ///     signature_index: 43,
    ///     index: 3,
    /// };
    ///
    /// let output = entry.to_string();
    /// assert_eq!(
    ///     output,
    ///     "start_pc: 10, length: 25, name_index: 42, signature_index: 43, index: 3"
    /// );
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "start_pc: {}, length: {}, name_index: {}, signature_index: {}, index: {}",
            self.start_pc, self.length, self.name_index, self.signature_index, self.index
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_display() {
        let local_variable_type_table = LocalVariableTypeTable {
            start_pc: 1,
            length: 2,
            name_index: 3,
            signature_index: 4,
            index: 5,
        };
        assert_eq!(
            "start_pc: 1, length: 2, name_index: 3, signature_index: 4, index: 5",
            local_variable_type_table.to_string()
        );
    }

    #[test]
    fn test_serialization() -> Result<()> {
        let local_variable_type_table = LocalVariableTypeTable {
            start_pc: 1,
            length: 2,
            name_index: 3,
            signature_index: 4,
            index: 5,
        };
        let expected_value = [0, 1, 0, 2, 0, 3, 0, 4, 0, 5];
        let mut bytes = Vec::new();
        local_variable_type_table.clone().to_bytes(&mut bytes)?;
        assert_eq!(expected_value, &bytes[..]);

        let mut bytes = Cursor::new(expected_value.to_vec());
        assert_eq!(
            local_variable_type_table,
            LocalVariableTypeTable::from_bytes(&mut bytes)?
        );
        Ok(())
    }
}
