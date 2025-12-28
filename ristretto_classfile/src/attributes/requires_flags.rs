use crate::error::Result;
use bitflags::bitflags;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fmt;
use std::io::Cursor;

bitflags! {
    /// Requires flags used in Java module declarations to specify module dependency characteristics.
    ///
    /// These flags correspond to the modifiers used in the `requires` directive of Java module
    /// declarations. They control how module dependencies are handled by the Java module system.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::RequiresFlags;
    ///
    /// let flags = RequiresFlags::TRANSITIVE | RequiresFlags::STATIC_PHASE;
    /// assert!(flags.contains(RequiresFlags::TRANSITIVE));
    /// assert!(flags.contains(RequiresFlags::STATIC_PHASE));
    /// assert!(!flags.contains(RequiresFlags::SYNTHETIC));
    ///
    /// // Serialize to bytes
    /// let mut bytes = Vec::new();
    /// flags.to_bytes(&mut bytes)?;
    /// assert_eq!(bytes, vec![0x00, 0x60]);
    ///
    /// // Deserialize from bytes
    /// use std::io::Cursor;
    /// let mut cursor = Cursor::new(bytes);
    /// let deserialized_flags = RequiresFlags::from_bytes(&mut cursor)?;
    /// assert_eq!(flags, deserialized_flags);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    ///
    /// # References
    ///
    /// - [JVMS §4.7.25](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.25)
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct RequiresFlags: u16 {
        /// Indicates that any module which depends on the current module, implicitly declares a
        /// dependence on the module indicated by this entry.
        const TRANSITIVE = 0x0020;
        /// Indicates that this dependence is mandatory in the static phase, i.e., at compile time,
        /// but is optional in the dynamic phase, i.e., at run time.
        const STATIC_PHASE = 0x0040;
        /// Indicates that this dependence was not explicitly or implicitly declared in the source
        /// of the module declaration.
        const SYNTHETIC = 0x1000;
        /// Indicates that this dependence was implicitly declared in the source of the module
        /// declaration.
        const MANDATED = 0x8000;
    }
}

impl Default for RequiresFlags {
    /// Default implementation for `RequiresFlags`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::RequiresFlags;
    ///
    /// // Create default flags (empty)
    /// let flags = RequiresFlags::default();
    ///
    /// // Verify that default flags are empty
    /// assert_eq!(flags, RequiresFlags::empty());
    /// assert_eq!(flags.bits(), 0);
    /// assert!(!flags.contains(RequiresFlags::TRANSITIVE));
    /// ```
    fn default() -> RequiresFlags {
        RequiresFlags::empty()
    }
}

impl RequiresFlags {
    /// Deserialize the `RequiresFlags` from bytes.
    ///
    /// Reads a `u16` value in big-endian format from the byte cursor and converts it to
    /// `RequiresFlags`, truncating any unknown bits.
    ///
    /// # Errors
    ///
    /// Returns an error if reading from the byte stream fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::RequiresFlags;
    /// use std::io::Cursor;
    ///
    /// // Create a cursor with bytes representing TRANSITIVE | STATIC_PHASE flags
    /// let bytes = vec![0x00, 0x60]; // 0x0060 = TRANSITIVE | STATIC_PHASE
    /// let mut cursor = Cursor::new(bytes);
    ///
    /// // Deserialize the flags
    /// let flags = RequiresFlags::from_bytes(&mut cursor).unwrap();
    ///
    /// // Verify flags were correctly deserialized
    /// assert!(flags.contains(RequiresFlags::TRANSITIVE));
    /// assert!(flags.contains(RequiresFlags::STATIC_PHASE));
    /// assert!(!flags.contains(RequiresFlags::SYNTHETIC));
    /// ```
    pub fn from_bytes(bytes: &mut Cursor<Vec<u8>>) -> Result<RequiresFlags> {
        let access_flags = bytes.read_u16::<BigEndian>()?;
        let access_flags = RequiresFlags::from_bits_truncate(access_flags);
        Ok(access_flags)
    }

    /// Serialize the `RequiresFlags` to bytes.
    ///
    /// Writes the flags as a `u16` value in big-endian format to the provided byte vector.
    ///
    /// # Errors
    ///
    /// Returns an error if writing to the byte stream fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::RequiresFlags;
    ///
    /// // Create flags
    /// let flags = RequiresFlags::TRANSITIVE | RequiresFlags::MANDATED;
    ///
    /// // Serialize to bytes
    /// let mut bytes = Vec::new();
    /// flags.to_bytes(&mut bytes).unwrap();
    ///
    /// // Verify serialization
    /// assert_eq!(bytes, vec![0x80, 0x20]); // 0x8020 = TRANSITIVE | MANDATED
    /// ```
    pub fn to_bytes(&self, bytes: &mut Vec<u8>) -> Result<()> {
        bytes.write_u16::<BigEndian>(self.bits())?;
        Ok(())
    }
}

impl fmt::Display for RequiresFlags {
    /// Display implementation for `RequiresFlags`.
    ///
    /// Formats the flags as a hexadecimal value followed by symbolic names of set flags.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::RequiresFlags;
    ///
    /// let output = RequiresFlags::TRANSITIVE.to_string();
    /// assert_eq!(output, "(0x0020) ACC_TRANSITIVE");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut access_flags = Vec::new();
        if self.contains(RequiresFlags::TRANSITIVE) {
            access_flags.push("ACC_TRANSITIVE");
        }
        if self.contains(RequiresFlags::STATIC_PHASE) {
            access_flags.push("ACC_STATIC_PHASE");
        }
        if self.contains(RequiresFlags::SYNTHETIC) {
            access_flags.push("ACC_SYNTHETIC");
        }
        if self.contains(RequiresFlags::MANDATED) {
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
        assert_eq!(RequiresFlags::empty(), RequiresFlags::default());
    }

    #[test]
    fn test_all_access_flags() {
        let access_flags: u16 = u16::MAX;
        let mut bytes = Cursor::new(access_flags.to_be_bytes().to_vec());
        assert_eq!(
            Ok(RequiresFlags::TRANSITIVE
                | RequiresFlags::STATIC_PHASE
                | RequiresFlags::SYNTHETIC
                | RequiresFlags::MANDATED),
            RequiresFlags::from_bytes(&mut bytes)
        );
    }

    #[test]
    fn test_access_flags() -> Result<()> {
        let access_flags = RequiresFlags::TRANSITIVE;
        let mut bytes = Vec::new();
        access_flags.to_bytes(&mut bytes)?;
        let mut bytes = Cursor::new(bytes);
        assert_eq!(Ok(access_flags), RequiresFlags::from_bytes(&mut bytes));
        Ok(())
    }

    #[test]
    fn test_to_string() {
        assert_eq!(
            "(0x0020) ACC_TRANSITIVE",
            RequiresFlags::TRANSITIVE.to_string()
        );
        assert_eq!(
            "(0x0040) ACC_STATIC_PHASE",
            RequiresFlags::STATIC_PHASE.to_string()
        );
        assert_eq!(
            "(0x1000) ACC_SYNTHETIC",
            RequiresFlags::SYNTHETIC.to_string()
        );
        assert_eq!("(0x8000) ACC_MANDATED", RequiresFlags::MANDATED.to_string());
    }
}
