use crate::base_type::BaseType;
use crate::error::Result;
use crate::Error::{InvalidFieldTypeCode, InvalidFieldTypeDescriptor};
use std::{fmt, io};

/// Implementation of `FieldType`.
///
/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.3.2>
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

    /// Return the class name for the `FieldType`.
    #[must_use]
    pub fn class_name(&self) -> String {
        match self {
            FieldType::Base(base_type) => base_type.class_name().to_string(),
            FieldType::Object(class_name) => class_name.to_string(),
            FieldType::Array(component_type) => match &**component_type {
                FieldType::Base(base_type) => format!("[{}", base_type.code()),
                FieldType::Object(class_name) => format!("L{class_name};"),
                FieldType::Array(component_type) => format!("[{component_type}"),
            },
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
            'L' => {
                let take_chars = descriptor.len().checked_sub(2).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidData, "Invalid descriptor length")
                })?;
                let class_name: String = chars.take(take_chars).collect();
                if !class_name.is_empty() && descriptor.ends_with(';') {
                    FieldType::Object(class_name)
                } else {
                    return Err(InvalidFieldTypeDescriptor(descriptor.to_string()));
                }
            }
            '[' => {
                let component_type = Self::parse(&chars.collect())?;
                FieldType::Array(component_type.into())
            }
            _ => {
                let Ok(base_type) = BaseType::parse(code) else {
                    return Err(InvalidFieldTypeCode(code));
                };
                FieldType::Base(base_type)
            }
        };

        Ok(field_type)
    }
}

impl fmt::Display for FieldType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FieldType::Base(base_type) => write!(f, "{}", base_type.class_name()),
            FieldType::Object(class_name) => write!(f, "{class_name}"),
            FieldType::Array(component_type) => write!(f, "{component_type}[]"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Error::IoError;

    #[test]
    fn test_invalid_code() {
        assert_eq!(
            Err(InvalidFieldTypeCode('0')),
            FieldType::parse(&"0".to_string())
        );
    }

    fn test_field_type(
        field_type: &FieldType,
        descriptor: &str,
        code: char,
        class_name: &str,
    ) -> Result<()> {
        assert_eq!(code, field_type.code());
        assert_eq!(class_name, field_type.class_name());
        let field_type_descriptor = field_type.descriptor();
        assert_eq!(descriptor.to_string(), field_type_descriptor);
        let parsed_field_type = FieldType::parse(&field_type_descriptor)?;
        assert_eq!(*field_type, parsed_field_type);
        Ok(())
    }

    #[test]
    fn test_base_boolean() -> Result<()> {
        let field_type = FieldType::Base(BaseType::Boolean);

        assert_eq!("boolean", field_type.to_string());
        test_field_type(&field_type, "Z", 'Z', "boolean")
    }

    #[test]
    fn test_base_byte() -> Result<()> {
        let field_type = FieldType::Base(BaseType::Byte);

        assert_eq!("byte", field_type.to_string());
        test_field_type(&field_type, "B", 'B', "byte")
    }

    #[test]
    fn test_base_char() -> Result<()> {
        let field_type = FieldType::Base(BaseType::Char);

        assert_eq!("char", field_type.to_string());
        test_field_type(&field_type, "C", 'C', "char")
    }

    #[test]
    fn test_base_double() -> Result<()> {
        let field_type = FieldType::Base(BaseType::Double);

        assert_eq!("double", field_type.to_string());
        test_field_type(&field_type, "D", 'D', "double")
    }

    #[test]
    fn test_base_float() -> Result<()> {
        let field_type = FieldType::Base(BaseType::Float);

        assert_eq!("float", field_type.to_string());
        test_field_type(&field_type, "F", 'F', "float")
    }

    #[test]
    fn test_base_int() -> Result<()> {
        let field_type = FieldType::Base(BaseType::Int);

        assert_eq!("int", field_type.to_string());
        test_field_type(&field_type, "I", 'I', "int")
    }

    #[test]
    fn test_base_long() -> Result<()> {
        let field_type = FieldType::Base(BaseType::Long);

        assert_eq!("long", field_type.to_string());
        test_field_type(&field_type, "J", 'J', "long")
    }

    #[test]
    fn test_base_short() -> Result<()> {
        let field_type = FieldType::Base(BaseType::Short);

        assert_eq!("short", field_type.to_string());
        test_field_type(&field_type, "S", 'S', "short")
    }

    #[test]
    fn test_object() -> Result<()> {
        let field_type = FieldType::Object("Foo".to_string());

        assert_eq!("Foo", field_type.to_string());
        test_field_type(&field_type, "LFoo;", 'L', "Foo")
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
        let field_type = FieldType::Array(component_type.into());

        assert_eq!("int[]", field_type.to_string());
        test_field_type(&field_type, "[I", '[', "[I")
    }

    #[test]
    fn test_parse_invalid() {
        let descriptor = "L".to_string();
        assert_eq!(
            Err(IoError("Invalid descriptor length".to_string())),
            FieldType::parse(&descriptor)
        );
    }
}
