use crate::error::Result;
use bitflags::bitflags;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fmt;
use std::io::Cursor;

bitflags! {
    /// Module access flags used in Java class files.
    ///
    /// The [`ModuleAccessFlags`] type represents the access flags in the Module attribute of a Java
    /// class file.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::ModuleAccessFlags;
    ///
    /// // Create flags using individual constants
    /// let flags = ModuleAccessFlags::OPEN | ModuleAccessFlags::SYNTHETIC;
    ///
    /// // Test for specific flags
    /// assert!(flags.contains(ModuleAccessFlags::OPEN));
    /// assert!(flags.contains(ModuleAccessFlags::SYNTHETIC));
    /// assert!(!flags.contains(ModuleAccessFlags::MANDATED));
    ///
    /// // Convert to string representation
    /// assert_eq!(flags.to_string(), "(0x1020) ACC_OPEN, ACC_SYNTHETIC");
    ///
    /// // Serialize to bytes
    /// let mut bytes = Vec::new();
    /// flags.to_bytes(&mut bytes)?;
    /// assert_eq!(bytes, vec![0x10, 0x20]);
    ///
    /// // Deserialize from bytes
    /// let mut cursor = std::io::Cursor::new(bytes);
    /// let deserialized = ModuleAccessFlags::from_bytes(&mut cursor)?;
    /// assert_eq!(deserialized, flags);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    ///
    /// # References
    ///
    /// - [JVM Specification §4.7.25](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.25)
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct ModuleAccessFlags: u16 {
        /// Indicates that this module is open.
        const OPEN = 0x0020;
        /// Indicates that this module was not explicitly or implicitly declared.
        const SYNTHETIC = 0x1000;
        /// Indicates that this module was implicitly declared.
        const MANDATED = 0x8000;
    }
}

impl Default for ModuleAccessFlags {
    fn default() -> ModuleAccessFlags {
        ModuleAccessFlags::empty()
    }
}

impl ModuleAccessFlags {
    /// Deserialize the `ModuleAccessFlags` from bytes.
    ///
    /// # Errors
    ///
    /// Should not occur; reserved for future use.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::ModuleAccessFlags;
    /// use std::io::Cursor;
    ///
    /// let mut bytes = Cursor::new(vec![0x00, 0x20]); // ACC_OPEN
    /// let flags = ModuleAccessFlags::from_bytes(&mut bytes)?;
    /// assert_eq!(flags, ModuleAccessFlags::OPEN);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    pub fn from_bytes(bytes: &mut Cursor<Vec<u8>>) -> Result<ModuleAccessFlags> {
        let access_flags = bytes.read_u16::<BigEndian>()?;
        let access_flags = ModuleAccessFlags::from_bits_truncate(access_flags);
        Ok(access_flags)
    }

    /// Serialize the `ModuleAccessFlags` to bytes.
    ///
    /// # Errors
    ///
    /// Should not occur; reserved for future use.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::ModuleAccessFlags;
    ///
    /// let flags = ModuleAccessFlags::OPEN | ModuleAccessFlags::SYNTHETIC;
    /// let mut bytes = Vec::new();
    /// flags.to_bytes(&mut bytes)?;
    /// assert_eq!(bytes, vec![0x10, 0x20]); // 0x0020 | 0x1000
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    pub fn to_bytes(&self, bytes: &mut Vec<u8>) -> Result<()> {
        bytes.write_u16::<BigEndian>(self.bits())?;
        Ok(())
    }
}

impl fmt::Display for ModuleAccessFlags {
    /// Implements the Display trait for `ModuleAccessFlags`, allowing flags to be printed in a human-readable format.
    ///
    /// The output format consists of:
    /// 1. The hexadecimal value of the flags in parentheses (e.g., "(0x0020)")
    /// 2. A list of named flags separated by commas (e.g., "`ACC_OPEN, ACC_SYNTHETIC`")
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::ModuleAccessFlags;
    ///
    /// let flags = ModuleAccessFlags::OPEN | ModuleAccessFlags::SYNTHETIC;
    ///
    /// let output = flags.to_string();
    /// assert_eq!(output, "(0x1020) ACC_OPEN, ACC_SYNTHETIC");
    ///
    /// let empty = ModuleAccessFlags::empty();
    /// assert_eq!(empty.to_string(), "(0x0000) ");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut access_flags = Vec::new();
        if self.contains(ModuleAccessFlags::OPEN) {
            access_flags.push("ACC_OPEN");
        }
        if self.contains(ModuleAccessFlags::SYNTHETIC) {
            access_flags.push("ACC_SYNTHETIC");
        }
        if self.contains(ModuleAccessFlags::MANDATED) {
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
        assert_eq!(ModuleAccessFlags::empty(), ModuleAccessFlags::default());
    }

    #[test]
    fn test_all_access_flags() {
        let access_flags: u16 = u16::MAX;
        let mut bytes = Cursor::new(access_flags.to_be_bytes().to_vec());
        assert_eq!(
            Ok(ModuleAccessFlags::OPEN
                | ModuleAccessFlags::SYNTHETIC
                | ModuleAccessFlags::MANDATED),
            ModuleAccessFlags::from_bytes(&mut bytes)
        );
    }

    #[test]
    fn test_access_flags() -> Result<()> {
        let access_flags = ModuleAccessFlags::OPEN;
        let mut bytes = Vec::new();
        access_flags.to_bytes(&mut bytes)?;
        let mut bytes = Cursor::new(bytes);
        assert_eq!(Ok(access_flags), ModuleAccessFlags::from_bytes(&mut bytes));
        Ok(())
    }

    #[test]
    fn test_to_string() {
        assert_eq!("(0x0020) ACC_OPEN", ModuleAccessFlags::OPEN.to_string());
        assert_eq!(
            "(0x1000) ACC_SYNTHETIC",
            ModuleAccessFlags::SYNTHETIC.to_string()
        );
        assert_eq!(
            "(0x8000) ACC_MANDATED",
            ModuleAccessFlags::MANDATED.to_string()
        );
    }
}
