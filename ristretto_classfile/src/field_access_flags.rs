use crate::error::Result;
use bitflags::bitflags;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fmt;
use std::io::Cursor;

bitflags! {
    /// Field access flags.
    ///
    /// See: <https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.5:~:text=field_info%20structure%20are%20as%20follows%3A-,access_flags,-The%20value%20of%20the%20access_flags>
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
    fn default() -> FieldAccessFlags {
        FieldAccessFlags::empty()
    }
}

impl FieldAccessFlags {
    /// Deserialize the `FieldAccessFlags` from bytes.
    ///
    /// # Errors
    /// Should not occur; reserved for future use.
    pub fn from_bytes(bytes: &mut Cursor<Vec<u8>>) -> Result<FieldAccessFlags> {
        let access_flags = bytes.read_u16::<BigEndian>()?;
        let access_flags = FieldAccessFlags::from_bits_truncate(access_flags);
        Ok(access_flags)
    }

    /// Serialize the `FieldAccessFlags` to bytes.
    ///
    /// # Errors
    /// Should not occur; reserved for future use.
    pub fn to_bytes(&self, bytes: &mut Vec<u8>) -> Result<()> {
        bytes.write_u16::<BigEndian>(self.bits())?;
        Ok(())
    }
}

impl fmt::Display for FieldAccessFlags {
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
