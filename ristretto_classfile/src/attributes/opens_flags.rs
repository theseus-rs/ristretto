use crate::error::Result;
use bitflags::bitflags;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fmt;
use std::io::Cursor;

bitflags! {
    /// Opens flags.
    ///
    /// See: <https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.7.25>
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct OpensFlags: u16 {
        /// Indicates that this opening was not explicitly or implicitly declared in the source of
        /// the module declaration.
        const SYNTHETIC = 0x1000;
        /// Indicates that this opening was implicitly declared in the source of the module
        /// declaration.
        const MANDATED = 0x8000;
    }
}

impl Default for OpensFlags {
    fn default() -> OpensFlags {
        OpensFlags::empty()
    }
}

impl OpensFlags {
    /// Deserialize the `OpensFlags` from bytes.
    ///
    /// # Errors
    /// Should not occur; reserved for future use.
    pub fn from_bytes(bytes: &mut Cursor<Vec<u8>>) -> Result<OpensFlags> {
        let access_flags = bytes.read_u16::<BigEndian>()?;
        let access_flags = OpensFlags::from_bits_truncate(access_flags);
        Ok(access_flags)
    }

    /// Serialize the `OpensFlags` to bytes.
    ///
    /// # Errors
    /// Should not occur; reserved for future use.
    pub fn to_bytes(&self, bytes: &mut Vec<u8>) -> Result<()> {
        bytes.write_u16::<BigEndian>(self.bits())?;
        Ok(())
    }
}

impl fmt::Display for OpensFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut access_flags = Vec::new();
        if self.contains(OpensFlags::SYNTHETIC) {
            access_flags.push("ACC_SYNTHETIC");
        }
        if self.contains(OpensFlags::MANDATED) {
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
        assert_eq!(OpensFlags::empty(), OpensFlags::default());
    }

    #[test]
    fn test_all_access_flags() {
        let access_flags: u16 = u16::MAX;
        let mut bytes = Cursor::new(access_flags.to_be_bytes().to_vec());
        assert_eq!(
            Ok(OpensFlags::SYNTHETIC | OpensFlags::MANDATED),
            OpensFlags::from_bytes(&mut bytes)
        );
    }

    #[test]
    fn test_access_flags() -> Result<()> {
        let access_flags = OpensFlags::MANDATED;
        let mut bytes = Vec::new();
        access_flags.to_bytes(&mut bytes)?;
        let mut bytes = Cursor::new(bytes);
        assert_eq!(Ok(access_flags), OpensFlags::from_bytes(&mut bytes));
        Ok(())
    }

    #[test]
    fn test_to_string() {
        assert_eq!("(0x1000) ACC_SYNTHETIC", OpensFlags::SYNTHETIC.to_string());
        assert_eq!("(0x8000) ACC_MANDATED", OpensFlags::MANDATED.to_string());
    }
}
