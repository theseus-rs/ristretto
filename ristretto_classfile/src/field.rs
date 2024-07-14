use crate::attributes::Attribute;
use crate::constant_pool::ConstantPool;
use crate::display::indent_lines;
use crate::error::Result;
use crate::field_access_flags::FieldAccessFlags;
use crate::FieldType;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fmt;
use std::io::Cursor;

/// Field.
///
/// See: <https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.5>
#[derive(Clone, Debug, PartialEq)]
pub struct Field {
    pub access_flags: FieldAccessFlags,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub field_type: FieldType,
    pub attributes: Vec<Attribute>,
}

impl Field {
    /// Deserialize the Field from bytes.
    ///
    /// # Errors
    /// Returns an error if the bytes do not represent a valid Field.
    pub fn from_bytes(constant_pool: &ConstantPool, bytes: &mut Cursor<Vec<u8>>) -> Result<Field> {
        let access_flags = FieldAccessFlags::from_bytes(bytes)?;
        let name_index = bytes.read_u16::<BigEndian>()?;
        let descriptor_index = bytes.read_u16::<BigEndian>()?;
        let field_type_descriptor = constant_pool.try_get_utf8(descriptor_index)?;
        let field_type = FieldType::parse(field_type_descriptor)?;

        let attribute_count = bytes.read_u16::<BigEndian>()?;
        let mut attributes = Vec::with_capacity(attribute_count as usize);
        for _ in 0..attribute_count {
            let attribute = Attribute::from_bytes(constant_pool, bytes)?;
            attributes.push(attribute);
        }

        let field = Field {
            access_flags,
            name_index,
            descriptor_index,
            field_type,
            attributes,
        };
        Ok(field)
    }

    /// Serialize the Field to bytes.
    ///
    /// # Errors
    /// If there are more than 65,534 attributes, an error is returned.
    pub fn to_bytes(&self, bytes: &mut Vec<u8>) -> Result<()> {
        self.access_flags.to_bytes(bytes)?;
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

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "access_flags: {}", self.access_flags)?;
        writeln!(f, "name_index: #{}", self.name_index)?;
        writeln!(f, "descriptor_index: #{}", self.descriptor_index)?;
        writeln!(f, "field_type: {:?}", self.field_type)?;
        writeln!(f, "attributes:")?;
        for attribute in &self.attributes {
            writeln!(f, "{}", indent_lines(&attribute.to_string(), "  "))?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::attributes::Attribute;
    use crate::field_access_flags::FieldAccessFlags;
    use crate::BaseType;
    use indoc::indoc;

    #[test]
    fn test_to_string() -> Result<()> {
        let mut constant_pool = ConstantPool::default();
        constant_pool.add_utf8("ConstantValue")?;
        constant_pool.add_utf8("I")?;
        let mut attribute_bytes = Cursor::new([0, 1, 0, 0, 0, 2, 4, 2].to_vec());
        let attribute = Attribute::from_bytes(&constant_pool, &mut attribute_bytes)?;
        let field = Field {
            access_flags: FieldAccessFlags::PUBLIC,
            name_index: 1,
            descriptor_index: 2,
            field_type: FieldType::Base(BaseType::Int),
            attributes: vec![attribute],
        };

        let expected = indoc! {r"
            access_flags: (0x0001) ACC_PUBLIC
            name_index: #1
            descriptor_index: #2
            field_type: Base(Int)
            attributes:
              ConstantValue { name_index: 1, constant_value_index: 1026 }
        "};
        assert_eq!(expected, field.to_string());
        Ok(())
    }

    #[test]
    fn test_field() -> Result<()> {
        let mut constant_pool = ConstantPool::default();
        constant_pool.add_utf8("ConstantValue")?;
        constant_pool.add_utf8("I")?;
        let mut attribute_bytes = Cursor::new([0, 1, 0, 0, 0, 2, 4, 2].to_vec());
        let attribute = Attribute::from_bytes(&constant_pool, &mut attribute_bytes)?;
        let field = Field {
            access_flags: FieldAccessFlags::PUBLIC,
            name_index: 1,
            descriptor_index: 2,
            field_type: FieldType::Base(BaseType::Int),
            attributes: vec![attribute],
        };

        let mut bytes = Vec::new();
        field.to_bytes(&mut bytes)?;

        let mut bytes = Cursor::new(bytes);
        let result = Field::from_bytes(&constant_pool, &mut bytes)?;
        assert_eq!(result, field);
        Ok(())
    }
}
