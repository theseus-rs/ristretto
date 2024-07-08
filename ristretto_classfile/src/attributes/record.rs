use crate::attributes::Attribute;
use crate::constant_pool::ConstantPool;
use crate::error::Result;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::Cursor;

/// Implementation of `Record`.
///
/// See: <https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.7.30>
#[derive(Clone, Debug, PartialEq)]
pub struct Record {
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes: Vec<Attribute>,
}

impl Record {
    /// Deserialize the record from bytes.
    ///
    /// # Errors
    /// Should not occur; reserved for future use.
    pub fn from_bytes(constant_pool: &ConstantPool, bytes: &mut Cursor<Vec<u8>>) -> Result<Record> {
        let name_index = bytes.read_u16::<BigEndian>()?;
        let descriptor_index = bytes.read_u16::<BigEndian>()?;
        let attributes_count = bytes.read_u16::<BigEndian>()? as usize;
        let mut attributes = Vec::with_capacity(attributes_count);
        for _ in 0..attributes_count {
            let attribute = Attribute::from_bytes(constant_pool, bytes)?;
            attributes.push(attribute);
        }
        let record = Record {
            name_index,
            descriptor_index,
            attributes,
        };
        Ok(record)
    }

    /// Serialize the record to bytes.
    ///
    /// # Errors
    /// - If the number of attributes exceeds 65,535.
    /// - If an attribute fails to serialize.
    pub fn to_bytes(&self, bytes: &mut Vec<u8>) -> Result<()> {
        bytes.write_u16::<BigEndian>(self.name_index)?;
        bytes.write_u16::<BigEndian>(self.descriptor_index)?;

        let attributes_length = u16::try_from(self.attributes.len())?;
        bytes.write_u16::<BigEndian>(attributes_length)?;
        for attribute in &self.attributes {
            attribute.to_bytes(bytes)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::constant::Constant;

    #[test]
    fn test_serialization() -> Result<()> {
        let attribute = Attribute::ConstantValue {
            name_index: 1,
            constantvalue_index: 42,
        };
        let mut constant_pool = ConstantPool::default();
        constant_pool.add(Constant::Utf8("ConstantValue".to_string()));
        let record = Record {
            name_index: 1,
            descriptor_index: 2,
            attributes: vec![attribute],
        };
        let expected_value = [0, 1, 0, 2, 0, 1, 0, 1, 0, 0, 0, 2, 0, 42];
        let mut bytes = Vec::new();
        record.clone().to_bytes(&mut bytes)?;
        assert_eq!(expected_value, &bytes[..]);

        let mut bytes = Cursor::new(expected_value.to_vec());
        assert_eq!(record, Record::from_bytes(&constant_pool, &mut bytes)?);
        Ok(())
    }
}
