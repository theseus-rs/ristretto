use crate::error::Result;
use bitflags::bitflags;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
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
}
