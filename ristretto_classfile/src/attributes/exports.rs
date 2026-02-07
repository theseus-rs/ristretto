use crate::attributes::ExportsFlags;
use crate::error::Result;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fmt;
use std::io::Cursor;

/// Represents an `exports` directive within a `Module` attribute.
///
/// An `exports` directive specifies a package that is exported by the current module,
/// potentially qualified to specific other modules.
///
/// See the [JVMS §4.7.25](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.25)
/// for more details on the `exports` table within the `Module` attribute.
///
/// # Fields
///
/// - `index`: An index into the `constant_pool` table. The entry at this index must be a
///   `CONSTANT_Package_info` structure representing the exported package.
/// - `flags`: A set of `ExportsFlags` that modify the export (e.g., `SYNTHETIC`, `MANDATED`).
/// - `to_index`: A vector of indices into the `constant_pool` table. Each entry must be a
///   `CONSTANT_Module_info` structure. If this vector is empty, the package is exported
///   unqualified (to all modules). Otherwise, it is a qualified export, only to the modules
///   specified in this list.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// use ristretto_classfile::attributes::{Exports, ExportsFlags};
/// use std::io::Cursor;
///
/// // Create an unqualified export for package at constant pool index 5
/// let unqualified_export = Exports {
///     index: 5, // Index to CONSTANT_Package_info for "com.example.api"
///     flags: ExportsFlags::empty(),
///     to_index: vec![],
/// };
///
/// // Create a qualified export for package at index 7 to modules at indices 10 and 11
/// let qualified_export = Exports {
///     index: 7, // Index to CONSTANT_Package_info for "com.example.internal"
///     flags: ExportsFlags::SYNTHETIC,
///     to_index: vec![10, 11], // Exported only to modules at CP index 10 and 11
/// };
///
/// // Serialize an export directive
/// let mut bytes = Vec::new();
/// unqualified_export.to_bytes(&mut bytes)?;
///
/// // Deserialize an export directive
/// let mut cursor = Cursor::new(bytes);
/// let deserialized_export = Exports::from_bytes(&mut cursor)?;
///
/// assert_eq!(unqualified_export, deserialized_export);
/// # Ok::<(), ristretto_classfile::Error>(())
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Exports {
    pub index: u16,
    pub flags: ExportsFlags,
    pub to_index: Vec<u16>,
}

impl Exports {
    /// Deserializes an `Exports` structure from a byte stream.
    ///
    /// The `bytes` cursor should be positioned at the start of the `exports` entry.
    ///
    /// # Errors
    ///
    /// Returns an error if reading from the byte stream fails or if deserializing
    /// the `ExportsFlags` fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::{Exports, ExportsFlags};
    /// use std::io::Cursor;
    ///
    /// let data = vec![0, 10, 0, 0, 0, 1, 0, 15];
    /// let mut cursor = Cursor::new(data);
    ///
    /// let exports_directive = Exports::from_bytes(&mut cursor)?;
    ///
    /// assert_eq!(exports_directive.index, 10);
    /// assert_eq!(exports_directive.flags, ExportsFlags::empty());
    /// assert_eq!(exports_directive.to_index.len(), 1);
    /// assert_eq!(exports_directive.to_index[0], 15);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    pub fn from_bytes(bytes: &mut Cursor<impl AsRef<[u8]>>) -> Result<Exports> {
        let index = bytes.read_u16::<BigEndian>()?;
        let flags = ExportsFlags::from_bytes(bytes)?;
        let to_index_count = bytes.read_u16::<BigEndian>()?;
        let mut to_index = Vec::with_capacity(to_index_count as usize);
        for _ in 0..to_index_count {
            to_index.push(bytes.read_u16::<BigEndian>()?);
        }
        let requires = Exports {
            index,
            flags,
            to_index,
        };
        Ok(requires)
    }

    /// Serializes the `Exports` structure to a byte vector.
    ///
    /// # Errors
    ///
    /// - If the number of `to_index` entries exceeds 65,535.
    /// - Propagates I/O errors or errors from serializing `ExportsFlags`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::{Exports, ExportsFlags};
    ///
    /// let exports_directive = Exports {
    ///     index: 5, // Package "com.example.feature"
    ///     flags: ExportsFlags::MANDATED,
    ///     to_index: vec![20], // Exported to module at CP index 20
    /// };
    ///
    /// let mut bytes = Vec::new();
    /// exports_directive.to_bytes(&mut bytes)?;
    ///
    /// let expected_bytes = vec![0, 5, 0x80, 0, 0, 1, 0, 20];
    /// assert_eq!(bytes, expected_bytes);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    pub fn to_bytes(&self, bytes: &mut Vec<u8>) -> Result<()> {
        bytes.write_u16::<BigEndian>(self.index)?;
        self.flags.to_bytes(bytes)?;

        let to_index_length = u16::try_from(self.to_index.len())?;
        bytes.write_u16::<BigEndian>(to_index_length)?;
        for index in &self.to_index {
            bytes.write_u16::<BigEndian>(*index)?;
        }
        Ok(())
    }
}

impl fmt::Display for Exports {
    /// Formats the `Exports` directive for human-readable display.
    ///
    /// This implementation produces a string representation showing the package index, export
    /// flags, and the list of modules this package is exported to (if any).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::{Exports, ExportsFlags};
    ///
    /// let exports = Exports {
    ///     index: 7,
    ///     flags: ExportsFlags::SYNTHETIC,
    ///     to_index: vec![10, 11],
    /// };
    ///
    /// let output = exports.to_string();
    /// assert_eq!(output, "index: 7, flags: (0x1000) ACC_SYNTHETIC, to_index: [10, 11]");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "index: {}, flags: {}, to_index: {:?}",
            self.index, self.flags, self.to_index
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_to_string() {
        let exports = Exports {
            index: 1,
            flags: ExportsFlags::MANDATED,
            to_index: vec![3],
        };
        assert_eq!(
            "index: 1, flags: (0x8000) ACC_MANDATED, to_index: [3]",
            exports.to_string()
        );
    }

    #[test]
    fn test_serialization() -> Result<()> {
        let exports = Exports {
            index: 1,
            flags: ExportsFlags::MANDATED,
            to_index: vec![3],
        };
        let expected_value = [0, 1, 128, 0, 0, 1, 0, 3];

        let mut bytes = Vec::new();
        exports.clone().to_bytes(&mut bytes)?;
        assert_eq!(expected_value, &bytes[..]);

        let mut bytes = Cursor::new(expected_value.to_vec());
        assert_eq!(exports, Exports::from_bytes(&mut bytes)?);
        Ok(())
    }
}
