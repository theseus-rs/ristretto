use crate::error::Result;
use bitflags::bitflags;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::Cursor;

bitflags! {
    /// Requires flags.
    ///
    /// See: <https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.7.25>
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct RequiresFlags: u16 {
        /// Indicates that any module which depends on the current module, implicitly declares a
        /// dependence on the module indicated by this entry.
        const TRANSITIVE = 0x0020;
        /// Indicates that this dependence is mandatory in the static phase, i.e., at compile time,
        /// but is optional in the dynamic phase, i.e., at run time.
        const STATIC_PHASE = 0x0040;
        /// Indicates that this dependence was not explicitly or implicitly declared in the source
        /// of the module declaration.
        const SYNTHETIC = 0x1000;
        /// Indicates that this dependence was implicitly declared in the source of the module
        /// declaration.
        const MANDATED = 0x8000;
    }
}

impl Default for RequiresFlags {
    fn default() -> RequiresFlags {
        RequiresFlags::empty()
    }
}

impl RequiresFlags {
    /// Deserialize the `RequiresFlags` from bytes.
    ///
    /// # Errors
    /// Should not occur; reserved for future use.
    pub fn from_bytes(bytes: &mut Cursor<Vec<u8>>) -> Result<RequiresFlags> {
        let access_flags = bytes.read_u16::<BigEndian>()?;
        let access_flags = RequiresFlags::from_bits_truncate(access_flags);
        Ok(access_flags)
    }

    /// Serialize the `RequiresFlags` to bytes.
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
        assert_eq!(RequiresFlags::empty(), RequiresFlags::default());
    }

    #[test]
    fn test_all_access_flags() {
        let access_flags: u16 = u16::MAX;
        let mut bytes = Cursor::new(access_flags.to_be_bytes().to_vec());
        assert_eq!(
            Ok(RequiresFlags::TRANSITIVE
                | RequiresFlags::STATIC_PHASE
                | RequiresFlags::SYNTHETIC
                | RequiresFlags::MANDATED),
            RequiresFlags::from_bytes(&mut bytes)
        );
    }

    #[test]
    fn test_access_flags() -> Result<()> {
        let access_flags = RequiresFlags::TRANSITIVE;
        let mut bytes = Vec::new();
        access_flags.to_bytes(&mut bytes)?;
        let mut bytes = Cursor::new(bytes);
        assert_eq!(Ok(access_flags), RequiresFlags::from_bytes(&mut bytes));
        Ok(())
    }
}
