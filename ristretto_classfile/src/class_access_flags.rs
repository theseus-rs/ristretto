use crate::error::Result;
use bitflags::bitflags;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fmt;
use std::io::Cursor;

bitflags! {
    /// Class access flags.
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
    fn default() -> ClassAccessFlags {
        ClassAccessFlags::empty()
    }
}

impl ClassAccessFlags {
    /// Deserialize the `ClassAccessFlags` from bytes.
    ///
    /// # Errors
    /// Should not occur; reserved for future use.
    pub fn from_bytes(bytes: &mut Cursor<Vec<u8>>) -> Result<ClassAccessFlags> {
        let access_flags = bytes.read_u16::<BigEndian>()?;
        let access_flags = ClassAccessFlags::from_bits_truncate(access_flags);
        Ok(access_flags)
    }

    /// Get the Class Access Flags as a string of code.
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
    /// # Errors
    /// Should not occur; reserved for future use.
    pub fn to_bytes(&self, bytes: &mut Vec<u8>) -> Result<()> {
        bytes.write_u16::<BigEndian>(self.bits())?;
        Ok(())
    }
}

impl fmt::Display for ClassAccessFlags {
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
