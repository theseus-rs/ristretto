use crate::error::Result;
use bitflags::bitflags;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fmt;
use std::io::Cursor;

bitflags! {
    /// Field access flags used in Java class files to specify the access permissions and properties
    /// of fields.
    ///
    /// These flags determine visibility (public, private, protected), mutability (final),and other
    /// characteristics of class fields. Multiple flags can be combined using bitwise OR operations.
    ///
    /// # Examples
    ///
    /// Creating field access flags for common field types:
    ///
    /// ```rust
    /// use ristretto_classfile::FieldAccessFlags;
    /// use std::io::Cursor;
    ///
    /// // A private final instance field
    /// let flags = FieldAccessFlags::PRIVATE | FieldAccessFlags::FINAL;
    ///
    /// // Check if specific flags are set
    /// assert!(flags.contains(FieldAccessFlags::PRIVATE));
    /// assert!(flags.contains(FieldAccessFlags::FINAL));
    /// assert!(!flags.contains(FieldAccessFlags::PUBLIC));
    /// assert!(!flags.contains(FieldAccessFlags::STATIC));
    ///
    /// // Get a code representation
    /// assert_eq!("private final", flags.as_code());
    ///
    /// // Serialize to bytes
    /// let mut bytes = Vec::new();
    /// flags.to_bytes(&mut bytes)?;
    /// assert_eq!(vec![0x00, 0x12], bytes); // 0x0012 = PRIVATE | FINAL
    ///
    /// // Deserialize from bytes
    /// let mut cursor = Cursor::new(bytes);
    /// let deserialized = FieldAccessFlags::from_bytes(&mut cursor)?;
    /// assert_eq!(flags, deserialized);
    ///
    /// // Display as string
    /// assert_eq!("(0x0012) ACC_PRIVATE, ACC_FINAL", flags.to_string());
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    ///
    /// # References
    ///
    /// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.5:~:text=field_info%20structure%20are%20as%20follows%3A-,access_flags,-The%20value%20of%20the%20access_flags>
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct FieldAccessFlags: u16 {
        /// Declared public; may be accessed from outside its package.
        const PUBLIC = 0x0001;
        /// Declared private; accessible only within the defining class and other classes belonging to the same nest (§5.4.4).
        const PRIVATE = 0x0002;
        /// Declared protected; may be accessed within subclasses.
        const PROTECTED = 0x0004;
        /// Declared static.
        const STATIC = 0x0008;
        /// Declared final; never directly assigned to after object construction (JLS §17.5).
        const FINAL = 0x0010;
        /// Declared volatile; cannot be cached.
        const VOLATILE = 0x0040;
        /// Declared transient; not written or read by a persistent object manager.
        const TRANSIENT = 0x0080;
        /// Declared synthetic; not present in the source code.
        const SYNTHETIC = 0x1000;
        /// Declared as an element of an enum class.
        const ENUM = 0x4000;
    }
}

impl Default for FieldAccessFlags {
    /// Creates a new `FieldAccessFlags` with no flags set.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::FieldAccessFlags;
    ///
    /// let flags = FieldAccessFlags::default();
    /// assert!(flags.is_empty());
    /// assert_eq!(0, flags.bits());
    /// ```
    fn default() -> FieldAccessFlags {
        FieldAccessFlags::empty()
    }
}

impl FieldAccessFlags {
    /// Deserialize the `FieldAccessFlags` from bytes.
    ///
    /// Reads a u16 value from the given cursor in big-endian order and constructs
    /// a `FieldAccessFlags` instance from it.
    ///
    /// # Errors
    ///
    /// Should not occur; reserved for future use.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io::Cursor;
    /// use ristretto_classfile::FieldAccessFlags;
    ///
    /// // Create a byte buffer representing a public static field
    /// let access_flags: u16 = 0x0009; // PUBLIC | STATIC
    /// let mut bytes = Cursor::new(access_flags.to_be_bytes().to_vec());
    ///
    /// // Parse the access flags
    /// let flags = FieldAccessFlags::from_bytes(&mut bytes)?;
    /// assert!(flags.contains(FieldAccessFlags::PUBLIC));
    /// assert!(flags.contains(FieldAccessFlags::STATIC));
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    pub fn from_bytes(bytes: &mut Cursor<impl AsRef<[u8]>>) -> Result<FieldAccessFlags> {
        let access_flags = bytes.read_u16::<BigEndian>()?;
        let access_flags = FieldAccessFlags::from_bits_truncate(access_flags);
        Ok(access_flags)
    }

    /// Serialize the `FieldAccessFlags` to bytes.
    ///
    /// Writes the flags as a u16 value in big-endian order to the given byte vector.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::FieldAccessFlags;
    ///
    /// // Create a flags instance representing a public final field
    /// let flags = FieldAccessFlags::PUBLIC | FieldAccessFlags::FINAL;
    ///
    /// // Serialize to bytes
    /// let mut bytes = Vec::new();
    /// flags.to_bytes(&mut bytes)?;
    ///
    /// // Check the serialized value (0x0011 = PUBLIC | FINAL)
    /// assert_eq!(bytes, [0x00, 0x11]);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    ///
    /// # Errors
    /// Should not occur; reserved for future use.
    pub fn to_bytes(&self, bytes: &mut Vec<u8>) -> Result<()> {
        bytes.write_u16::<BigEndian>(self.bits())?;
        Ok(())
    }

    /// Get the `FieldAccessFlags` as a string of Java modifiers.
    ///
    /// This method converts the flags to a string representation that
    /// matches how the modifiers would appear in Java source code.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::FieldAccessFlags;
    ///
    /// // Public static final field (common for constants)
    /// let flags = FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL;
    /// assert_eq!("public static final", flags.as_code());
    ///
    /// // Private volatile field
    /// let flags = FieldAccessFlags::PRIVATE | FieldAccessFlags::VOLATILE;
    /// assert_eq!("private volatile", flags.as_code());
    ///
    /// // Flags without Java modifiers return empty strings
    /// assert_eq!("", FieldAccessFlags::empty().as_code());
    /// ```
    #[must_use]
    pub fn as_code(&self) -> String {
        let mut modifiers = Vec::new();
        if self.contains(FieldAccessFlags::PUBLIC) {
            modifiers.push("public");
        }
        if self.contains(FieldAccessFlags::PRIVATE) {
            modifiers.push("private");
        }
        if self.contains(FieldAccessFlags::PROTECTED) {
            modifiers.push("protected");
        }
        if self.contains(FieldAccessFlags::STATIC) {
            modifiers.push("static");
        }
        if self.contains(FieldAccessFlags::VOLATILE) {
            modifiers.push("volatile");
        }
        if self.contains(FieldAccessFlags::TRANSIENT) {
            modifiers.push("transient");
        }
        if self.contains(FieldAccessFlags::FINAL) {
            modifiers.push("final");
        }
        if self.contains(FieldAccessFlags::SYNTHETIC) {
            modifiers.push("synthetic");
        }
        if self.contains(FieldAccessFlags::ENUM) {
            modifiers.push("enum");
        }

        modifiers.join(" ")
    }
}

impl fmt::Display for FieldAccessFlags {
    /// Formats the `FieldAccessFlags` as a string showing the hexadecimal value and the individual flag
    /// constants.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::FieldAccessFlags;
    ///
    /// // Public static field
    /// let flags = FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC;
    /// assert_eq!("(0x0009) ACC_PUBLIC, ACC_STATIC", flags.to_string());
    ///
    /// // Private final field
    /// let flags = FieldAccessFlags::PRIVATE | FieldAccessFlags::FINAL;
    /// assert_eq!("(0x0012) ACC_PRIVATE, ACC_FINAL", flags.to_string());
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut access_flags = Vec::new();
        if self.contains(FieldAccessFlags::PUBLIC) {
            access_flags.push("ACC_PUBLIC");
        }
        if self.contains(FieldAccessFlags::PRIVATE) {
            access_flags.push("ACC_PRIVATE");
        }
        if self.contains(FieldAccessFlags::PROTECTED) {
            access_flags.push("ACC_PROTECTED");
        }
        if self.contains(FieldAccessFlags::STATIC) {
            access_flags.push("ACC_STATIC");
        }
        if self.contains(FieldAccessFlags::FINAL) {
            access_flags.push("ACC_FINAL");
        }
        if self.contains(FieldAccessFlags::VOLATILE) {
            access_flags.push("ACC_VOLATILE");
        }
        if self.contains(FieldAccessFlags::TRANSIENT) {
            access_flags.push("ACC_TRANSIENT");
        }
        if self.contains(FieldAccessFlags::SYNTHETIC) {
            access_flags.push("ACC_SYNTHETIC");
        }
        if self.contains(FieldAccessFlags::ENUM) {
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
        assert_eq!(FieldAccessFlags::empty(), FieldAccessFlags::default());
    }

    #[test]
    fn test_all_access_flags() {
        let access_flags: u16 = u16::MAX;
        let mut bytes = Cursor::new(access_flags.to_be_bytes().to_vec());
        assert_eq!(
            Ok(FieldAccessFlags::PUBLIC
                | FieldAccessFlags::PRIVATE
                | FieldAccessFlags::PROTECTED
                | FieldAccessFlags::STATIC
                | FieldAccessFlags::FINAL
                | FieldAccessFlags::VOLATILE
                | FieldAccessFlags::TRANSIENT
                | FieldAccessFlags::SYNTHETIC
                | FieldAccessFlags::ENUM),
            FieldAccessFlags::from_bytes(&mut bytes)
        );
    }

    #[test]
    fn test_access_flags() -> Result<()> {
        let access_flags = FieldAccessFlags::PUBLIC | FieldAccessFlags::FINAL;
        let mut bytes = Vec::new();
        access_flags.to_bytes(&mut bytes)?;
        let mut bytes = Cursor::new(bytes);
        assert_eq!(Ok(access_flags), FieldAccessFlags::from_bytes(&mut bytes));
        Ok(())
    }

    #[test]
    fn test_as_code() {
        assert_eq!("public", FieldAccessFlags::PUBLIC.as_code());
        assert_eq!("private", FieldAccessFlags::PRIVATE.as_code());
        assert_eq!("protected", FieldAccessFlags::PROTECTED.as_code());
        assert_eq!("static", FieldAccessFlags::STATIC.as_code());
        assert_eq!("final", FieldAccessFlags::FINAL.as_code());
        assert_eq!("volatile", FieldAccessFlags::VOLATILE.as_code());
        assert_eq!("transient", FieldAccessFlags::TRANSIENT.as_code());
        assert_eq!("synthetic", FieldAccessFlags::SYNTHETIC.as_code());
        assert_eq!("enum", FieldAccessFlags::ENUM.as_code());
        let access_flags =
            FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL;
        assert_eq!("public static final", access_flags.as_code());
    }

    #[test]
    fn test_to_string() {
        assert_eq!("(0x0001) ACC_PUBLIC", FieldAccessFlags::PUBLIC.to_string());
        assert_eq!(
            "(0x0002) ACC_PRIVATE",
            FieldAccessFlags::PRIVATE.to_string()
        );
        assert_eq!(
            "(0x0004) ACC_PROTECTED",
            FieldAccessFlags::PROTECTED.to_string()
        );
        assert_eq!("(0x0008) ACC_STATIC", FieldAccessFlags::STATIC.to_string());
        assert_eq!("(0x0010) ACC_FINAL", FieldAccessFlags::FINAL.to_string());
        assert_eq!(
            "(0x0040) ACC_VOLATILE",
            FieldAccessFlags::VOLATILE.to_string()
        );
        assert_eq!(
            "(0x0080) ACC_TRANSIENT",
            FieldAccessFlags::TRANSIENT.to_string()
        );
        assert_eq!(
            "(0x1000) ACC_SYNTHETIC",
            FieldAccessFlags::SYNTHETIC.to_string()
        );
        assert_eq!("(0x4000) ACC_ENUM", FieldAccessFlags::ENUM.to_string());
    }
}
