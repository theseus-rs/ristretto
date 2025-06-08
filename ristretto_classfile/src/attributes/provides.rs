use crate::error::Result;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fmt;
use std::io::Cursor;

/// Implementation of `Provides`.
///
/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.7.25>
#[derive(Clone, Debug, PartialEq)]
pub struct Provides {
    pub index: u16,
    pub with_index: Vec<u16>,
}

impl Provides {
    /// Deserialize the provides from bytes.
    ///
    /// # Errors
    ///
    /// Should not occur; reserved for future use.
    pub fn from_bytes(bytes: &mut Cursor<Vec<u8>>) -> Result<Provides> {
        let index = bytes.read_u16::<BigEndian>()?;
        let to_index_count = bytes.read_u16::<BigEndian>()?;
        let mut with_index = Vec::with_capacity(to_index_count as usize);
        for _ in 0..to_index_count {
            with_index.push(bytes.read_u16::<BigEndian>()?);
        }
        let requires = Provides { index, with_index };
        Ok(requires)
    }

    /// Serialize the provides to bytes.
    ///
    /// # Errors
    ///
    /// If there are more than 65,534 `with_index` values.
    pub fn to_bytes(&self, bytes: &mut Vec<u8>) -> Result<()> {
        bytes.write_u16::<BigEndian>(self.index)?;

        let with_index_length = u16::try_from(self.with_index.len())?;
        bytes.write_u16::<BigEndian>(with_index_length)?;
        for index in &self.with_index {
            bytes.write_u16::<BigEndian>(*index)?;
        }

        Ok(())
    }
}

impl fmt::Display for Provides {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Provides[index={}, with_index={:?}]",
            self.index, self.with_index
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_to_string() {
        let provides = Provides {
            index: 1,
            with_index: vec![2],
        };
        assert_eq!("Provides[index=1, with_index=[2]]", provides.to_string());
    }

    #[test]
    fn test_serialization() -> Result<()> {
        let provides = Provides {
            index: 1,
            with_index: vec![2],
        };
        let expected_value = [0, 1, 0, 1, 0, 2];
        let mut bytes = Vec::new();
        provides.clone().to_bytes(&mut bytes)?;
        assert_eq!(expected_value, &bytes[..]);

        let mut bytes = Cursor::new(expected_value.to_vec());
        assert_eq!(provides, Provides::from_bytes(&mut bytes)?);
        Ok(())
    }
}
