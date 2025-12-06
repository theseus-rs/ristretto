use crate::error::Result;
use bitflags::bitflags;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fmt;
use std::io::Cursor;

bitflags! {
    /// Opens flags.
    ///
    /// `OpensFlags` represents the access flags for a module's opens entry in the Java class file
    /// format. These flags provide metadata about how a package is opened to other modules in the
    /// Java Module System.
    ///
    /// # Java Module System Context
    ///
    /// In the Java Module System, a module can "open" packages to other modules, allowing
    /// reflective access to its classes. The `OpensFlags` indicate whether the opening was declared
    /// explicitly in the source code or added by the compiler.
    ///
    /// # Available Flags
    ///
    /// - `SYNTHETIC`: Indicates the opening was not explicitly declared in source code but added by
    ///   the compiler
    /// - `MANDATED`: Indicates the opening was implicitly declared in the source of the module
    ///   declaration
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::OpensFlags;
    /// use std::io::Cursor;
    ///
    /// // Serialize the flags to bytes
    /// let mut bytes = vec![0x10, 0x00];
    ///
    /// // Deserialize the flags from bytes
    /// let mut cursor = Cursor::new(bytes);
    /// let deserialized_flags = OpensFlags::from_bytes(&mut cursor)?;
    /// assert_eq!(deserialized_flags, OpensFlags::SYNTHETIC);
    ///
    /// // Check individual flags
    /// assert!(deserialized_flags.contains(OpensFlags::SYNTHETIC));
    ///
    /// // Display the flags in a human-readable format
    /// let flag_string = deserialized_flags.to_string();
    /// assert_eq!(flag_string, "(0x1000) ACC_SYNTHETIC");
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    ///
    /// # References
    ///
    /// - [JVM Specification §4.7.25](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.25)
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct OpensFlags: u16 {
        /// Indicates that this opening was not explicitly or implicitly declared in the source of
        /// the module declaration.
        const SYNTHETIC = 0x1000;
        /// Indicates that this opening was implicitly declared in the source of the module
        /// declaration.
        const MANDATED = 0x8000;
    }
}

impl Default for OpensFlags {
    fn default() -> OpensFlags {
        OpensFlags::empty()
    }
}

impl OpensFlags {
    /// Deserialize the `OpensFlags` from bytes.
    ///
    /// # Errors
    ///
    /// Should not occur; reserved for future use.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::OpensFlags;
    /// use std::io::Cursor;
    ///
    /// let mut bytes = Cursor::new(vec![0x10, 0x00]);
    /// let flags = OpensFlags::from_bytes(&mut bytes)?;
    /// assert_eq!(flags, OpensFlags::SYNTHETIC);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    pub fn from_bytes(bytes: &mut Cursor<Vec<u8>>) -> Result<OpensFlags> {
        let access_flags = bytes.read_u16::<BigEndian>()?;
        let access_flags = OpensFlags::from_bits_truncate(access_flags);
        Ok(access_flags)
    }

    /// Serialize the `OpensFlags` to bytes.
    ///
    /// # Errors
    ///
    /// Should not occur; reserved for future use.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::OpensFlags;
    ///
    /// let flags = OpensFlags::SYNTHETIC | OpensFlags::MANDATED;
    /// let mut bytes = Vec::new();
    /// flags.to_bytes(&mut bytes)?;
    /// assert_eq!(bytes, vec![0x90, 0x00]);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    pub fn to_bytes(&self, bytes: &mut Vec<u8>) -> Result<()> {
        bytes.write_u16::<BigEndian>(self.bits())?;
        Ok(())
    }
}

impl fmt::Display for OpensFlags {
    /// Provides a string representation of the `OpensFlags`.
    ///
    /// The string representation includes the hexadecimal value of the flags and a comma-separated
    /// list of the flag names that are set. The format is: `({hex value}) {flag1}, {flag2}, ...`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::OpensFlags;
    ///
    /// let synthetic = OpensFlags::SYNTHETIC;
    /// assert_eq!("(0x1000) ACC_SYNTHETIC", synthetic.to_string());
    ///
    /// let empty = OpensFlags::empty();
    /// assert_eq!("(0x0000) ", empty.to_string());
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut access_flags = Vec::new();
        if self.contains(OpensFlags::SYNTHETIC) {
            access_flags.push("ACC_SYNTHETIC");
        }
        if self.contains(OpensFlags::MANDATED) {
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
        assert_eq!(OpensFlags::empty(), OpensFlags::default());
    }

    #[test]
    fn test_all_access_flags() {
        let access_flags: u16 = u16::MAX;
        let mut bytes = Cursor::new(access_flags.to_be_bytes().to_vec());
        assert_eq!(
            Ok(OpensFlags::SYNTHETIC | OpensFlags::MANDATED),
            OpensFlags::from_bytes(&mut bytes)
        );
    }

    #[test]
    fn test_access_flags() -> Result<()> {
        let access_flags = OpensFlags::MANDATED;
        let mut bytes = Vec::new();
        access_flags.to_bytes(&mut bytes)?;
        let mut bytes = Cursor::new(bytes);
        assert_eq!(Ok(access_flags), OpensFlags::from_bytes(&mut bytes));
        Ok(())
    }

    #[test]
    fn test_to_string() {
        assert_eq!("(0x1000) ACC_SYNTHETIC", OpensFlags::SYNTHETIC.to_string());
        assert_eq!("(0x8000) ACC_MANDATED", OpensFlags::MANDATED.to_string());
    }
}
