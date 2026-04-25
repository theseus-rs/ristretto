use crate::Error::{InvalidFieldTypeCode, InvalidFieldTypeDescriptor, InvalidMethodDescriptor};
use crate::base_type::BaseType;
use crate::error::Result;
use crate::java_string::{JavaStr, JavaString};
use std::fmt;

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
/// use ristretto_classfile::{BaseType, FieldType, JavaString};
///
/// // Create a primitive type (int)
/// let int_type = FieldType::Base(BaseType::Int);
/// assert_eq!(int_type.descriptor(), "I");
/// assert_eq!(int_type.to_string(), "int");
///
/// // Create an object type (String)
/// let string_type = FieldType::Object(JavaString::from("java/lang/String"));
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
///         FieldType::Object(JavaString::from("java/lang/String"))
///     ))
/// ));
/// assert_eq!(string_2d_array.descriptor(), "[[Ljava/lang/String;");
/// assert_eq!(string_2d_array.to_string(), "java/lang/String[][]");
/// ```
///
/// Parsing a field type from a descriptor:
///
/// ```rust
/// use ristretto_classfile::{BaseType, FieldType, JavaString};
///
/// // Parse a field descriptor
/// let field_type = FieldType::parse(&"Ljava/lang/Object;".to_string())?;
/// assert_eq!(field_type, FieldType::Object(JavaString::from("java/lang/Object")));
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
/// use ristretto_classfile::{BaseType, FieldType, JavaStr, JavaString};
///
/// // Parse a method descriptor: String toString()
/// let descriptor = JavaStr::try_from_str("()Ljava/lang/String;")?;
/// let (params, ret) = FieldType::parse_method_descriptor(descriptor)?;
/// assert!(params.is_empty());
/// assert_eq!(ret, Some(FieldType::Object(JavaString::from("java/lang/String"))));
///
/// // Parse a more complex method: static int compare(Object o1, Object o2)
/// let descriptor = JavaStr::try_from_str("(Ljava/lang/Object;Ljava/lang/Object;)I")?;
/// let (params, ret) = FieldType::parse_method_descriptor(descriptor)?;
/// assert_eq!(params.len(), 2);
/// assert_eq!(params[0], FieldType::Object(JavaString::from("java/lang/Object")));
/// assert_eq!(params[1], FieldType::Object(JavaString::from("java/lang/Object")));
/// assert_eq!(ret, Some(FieldType::Base(BaseType::Int)));
/// # Ok::<(), ristretto_classfile::Error>(())
/// ```
///
/// # References
///
/// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.3.2>
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FieldType {
    Base(BaseType),
    Object(JavaString),
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
    /// use ristretto_classfile::{BaseType, FieldType, JavaString};
    ///
    /// let int_type = FieldType::Base(BaseType::Int);
    /// assert_eq!(int_type.code(), 'I');
    ///
    /// let object_type = FieldType::Object(JavaString::from("java/lang/String"));
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
    /// use ristretto_classfile::{BaseType, FieldType, JavaString};
    ///
    /// let int_type = FieldType::Base(BaseType::Int);
    /// assert_eq!(int_type.class_name(), "int");
    ///
    /// let object_type = FieldType::Object(JavaString::from("java/lang/String"));
    /// assert_eq!(object_type.class_name(), "java/lang/String");
    ///
    /// let array_type = FieldType::Array(Box::new(FieldType::Base(BaseType::Int)));
    /// assert_eq!(array_type.class_name(), "[I");
    ///
    /// let object_array_type = FieldType::Array(Box::new(FieldType::Object(JavaString::from("java/lang/String"))));
    /// assert_eq!(object_array_type.class_name(), "[Ljava/lang/String;");
    /// ```
    #[must_use]
    pub fn class_name(&self) -> String {
        match self {
            FieldType::Base(base_type) => base_type.class_name().to_string(),
            FieldType::Object(class_name) => class_name.to_rust_string(),
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
    /// use ristretto_classfile::{BaseType, FieldType, JavaString};
    ///
    /// let int_type = FieldType::Base(BaseType::Int);
    /// assert_eq!(int_type.descriptor(), "I");
    ///
    /// let object_type = FieldType::Object(JavaString::from("java/lang/String"));
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

    /// Returns the number of operand-stack / local-variable slots a value of
    /// this type occupies, per [JVMS §2.6.1] and [JVMS §2.6.2].
    ///
    /// `long` and `double` are category 2 values and occupy two slots; every
    /// other field type (including object and array references) is a category
    /// 1 value and occupies a single slot.
    ///
    /// [JVMS §2.6.1]: https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-2.html#jvms-2.6.1
    /// [JVMS §2.6.2]: https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-2.html#jvms-2.6.2
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::{BaseType, FieldType, JavaString};
    ///
    /// assert_eq!(FieldType::Base(BaseType::Int).slot_count(), 1);
    /// assert_eq!(FieldType::Base(BaseType::Long).slot_count(), 2);
    /// assert_eq!(FieldType::Base(BaseType::Double).slot_count(), 2);
    /// assert_eq!(
    ///     FieldType::Object(JavaString::from("java/lang/Object")).slot_count(),
    ///     1
    /// );
    /// assert_eq!(
    ///     FieldType::Array(Box::new(FieldType::Base(BaseType::Long))).slot_count(),
    ///     1
    /// );
    /// ```
    #[must_use]
    pub const fn slot_count(&self) -> u8 {
        match self {
            FieldType::Base(BaseType::Long | BaseType::Double) => 2,
            _ => 1,
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
    /// use ristretto_classfile::{BaseType, FieldType, JavaString};
    ///
    /// // Parse a base type
    /// let int_type = FieldType::parse(&"I".to_string())?;
    /// assert_eq!(int_type, FieldType::Base(BaseType::Int));
    ///
    /// // Parse an object type
    /// let object_type = FieldType::parse(&"Ljava/lang/String;".to_string())?;
    /// assert_eq!(object_type, FieldType::Object(JavaString::from("java/lang/String")));
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
    ///         FieldType::Array(Box::new(FieldType::Object(JavaString::from("java/lang/Object"))))
    ///     ))
    /// );
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    pub fn parse(descriptor: &str) -> Result<FieldType> {
        Self::parse_bytes(descriptor.as_bytes(), descriptor)
    }

    /// Parse a field type descriptor from a `&JavaStr` (MUTF-8 bytes).
    ///
    /// This avoids the overhead of converting MUTF-8 to UTF-8 before parsing,
    /// since field type descriptors only use ASCII characters for type codes
    /// and delimiters (`L`, `[`, `;`, `B`, `C`, etc.).
    ///
    /// # Errors
    /// Returns an error if the descriptor is invalid.
    pub fn parse_java_str(descriptor: &JavaStr) -> Result<FieldType> {
        Self::parse_bytes(descriptor.as_bytes(), descriptor)
    }

    /// Internal byte-level field type parser shared by `parse` and `parse_java_str`.
    fn parse_bytes(bytes: &[u8], display_desc: impl fmt::Display) -> Result<FieldType> {
        let code = bytes.first().copied().unwrap_or_default();
        match code {
            b'L' => {
                let len = bytes.len();
                if len >= 3 && bytes[len - 1] == b';' {
                    let class_bytes = &bytes[1..len - 1];
                    // Store as JavaString (MUTF-8) to avoid lossy UTF-8 conversion
                    let class_name = match JavaStr::from_mutf8(class_bytes) {
                        Ok(java_str) => java_str.to_java_string(),
                        Err(_) => JavaString::from(String::from_utf8_lossy(class_bytes).as_ref()),
                    };
                    Ok(FieldType::Object(class_name))
                } else {
                    Err(InvalidFieldTypeDescriptor(display_desc.to_string()))
                }
            }
            b'[' => {
                let component_type = Self::parse_bytes(&bytes[1..], display_desc)?;
                Ok(FieldType::Array(component_type.into()))
            }
            _ => {
                let Ok(base_type) = BaseType::parse(code as char) else {
                    return Err(InvalidFieldTypeCode(code as char));
                };
                Ok(FieldType::Base(base_type))
            }
        }
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
    /// use ristretto_classfile::{BaseType, FieldType, JavaStr, JavaString};
    ///
    /// // Parse a method with no parameters that returns void: void methodName()
    /// let descriptor = JavaStr::try_from_str("()V")?;
    /// let (params, ret) = FieldType::parse_method_descriptor(descriptor)?;
    /// assert!(params.is_empty());
    /// assert_eq!(ret, None);
    ///
    /// // Parse a method with an int parameter that returns boolean: boolean methodName(int)
    /// let descriptor = JavaStr::try_from_str("(I)Z")?;
    /// let (params, ret) = FieldType::parse_method_descriptor(descriptor)?;
    /// assert_eq!(params.len(), 1);
    /// assert_eq!(params[0], FieldType::Base(BaseType::Int));
    /// assert_eq!(ret, Some(FieldType::Base(BaseType::Boolean)));
    ///
    /// // Parse a method with String and int parameters that returns String: String methodName(String, int)
    /// let descriptor = JavaStr::try_from_str("(Ljava/lang/String;I)Ljava/lang/String;")?;
    /// let (params, ret) = FieldType::parse_method_descriptor(descriptor)?;
    /// assert_eq!(params.len(), 2);
    /// assert_eq!(params[0], FieldType::Object(JavaString::from("java/lang/String")));
    /// assert_eq!(params[1], FieldType::Base(BaseType::Int));
    /// assert_eq!(ret, Some(FieldType::Object(JavaString::from("java/lang/String"))));
    ///
    /// // Parse a method with array parameter and return type: int[] methodName(boolean[])
    /// let descriptor = JavaStr::try_from_str("([Z)[I")?;
    /// let (params, ret) = FieldType::parse_method_descriptor(descriptor)?;
    /// assert_eq!(params.len(), 1);
    /// assert_eq!(params[0], FieldType::Array(Box::new(FieldType::Base(BaseType::Boolean))));
    /// assert_eq!(ret, Some(FieldType::Array(Box::new(FieldType::Base(BaseType::Int)))));
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// # Errors
    /// - Returns `InvalidMethodDescriptor` if the descriptor format is invalid
    /// - Returns other errors if field types cannot be parsed
    pub fn parse_method_descriptor(
        descriptor: &JavaStr,
    ) -> Result<(Vec<FieldType>, Option<FieldType>)> {
        Self::parse_method_descriptor_bytes(descriptor.as_bytes(), descriptor)
    }

    /// Internal byte-level method descriptor parser.
    fn parse_method_descriptor_bytes(
        bytes: &[u8],
        display_desc: impl fmt::Display + Copy,
    ) -> Result<(Vec<FieldType>, Option<FieldType>)> {
        let mut parameters = Vec::new();
        let mut return_type = None;
        let mut pos = 0;

        if bytes.first().copied() != Some(b'(') {
            return Err(InvalidMethodDescriptor(display_desc.to_string()));
        }
        pos += 1;

        while pos < bytes.len() {
            if bytes[pos] == b')' {
                pos += 1;
                break;
            }
            let (ft, consumed) = Self::parse_field_type_bytes(&bytes[pos..], display_desc)?;
            parameters.push(ft);
            pos += consumed;
        }

        match bytes.get(pos).copied() {
            Some(b'V') => {}
            Some(_) => {
                let (ft, _) = Self::parse_field_type_bytes(&bytes[pos..], display_desc)?;
                return_type = Some(ft);
            }
            None => return Err(InvalidMethodDescriptor(display_desc.to_string())),
        }

        Ok((parameters, return_type))
    }

    /// Parse a single field type from a byte slice, returning the type and bytes consumed.
    fn parse_field_type_bytes(
        bytes: &[u8],
        display_desc: impl fmt::Display,
    ) -> Result<(FieldType, usize)> {
        match bytes.first().copied() {
            Some(b'L') => {
                let mut end = 1;
                while end < bytes.len() {
                    if bytes[end] == b';' {
                        break;
                    }
                    end += 1;
                }
                if end == bytes.len() {
                    return Err(InvalidFieldTypeDescriptor(display_desc.to_string()));
                }
                let class_bytes = &bytes[1..end];
                // Store as JavaString (MUTF-8) to avoid lossy UTF-8 conversion
                let class_name = match JavaStr::from_mutf8(class_bytes) {
                    Ok(java_str) => java_str.to_java_string(),
                    Err(_) => JavaString::from(String::from_utf8_lossy(class_bytes).as_ref()),
                };
                Ok((FieldType::Object(class_name), end + 1)) // +1 for ';'
            }
            Some(b'[') => {
                let (component_type, consumed) =
                    Self::parse_field_type_bytes(&bytes[1..], display_desc)?;
                Ok((FieldType::Array(Box::new(component_type)), 1 + consumed))
            }
            Some(code) => {
                let base_type = BaseType::parse(code as char)?;
                Ok((FieldType::Base(base_type), 1))
            }
            None => Err(InvalidMethodDescriptor(display_desc.to_string())),
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
        let field_type = FieldType::Object(JavaString::from("Foo"));

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
            Err(InvalidFieldTypeDescriptor("L".to_string())),
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
        let field_type = FieldType::Object(JavaString::from("java/lang/Object"));
        assert_eq!("java/lang/Object", field_type.class_name());
        let field_type_array = FieldType::Array(Box::new(field_type.clone()));
        assert_eq!("[Ljava/lang/Object;", field_type_array.class_name());
        let field_type_multi_array = FieldType::Array(Box::new(field_type_array));
        assert_eq!("[[Ljava/lang/Object;", field_type_multi_array.class_name());
    }

    #[test]
    fn test_parse_method_descriptor() -> Result<()> {
        let (parameters, return_type) =
            FieldType::parse_method_descriptor(JavaStr::try_from_str("()V")?)?;
        assert!(parameters.is_empty());
        assert_eq!(return_type, None);

        let (parameters, return_type) =
            FieldType::parse_method_descriptor(JavaStr::try_from_str("()I")?)?;
        assert!(parameters.is_empty());
        assert_eq!(return_type, Some(FieldType::Base(BaseType::Int)));

        let (parameters, return_type) =
            FieldType::parse_method_descriptor(JavaStr::try_from_str("(I)V")?)?;
        assert_eq!(parameters, vec![FieldType::Base(BaseType::Int)]);
        assert_eq!(return_type, None);

        let (parameters, return_type) =
            FieldType::parse_method_descriptor(JavaStr::try_from_str("(Ljava.lang.String;)V")?)?;
        assert_eq!(
            parameters,
            vec![FieldType::Object(JavaString::from("java.lang.String"))]
        );
        assert_eq!(return_type, None);

        let (parameters, return_type) =
            FieldType::parse_method_descriptor(JavaStr::try_from_str("(Ljava.lang.String;I)V")?)?;
        assert_eq!(
            parameters,
            vec![
                FieldType::Object(JavaString::from("java.lang.String")),
                FieldType::Base(BaseType::Int)
            ]
        );
        assert_eq!(return_type, None);

        let (parameters, return_type) =
            FieldType::parse_method_descriptor(JavaStr::try_from_str("(Ljava.lang.String;I)I")?)?;
        assert_eq!(
            parameters,
            vec![
                FieldType::Object(JavaString::from("java.lang.String")),
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
            FieldType::parse_method_descriptor(JavaStr::try_from_str(&descriptor).unwrap()),
            Err(InvalidMethodDescriptor(_))
        ));

        assert!(matches!(
            FieldType::parse_method_descriptor(JavaStr::try_from_str("()").unwrap()),
            Err(InvalidMethodDescriptor(_))
        ));
    }

    #[test]
    fn test_parse_field_type_bytes() -> Result<()> {
        let (ft, consumed) = FieldType::parse_field_type_bytes(b"I", "")?;
        assert_eq!(ft, FieldType::Base(BaseType::Int));
        assert_eq!(consumed, 1);

        let (ft, consumed) = FieldType::parse_field_type_bytes(b"J", "")?;
        assert_eq!(ft, FieldType::Base(BaseType::Long));
        assert_eq!(consumed, 1);

        let (ft, consumed) = FieldType::parse_field_type_bytes(b"S", "")?;
        assert_eq!(ft, FieldType::Base(BaseType::Short));
        assert_eq!(consumed, 1);

        let (ft, consumed) = FieldType::parse_field_type_bytes(b"Z", "")?;
        assert_eq!(ft, FieldType::Base(BaseType::Boolean));
        assert_eq!(consumed, 1);

        let (ft, consumed) = FieldType::parse_field_type_bytes(b"Ljava.lang.String;", "")?;
        assert_eq!(ft, FieldType::Object(JavaString::from("java.lang.String")));
        assert_eq!(consumed, 18);

        let (ft, consumed) = FieldType::parse_field_type_bytes(b"[Ljava.lang.String;", "")?;
        assert_eq!(
            ft,
            FieldType::Array(Box::new(FieldType::Object(JavaString::from(
                "java.lang.String"
            ))))
        );
        assert_eq!(consumed, 19);

        Ok(())
    }

    #[test]
    fn test_parse_field_type_bytes_invalid() {
        assert!(matches!(
            FieldType::parse_field_type_bytes(b"", ""),
            Err(InvalidMethodDescriptor(_))
        ));
    }
}
