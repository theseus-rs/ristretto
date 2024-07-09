use crate::attributes::nested_class_access_flags::NestedClassAccessFlags;
use crate::error::Result;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::Cursor;

/// Implementation of `InnerClass`.
///
/// See: <https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.7.6>
#[derive(Clone, Debug, PartialEq)]
pub struct InnerClass {
    pub class_info_index: u16,
    pub outer_class_info_index: u16,
    pub name_index: u16,
    pub access_flags: NestedClassAccessFlags,
}

impl InnerClass {
    /// Deserialize the inner class from bytes.
    ///
    /// # Errors
    /// Should not occur; reserved for future use.
    pub fn from_bytes(bytes: &mut Cursor<Vec<u8>>) -> Result<InnerClass> {
        let class_info_index = bytes.read_u16::<BigEndian>()?;
        let outer_class_info_index = bytes.read_u16::<BigEndian>()?;
        let name_index = bytes.read_u16::<BigEndian>()?;
        let access_flags = NestedClassAccessFlags::from_bytes(bytes)?;

        let inner_class = InnerClass {
            class_info_index,
            outer_class_info_index,
            name_index,
            access_flags,
        };
        Ok(inner_class)
    }

    /// Serialize the inner class to bytes.
    ///
    /// # Errors
    /// If class access flags cannot be serialized.
    pub fn to_bytes(&self, bytes: &mut Vec<u8>) -> Result<()> {
        bytes.write_u16::<BigEndian>(self.class_info_index)?;
        bytes.write_u16::<BigEndian>(self.outer_class_info_index)?;
        bytes.write_u16::<BigEndian>(self.name_index)?;
        self.access_flags.to_bytes(bytes)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_serialization() -> Result<()> {
        let inner_class = InnerClass {
            class_info_index: 1,
            outer_class_info_index: 2,
            name_index: 3,
            access_flags: NestedClassAccessFlags::PUBLIC,
        };
        let expected_value = [0, 1, 0, 2, 0, 3, 0, 1];
        let mut bytes = Vec::new();
        inner_class.clone().to_bytes(&mut bytes)?;
        assert_eq!(expected_value, &bytes[..]);

        let mut bytes = Cursor::new(expected_value.to_vec());
        assert_eq!(inner_class, InnerClass::from_bytes(&mut bytes)?);
        Ok(())
    }
}
