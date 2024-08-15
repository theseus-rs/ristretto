use crate::error::Result;
use bitflags::bitflags;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fmt;
use std::io::Cursor;

bitflags! {
    /// Module access flags.
    ///
    /// See: <https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.7.25>
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
    /// Should not occur; reserved for future use.
    pub fn from_bytes(bytes: &mut Cursor<Vec<u8>>) -> Result<ModuleAccessFlags> {
        let access_flags = bytes.read_u16::<BigEndian>()?;
        let access_flags = ModuleAccessFlags::from_bits_truncate(access_flags);
        Ok(access_flags)
    }

    /// Serialize the `ModuleAccessFlags` to bytes.
    ///
    /// # Errors
    /// Should not occur; reserved for future use.
    pub fn to_bytes(&self, bytes: &mut Vec<u8>) -> Result<()> {
        bytes.write_u16::<BigEndian>(self.bits())?;
        Ok(())
    }
}

impl fmt::Display for ModuleAccessFlags {
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

    #[test_log::test]
    fn test_default() {
        assert_eq!(ModuleAccessFlags::empty(), ModuleAccessFlags::default());
    }

    #[test_log::test]
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

    #[test_log::test]
    fn test_access_flags() -> Result<()> {
        let access_flags = ModuleAccessFlags::OPEN;
        let mut bytes = Vec::new();
        access_flags.to_bytes(&mut bytes)?;
        let mut bytes = Cursor::new(bytes);
        assert_eq!(Ok(access_flags), ModuleAccessFlags::from_bytes(&mut bytes));
        Ok(())
    }

    #[test_log::test]
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
