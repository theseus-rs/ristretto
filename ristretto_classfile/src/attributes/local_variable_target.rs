use crate::error::Result;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fmt;
use std::io::Cursor;

/// Represents the `localvar_target` structure, part of a `TypeAnnotation` when the `target_type`
/// indicates an annotation on a local variable declaration.
///
/// This structure specifies which local variable is being annotated by identifying its scope and
/// its index in the local variable array of the current stack frame.
///
/// **Note on PC representation:** Similar to `LineNumber` and `LocalVariableTable` entries,
/// `start_pc` and `length` here define a range of instruction indices within the method's code.
///
/// See the [JVM Specification §4.7.20.1](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.20.1)
/// as part of the `TypeAnnotation` structure.
///
/// # Fields
///
/// - `start_pc`: The instruction index (program counter) from which the local variable is in scope.
/// - `length`: The number of subsequent instruction indices for which the variable remains in
///   scope. The variable is in scope from `start_pc` to `start_pc + length - 1` inclusive.
/// - `index`: The local variable's index in the current frame's local variable array.
///
/// # Examples
///
/// ```rust
/// use ristretto_classfile::attributes::LocalVariableTarget;
/// use std::io::Cursor;
///
/// // Example: Targeting a local variable in slot 3,
/// //          scoped from instruction index 20 for 5 instructions.
/// let lv_target = LocalVariableTarget {
///     start_pc: 20,    // Scope starts at instruction index 20
///     length: 5,       // In scope for 5 instructions (indices 20-24)
///     index: 3,        // Local variable array slot 3
/// };
///
/// // Serialize the target
/// let mut bytes = Vec::new();
/// lv_target.to_bytes(&mut bytes)?;
///
/// // Deserialize the target
/// let mut cursor = Cursor::new(bytes);
/// let deserialized_target = LocalVariableTarget::from_bytes(&mut cursor)?;
///
/// assert_eq!(lv_target, deserialized_target);
/// assert_eq!(lv_target.to_string(), "start_pc: 20, length: 5, index: 3");
/// # Ok::<(), ristretto_classfile::Error>(())
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalVariableTarget {
    pub start_pc: u16,
    pub length: u16,
    pub index: u16,
}

impl LocalVariableTarget {
    /// Deserializes a `LocalVariableTarget` from a byte stream.
    ///
    /// Reads `start_pc`, `length`, and `index` from the stream. Note that `start_pc` and `length`
    /// are read as raw PC values/lengths and may need mapping to logical instruction indices/counts
    /// depending on the context.
    ///
    /// # Errors
    ///
    /// Returns an error if reading from the byte stream fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::LocalVariableTarget;
    /// use std::io::Cursor;
    ///
    /// // Byte data for LocalVariableTarget: start_pc=5, length=10, index=2
    /// let data = vec![0, 5, 0, 10, 0, 2];
    /// let mut cursor = Cursor::new(data);
    ///
    /// let lv_target = LocalVariableTarget::from_bytes(&mut cursor)?;
    ///
    /// assert_eq!(lv_target.start_pc, 5);
    /// assert_eq!(lv_target.length, 10);
    /// assert_eq!(lv_target.index, 2);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
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

    /// Serializes the `LocalVariableTarget` to a byte vector.
    ///
    /// Writes `start_pc`, `length`, and `index` to the vector. Note that `start_pc` and `length`
    /// are written directly; if they represent logical instruction indices/counts, they must be
    /// converted to byte offsets/lengths before serialization in the context of a `TypeAnnotation`
    /// attribute.
    ///
    /// # Errors
    ///
    /// Returns an error if writing to the byte vector fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::LocalVariableTarget;
    ///
    /// let lv_target = LocalVariableTarget {
    ///     start_pc: 100,
    ///     length: 200,
    ///     index: 1,
    /// };
    ///
    /// let mut bytes = Vec::new();
    /// lv_target.to_bytes(&mut bytes)?;
    ///
    /// // Expected: start_pc(100), length(200), index(1)
    /// assert_eq!(bytes, vec![0, 100, 0, 200, 0, 1]);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    pub fn to_bytes(&self, bytes: &mut Vec<u8>) -> Result<()> {
        bytes.write_u16::<BigEndian>(self.start_pc)?;
        bytes.write_u16::<BigEndian>(self.length)?;
        bytes.write_u16::<BigEndian>(self.index)?;
        Ok(())
    }
}

impl fmt::Display for LocalVariableTarget {
    /// Provides a string representation of the `LocalVariableTarget`.
    ///
    /// The implementation formats the local variable target information in a readable form showing
    /// the scope (`start_pc` and `length`) and the local variable `index`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::LocalVariableTarget;
    ///
    /// let target = LocalVariableTarget {
    ///     start_pc: 15,
    ///     length: 25,
    ///     index: 2,
    /// };
    ///
    /// assert_eq!(target.to_string(), "start_pc: 15, length: 25, index: 2");
    /// ```
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
