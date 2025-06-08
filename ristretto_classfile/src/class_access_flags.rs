use crate::error::Result;
use bitflags::bitflags;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fmt;
use std::io::Cursor;

bitflags! {
    /// Class access flags.
    ///
    /// These flags define various properties and access restrictions for a Java class, interface,
    /// enum, or module as defined in the JVM specification.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::ClassAccessFlags;
    /// use std::io::Cursor;
    ///
    /// // Create flags for a public final class
    /// let flags = ClassAccessFlags::PUBLIC | ClassAccessFlags::FINAL;
    ///
    /// // Check if specific flags are set
    /// assert!(flags.contains(ClassAccessFlags::PUBLIC));
    /// assert!(flags.contains(ClassAccessFlags::FINAL));
    /// assert!(!flags.contains(ClassAccessFlags::INTERFACE));
    ///
    /// // Get a code representation
    /// assert_eq!("public final class", flags.as_code());
    ///
    /// // Serialize to bytes
    /// let mut bytes = Vec::new();
    /// flags.to_bytes(&mut bytes)?;
    /// assert_eq!(vec![0x00, 0x11], bytes); // 0x0011 = PUBLIC | FINAL
    ///
    /// // Deserialize from bytes
    /// let mut cursor = Cursor::new(bytes);
    /// let deserialized = ClassAccessFlags::from_bytes(&mut cursor)?;
    /// assert_eq!(flags, deserialized);
    ///
    /// // Display as string
    /// assert_eq!("(0x0011) ACC_PUBLIC, ACC_FINAL", flags.to_string());
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    ///
    /// # References
    ///
    /// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.1:~:text=constant_pool_count%20%2D%201.-,access_flags,-The%20value%20of>
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct ClassAccessFlags: u16 {
        /// Declared public; may be accessed from outside its package.
        const PUBLIC = 0x0001;
        /// Declared final; no subclasses allowed.
        const FINAL = 0x0010;
        /// Treat superclass methods specially when invoked by the invokespecial instruction.
        const SUPER = 0x0020;
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
        /// Is a module, not a class or interface.
        const MODULE = 0x8000;
    }
}

impl Default for ClassAccessFlags {
    /// Default implementation for `ClassAccessFlags`
    ///
    /// Creates an empty set of access flags.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::ClassAccessFlags;
    ///
    /// let flags = ClassAccessFlags::default();
    /// assert!(flags.is_empty());
    /// assert_eq!(flags.bits(), 0);
    /// ```
    fn default() -> ClassAccessFlags {
        ClassAccessFlags::empty()
    }
}

impl ClassAccessFlags {
    /// Deserialize the `ClassAccessFlags` from bytes.
    ///
    /// Reads a u16 value from the provided byte cursor and converts it to a `ClassAccessFlags`
    /// bitflag set. Any unrecognized bits are silently truncated.
    ///
    /// # Errors
    ///
    /// Returns an error if reading from the byte source fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::ClassAccessFlags;
    /// use std::io::Cursor;
    ///
    /// // Create a cursor with bytes representing PUBLIC | FINAL (0x0011)
    /// let bytes = vec![0x00, 0x11];
    /// let mut cursor = Cursor::new(bytes);
    ///
    /// // Deserialize the bytes into ClassAccessFlags
    /// let flags = ClassAccessFlags::from_bytes(&mut cursor)?;
    ///
    /// // Verify the flags were properly read
    /// assert!(flags.contains(ClassAccessFlags::PUBLIC));
    /// assert!(flags.contains(ClassAccessFlags::FINAL));
    /// assert_eq!(flags.bits(), 0x0011);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    pub fn from_bytes(bytes: &mut Cursor<Vec<u8>>) -> Result<ClassAccessFlags> {
        let access_flags = bytes.read_u16::<BigEndian>()?;
        let access_flags = ClassAccessFlags::from_bits_truncate(access_flags);
        Ok(access_flags)
    }

    /// Get the `ClassAccessFlags` as a string of code.
    ///
    /// Converts the access flags to a Java-like code representation, including modifiers like
    /// "public", "abstract", "final", etc., and the appropriate type descriptor ("class",
    /// "interface", "enum", etc.).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::ClassAccessFlags;
    ///
    /// // For a public class
    /// let flags = ClassAccessFlags::PUBLIC;
    /// assert_eq!("public class", flags.as_code());
    ///
    /// // For a public abstract interface
    /// let flags = ClassAccessFlags::PUBLIC | ClassAccessFlags::ABSTRACT | ClassAccessFlags::INTERFACE;
    /// assert_eq!("public abstract interface", flags.as_code());
    ///
    /// // For a public enum
    /// let flags = ClassAccessFlags::PUBLIC | ClassAccessFlags::ENUM;
    /// assert_eq!("public enum", flags.as_code());
    ///
    /// // For a public annotation
    /// let flags = ClassAccessFlags::PUBLIC | ClassAccessFlags::ANNOTATION;
    /// assert_eq!("public annotation", flags.as_code());
    ///
    /// // Flags without Java modifiers returns "class
    /// assert_eq!("class", ClassAccessFlags::empty().as_code());
    /// ```
    #[must_use]
    pub fn as_code(&self) -> String {
        let mut modifiers = Vec::new();
        if self.contains(ClassAccessFlags::PUBLIC) {
            modifiers.push("public");
        }
        if self.contains(ClassAccessFlags::ABSTRACT) {
            modifiers.push("abstract");
        }
        if self.contains(ClassAccessFlags::FINAL) {
            modifiers.push("final");
        }
        if self.contains(ClassAccessFlags::SYNTHETIC) {
            modifiers.push("synthetic");
        }
        if self.contains(ClassAccessFlags::ANNOTATION) {
            modifiers.push("annotation");
        } else if self.contains(ClassAccessFlags::ENUM) {
            modifiers.push("enum");
        } else if self.contains(ClassAccessFlags::INTERFACE) {
            modifiers.push("interface");
        } else if self.contains(ClassAccessFlags::MODULE) {
            modifiers.push("module");
        } else {
            modifiers.push("class");
        }

        modifiers.join(" ")
    }

    /// Serialize the `ClassAccessFlags` to bytes.
    ///
    /// Writes the bit representation of the access flags as a big-endian u16 to the provided byte
    /// vector.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::ClassAccessFlags;
    ///
    /// // Create flags for a public final class
    /// let flags = ClassAccessFlags::PUBLIC | ClassAccessFlags::FINAL;
    ///
    /// // Serialize to bytes
    /// let mut bytes = Vec::new();
    /// flags.to_bytes(&mut bytes)?;
    ///
    /// // Verify the bytes match the expected value (0x0011 = PUBLIC | FINAL)
    /// assert_eq!(vec![0x00, 0x11], bytes);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if writing to the byte vector fails.
    pub fn to_bytes(&self, bytes: &mut Vec<u8>) -> Result<()> {
        bytes.write_u16::<BigEndian>(self.bits())?;
        Ok(())
    }
}

impl fmt::Display for ClassAccessFlags {
    /// Display implementation for `ClassAccessFlags`
    ///
    /// Formats the access flags as a hexadecimal value followed by a list of JVM access flag names
    /// (e.g., `ACC_PUBLIC`, `ACC_FINAL`).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::ClassAccessFlags;
    ///
    /// // Display a single flag
    /// let flags = ClassAccessFlags::PUBLIC;
    /// assert_eq!("(0x0001) ACC_PUBLIC", flags.to_string());
    ///
    /// // Display multiple flags
    /// let flags = ClassAccessFlags::PUBLIC | ClassAccessFlags::FINAL;
    /// assert_eq!("(0x0011) ACC_PUBLIC, ACC_FINAL", flags.to_string());
    ///
    /// // Display no flags
    /// let flags = ClassAccessFlags::empty();
    /// assert_eq!("(0x0000) ", flags.to_string());
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut access_flags = Vec::new();
        if self.contains(ClassAccessFlags::PUBLIC) {
            access_flags.push("ACC_PUBLIC");
        }
        if self.contains(ClassAccessFlags::FINAL) {
            access_flags.push("ACC_FINAL");
        }
        if self.contains(ClassAccessFlags::SUPER) {
            access_flags.push("ACC_SUPER");
        }
        if self.contains(ClassAccessFlags::INTERFACE) {
            access_flags.push("ACC_INTERFACE");
        }
        if self.contains(ClassAccessFlags::ABSTRACT) {
            access_flags.push("ACC_ABSTRACT");
        }
        if self.contains(ClassAccessFlags::SYNTHETIC) {
            access_flags.push("ACC_SYNTHETIC");
        }
        if self.contains(ClassAccessFlags::ANNOTATION) {
            access_flags.push("ACC_ANNOTATION");
        }
        if self.contains(ClassAccessFlags::ENUM) {
            access_flags.push("ACC_ENUM");
        }
        if self.contains(ClassAccessFlags::MODULE) {
            access_flags.push("ACC_MODULE");
        }
        write!(f, "({:#06X}) {}", self.bits(), access_flags.join(", "))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_default() {
        assert_eq!(ClassAccessFlags::empty(), ClassAccessFlags::default());
    }

    #[test]
    fn test_all_access_flags() {
        let access_flags: u16 = u16::MAX;
        let mut bytes = Cursor::new(access_flags.to_be_bytes().to_vec());
        assert_eq!(
            Ok(ClassAccessFlags::PUBLIC
                | ClassAccessFlags::FINAL
                | ClassAccessFlags::SUPER
                | ClassAccessFlags::INTERFACE
                | ClassAccessFlags::ABSTRACT
                | ClassAccessFlags::SYNTHETIC
                | ClassAccessFlags::ANNOTATION
                | ClassAccessFlags::ENUM
                | ClassAccessFlags::MODULE),
            ClassAccessFlags::from_bytes(&mut bytes)
        );
    }

    #[test]
    fn test_access_flags() -> Result<()> {
        let access_flags = ClassAccessFlags::PUBLIC | ClassAccessFlags::FINAL;
        let mut bytes = Vec::new();
        access_flags.to_bytes(&mut bytes)?;
        let mut bytes = Cursor::new(bytes);
        assert_eq!(Ok(access_flags), ClassAccessFlags::from_bytes(&mut bytes));
        Ok(())
    }

    #[test]
    fn test_as_code() {
        assert_eq!("public class", ClassAccessFlags::PUBLIC.as_code());
        assert_eq!("final class", ClassAccessFlags::FINAL.as_code());
        assert_eq!("class", ClassAccessFlags::SUPER.as_code());
        assert_eq!("interface", ClassAccessFlags::INTERFACE.as_code());
        assert_eq!("abstract class", ClassAccessFlags::ABSTRACT.as_code());
        assert_eq!("synthetic class", ClassAccessFlags::SYNTHETIC.as_code());
        assert_eq!("annotation", ClassAccessFlags::ANNOTATION.as_code());
        assert_eq!("enum", ClassAccessFlags::ENUM.as_code());
        assert_eq!("module", ClassAccessFlags::MODULE.as_code());
    }

    #[test]
    fn test_to_string() {
        assert_eq!("(0x0001) ACC_PUBLIC", ClassAccessFlags::PUBLIC.to_string());
        assert_eq!("(0x0010) ACC_FINAL", ClassAccessFlags::FINAL.to_string());
        assert_eq!("(0x0020) ACC_SUPER", ClassAccessFlags::SUPER.to_string());
        assert_eq!(
            "(0x0200) ACC_INTERFACE",
            ClassAccessFlags::INTERFACE.to_string()
        );
        assert_eq!(
            "(0x0400) ACC_ABSTRACT",
            ClassAccessFlags::ABSTRACT.to_string()
        );
        assert_eq!(
            "(0x1000) ACC_SYNTHETIC",
            ClassAccessFlags::SYNTHETIC.to_string()
        );
        assert_eq!(
            "(0x2000) ACC_ANNOTATION",
            ClassAccessFlags::ANNOTATION.to_string()
        );
        assert_eq!("(0x4000) ACC_ENUM", ClassAccessFlags::ENUM.to_string());
        assert_eq!("(0x8000) ACC_MODULE", ClassAccessFlags::MODULE.to_string());

        let access_flags = ClassAccessFlags::PUBLIC | ClassAccessFlags::SUPER;
        assert_eq!("(0x0021) ACC_PUBLIC, ACC_SUPER", access_flags.to_string());
    }
}
