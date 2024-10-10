use crate::Error::{IllegalAccessError, PoisonedLock};
use crate::{Result, Value};
use ristretto_classfile::attributes::Attribute;
use ristretto_classfile::{BaseType, ClassFile, ConstantPool, FieldAccessFlags, FieldType};
use std::sync::{Arc, RwLock};

#[derive(Clone, Debug)]
pub struct Field {
    access_flags: FieldAccessFlags,
    field_type: FieldType,
    name: String,
    value: Arc<RwLock<Value>>,
}

impl Field {
    /// Create a new class field with the given parameters.
    #[must_use]
    pub fn new(
        access_flags: FieldAccessFlags,
        field_type: FieldType,
        name: String,
        value: Value,
    ) -> Self {
        Self {
            access_flags,
            field_type,
            name,
            value: Arc::new(RwLock::new(value)),
        }
    }

    /// Create a new class field with the given definition.
    ///
    /// # Errors
    /// if the field name cannot be read.
    pub fn from(class_file: &ClassFile, definition: &ristretto_classfile::Field) -> Result<Self> {
        let constant_pool = &class_file.constant_pool;
        let access_flags = definition.access_flags;
        let name = constant_pool.try_get_utf8(definition.name_index)?.clone();
        let field_type = definition.field_type.clone();
        let mut value = if access_flags.contains(FieldAccessFlags::FINAL) {
            // Final fields are initialized by the class or object initializer
            Value::Unused
        } else {
            match field_type {
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
        };

        if access_flags.contains(FieldAccessFlags::STATIC) {
            for attribute in &definition.attributes {
                if let Attribute::ConstantValue {
                    constant_value_index,
                    ..
                } = attribute
                {
                    value = get_typed_value(&field_type, constant_pool, *constant_value_index)?;
                    break;
                }
            }
        }

        Ok(Self {
            access_flags,
            field_type,
            name,
            value: Arc::new(RwLock::new(value)),
        })
    }

    /// Get the field access flags.
    #[must_use]
    pub fn access_flags(&self) -> &FieldAccessFlags {
        &self.access_flags
    }

    /// Get the field name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the field type.
    #[must_use]
    pub fn field_type(&self) -> &FieldType {
        &self.field_type
    }

    /// Get the field value.
    ///
    /// # Errors
    pub fn value(&self) -> Result<Value> {
        let value = self
            .value
            .read()
            .map_err(|error| PoisonedLock(error.to_string()))?;
        Ok(value.clone())
    }

    /// Set the field value.
    ///
    /// # Errors
    /// if the field is final.
    pub fn set_value(&self, value: Value) -> Result<()> {
        let mut guarded_value = self
            .value
            .write()
            .map_err(|error| PoisonedLock(error.to_string()))?;
        // TODO: Check that the field is not final
        // if self.access_flags.contains(FieldAccessFlags::FINAL) && *guarded_value != Value::Unused {
        //     let error = format!("Cannot set final field: {}", self.name);
        //     return Err(IllegalAccessError(error));
        // }
        // Check that the value permissible for the field type
        // See: https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.putstatic
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

        *guarded_value = value;
        Ok(())
    }

    /// Set the field value without checking field permissions or type.
    ///
    /// # Errors
    /// if the field is final.
    pub fn unsafe_set_value(&self, value: Value) -> Result<()> {
        let mut guarded_value = self
            .value
            .write()
            .map_err(|error| PoisonedLock(error.to_string()))?;
        *guarded_value = value;
        Ok(())
    }
}

fn get_typed_value(
    field_type: &FieldType,
    constant_pool: &ConstantPool,
    index: u16,
) -> Result<Value> {
    let value = match field_type {
        FieldType::Base(BaseType::Boolean) => {
            let value = constant_pool.try_get_integer(index)?;
            Value::Int(*value)
        }
        FieldType::Base(BaseType::Byte) => {
            let value = constant_pool.try_get_integer(index)?;
            Value::Int(*value)
        }
        FieldType::Base(BaseType::Char) => {
            let value = constant_pool.try_get_integer(index)?;
            Value::Int(*value)
        }
        FieldType::Base(BaseType::Double) => {
            let value = constant_pool.try_get_double(index)?;
            Value::Double(*value)
        }
        FieldType::Base(BaseType::Float) => {
            let value = constant_pool.try_get_float(index)?;
            Value::Float(*value)
        }
        FieldType::Base(BaseType::Int) => {
            let value = constant_pool.try_get_integer(index)?;
            Value::Int(*value)
        }
        FieldType::Base(BaseType::Long) => {
            let value = constant_pool.try_get_long(index)?;
            Value::Long(*value)
        }
        FieldType::Base(BaseType::Short) => {
            let value = constant_pool.try_get_integer(index)?;
            Value::Int(*value)
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
    Ok(value)
}

impl PartialEq for Field {
    fn eq(&self, other: &Self) -> bool {
        let value = self.value.read().expect("poisoned lock");
        let other_value = other.value.read().expect("poisoned lock");
        self.access_flags == other.access_flags
            && self.field_type == other.field_type
            && self.name == other.name
            && *value == *other_value
    }
}
