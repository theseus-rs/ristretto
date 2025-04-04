use crate::error::Result;
use bitflags::bitflags;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fmt;
use std::io::Cursor;

bitflags! {
    /// Nest class access flags.
    ///
    /// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.7.6-300-D.1-D.1>
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
    /// Should not occur; reserved for future use.
    pub fn from_bytes(bytes: &mut Cursor<Vec<u8>>) -> Result<NestedClassAccessFlags> {
        let access_flags = bytes.read_u16::<BigEndian>()?;
        let access_flags = NestedClassAccessFlags::from_bits_truncate(access_flags);
        Ok(access_flags)
    }

    /// Serialize the `NestedClassAccessFlags` to bytes.
    ///
    /// # Errors
    /// Should not occur; reserved for future use.
    pub fn to_bytes(&self, bytes: &mut Vec<u8>) -> Result<()> {
        bytes.write_u16::<BigEndian>(self.bits())?;
        Ok(())
    }
}

impl fmt::Display for NestedClassAccessFlags {
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
