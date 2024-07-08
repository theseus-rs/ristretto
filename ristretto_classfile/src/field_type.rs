use crate::base_type::BaseType;
use crate::error::Result;
use crate::Error::{InvalidFieldTypeCode, InvalidFieldTypeDescriptor};

/// Implementation of `FieldType`.
///
/// See: <https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.3.2>
#[derive(Clone, Debug, PartialEq)]
pub enum FieldType {
    Base(BaseType),
    Object(String),
    Array(Box<FieldType>),
}

impl FieldType {
    /// Return the code for the `FieldType`.
    #[must_use]
    pub fn code(&self) -> char {
        match self {
            FieldType::Base(base_type) => base_type.code(),
            FieldType::Object(..) => 'L',
            FieldType::Array(..) => '[',
        }
    }

    /// Return the descriptor for the `FieldType`.
    #[must_use]
    pub fn descriptor(&self) -> String {
        match self {
            FieldType::Base(base_type) => base_type.code().to_string(),
            FieldType::Object(class_name) => format!("L{class_name};"),
            FieldType::Array(component_type) => {
                format!("[{}", component_type.descriptor())
            }
        }
    }

    /// Return the `FieldType` for a given code.
    ///
    /// # Errors
    /// - Returns an error if the code is invalid.
    /// - Returns an error if the descriptor is invalid.
    pub fn parse(descriptor: &String) -> Result<FieldType> {
        let mut chars = descriptor.chars();
        let code = chars.next().unwrap_or_default();
        let field_type = match code {
            'B' => FieldType::Base(BaseType::Byte),
            'C' => FieldType::Base(BaseType::Char),
            'D' => FieldType::Base(BaseType::Double),
            'F' => FieldType::Base(BaseType::Float),
            'I' => FieldType::Base(BaseType::Int),
            'J' => FieldType::Base(BaseType::Long),
            'S' => FieldType::Base(BaseType::Short),
            'Z' => FieldType::Base(BaseType::Boolean),
            'L' => {
                let class_name: String = chars.take(descriptor.len() - 2).collect();
                if !class_name.is_empty() && descriptor.ends_with(';') {
                    FieldType::Object(class_name)
                } else {
                    return Err(InvalidFieldTypeDescriptor(descriptor.to_string()));
                }
            }
            '[' => {
                let component_type = Self::parse(&chars.collect())?;
                FieldType::Array(Box::new(component_type))
            }
            _ => return Err(InvalidFieldTypeCode(code)),
        };

        Ok(field_type)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_invalid_code() {
        assert_eq!(
            Err(InvalidFieldTypeCode('0')),
            FieldType::parse(&"0".to_string())
        );
    }

    fn test_field_type(field_type: &FieldType, descriptor: &str, code: char) -> Result<()> {
        assert_eq!(code, field_type.code());
        let field_type_descriptor = field_type.descriptor();
        assert_eq!(descriptor.to_string(), field_type_descriptor);
        let parsed_field_type = FieldType::parse(&field_type_descriptor)?;
        assert_eq!(*field_type, parsed_field_type);
        Ok(())
    }

    #[test]
    fn test_base_byte() -> Result<()> {
        test_field_type(&FieldType::Base(BaseType::Byte), "B", 'B')
    }

    #[test]
    fn test_base_char() -> Result<()> {
        test_field_type(&FieldType::Base(BaseType::Char), "C", 'C')
    }

    #[test]
    fn test_base_double() -> Result<()> {
        test_field_type(&FieldType::Base(BaseType::Double), "D", 'D')
    }

    #[test]
    fn test_base_float() -> Result<()> {
        test_field_type(&FieldType::Base(BaseType::Float), "F", 'F')
    }

    #[test]
    fn test_base_int() -> Result<()> {
        test_field_type(&FieldType::Base(BaseType::Int), "I", 'I')
    }

    #[test]
    fn test_base_long() -> Result<()> {
        test_field_type(&FieldType::Base(BaseType::Long), "J", 'J')
    }

    #[test]
    fn test_base_short() -> Result<()> {
        test_field_type(&FieldType::Base(BaseType::Short), "S", 'S')
    }

    #[test]
    fn test_base_boolean() -> Result<()> {
        test_field_type(&FieldType::Base(BaseType::Boolean), "Z", 'Z')
    }

    #[test]
    fn test_object() -> Result<()> {
        test_field_type(&FieldType::Object("Foo".to_string()), "LFoo;", 'L')
    }

    #[test]
    fn test_object_no_semicolon_invalid() {
        let descriptor = "Lfoo".to_string();
        assert_eq!(
            Err(InvalidFieldTypeDescriptor(descriptor.clone())),
            FieldType::parse(&descriptor)
        );
    }

    #[test]
    fn test_object_no_class_name_invalid() {
        let descriptor = "L;".to_string();
        assert_eq!(
            Err(InvalidFieldTypeDescriptor(descriptor.clone())),
            FieldType::parse(&descriptor)
        );
    }

    #[test]
    fn test_array() -> Result<()> {
        let component_type = FieldType::Base(BaseType::Int);
        test_field_type(&FieldType::Array(Box::new(component_type)), "[I", '[')
    }
}
