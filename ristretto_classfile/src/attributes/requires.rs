use crate::attributes::RequiresFlags;
use crate::error::Result;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fmt;
use std::io::Cursor;

/// Implementation of `Requires`.
///
/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.7.25>
#[derive(Clone, Debug, PartialEq)]
pub struct Requires {
    pub index: u16,
    pub flags: RequiresFlags,
    pub version_index: u16,
}

impl Requires {
    /// Deserialize `Requires` from bytes.
    ///
    /// # Errors
    /// Should not occur; reserved for future use.
    pub fn from_bytes(bytes: &mut Cursor<Vec<u8>>) -> Result<Requires> {
        let index = bytes.read_u16::<BigEndian>()?;
        let flags = RequiresFlags::from_bytes(bytes)?;
        let version_index = bytes.read_u16::<BigEndian>()?;
        let require = Requires {
            index,
            flags,
            version_index,
        };
        Ok(require)
    }

    /// Serialize `Requires` to bytes.
    ///
    /// # Errors
    /// If the flags cannot be serialized.
    pub fn to_bytes(&self, bytes: &mut Vec<u8>) -> Result<()> {
        bytes.write_u16::<BigEndian>(self.index)?;
        self.flags.to_bytes(bytes)?;
        bytes.write_u16::<BigEndian>(self.version_index)?;
        Ok(())
    }
}

impl fmt::Display for Requires {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Requires[index={}, flags={}, version_index={}]",
            self.index, self.flags, self.version_index
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_to_string() {
        let requires = Requires {
            index: 1,
            flags: RequiresFlags::MANDATED,
            version_index: 3,
        };
        assert_eq!(
            "Requires[index=1, flags=(0x8000) ACC_MANDATED, version_index=3]",
            requires.to_string()
        );
    }

    #[test]
    fn test_serialization() -> Result<()> {
        let requires = Requires {
            index: 1,
            flags: RequiresFlags::MANDATED,
            version_index: 3,
        };
        let expected_value = [0, 1, 128, 0, 0, 3];
        let mut bytes = Vec::new();
        requires.clone().to_bytes(&mut bytes)?;
        assert_eq!(expected_value, &bytes[..]);

        let mut bytes = Cursor::new(expected_value.to_vec());
        assert_eq!(requires, Requires::from_bytes(&mut bytes)?);
        Ok(())
    }
}
