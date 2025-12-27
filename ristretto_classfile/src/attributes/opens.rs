use crate::attributes::OpensFlags;
use crate::error::Result;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fmt;
use std::io::Cursor;

/// Implementation of `Opens`.
///
/// Represents an opens entry in the Module attribute structure of a Java class file.
/// Each opens entry describes a package that a module opens to other modules.
///
/// # Fields
///
/// * `index` - The index into the constant pool representing the package name.
/// * `flags` - The module opens flags that describe the access to the opened package.
/// * `to_index` - A list of indices into the constant pool representing modules to which the package is opened.
///   If empty, the package is opened to all modules.
///
/// # Examples
///
/// ```rust
/// use ristretto_classfile::attributes::{Opens, OpensFlags};
///
/// // Create an Opens entry that opens a package with index 1 to the module with index 3
/// let opens = Opens {
///     index: 1,
///     flags: OpensFlags::MANDATED,
///     to_index: vec![3],
/// };
///
/// // Serialize the Opens entry
/// let mut bytes = Vec::new();
/// opens.to_bytes(&mut bytes)?;
///
/// // Deserialize the Opens entry
/// let mut cursor = std::io::Cursor::new(bytes);
/// let deserialized = Opens::from_bytes(&mut cursor)?;
///
/// assert_eq!(opens, deserialized);
/// # Ok::<(), ristretto_classfile::Error>(())
/// ```
///
/// # References
///
/// - [JVMS §4.7.25](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.25)
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Opens {
    pub index: u16,
    pub flags: OpensFlags,
    pub to_index: Vec<u16>,
}

impl Opens {
    /// Deserialize the opens from bytes.
    ///
    /// # Errors
    ///
    /// Should not occur; reserved for future use.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::{Opens, OpensFlags};
    /// use std::io::Cursor;
    ///
    /// // index: 1, flags: MANDATED, to_index: [3]
    /// let mut bytes = Cursor::new(vec![0x00, 0x01, 0x80, 0x00, 0x00, 0x01, 0x00, 0x03]);
    /// let opens = Opens::from_bytes(&mut bytes)?;
    /// assert_eq!(opens.index, 1);
    /// assert_eq!(opens.flags, OpensFlags::MANDATED);
    /// assert_eq!(opens.to_index, vec![3]);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    pub fn from_bytes(bytes: &mut Cursor<Vec<u8>>) -> Result<Opens> {
        let index = bytes.read_u16::<BigEndian>()?;
        let flags = OpensFlags::from_bytes(bytes)?;
        let to_index_count = bytes.read_u16::<BigEndian>()?;
        let mut to_index = Vec::with_capacity(to_index_count as usize);
        for _ in 0..to_index_count {
            to_index.push(bytes.read_u16::<BigEndian>()?);
        }
        let requires = Opens {
            index,
            flags,
            to_index,
        };
        Ok(requires)
    }

    /// Serialize the opens to bytes.
    ///
    /// # Errors
    ///
    /// If there are more than 65,534 `to_index` values.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::{Opens, OpensFlags};
    ///
    /// let opens = Opens {
    ///     index: 1,
    ///     flags: OpensFlags::MANDATED,
    ///     to_index: vec![3],
    /// };
    /// let mut bytes = Vec::new();
    /// opens.to_bytes(&mut bytes)?;
    /// assert_eq!(bytes, vec![0x00, 0x01, 0x80, 0x00, 0x00, 0x01, 0x00, 0x03]);
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

impl fmt::Display for Opens {
    /// Formats the `Opens` struct for display.
    ///
    /// Returns a string representation of the Opens entry, showing the `index`, `flags`, and
    /// `to_index` values in a human-readable format.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::{Opens, OpensFlags};
    ///
    /// let opens = Opens {
    ///     index: 1,
    ///     flags: OpensFlags::MANDATED,
    ///     to_index: vec![3, 4],
    /// };
    ///
    /// let output = opens.to_string();
    /// assert!(output.contains("index: 1"));
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
    fn test_display() {
        let opens = Opens {
            index: 1,
            flags: OpensFlags::MANDATED,
            to_index: vec![3],
        };
        assert_eq!(
            "index: 1, flags: (0x8000) ACC_MANDATED, to_index: [3]",
            opens.to_string()
        );
    }

    #[test]
    fn test_serialization() -> Result<()> {
        let opens = Opens {
            index: 1,
            flags: OpensFlags::MANDATED,
            to_index: vec![3],
        };
        let expected_value = [0, 1, 128, 0, 0, 1, 0, 3];
        let mut bytes = Vec::new();
        opens.clone().to_bytes(&mut bytes)?;
        assert_eq!(expected_value, &bytes[..]);

        let mut bytes = Cursor::new(expected_value.to_vec());
        assert_eq!(opens, Opens::from_bytes(&mut bytes)?);
        Ok(())
    }
}
