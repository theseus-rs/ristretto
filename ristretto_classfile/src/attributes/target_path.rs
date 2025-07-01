use crate::error::Result;
use byteorder::{ReadBytesExt, WriteBytesExt};
use std::fmt;
use std::io::Cursor;

/// Implementation of `TargetPath`.
///
/// The `TargetPath` structure is used in `TypeAnnotation` structures to specify
/// the location of the annotated type in a type path.
///
/// - `type_path_kind`: Defines the kind of path, e.g., array element, nested type argument.
///   - 0: Annotation is on a type in an array type.
///   - 1: Annotation is on a type in a nested type.
///   - 2: Annotation is on a type in a wildcard type bound.
///   - 3: Annotation is on a type argument of a parameterized type.
/// - `type_argument_index`: If `type_path_kind` is 0, 1, or 2, then `type_argument_index` must be 0.
///   If `type_path_kind` is 3, then `type_argument_index` specifies which type argument of a
///   parameterized type is annotated, where 0 indicates the first type argument.
///
/// # Examples
///
/// ```rust
/// use ristretto_classfile::attributes::TargetPath;
///
/// let target_path = TargetPath {
///     type_path_kind: 1, // Nested type
///     type_argument_index: 0,
/// };
/// ```
///
/// # References
///
/// - [JVM Specification §4.7.20.2](https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.7.20.2)
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TargetPath {
    pub type_path_kind: u8,
    pub type_argument_index: u8,
}

impl TargetPath {
    /// Deserialize the target path from bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if reading from the byte stream fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::TargetPath;
    /// use std::io::Cursor;
    ///
    /// let bytes_data = vec![0x01, 0x00];
    /// let mut cursor = Cursor::new(bytes_data);
    /// let target_path = TargetPath::from_bytes(&mut cursor)?;
    ///
    /// assert_eq!(target_path.type_path_kind, 1);
    /// assert_eq!(target_path.type_argument_index, 0);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    pub fn from_bytes(bytes: &mut Cursor<Vec<u8>>) -> Result<TargetPath> {
        let type_path_kind = bytes.read_u8()?;
        let type_argument_index = bytes.read_u8()?;

        let target_path = TargetPath {
            type_path_kind,
            type_argument_index,
        };
        Ok(target_path)
    }

    /// Serialize the target path to bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if writing to the byte stream fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::TargetPath;
    ///
    /// let target_path = TargetPath {
    ///     type_path_kind: 3,
    ///     type_argument_index: 1,
    /// };
    /// let mut bytes_buffer = Vec::new();
    /// target_path.to_bytes(&mut bytes_buffer)?;
    ///
    /// assert_eq!(bytes_buffer, vec![0x03, 0x01]);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    pub fn to_bytes(&self, bytes: &mut Vec<u8>) -> Result<()> {
        bytes.write_u8(self.type_path_kind)?;
        bytes.write_u8(self.type_argument_index)?;
        Ok(())
    }
}

impl fmt::Display for TargetPath {
    /// Implements the `Display` trait for `TargetPath`.
    ///
    /// Formats the `TargetPath` as a string showing its `type_path_kind` and `type_argument_index`
    /// values.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::TargetPath;
    /// use std::fmt;
    ///
    /// let target_path = TargetPath {
    ///     type_path_kind: 3,
    ///     type_argument_index: 1,
    /// };
    ///
    /// let output = target_path.to_string();
    /// assert_eq!(output, "TargetPath[type_path_kind=3, type_argument_index=1]");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "TargetPath[type_path_kind={}, type_argument_index={}]",
            self.type_path_kind, self.type_argument_index
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_serialization() -> Result<()> {
        let target_path = TargetPath {
            type_path_kind: 1,
            type_argument_index: 2,
        };
        let expected_value = [1, 2];

        assert_eq!(
            "TargetPath[type_path_kind=1, type_argument_index=2]",
            target_path.to_string()
        );

        let mut bytes = Vec::new();
        target_path.clone().to_bytes(&mut bytes)?;
        assert_eq!(expected_value, &bytes[..]);

        let mut bytes = Cursor::new(expected_value.to_vec());
        assert_eq!(target_path, TargetPath::from_bytes(&mut bytes)?);
        Ok(())
    }
}
