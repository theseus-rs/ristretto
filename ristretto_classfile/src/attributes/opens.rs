use crate::attributes::OpensFlags;
use crate::error::Result;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fmt;
use std::io::Cursor;

/// Implementation of `Opens`.
///
/// See: <https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.7.25>
#[derive(Clone, Debug, PartialEq)]
pub struct Opens {
    pub index: u16,
    pub flags: OpensFlags,
    pub to_index: Vec<u16>,
}

impl Opens {
    /// Deserialize the opens from bytes.
    ///
    /// # Errors
    /// Should not occur; reserved for future use.
    pub fn from_bytes(bytes: &mut Cursor<Vec<u8>>) -> Result<Opens> {
        let index = bytes.read_u16::<BigEndian>()?;
        let flags = OpensFlags::from_bytes(bytes)?;
        let to_index_count = bytes.read_u16::<BigEndian>()?;
        let mut to_index = Vec::with_capacity(to_index_count as usize);
        for _ in 0..to_index_count {
            to_index.push(bytes.read_u16::<BigEndian>()?);
        }
        let requires = Opens {
            index,
            flags,
            to_index,
        };
        Ok(requires)
    }

    /// Serialize the opens to bytes.
    ///
    /// # Errors
    /// If there are more than 65,534 `to_index` values.
    pub fn to_bytes(&self, bytes: &mut Vec<u8>) -> Result<()> {
        bytes.write_u16::<BigEndian>(self.index)?;
        self.flags.to_bytes(bytes)?;

        let to_index_length = u16::try_from(self.to_index.len())?;
        bytes.write_u16::<BigEndian>(to_index_length)?;
        for index in &self.to_index {
            bytes.write_u16::<BigEndian>(*index)?;
        }
        Ok(())
    }
}

impl fmt::Display for Opens {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "index: {}, flags: {}, to_index: {:?}",
            self.index, self.flags, self.to_index
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test_log::test]
    fn test_display() {
        let opens = Opens {
            index: 1,
            flags: OpensFlags::MANDATED,
            to_index: vec![3],
        };
        assert_eq!(
            "index: 1, flags: (0x8000) ACC_MANDATED, to_index: [3]",
            opens.to_string()
        );
    }

    #[test_log::test]
    fn test_serialization() -> Result<()> {
        let opens = Opens {
            index: 1,
            flags: OpensFlags::MANDATED,
            to_index: vec![3],
        };
        let expected_value = [0, 1, 128, 0, 0, 1, 0, 3];
        let mut bytes = Vec::new();
        opens.clone().to_bytes(&mut bytes)?;
        assert_eq!(expected_value, &bytes[..]);

        let mut bytes = Cursor::new(expected_value.to_vec());
        assert_eq!(opens, Opens::from_bytes(&mut bytes)?);
        Ok(())
    }
}
