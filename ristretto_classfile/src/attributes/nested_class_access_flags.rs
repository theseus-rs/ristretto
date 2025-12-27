use crate::error::Result;
use bitflags::bitflags;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fmt;
use std::io::Cursor;

bitflags! {
    /// Access flags for nested classes in a Java class file.
    ///
    /// These flags determine the accessibility, mutability, and other properties of nested classes.
    /// Each flag is represented by a specific bit in a 16-bit value, following the Java Virtual
    /// Machine specification.
    ///
    /// # Usage
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::NestedClassAccessFlags;
    /// use std::io::Cursor;
    ///
    /// // Create flags for a public final nested class
    /// let flags = NestedClassAccessFlags::PUBLIC | NestedClassAccessFlags::FINAL;
    ///
    /// // Test if specific flags are set
    /// assert!(flags.contains(NestedClassAccessFlags::PUBLIC));
    /// assert!(flags.contains(NestedClassAccessFlags::FINAL));
    /// assert!(!flags.contains(NestedClassAccessFlags::ABSTRACT));
    ///
    /// // Convert flags to a string representation
    /// assert_eq!(flags.to_string(), "(0x0011) ACC_PUBLIC, ACC_FINAL");
    ///
    /// // Serialize to bytes and back
    /// let mut bytes = Vec::new();
    /// flags.to_bytes(&mut bytes)?;
    /// assert_eq!(bytes, vec![0x00, 0x11]);
    ///
    /// let mut cursor = Cursor::new(bytes);
    /// let deserialized = NestedClassAccessFlags::from_bytes(&mut cursor)?;
    /// assert_eq!(deserialized, flags);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    ///
    /// # References
    ///
    /// - [JVMS §4.7.6](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.6)
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct NestedClassAccessFlags: u16 {
        /// Declared public; may be accessed from outside its package.
        const PUBLIC = 0x0001;
        /// Declared private; accessible only within the defining class and other classes belonging to the same nest (§5.4.4).
        const PRIVATE = 0x0002;
        /// Declared protected; may be accessed within subclasses.
        const PROTECTED = 0x0004;
        /// Declared static.
        const STATIC = 0x0008;
        /// Declared final; no subclasses allowed.
        const FINAL = 0x0010;
        /// Is an interface, not a class.
        const INTERFACE = 0x0200;
        /// Declared abstract; must not be instantiated.
        const ABSTRACT = 0x0400;
        /// Declared synthetic; not present in the source code.
        const SYNTHETIC = 0x1000;
        /// Declared as an annotation interface.
        const ANNOTATION = 0x2000;
        /// Declared as an enum class.
        const ENUM = 0x4000;
    }
}

impl Default for NestedClassAccessFlags {
    fn default() -> NestedClassAccessFlags {
        NestedClassAccessFlags::empty()
    }
}

impl NestedClassAccessFlags {
    /// Deserialize the `NestedClassAccessFlags` from bytes.
    ///
    /// # Errors
    ///
    /// Should not occur; reserved for future use.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::NestedClassAccessFlags;
    /// use std::io::Cursor;
    ///
    /// let mut bytes = Cursor::new(vec![0x00, 0x01]);
    /// let flags = NestedClassAccessFlags::from_bytes(&mut bytes)?;
    /// assert_eq!(flags, NestedClassAccessFlags::PUBLIC);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    pub fn from_bytes(bytes: &mut Cursor<Vec<u8>>) -> Result<NestedClassAccessFlags> {
        let access_flags = bytes.read_u16::<BigEndian>()?;
        let access_flags = NestedClassAccessFlags::from_bits_truncate(access_flags);
        Ok(access_flags)
    }

    /// Serialize the `NestedClassAccessFlags` to bytes.
    ///
    /// # Errors
    ///
    /// Should not occur; reserved for future use.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::NestedClassAccessFlags;
    ///
    /// let flags = NestedClassAccessFlags::PUBLIC | NestedClassAccessFlags::FINAL;
    /// let mut bytes = Vec::new();
    /// flags.to_bytes(&mut bytes)?;
    /// assert_eq!(bytes, vec![0x00, 0x11]);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    pub fn to_bytes(&self, bytes: &mut Vec<u8>) -> Result<()> {
        bytes.write_u16::<BigEndian>(self.bits())?;
        Ok(())
    }
}

impl fmt::Display for NestedClassAccessFlags {
    /// Formats the `NestedClassAccessFlags` for display purposes.
    ///
    /// The output format consists of:
    /// - The hexadecimal value of the flags in parentheses (e.g., `(0x0011)`)
    /// - Followed by a space-separated list of named flags that are set (e.g., `ACC_PUBLIC, ACC_FINAL`)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::NestedClassAccessFlags;
    ///
    /// let flags = NestedClassAccessFlags::PUBLIC | NestedClassAccessFlags::STATIC;
    ///
    /// let output = flags.to_string();
    /// assert_eq!(output, "(0x0009) ACC_PUBLIC, ACC_STATIC");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut access_flags = Vec::new();
        if self.contains(NestedClassAccessFlags::PUBLIC) {
            access_flags.push("ACC_PUBLIC");
        }
        if self.contains(NestedClassAccessFlags::PRIVATE) {
            access_flags.push("ACC_PRIVATE");
        }
        if self.contains(NestedClassAccessFlags::PROTECTED) {
            access_flags.push("ACC_PROTECTED");
        }
        if self.contains(NestedClassAccessFlags::STATIC) {
            access_flags.push("ACC_STATIC");
        }
        if self.contains(NestedClassAccessFlags::FINAL) {
            access_flags.push("ACC_FINAL");
        }
        if self.contains(NestedClassAccessFlags::INTERFACE) {
            access_flags.push("ACC_INTERFACE");
        }
        if self.contains(NestedClassAccessFlags::ABSTRACT) {
            access_flags.push("ACC_ABSTRACT");
        }
        if self.contains(NestedClassAccessFlags::SYNTHETIC) {
            access_flags.push("ACC_SYNTHETIC");
        }
        if self.contains(NestedClassAccessFlags::ANNOTATION) {
            access_flags.push("ACC_ANNOTATION");
        }
        if self.contains(NestedClassAccessFlags::ENUM) {
            access_flags.push("ACC_ENUM");
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
            NestedClassAccessFlags::empty(),
            NestedClassAccessFlags::default()
        );
    }

    #[test]
    fn test_all_access_flags() {
        let access_flags: u16 = u16::MAX;
        let mut bytes = Cursor::new(access_flags.to_be_bytes().to_vec());
        assert_eq!(
            Ok(NestedClassAccessFlags::PUBLIC
                | NestedClassAccessFlags::PRIVATE
                | NestedClassAccessFlags::PROTECTED
                | NestedClassAccessFlags::STATIC
                | NestedClassAccessFlags::FINAL
                | NestedClassAccessFlags::INTERFACE
                | NestedClassAccessFlags::ABSTRACT
                | NestedClassAccessFlags::SYNTHETIC
                | NestedClassAccessFlags::ANNOTATION
                | NestedClassAccessFlags::ENUM),
            NestedClassAccessFlags::from_bytes(&mut bytes)
        );
    }

    #[test]
    fn test_access_flags() -> Result<()> {
        let access_flags = NestedClassAccessFlags::PUBLIC | NestedClassAccessFlags::FINAL;
        let mut bytes = Vec::new();
        access_flags.to_bytes(&mut bytes)?;
        let mut bytes = Cursor::new(bytes);
        assert_eq!(
            Ok(access_flags),
            NestedClassAccessFlags::from_bytes(&mut bytes)
        );
        Ok(())
    }

    #[test]
    fn test_to_string() {
        assert_eq!(
            "(0x0001) ACC_PUBLIC",
            NestedClassAccessFlags::PUBLIC.to_string()
        );
        assert_eq!(
            "(0x0002) ACC_PRIVATE",
            NestedClassAccessFlags::PRIVATE.to_string()
        );
        assert_eq!(
            "(0x0004) ACC_PROTECTED",
            NestedClassAccessFlags::PROTECTED.to_string()
        );
        assert_eq!(
            "(0x0008) ACC_STATIC",
            NestedClassAccessFlags::STATIC.to_string()
        );
        assert_eq!(
            "(0x0010) ACC_FINAL",
            NestedClassAccessFlags::FINAL.to_string()
        );
        assert_eq!(
            "(0x0200) ACC_INTERFACE",
            NestedClassAccessFlags::INTERFACE.to_string()
        );
        assert_eq!(
            "(0x0400) ACC_ABSTRACT",
            NestedClassAccessFlags::ABSTRACT.to_string()
        );
        assert_eq!(
            "(0x1000) ACC_SYNTHETIC",
            NestedClassAccessFlags::SYNTHETIC.to_string()
        );
        assert_eq!(
            "(0x2000) ACC_ANNOTATION",
            NestedClassAccessFlags::ANNOTATION.to_string()
        );
        assert_eq!(
            "(0x4000) ACC_ENUM",
            NestedClassAccessFlags::ENUM.to_string()
        );
    }
}
