use crate::attributes::Attribute;
use crate::constant_pool::ConstantPool;
use crate::error::Result;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fmt;
use std::io::Cursor;

/// Implementation of `Record`.
///
/// # Examples
///
/// ```rust
/// use ristretto_classfile::attributes::{Attribute, Record};
/// use ristretto_classfile::{ConstantPool, Result};
/// use std::io::Cursor;
///
/// let attribute = Attribute::ConstantValue {
///     name_index: 1, // Index in constant pool for "ConstantValue"
///     constant_value_index: 42, // Index in constant pool for the actual constant
/// };
/// let mut constant_pool = ConstantPool::default();
/// // Add necessary constants for the attribute to be valid during serialization,
/// // especially if the attribute itself needs to resolve names from the pool.
/// // For ConstantValue, name_index refers to "ConstantValue" UTF8 string.
/// let _name_idx = constant_pool.add_utf8("ConstantValue")?;
///
/// let record = Record {
///     name_index: 2, // Index to a Utf8 for record component name
///     descriptor_index: 3, // Index to a Utf8 for record component descriptor
///     attributes: vec![attribute.clone()],
/// };
///
/// // Serialize
/// let mut bytes = Vec::new();
/// record.to_bytes(&mut bytes)?;
///
/// // Deserialize
/// let mut cursor = Cursor::new(bytes);
/// let deserialized_record = Record::from_bytes(&constant_pool, &mut cursor)?;
///
/// assert_eq!(record, deserialized_record);
/// # Ok::<(), ristretto_classfile::Error>(())
/// ```
///
/// # References
///
/// - [JVM Specification §4.7.30](https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.7.30)
#[derive(Clone, Debug, PartialEq)]
pub struct Record {
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes: Vec<Attribute>,
}

impl Record {
    /// Deserialize the record from bytes.
    ///
    /// This function reads a Record structure from a byte stream according to the
    /// Java class file format specification.
    ///
    /// # Errors
    ///
    /// If the byte stream does not contain enough data to read a complete Record.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::Record;
    /// use ristretto_classfile::ConstantPool;
    /// use std::io::Cursor;
    ///
    /// let mut constant_pool = ConstantPool::default();
    /// // Create a byte array representing a serialized Record
    /// let bytes = vec![
    ///     0, 5,             // name_index (5)
    ///     0, 10,            // descriptor_index (10)
    ///     0, 0              // attributes_count (0)
    /// ];
    /// let mut cursor = Cursor::new(bytes);
    ///
    /// // Deserialize the Record
    /// let record = Record::from_bytes(&constant_pool, &mut cursor)?;
    ///
    /// assert_eq!(record.name_index, 5);
    /// assert_eq!(record.descriptor_index, 10);
    /// assert!(record.attributes.is_empty());
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
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
    /// This function writes the Record structure to a byte array according to the
    /// Java class file format specification.
    ///
    /// # Errors
    ///
    /// If the number of attributes exceeds `u16::MAX`, or if the indices are out of bounds.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::{Attribute, Record};
    /// use ristretto_classfile::ConstantPool;
    ///
    /// // Create a Record instance
    /// let record = Record {
    ///     name_index: 5,
    ///     descriptor_index: 10,
    ///     attributes: vec![],
    /// };
    ///
    /// // Serialize to bytes
    /// let mut bytes = Vec::new();
    /// record.to_bytes(&mut bytes)?;
    ///
    /// // Expected serialized format
    /// let expected = vec![
    ///     0, 5,             // name_index (5)
    ///     0, 10,            // descriptor_index (10)
    ///     0, 0              // attributes_count (0)
    /// ];
    ///
    /// assert_eq!(bytes, expected);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
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

impl fmt::Display for Record {
    /// Implementation of the Display trait for Record.
    ///
    /// This allows a Record to be formatted as a string in a human-readable form.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::{Attribute, Record};
    ///
    /// # fn main() {
    /// let record = Record {
    ///     name_index: 5,
    ///     descriptor_index: 10,
    ///     attributes: vec![],
    /// };
    ///
    /// let output = record.to_string();
    /// assert_eq!(output, "Record[name_index=5, descriptor_index=10, attributes=[]]");
    /// # }
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Record[name_index={}, descriptor_index={}, attributes={:?}]",
            self.name_index, self.descriptor_index, self.attributes
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_to_string() {
        let attribute = Attribute::ConstantValue {
            name_index: 1,
            constant_value_index: 42,
        };
        let record = Record {
            name_index: 1,
            descriptor_index: 2,
            attributes: vec![attribute],
        };
        assert_eq!(
            "Record[name_index=1, descriptor_index=2, attributes=[ConstantValue { name_index: 1, constant_value_index: 42 }]]",
            record.to_string()
        );
    }

    #[test]
    fn test_serialization() -> Result<()> {
        let attribute = Attribute::ConstantValue {
            name_index: 1,
            constant_value_index: 42,
        };
        let mut constant_pool = ConstantPool::default();
        constant_pool.add_utf8("ConstantValue")?;
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
