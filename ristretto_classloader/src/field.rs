use crate::Error::IllegalAccessError;
use crate::{Result, Value};
use ristretto_classfile::attributes::Attribute;
use ristretto_classfile::{BaseType, ClassFile, ConstantPool, FieldAccessFlags, FieldType};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::sync::Arc;

#[expect(clippy::struct_field_names)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Field {
    offset: u16,
    access_flags: FieldAccessFlags,
    field_type: FieldType,
    name: String,
    attributes: Vec<Attribute>,
}

impl Field {
    /// Create a new class field with the given parameters.
    #[must_use]
    pub fn new(
        offset: u16,
        access_flags: FieldAccessFlags,
        field_type: FieldType,
        name: String,
        attributes: Vec<Attribute>,
    ) -> Self {
        Self {
            offset,
            access_flags,
            field_type,
            name,
            attributes,
        }
    }

    /// Create a new class field with the given definition.
    ///
    /// # Errors
    ///
    /// if the field name cannot be read.
    pub fn from(
        class_file: &ClassFile,
        offset: u16,
        definition: &ristretto_classfile::Field,
    ) -> Result<Self> {
        let constant_pool = &class_file.constant_pool;
        let access_flags = definition.access_flags;
        let name = constant_pool.try_get_utf8(definition.name_index)?;
        let field_type = definition.field_type.clone();

        Ok(Self {
            offset,
            access_flags,
            field_type,
            name: name.to_string(),
            attributes: definition.attributes.clone(),
        })
    }

    /// Get the field offset.
    #[must_use]
    pub fn offset(&self) -> u16 {
        self.offset
    }

    /// Get the field access flags.
    #[must_use]
    pub fn access_flags(&self) -> &FieldAccessFlags {
        &self.access_flags
    }

    /// Get the field type.
    #[must_use]
    pub fn field_type(&self) -> &FieldType {
        &self.field_type
    }

    /// Get the field name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set the field value.
    ///
    /// # Errors
    ///
    /// - if the field is final.
    /// - if the value is not permissible for the field type.
    /// - if the lock is poisoned.
    pub fn check_value(&self, value: &Value) -> Result<()> {
        // TODO: Check that the field is not final
        // if self.access_flags.contains(FieldAccessFlags::FINAL) && *guarded_value != Value::Unused {
        //     let error = format!("Cannot set final field: {}", self.name);
        //     return Err(IllegalAccessError(error));
        // }
        // Check that the value permissible for the field type
        // See: https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.putstatic
        match self.field_type {
            FieldType::Base(
                BaseType::Boolean
                | BaseType::Byte
                | BaseType::Char
                | BaseType::Int
                | BaseType::Short,
            ) => {
                if !matches!(value, Value::Int(_)) {
                    return Err(IllegalAccessError(format!(
                        "Invalid value for {} field",
                        self.field_type
                    )));
                }
            }
            FieldType::Base(BaseType::Double) => {
                if !matches!(value, Value::Double(_)) {
                    return Err(IllegalAccessError(
                        "Invalid value for double field".to_string(),
                    ));
                }
            }
            FieldType::Base(BaseType::Float) => {
                if !matches!(value, Value::Float(_)) {
                    return Err(IllegalAccessError(
                        "Invalid value for float field".to_string(),
                    ));
                }
            }
            FieldType::Base(BaseType::Long) => {
                if !matches!(value, Value::Long(_)) {
                    return Err(IllegalAccessError(
                        "Invalid value for long field".to_string(),
                    ));
                }
            }
            FieldType::Object(_) | FieldType::Array(_) => {
                // TODO: Check that the value is of the correct type
                if !matches!(value, Value::Object(_)) {
                    return Err(IllegalAccessError(
                        "Invalid value for array field".to_string(),
                    ));
                }
            }
        }
        Ok(())
    }

    /// Get the attributes.
    #[must_use]
    pub fn attributes(&self) -> &Vec<Attribute> {
        &self.attributes
    }

    /// Get the default value for the field type.
    #[must_use]
    pub fn default_value(&self) -> Value {
        match self.field_type {
            FieldType::Base(
                BaseType::Boolean
                | BaseType::Byte
                | BaseType::Char
                | BaseType::Int
                | BaseType::Short,
            ) => Value::Int(0),
            FieldType::Base(BaseType::Double) => Value::Double(0.0),
            FieldType::Base(BaseType::Float) => Value::Float(0.0),
            FieldType::Base(BaseType::Long) => Value::Long(0),
            FieldType::Object(_) | FieldType::Array(_) => Value::Object(None),
        }
    }

    /// Get the default static value for the field type.
    ///
    /// # Errors
    ///
    /// - if the index is out of bounds for the constant pool.
    /// - if the value cannot be converted to the expected type.
    pub fn default_static_value(&self, constant_pool: &ConstantPool) -> Result<Value> {
        if self.access_flags.contains(FieldAccessFlags::STATIC) {
            let constant_value_index = self.attributes.iter().find_map(|attribute| {
                if let Attribute::ConstantValue {
                    constant_value_index,
                    ..
                } = attribute
                {
                    Some(*constant_value_index)
                } else {
                    None
                }
            });

            if let Some(constant_value_index) = constant_value_index {
                let value = match &self.field_type {
                    FieldType::Base(
                        BaseType::Boolean
                        | BaseType::Byte
                        | BaseType::Char
                        | BaseType::Int
                        | BaseType::Short,
                    ) => {
                        let value = constant_pool.try_get_integer(constant_value_index)?;
                        Value::Int(*value)
                    }
                    FieldType::Base(BaseType::Double) => {
                        let value = constant_pool.try_get_double(constant_value_index)?;
                        Value::Double(*value)
                    }
                    FieldType::Base(BaseType::Float) => {
                        let value = constant_pool.try_get_float(constant_value_index)?;
                        Value::Float(*value)
                    }
                    FieldType::Base(BaseType::Long) => {
                        let value = constant_pool.try_get_long(constant_value_index)?;
                        Value::Long(*value)
                    }
                    FieldType::Object(_class_name) => {
                        // Objects are loaded through a class initializer
                        Value::Unused
                    }
                    FieldType::Array(_field_type) => {
                        // Arrays are loaded through a class initializer
                        Value::Unused
                    }
                };
                return Ok(value);
            }
        }
        Ok(self.default_value())
    }
}

/// Trait for getting a field by either the name, or the offset.
pub trait FieldKey: Display + Debug + Copy + Eq + Hash {
    /// Check if the key is numeric (i.e., an offset).
    fn is_numeric_key(&self) -> bool {
        false
    }

    /// Check if the key matches the field.
    fn matches_field(&self, field: &Field) -> bool;

    /// Get the field by the key from the provided fields.
    fn get_field<'a>(&self, fields: &'a [Arc<Field>]) -> Option<(usize, &'a Arc<Field>)>;
}

/// Implementation of `FieldKey` for the offset.
impl FieldKey for usize {
    fn is_numeric_key(&self) -> bool {
        true
    }

    fn matches_field(&self, field: &Field) -> bool {
        field.offset as usize == *self
    }

    fn get_field<'a>(&self, fields: &'a [Arc<Field>]) -> Option<(usize, &'a Arc<Field>)> {
        if let Some(field) = fields.get(*self) {
            return Some((*self, field));
        }
        None
    }
}

/// Implementation of `FieldKey` for field name.
impl FieldKey for &String {
    fn matches_field(&self, field: &Field) -> bool {
        self.as_str().matches_field(field)
    }

    fn get_field<'a>(&self, fields: &'a [Arc<Field>]) -> Option<(usize, &'a Arc<Field>)> {
        self.as_str().get_field(fields)
    }
}

/// Implementation of `FieldKey` for field name.
impl FieldKey for &str {
    fn matches_field(&self, field: &Field) -> bool {
        field.name == *self
    }

    fn get_field<'a>(&self, fields: &'a [Arc<Field>]) -> Option<(usize, &'a Arc<Field>)> {
        fields
            .iter()
            .enumerate()
            .find(|(_, field)| self.matches_field(field))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ristretto_classfile::FieldAccessFlags;

    #[test]
    fn test_field_new() {
        let field = Field::new(
            0,
            FieldAccessFlags::PUBLIC,
            FieldType::Base(BaseType::Int),
            "test".to_string(),
            vec![],
        );
        assert_eq!(field.offset(), 0);
        assert_eq!(field.access_flags(), &FieldAccessFlags::PUBLIC);
        assert_eq!(field.field_type(), &FieldType::Base(BaseType::Int));
        assert_eq!(field.name(), "test");
        assert!(field.attributes.is_empty());
    }

    #[test]
    fn test_check_value_boolean() -> Result<()> {
        let field = Field::new(
            0,
            FieldAccessFlags::PUBLIC,
            FieldType::Base(BaseType::Boolean),
            "test".to_string(),
            vec![],
        );
        field.check_value(&Value::from(true))?;
        Ok(())
    }

    #[test]
    fn test_check_value_byte() -> Result<()> {
        let field = Field::new(
            0,
            FieldAccessFlags::PUBLIC,
            FieldType::Base(BaseType::Byte),
            "test".to_string(),
            vec![],
        );
        field.check_value(&Value::Int(1))?;
        Ok(())
    }

    #[test]
    fn test_check_value_char() -> Result<()> {
        let field = Field::new(
            0,
            FieldAccessFlags::PUBLIC,
            FieldType::Base(BaseType::Char),
            "test".to_string(),
            vec![],
        );
        field.check_value(&Value::Int(1))?;
        Ok(())
    }

    #[test]
    fn test_check_value_double() -> Result<()> {
        let field = Field::new(
            0,
            FieldAccessFlags::PUBLIC,
            FieldType::Base(BaseType::Double),
            "test".to_string(),
            vec![],
        );
        field.check_value(&Value::Double(1.0))?;
        Ok(())
    }

    #[test]
    fn test_check_value_float() -> Result<()> {
        let field = Field::new(
            0,
            FieldAccessFlags::PUBLIC,
            FieldType::Base(BaseType::Float),
            "test".to_string(),
            vec![],
        );
        field.check_value(&Value::Float(1.0))?;
        Ok(())
    }

    #[test]
    fn test_check_value_int() -> Result<()> {
        let field = Field::new(
            0,
            FieldAccessFlags::PUBLIC,
            FieldType::Base(BaseType::Int),
            "test".to_string(),
            vec![],
        );
        field.check_value(&Value::Int(1))?;
        Ok(())
    }

    #[test]
    fn test_check_value_long() -> Result<()> {
        let field = Field::new(
            0,
            FieldAccessFlags::PUBLIC,
            FieldType::Base(BaseType::Long),
            "test".to_string(),
            vec![],
        );
        field.check_value(&Value::Long(1))?;
        Ok(())
    }

    #[test]
    fn test_check_value_object() -> Result<()> {
        let field = Field::new(
            0,
            FieldAccessFlags::PUBLIC,
            FieldType::Object("java/lang/Object".to_string()),
            "test".to_string(),
            vec![],
        );
        field.check_value(&Value::Object(None))?;
        Ok(())
    }

    #[test]
    fn test_check_value_array() -> Result<()> {
        let field = Field::new(
            0,
            FieldAccessFlags::PUBLIC,
            FieldType::Array(Box::new(FieldType::Base(BaseType::Int))),
            "test".to_string(),
            vec![],
        );
        let value: Value = vec![42i32].into();
        field.check_value(&value)?;
        Ok(())
    }

    #[test]
    fn test_check_value_short() -> Result<()> {
        let field = Field::new(
            0,
            FieldAccessFlags::PUBLIC,
            FieldType::Base(BaseType::Short),
            "test".to_string(),
            vec![],
        );
        field.check_value(&Value::Int(1))?;
        Ok(())
    }

    #[test]
    fn test_check_value_invalid() {
        let field = Field::new(
            0,
            FieldAccessFlags::PUBLIC,
            FieldType::Base(BaseType::Int),
            "test".to_string(),
            vec![],
        );
        let result = field.check_value(&Value::Double(1.0));
        assert!(result.is_err());
    }

    #[test]
    fn test_field_key_get_by_offset() {
        let fields = vec![
            Arc::new(Field::new(
                0,
                FieldAccessFlags::PUBLIC,
                FieldType::Base(BaseType::Int),
                "field1".to_string(),
                vec![],
            )),
            Arc::new(Field::new(
                0,
                FieldAccessFlags::PUBLIC,
                FieldType::Base(BaseType::Int),
                "field2".to_string(),
                vec![],
            )),
        ];
        let key: usize = 1;
        let expected = fields.get(key).map(|field| (key, field));
        assert!(key.is_numeric_key());
        assert_eq!(expected, key.get_field(&fields));
    }

    #[test]
    fn test_field_key_get_by_name() {
        let fields = vec![
            Arc::new(Field::new(
                0,
                FieldAccessFlags::PUBLIC,
                FieldType::Base(BaseType::Int),
                "field1".to_string(),
                vec![],
            )),
            Arc::new(Field::new(
                0,
                FieldAccessFlags::PUBLIC,
                FieldType::Base(BaseType::Int),
                "field2".to_string(),
                vec![],
            )),
        ];
        let key = &"field2".to_string();
        let expected = fields
            .iter()
            .enumerate()
            .find(|(_, field)| field.name == key.as_str());
        assert!(!key.is_numeric_key());
        assert_eq!(expected, key.get_field(&fields));

        let key = key.as_str();
        let expected = fields
            .iter()
            .enumerate()
            .find(|(_, field)| field.name == key);
        assert!(!key.is_numeric_key());
        assert_eq!(expected, key.get_field(&fields));
    }
}
