use crate::byte_reader::ByteReader;
use crate::error::Result;
use bitflags::bitflags;
use byteorder::{BigEndian, WriteBytesExt};
use std::fmt;

bitflags! {
    /// Method parameter access flags used in Java class files.
    ///
    /// The [`MethodParameterAccessFlags`] type represents the access flags in the `MethodParameters` attribute of a Java
    /// class file.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::byte_reader::ByteReader;
    /// use ristretto_classfile::attributes::MethodParameterAccessFlags;
    ///
    /// // Create flags using individual constants
    /// let flags = MethodParameterAccessFlags::FINAL | MethodParameterAccessFlags::SYNTHETIC;
    ///
    /// // Test for specific flags
    /// assert!(flags.contains(MethodParameterAccessFlags::FINAL));
    /// assert!(flags.contains(MethodParameterAccessFlags::SYNTHETIC));
    /// assert!(!flags.contains(MethodParameterAccessFlags::MANDATED));
    ///
    /// // Convert to string representation
    /// assert_eq!(flags.to_string(), "(0x1010) ACC_FINAL, ACC_SYNTHETIC");
    ///
    /// // Serialize to bytes
    /// let mut bytes = Vec::new();
    /// flags.to_bytes(&mut bytes)?;
    /// assert_eq!(bytes, vec![0x10, 0x10]);
    ///
    /// // Deserialize from bytes
    /// let mut reader = ByteReader::new(&bytes);
    /// let deserialized = MethodParameterAccessFlags::from_bytes(&mut reader)?;
    /// assert_eq!(deserialized, flags);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    ///
    /// # References
    ///
    /// - [JVMS §4.7.24](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.24)
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct MethodParameterAccessFlags: u16 {
        /// Indicates that this parameter is final.
        const FINAL = 0x0010;
        /// Indicates that this parameter was not explicitly or implicitly declared.
        const SYNTHETIC = 0x1000;
        /// Indicates that this parameter was implicitly declared.
        const MANDATED = 0x8000;
    }
}

impl Default for MethodParameterAccessFlags {
    fn default() -> MethodParameterAccessFlags {
        MethodParameterAccessFlags::empty()
    }
}

impl MethodParameterAccessFlags {
    /// Deserialize the `MethodParameterAccessFlags` from bytes.
    ///
    /// # Errors
    ///
    /// Should not occur; reserved for future use.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::byte_reader::ByteReader;
    /// use ristretto_classfile::attributes::MethodParameterAccessFlags;
    ///
    /// let mut bytes = ByteReader::new(&[0x00, 0x10]); // ACC_FINAL
    /// let flags = MethodParameterAccessFlags::from_bytes(&mut bytes)?;
    /// assert_eq!(flags, MethodParameterAccessFlags::FINAL);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    pub fn from_bytes(bytes: &mut ByteReader<'_>) -> Result<MethodParameterAccessFlags> {
        let access_flags = bytes.read_u16()?;
        let access_flags = MethodParameterAccessFlags::from_bits_truncate(access_flags);
        Ok(access_flags)
    }

    /// Serialize the `MethodParameterAccessFlags` to bytes.
    ///
    /// # Errors
    ///
    /// Should not occur; reserved for future use.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::MethodParameterAccessFlags;
    ///
    /// let flags = MethodParameterAccessFlags::FINAL | MethodParameterAccessFlags::SYNTHETIC;
    /// let mut bytes = Vec::new();
    /// flags.to_bytes(&mut bytes)?;
    /// assert_eq!(bytes, vec![0x10, 0x10]); // 0x0010 | 0x1000
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    pub fn to_bytes(&self, bytes: &mut Vec<u8>) -> Result<()> {
        bytes.write_u16::<BigEndian>(self.bits())?;
        Ok(())
    }
}

impl fmt::Display for MethodParameterAccessFlags {
    /// Implements the Display trait for `MethodParameterAccessFlags`, allowing flags to be printed in a human-readable format.
    ///
    /// The output format consists of:
    /// 1. The hexadecimal value of the flags in parentheses (e.g., "(0x0010)")
    /// 2. A list of named flags separated by commas (e.g., "`ACC_FINAL, ACC_SYNTHETIC`")
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::MethodParameterAccessFlags;
    ///
    /// let flags = MethodParameterAccessFlags::FINAL | MethodParameterAccessFlags::SYNTHETIC;
    ///
    /// let output = flags.to_string();
    /// assert_eq!(output, "(0x1010) ACC_FINAL, ACC_SYNTHETIC");
    ///
    /// let empty = MethodParameterAccessFlags::empty();
    /// assert_eq!(empty.to_string(), "(0x0000) ");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut access_flags = Vec::new();
        if self.contains(MethodParameterAccessFlags::FINAL) {
            access_flags.push("ACC_FINAL");
        }
        if self.contains(MethodParameterAccessFlags::SYNTHETIC) {
            access_flags.push("ACC_SYNTHETIC");
        }
        if self.contains(MethodParameterAccessFlags::MANDATED) {
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
        assert_eq!(
            MethodParameterAccessFlags::empty(),
            MethodParameterAccessFlags::default()
        );
    }

    #[test]
    fn test_all_access_flags() {
        let access_flags: u16 = u16::MAX;
        let binding = access_flags.to_be_bytes();
        let mut bytes = ByteReader::new(&binding);
        assert_eq!(
            Ok(MethodParameterAccessFlags::FINAL
                | MethodParameterAccessFlags::SYNTHETIC
                | MethodParameterAccessFlags::MANDATED),
            MethodParameterAccessFlags::from_bytes(&mut bytes)
        );
    }

    #[test]
    fn test_access_flags() -> Result<()> {
        let access_flags = MethodParameterAccessFlags::FINAL;
        let mut bytes = Vec::new();
        access_flags.to_bytes(&mut bytes)?;
        let mut bytes = ByteReader::new(&bytes);
        assert_eq!(
            Ok(access_flags),
            MethodParameterAccessFlags::from_bytes(&mut bytes)
        );
        Ok(())
    }

    #[test]
    fn test_to_string() {
        assert_eq!(
            "(0x0010) ACC_FINAL",
            MethodParameterAccessFlags::FINAL.to_string()
        );
        assert_eq!(
            "(0x1000) ACC_SYNTHETIC",
            MethodParameterAccessFlags::SYNTHETIC.to_string()
        );
        assert_eq!(
            "(0x8000) ACC_MANDATED",
            MethodParameterAccessFlags::MANDATED.to_string()
        );
    }
}
