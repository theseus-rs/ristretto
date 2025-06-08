use crate::FieldType;
use crate::attributes::Attribute;
use crate::constant_pool::ConstantPool;
use crate::display::indent_lines;
use crate::error::Result;
use crate::field_access_flags::FieldAccessFlags;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fmt;
use std::io::Cursor;

/// Represents a field in a Java class file.
///
/// Fields store data in Java classes and can be static or instance variables.
/// Each field has access flags (e.g., public, private), a name, a descriptor
/// indicating its type, and optional attributes for additional metadata.
///
/// # Examples
///
/// Creating a field representing a public static final integer constant:
///
/// ```rust
/// use ristretto_classfile::{BaseType, Field, FieldAccessFlags, FieldType};
///
/// // Create the field with appropriate access flags
/// let field = Field {
///     access_flags: FieldAccessFlags::PUBLIC,
///     name_index: 1,
///     descriptor_index: 2,
///     field_type: FieldType::Base(BaseType::Int),
///     attributes: vec![],
/// };
///
/// // Use the field
/// println!("Field name index: {}", field.name_index);
/// println!("Field access flags: {}", field.access_flags);
/// # Ok::<(), ristretto_classfile::Error>(())
/// ```
///
/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.5>
#[derive(Clone, Debug, PartialEq)]
pub struct Field {
    pub access_flags: FieldAccessFlags,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub field_type: FieldType,
    pub attributes: Vec<Attribute>,
}

impl Field {
    /// Deserialize a `Field` from bytes in a Java class file.
    ///
    /// This function reads a `field_info` structure from the provided byte cursor and constructs a
    /// `Field` struct. It reads access flags, name and descriptor indices, parses the field type,
    /// and reads all attributes associated with the field.
    ///
    /// # Errors
    ///
    /// Returns an error if the bytes do not represent a valid Field.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::{Field, ConstantPool};
    /// use std::io::Cursor;
    ///
    /// // Prepare a constant pool with necessary entries
    /// let mut constant_pool = ConstantPool::default();
    /// let name_index = constant_pool.add_utf8("count")?;
    /// let descriptor_index = constant_pool.add_utf8("I")?;
    ///
    /// // Create a byte array representing a field_info structure
    /// // Format: [access_flags(2), name_index(2), descriptor_index(2), attributes_count(2), attributes(...)]
    /// let field_bytes = vec![
    ///     0x00, 0x01, // ACC_PUBLIC
    ///     0x00, name_index as u8, // name_index
    ///     0x00, descriptor_index as u8, // descriptor_index
    ///     0x00, 0x00, // attributes_count = 0
    /// ];
    ///
    /// // Parse the field from bytes
    /// let mut cursor = Cursor::new(field_bytes);
    /// let field = Field::from_bytes(&constant_pool, &mut cursor)?;
    ///
    /// assert_eq!(field.name_index, name_index);
    /// assert_eq!(field.descriptor_index, descriptor_index);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
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

    /// Serialize the `Field` to bytes for inclusion in a class file.
    ///
    /// This function writes the `field_info` structure to the provided byte vector according to the
    /// Java class file format specification.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::{BaseType, Field, FieldAccessFlags, FieldType};
    ///
    /// // Create a field
    /// let field = Field {
    ///     access_flags: FieldAccessFlags::PRIVATE | FieldAccessFlags::FINAL,
    ///     name_index: 5, // index in constant pool
    ///     descriptor_index: 6, // index in constant pool
    ///     field_type: FieldType::Base(BaseType::Int),
    ///     attributes: vec![],
    /// };
    ///
    /// // Serialize to bytes
    /// let mut bytes = Vec::new();
    /// field.to_bytes(&mut bytes)?;
    ///
    /// // The resulting bytes can be included in a class file
    /// assert_eq!(bytes.len(), 8); // 2 (flags) + 2 (name_idx) + 2 (desc_idx) + 2 (attr_count)
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
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
    /// Formats the `Field` for display.
    ///
    /// This produces a human-readable multi-line representation of the field, including:
    /// - Access flags
    /// - Name index
    /// - Descriptor index
    /// - Field type
    /// - Attributes (indented)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::{BaseType, ConstantPool, Field, FieldAccessFlags, FieldType};
    /// use std::io::Cursor;
    ///
    /// // Set up a simple field with one attribute
    /// let mut constant_pool = ConstantPool::default();
    /// constant_pool.add_utf8("ConstantValue")?;
    /// constant_pool.add_utf8("I")?;
    ///
    /// let field = Field {
    ///     access_flags: FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC,
    ///     name_index: 1,
    ///     descriptor_index: 2,
    ///     field_type: FieldType::Base(BaseType::Int),
    ///     attributes: vec![],
    /// };
    ///
    /// // Display output will look like:
    /// // flags: (0x0009) ACC_PUBLIC, ACC_STATIC
    /// // name_index: #1
    /// // descriptor_index: #2
    /// // field_type: Base(Int)
    /// println!("{field}");
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "flags: {}", self.access_flags)?;
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
    use crate::BaseType;
    use crate::attributes::Attribute;
    use crate::field_access_flags::FieldAccessFlags;
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
            flags: (0x0001) ACC_PUBLIC
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
