use crate::error::Result;
use bitflags::bitflags;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::Cursor;

bitflags! {
    /// Class access flags.
    ///
    /// See: <https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.1:~:text=constant_pool_count%20%2D%201.-,access_flags,-The%20value%20of>
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

    /// Serialize the `ClassAccessFlags` to bytes.
    ///
    /// # Errors
    /// Should not occur; reserved for future use.
    pub fn to_bytes(&self, bytes: &mut Vec<u8>) -> Result<()> {
        bytes.write_u16::<BigEndian>(self.bits())?;
        Ok(())
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
}
