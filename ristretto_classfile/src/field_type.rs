use crate::Error::{InvalidFieldTypeCode, InvalidFieldTypeDescriptor, InvalidMethodDescriptor};
use crate::base_type::BaseType;
use crate::error::Result;
use std::{fmt, io};

/// Represents a Java field type descriptor as defined in the JVM specification.
///
/// A `FieldType` can be:
/// - A base type (primitive Java types like `int`, `boolean`, etc.)
/// - An object type (class or interface)
/// - An array type (arrays of any other field type)
///
/// This enum is used to represent Java type signatures in classfile structures such as field
/// descriptors, method descriptors, and signatures.
///
/// # Examples
///
/// Creating different types of field descriptors:
///
/// ```rust
/// use ristretto_classfile::{BaseType, FieldType};
///
/// // Create a primitive type (int)
/// let int_type = FieldType::Base(BaseType::Int);
/// assert_eq!(int_type.descriptor(), "I");
/// assert_eq!(int_type.to_string(), "int");
///
/// // Create an object type (String)
/// let string_type = FieldType::Object("java/lang/String".to_string());
/// assert_eq!(string_type.descriptor(), "Ljava/lang/String;");
/// assert_eq!(string_type.to_string(), "java/lang/String");
///
/// // Create an array type (int[])
/// let int_array = FieldType::Array(Box::new(FieldType::Base(BaseType::Int)));
/// assert_eq!(int_array.descriptor(), "[I");
/// assert_eq!(int_array.to_string(), "int[]");
///
/// // Create a multidimensional array (String[][])
/// let string_2d_array = FieldType::Array(Box::new(
///     FieldType::Array(Box::new(
///         FieldType::Object("java/lang/String".to_string())
///     ))
/// ));
/// assert_eq!(string_2d_array.descriptor(), "[[Ljava/lang/String;");
/// assert_eq!(string_2d_array.to_string(), "java/lang/String[][]");
/// ```
///
/// Parsing a field type from a descriptor:
///
/// ```rust
/// use ristretto_classfile::{BaseType, FieldType};
///
/// // Parse a field descriptor
/// let field_type = FieldType::parse(&"Ljava/lang/Object;".to_string())?;
/// assert_eq!(field_type, FieldType::Object("java/lang/Object".to_string()));
///
/// // Parse an array descriptor
/// let array_type = FieldType::parse(&"[[Z".to_string())?;
/// let expected = FieldType::Array(Box::new(
///     FieldType::Array(Box::new(
///         FieldType::Base(BaseType::Boolean)
///     ))
/// ));
/// assert_eq!(array_type, expected);
/// # Ok::<(), ristretto_classfile::Error>(())
/// ```
///
/// Parsing a method descriptor:
///
/// ```rust
/// use ristretto_classfile::{BaseType, FieldType};
///
/// // Parse a method descriptor: String toString()
/// let (params, ret) = FieldType::parse_method_descriptor("()Ljava/lang/String;")?;
/// assert!(params.is_empty());
/// assert_eq!(ret, Some(FieldType::Object("java/lang/String".to_string())));
///
/// // Parse a more complex method: static int compare(Object o1, Object o2)
/// let (params, ret) = FieldType::parse_method_descriptor(
///     "(Ljava/lang/Object;Ljava/lang/Object;)I"
/// )?;
/// assert_eq!(params.len(), 2);
/// assert_eq!(params[0], FieldType::Object("java/lang/Object".to_string()));
/// assert_eq!(params[1], FieldType::Object("java/lang/Object".to_string()));
/// assert_eq!(ret, Some(FieldType::Base(BaseType::Int)));
/// # Ok::<(), ristretto_classfile::Error>(())
/// ```
///
/// # References
///
/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.3.2>
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FieldType {
    Base(BaseType),
    Object(String),
    Array(Box<FieldType>),
}

impl FieldType {
    /// Return the code for the `FieldType`.
    ///
    /// Returns a character that represents the type in JVM field descriptors:
    /// - For base types, returns the type code (e.g., 'I' for int)
    /// - For object types, returns 'L'
    /// - For array types, returns '['
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::{BaseType, FieldType};
    ///
    /// let int_type = FieldType::Base(BaseType::Int);
    /// assert_eq!(int_type.code(), 'I');
    ///
    /// let object_type = FieldType::Object("java/lang/String".to_string());
    /// assert_eq!(object_type.code(), 'L');
    ///
    /// let array_type = FieldType::Array(Box::new(FieldType::Base(BaseType::Int)));
    /// assert_eq!(array_type.code(), '[');
    /// ```
    #[must_use]
    pub fn code(&self) -> char {
        match self {
            FieldType::Base(base_type) => base_type.code(),
            FieldType::Object(..) => 'L',
            FieldType::Array(..) => '[',
        }
    }

    /// Return the class name for the `FieldType`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::{BaseType, FieldType};
    ///
    /// let int_type = FieldType::Base(BaseType::Int);
    /// assert_eq!(int_type.class_name(), "int");
    ///
    /// let object_type = FieldType::Object("java/lang/String".to_string());
    /// assert_eq!(object_type.class_name(), "java/lang/String");
    ///
    /// let array_type = FieldType::Array(Box::new(FieldType::Base(BaseType::Int)));
    /// assert_eq!(array_type.class_name(), "[I");
    ///
    /// let object_array_type = FieldType::Array(Box::new(FieldType::Object("java/lang/String".to_string())));
    /// assert_eq!(object_array_type.class_name(), "[Ljava/lang/String;");
    /// ```
    #[must_use]
    pub fn class_name(&self) -> String {
        match self {
            FieldType::Base(base_type) => base_type.class_name().to_string(),
            FieldType::Object(class_name) => class_name.clone(),
            FieldType::Array(component_type) => match &**component_type {
                FieldType::Base(base_type) => format!("[{}", base_type.code()),
                FieldType::Object(class_name) => format!("[L{class_name};"),
                FieldType::Array(_) => format!("[{}", component_type.class_name()),
            },
        }
    }

    /// Return the descriptor for the `FieldType`.
    ///
    /// The descriptor is a string representation used in the JVM to identify types:
    /// - Base types use a single character (e.g., 'I' for int)
    /// - Object types use the format "L`<className>`;"
    /// - Array types use "[" followed by the component type descriptor
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::{BaseType, FieldType};
    ///
    /// let int_type = FieldType::Base(BaseType::Int);
    /// assert_eq!(int_type.descriptor(), "I");
    ///
    /// let object_type = FieldType::Object("java/lang/String".to_string());
    /// assert_eq!(object_type.descriptor(), "Ljava/lang/String;");
    ///
    /// let array_type = FieldType::Array(Box::new(FieldType::Base(BaseType::Int)));
    /// assert_eq!(array_type.descriptor(), "[I");
    ///
    /// let multi_array_type = FieldType::Array(Box::new(
    ///     FieldType::Array(Box::new(FieldType::Base(BaseType::Int)))
    /// ));
    /// assert_eq!(multi_array_type.descriptor(), "[[I");
    /// ```
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

    /// Parse a field descriptor string and return the corresponding `FieldType`.
    ///
    /// # Errors
    ///
    /// - Returns an error if the code is invalid.
    /// - Returns an error if the descriptor is invalid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::{BaseType, FieldType};
    ///
    /// // Parse a base type
    /// let int_type = FieldType::parse(&"I".to_string())?;
    /// assert_eq!(int_type, FieldType::Base(BaseType::Int));
    ///
    /// // Parse an object type
    /// let object_type = FieldType::parse(&"Ljava/lang/String;".to_string())?;
    /// assert_eq!(object_type, FieldType::Object("java/lang/String".to_string()));
    ///
    /// // Parse an array type
    /// let array_type = FieldType::parse(&"[I".to_string())?;
    /// assert_eq!(array_type, FieldType::Array(Box::new(FieldType::Base(BaseType::Int))));
    ///
    /// // Parse a multi-dimensional array
    /// let multi_array = FieldType::parse(&"[[Ljava/lang/Object;".to_string())?;
    /// assert_eq!(
    ///     multi_array,
    ///     FieldType::Array(Box::new(
    ///         FieldType::Array(Box::new(FieldType::Object("java/lang/Object".to_string())))
    ///     ))
    /// );
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    pub fn parse(descriptor: &str) -> Result<FieldType> {
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
                let characters: String = chars.collect();
                let component_type = Self::parse(&characters)?;
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

    /// Parse the method descriptor. The descriptor is a string representing the method signature.
    /// The descriptor has the following format:
    ///
    /// ```text
    /// MethodDescriptor:
    ///     ( ParameterDescriptor* ) ReturnDescriptor
    ///
    /// ParameterDescriptor:
    ///     FieldType
    ///
    /// ReturnDescriptor:
    ///     FieldType
    ///     V  // represents void
    /// ```
    ///
    /// The method returns a tuple containing:
    /// - A vector of `FieldType` representing the parameter types
    /// - An optional `FieldType` representing the return type (None for void)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::{BaseType, FieldType};
    ///
    /// // Parse a method with no parameters that returns void: void methodName()
    /// let (params, ret) = FieldType::parse_method_descriptor("()V")?;
    /// assert!(params.is_empty());
    /// assert_eq!(ret, None);
    ///
    /// // Parse a method with an int parameter that returns boolean: boolean methodName(int)
    /// let (params, ret) = FieldType::parse_method_descriptor("(I)Z")?;
    /// assert_eq!(params.len(), 1);
    /// assert_eq!(params[0], FieldType::Base(BaseType::Int));
    /// assert_eq!(ret, Some(FieldType::Base(BaseType::Boolean)));
    ///
    /// // Parse a method with String and int parameters that returns String: String methodName(String, int)
    /// let (params, ret) = FieldType::parse_method_descriptor("(Ljava/lang/String;I)Ljava/lang/String;")?;
    /// assert_eq!(params.len(), 2);
    /// assert_eq!(params[0], FieldType::Object("java/lang/String".to_string()));
    /// assert_eq!(params[1], FieldType::Base(BaseType::Int));
    /// assert_eq!(ret, Some(FieldType::Object("java/lang/String".to_string())));
    ///
    /// // Parse a method with array parameter and return type: int[] methodName(boolean[])
    /// let (params, ret) = FieldType::parse_method_descriptor("([Z)[I")?;
    /// assert_eq!(params.len(), 1);
    /// assert_eq!(params[0], FieldType::Array(Box::new(FieldType::Base(BaseType::Boolean))));
    /// assert_eq!(ret, Some(FieldType::Array(Box::new(FieldType::Base(BaseType::Int)))));
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    ///
    /// # Errors
    /// - Returns `InvalidMethodDescriptor` if the descriptor format is invalid
    /// - Returns other errors if field types cannot be parsed
    pub fn parse_method_descriptor(
        descriptor: &str,
    ) -> Result<(Vec<FieldType>, Option<FieldType>)> {
        let mut chars = descriptor.chars().peekable();
        let mut parameters = Vec::new();
        let mut return_type = None;

        if chars.next() != Some('(') {
            return Err(InvalidMethodDescriptor(descriptor.to_string()));
        }

        while let Some(&ch) = chars.peek() {
            if ch == ')' {
                chars.next();
                break;
            }
            parameters.push(Self::parse_field_type(descriptor, &mut chars)?);
        }

        match chars.next() {
            Some('V') => {}
            Some(ch) => {
                return_type = Some(Self::parse_field_type(
                    descriptor,
                    &mut std::iter::once(ch).chain(chars),
                )?);
            }
            None => return Err(InvalidMethodDescriptor(descriptor.to_string())),
        }

        Ok((parameters, return_type))
    }

    /// Parse a field type from a character iterator.
    ///
    /// This is a helper method used internally by `parse_method_descriptor` to parse individual
    /// field types from a method descriptor string.
    ///
    /// # Errors
    ///
    /// if the field type cannot be parsed
    fn parse_field_type<I>(descriptor: &str, chars: &mut I) -> Result<FieldType>
    where
        I: Iterator<Item = char>,
    {
        match chars.next() {
            Some('L') => {
                let mut class_name = String::new();
                for ch in chars.by_ref() {
                    if ch == ';' {
                        break;
                    }
                    class_name.push(ch);
                }
                Ok(FieldType::Object(class_name))
            }
            Some('[') => {
                let component_type = Self::parse_field_type(descriptor, chars)?;
                Ok(FieldType::Array(Box::new(component_type)))
            }
            Some(value) => {
                let base_type = BaseType::parse(value)?;
                Ok(FieldType::Base(base_type))
            }
            None => Err(InvalidMethodDescriptor(descriptor.to_string())),
        }
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
        assert_eq!(Err(InvalidFieldTypeCode('0')), FieldType::parse("0"));
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

    #[test]
    fn test_class_name_base() {
        let field_type = FieldType::Base(BaseType::Int);
        assert_eq!("int", field_type.class_name());
        let field_type_array = FieldType::Array(Box::new(field_type.clone()));
        assert_eq!("[I", field_type_array.class_name());
        let field_type_multi_array = FieldType::Array(Box::new(field_type_array));
        assert_eq!("[[I", field_type_multi_array.class_name());
    }

    #[test]
    fn test_class_name_object() {
        let field_type = FieldType::Object("java/lang/Object".to_string());
        assert_eq!("java/lang/Object", field_type.class_name());
        let field_type_array = FieldType::Array(Box::new(field_type.clone()));
        assert_eq!("[Ljava/lang/Object;", field_type_array.class_name());
        let field_type_multi_array = FieldType::Array(Box::new(field_type_array));
        assert_eq!("[[Ljava/lang/Object;", field_type_multi_array.class_name());
    }

    #[test]
    fn test_parse_method_descriptor() -> Result<()> {
        let (parameters, return_type) = FieldType::parse_method_descriptor("()V")?;
        assert!(parameters.is_empty());
        assert_eq!(return_type, None);

        let (parameters, return_type) = FieldType::parse_method_descriptor("()I")?;
        assert!(parameters.is_empty());
        assert_eq!(return_type, Some(FieldType::Base(BaseType::Int)));

        let (parameters, return_type) = FieldType::parse_method_descriptor("(I)V")?;
        assert_eq!(parameters, vec![FieldType::Base(BaseType::Int)]);
        assert_eq!(return_type, None);

        let (parameters, return_type) =
            FieldType::parse_method_descriptor("(Ljava.lang.String;)V")?;
        assert_eq!(
            parameters,
            vec![FieldType::Object("java.lang.String".to_string())]
        );
        assert_eq!(return_type, None);

        let (parameters, return_type) =
            FieldType::parse_method_descriptor("(Ljava.lang.String;I)V")?;
        assert_eq!(
            parameters,
            vec![
                FieldType::Object("java.lang.String".to_string()),
                FieldType::Base(BaseType::Int)
            ]
        );
        assert_eq!(return_type, None);

        let (parameters, return_type) =
            FieldType::parse_method_descriptor("(Ljava.lang.String;I)I")?;
        assert_eq!(
            parameters,
            vec![
                FieldType::Object("java.lang.String".to_string()),
                FieldType::Base(BaseType::Int)
            ]
        );
        assert_eq!(return_type, Some(FieldType::Base(BaseType::Int)));

        Ok(())
    }

    #[test]
    fn test_parse_method_descriptor_invalid() {
        let descriptor = String::new();
        assert!(matches!(
            FieldType::parse_method_descriptor(&descriptor),
            Err(InvalidMethodDescriptor(_))
        ));

        let descriptor = "()";
        assert!(matches!(
            FieldType::parse_method_descriptor(descriptor),
            Err(InvalidMethodDescriptor(_))
        ));
    }

    #[test]
    fn test_parse_field_type() -> Result<()> {
        assert_eq!(
            FieldType::parse_field_type("", &mut "I".chars())?,
            FieldType::Base(BaseType::Int)
        );
        assert_eq!(
            FieldType::parse_field_type("", &mut "J".chars())?,
            FieldType::Base(BaseType::Long)
        );
        assert_eq!(
            FieldType::parse_field_type("", &mut "S".chars())?,
            FieldType::Base(BaseType::Short)
        );
        assert_eq!(
            FieldType::parse_field_type("", &mut "Z".chars())?,
            FieldType::Base(BaseType::Boolean)
        );
        assert_eq!(
            FieldType::parse_field_type("", &mut "Ljava.lang.String;".chars())?,
            FieldType::Object("java.lang.String".to_string())
        );
        assert_eq!(
            FieldType::parse_field_type("", &mut "[Ljava.lang.String;".chars())?,
            FieldType::Array(Box::new(FieldType::Object("java.lang.String".to_string())))
        );
        Ok(())
    }

    #[test]
    fn test_parse_field_type_invalid() {
        let descriptor = String::new();
        assert!(matches!(
            FieldType::parse_field_type(&descriptor, &mut descriptor.chars()),
            Err(InvalidMethodDescriptor(_))
        ));
    }
}
