use crate::error::Result;
use bitflags::bitflags;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fmt;
use std::io::Cursor;

bitflags! {
    /// Exports flags.
    ///
    /// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.7.25>
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct ExportsFlags: u16 {
        /// Indicates that this export was not explicitly or implicitly declared in the source of
        /// the module declaration.
        const SYNTHETIC = 0x1000;
        /// Indicates that this export was implicitly declared in the source of the module
        /// declaration.
        const MANDATED = 0x8000;
    }
}

impl Default for ExportsFlags {
    fn default() -> ExportsFlags {
        ExportsFlags::empty()
    }
}

impl ExportsFlags {
    /// Deserialize the `ExportsFlags` from bytes.
    ///
    /// # Errors
    /// Should not occur; reserved for future use.
    pub fn from_bytes(bytes: &mut Cursor<Vec<u8>>) -> Result<ExportsFlags> {
        let access_flags = bytes.read_u16::<BigEndian>()?;
        let access_flags = ExportsFlags::from_bits_truncate(access_flags);
        Ok(access_flags)
    }

    /// Serialize the `ExportsFlags` to bytes.
    ///
    /// # Errors
    /// Should not occur; reserved for future use.
    pub fn to_bytes(&self, bytes: &mut Vec<u8>) -> Result<()> {
        bytes.write_u16::<BigEndian>(self.bits())?;
        Ok(())
    }
}

impl fmt::Display for ExportsFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut access_flags = Vec::new();
        if self.contains(ExportsFlags::SYNTHETIC) {
            access_flags.push("ACC_SYNTHETIC");
        }
        if self.contains(ExportsFlags::MANDATED) {
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
        assert_eq!(ExportsFlags::empty(), ExportsFlags::default());
    }

    #[test]
    fn test_all_access_flags() {
        let access_flags: u16 = u16::MAX;
        let mut bytes = Cursor::new(access_flags.to_be_bytes().to_vec());
        assert_eq!(
            Ok(ExportsFlags::SYNTHETIC | ExportsFlags::MANDATED),
            ExportsFlags::from_bytes(&mut bytes)
        );
    }

    #[test]
    fn test_access_flags() -> Result<()> {
        let access_flags = ExportsFlags::MANDATED;
        let mut bytes = Vec::new();
        access_flags.to_bytes(&mut bytes)?;
        let mut bytes = Cursor::new(bytes);
        assert_eq!(Ok(access_flags), ExportsFlags::from_bytes(&mut bytes));
        Ok(())
    }

    #[test]
    fn test_to_string() {
        assert_eq!(
            "(0x1000) ACC_SYNTHETIC",
            ExportsFlags::SYNTHETIC.to_string()
        );
        assert_eq!("(0x8000) ACC_MANDATED", ExportsFlags::MANDATED.to_string());
    }
}
