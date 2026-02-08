use crate::error::Result;
use bitflags::bitflags;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fmt;
use std::io::Cursor;

bitflags! {
    /// Flags that modify an `exports` directive in a `Module` attribute.
    ///
    /// These flags provide additional information about how an export was declared.
    ///
    /// See the [JVMS §4.7.25](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.25)
    /// within the `Module` attribute structure.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::ExportsFlags;
    ///
    /// // A synthetic export
    /// let flags1 = ExportsFlags::SYNTHETIC;
    /// assert!(flags1.contains(ExportsFlags::SYNTHETIC));
    ///
    /// // A mandated export
    /// let flags2 = ExportsFlags::MANDATED;
    /// assert_eq!(flags2.bits(), 0x8000);
    /// ```
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct ExportsFlags: u16 {
        /// Indicates that this export was not explicitly or implicitly declared in the source of
        /// the module declaration.
        const SYNTHETIC = 0x1000;
        /// Indicates that this export was implicitly declared in the source of the module
        /// declaration.
        const MANDATED = 0x8000;
    }
}

impl Default for ExportsFlags {
    /// Returns an empty set of `ExportsFlags`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::ExportsFlags;
    ///
    /// let default_flags = ExportsFlags::default();
    /// assert!(default_flags.is_empty());
    /// ```
    fn default() -> ExportsFlags {
        ExportsFlags::empty()
    }
}

impl ExportsFlags {
    /// Deserializes `ExportsFlags` from a `u16` value read from a byte stream.
    ///
    /// Unknown bits in the input value are truncated (ignored).
    ///
    /// # Errors
    ///
    /// Returns an error if reading the `u16` value from the byte stream fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::ExportsFlags;
    /// use std::io::Cursor;
    ///
    /// // Bytes for ACC_SYNTHETIC (0x1000)
    /// let data = vec![0x10, 0x00];
    /// let mut cursor = Cursor::new(data);
    /// let flags = ExportsFlags::from_bytes(&mut cursor)?;
    /// assert_eq!(flags, ExportsFlags::SYNTHETIC);
    ///
    /// // Bytes for ACC_MANDATED (0x8000)
    /// let data_mandated = vec![0x80, 0x00];
    /// let mut cursor_mandated = Cursor::new(data_mandated);
    /// let flags_mandated = ExportsFlags::from_bytes(&mut cursor_mandated)?;
    /// assert_eq!(flags_mandated, ExportsFlags::MANDATED);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    pub fn from_bytes(bytes: &mut Cursor<impl AsRef<[u8]>>) -> Result<ExportsFlags> {
        let access_flags = bytes.read_u16::<BigEndian>()?;
        let access_flags = ExportsFlags::from_bits_truncate(access_flags);
        Ok(access_flags)
    }

    /// Serializes the `ExportsFlags` to their `u16` bit representation into a byte vector.
    ///
    /// # Errors
    ///
    /// Returns an error if writing the `u16` value to the byte vector fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::ExportsFlags;
    ///
    /// let flags = ExportsFlags::SYNTHETIC | ExportsFlags::MANDATED;
    /// let mut byte_vec = Vec::new();
    /// flags.to_bytes(&mut byte_vec)?;
    ///
    /// // Expected: 0x1000 | 0x8000 = 0x9000
    /// assert_eq!(byte_vec, vec![0x90, 0x00]);
    ///
    /// let empty_flags = ExportsFlags::empty();
    /// let mut byte_vec_empty = Vec::new();
    /// empty_flags.to_bytes(&mut byte_vec_empty)?;
    /// assert_eq!(byte_vec_empty, vec![0x00, 0x00]);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    pub fn to_bytes(&self, bytes: &mut Vec<u8>) -> Result<()> {
        bytes.write_u16::<BigEndian>(self.bits())?;
        Ok(())
    }
}

impl fmt::Display for ExportsFlags {
    /// Formats the `ExportsFlags` for display purposes.
    ///
    /// The output format consists of two parts:
    /// - A hexadecimal representation of the flag bits in the format `(0xXXXX)`
    /// - A comma-separated list of the flag names that are set
    ///
    /// If no flags are set, the output will be `(0x0000) ` with an empty string following.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::ExportsFlags;
    ///
    /// // Display a single flag
    /// let synthetic = ExportsFlags::SYNTHETIC;
    /// assert_eq!(synthetic.to_string(), "(0x1000) ACC_SYNTHETIC");
    ///
    /// // Display multiple flags
    /// let combined = ExportsFlags::SYNTHETIC | ExportsFlags::MANDATED;
    /// assert_eq!(combined.to_string(), "(0x9000) ACC_SYNTHETIC, ACC_MANDATED");
    ///
    /// // Display empty flags
    /// let empty = ExportsFlags::empty();
    /// assert_eq!(empty.to_string(), "(0x0000) ");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut access_flags = Vec::new();
        if self.contains(ExportsFlags::SYNTHETIC) {
            access_flags.push("ACC_SYNTHETIC");
        }
        if self.contains(ExportsFlags::MANDATED) {
            access_flags.push("ACC_MANDATED");
        }
        write!(f, "({:#06X}) {}", self.bits(), access_flags.join(", "))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_default() {
        assert_eq!(ExportsFlags::empty(), ExportsFlags::default());
    }

    #[test]
    fn test_all_access_flags() {
        let access_flags: u16 = u16::MAX;
        let mut bytes = Cursor::new(access_flags.to_be_bytes().to_vec());
        assert_eq!(
            Ok(ExportsFlags::SYNTHETIC | ExportsFlags::MANDATED),
            ExportsFlags::from_bytes(&mut bytes)
        );
    }

    #[test]
    fn test_access_flags() -> Result<()> {
        let access_flags = ExportsFlags::MANDATED;
        let mut bytes = Vec::new();
        access_flags.to_bytes(&mut bytes)?;
        let mut bytes = Cursor::new(bytes);
        assert_eq!(Ok(access_flags), ExportsFlags::from_bytes(&mut bytes));
        Ok(())
    }

    #[test]
    fn test_to_string() {
        assert_eq!(
            "(0x1000) ACC_SYNTHETIC",
            ExportsFlags::SYNTHETIC.to_string()
        );
        assert_eq!("(0x8000) ACC_MANDATED", ExportsFlags::MANDATED.to_string());
    }
}
