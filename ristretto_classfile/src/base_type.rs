use crate::Error::InvalidBaseTypeCode;
use crate::error::Result;
use std::fmt;

/// Represents the primitive types in the Java Virtual Machine.
///
/// `BaseType` corresponds to the basic primitive types in Java, each with a specific type
/// descriptor character used in field and method descriptors in the class file format.
///
/// # Examples
///
/// ```rust
/// use ristretto_classfile::{BaseType, Result};
///
/// // Parse a base type from its descriptor code
/// let int_type = BaseType::parse('I')?;
/// assert_eq!(int_type, BaseType::Int);
///
/// // Get the class name for display purposes
/// assert_eq!(BaseType::Boolean.class_name(), "boolean");
/// # Ok::<(), ristretto_classfile::Error>(())
/// ```
///
/// # References
///
/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.3.2>
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BaseType {
    /// Represents the boolean primitive type (Java `boolean`)
    Boolean,
    /// Represents the byte primitive type (Java `byte`)
    Byte,
    /// Represents the character primitive type (Java `char`)
    Char,
    /// Represents the double-precision floating-point primitive type (Java `double`)
    Double,
    /// Represents the single-precision floating-point primitive type (Java `float`)
    Float,
    /// Represents the integer primitive type (Java `int`)
    Int,
    /// Represents the long integer primitive type (Java `long`)
    Long,
    /// Represents the short integer primitive type (Java `short`)
    Short,
}

impl BaseType {
    /// Returns the JVM descriptor character for the `BaseType`.
    ///
    /// Each primitive type in the JVM has a single character that represents it in field and method
    /// descriptors.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::BaseType;
    ///
    /// assert_eq!(BaseType::Int.code(), 'I');
    /// assert_eq!(BaseType::Boolean.code(), 'Z');
    /// ```
    #[must_use]
    pub fn code(&self) -> char {
        match self {
            BaseType::Boolean => 'Z',
            BaseType::Byte => 'B',
            BaseType::Char => 'C',
            BaseType::Double => 'D',
            BaseType::Float => 'F',
            BaseType::Int => 'I',
            BaseType::Long => 'J',
            BaseType::Short => 'S',
        }
    }

    /// Returns the Java language class name for the `BaseType`.
    ///
    /// This method returns the standard Java language keyword that corresponds to this primitive
    /// type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::BaseType;
    ///
    /// assert_eq!(BaseType::Int.class_name(), "int");
    /// assert_eq!(BaseType::Boolean.class_name(), "boolean");
    /// ```
    #[must_use]
    pub fn class_name(&self) -> &'static str {
        match self {
            BaseType::Boolean => "boolean",
            BaseType::Byte => "byte",
            BaseType::Char => "char",
            BaseType::Double => "double",
            BaseType::Float => "float",
            BaseType::Int => "int",
            BaseType::Long => "long",
            BaseType::Short => "short",
        }
    }

    /// Parses a JVM descriptor character and returns the corresponding `BaseType`.
    ///
    /// This function takes a single character from a field or method descriptor and converts it to
    /// the appropriate `BaseType` enum variant.
    ///
    /// # Errors
    ///
    /// Returns an error if the code is invalid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::{BaseType, Result};
    ///
    /// // Parse valid base types
    /// let int_type = BaseType::parse('I')?;
    /// assert_eq!(int_type, BaseType::Int);
    ///
    /// let boolean_type = BaseType::parse('Z')?;
    /// assert_eq!(boolean_type, BaseType::Boolean);
    ///
    /// // Invalid codes will return an error
    /// assert!(BaseType::parse('X').is_err());
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    pub fn parse(code: char) -> Result<BaseType> {
        let base_type = match code {
            'Z' => BaseType::Boolean,
            'B' => BaseType::Byte,
            'C' => BaseType::Char,
            'D' => BaseType::Double,
            'F' => BaseType::Float,
            'I' => BaseType::Int,
            'J' => BaseType::Long,
            'S' => BaseType::Short,
            _ => return Err(InvalidBaseTypeCode(code)),
        };

        Ok(base_type)
    }
}

/// Implements the `Display` trait for `BaseType`.
///
/// This implementation allows for convenient string representation of base types by formatting them
/// as their Java language names (e.g., "int", "boolean").
///
/// # Examples
///
/// ```rust
/// use ristretto_classfile::BaseType;
/// use std::fmt::Display;
///
/// // Direct string conversion
/// let type_str = BaseType::Int.to_string();
/// assert_eq!(type_str, "int");
///
/// // Use in string formatting
/// let formatted = format!("The type is: {}", BaseType::Boolean);
/// assert_eq!(formatted, "The type is: boolean");
/// ```
impl fmt::Display for BaseType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.class_name())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_invalid_code() {
        assert_eq!(Err(InvalidBaseTypeCode('0')), BaseType::parse('0'));
    }

    #[test]
    fn test_boolean() -> Result<()> {
        assert_eq!(BaseType::Boolean.code(), 'Z');
        assert_eq!(BaseType::Boolean, BaseType::parse('Z')?);
        assert_eq!("boolean", BaseType::Boolean.class_name());
        assert_eq!("boolean", BaseType::Boolean.to_string());
        Ok(())
    }

    #[test]
    fn test_byte() -> Result<()> {
        assert_eq!(BaseType::Byte.code(), 'B');
        assert_eq!(BaseType::Byte, BaseType::parse('B')?);
        assert_eq!("byte", BaseType::Byte.class_name());
        assert_eq!("byte", BaseType::Byte.to_string());
        Ok(())
    }

    #[test]
    fn test_char() -> Result<()> {
        assert_eq!(BaseType::Char.code(), 'C');
        assert_eq!(BaseType::Char, BaseType::parse('C')?);
        assert_eq!("char", BaseType::Char.class_name());
        assert_eq!("char", BaseType::Char.to_string());
        Ok(())
    }

    #[test]
    fn test_double() -> Result<()> {
        assert_eq!(BaseType::Double.code(), 'D');
        assert_eq!(BaseType::Double, BaseType::parse('D')?);
        assert_eq!("double", BaseType::Double.class_name());
        assert_eq!("double", BaseType::Double.to_string());
        Ok(())
    }

    #[test]
    fn test_float() -> Result<()> {
        assert_eq!(BaseType::Float.code(), 'F');
        assert_eq!(BaseType::Float, BaseType::parse('F')?);
        assert_eq!("float", BaseType::Float.class_name());
        assert_eq!("float", BaseType::Float.to_string());
        Ok(())
    }

    #[test]
    fn test_int() -> Result<()> {
        assert_eq!(BaseType::Int.code(), 'I');
        assert_eq!(BaseType::Int, BaseType::parse('I')?);
        assert_eq!("int", BaseType::Int.class_name());
        assert_eq!("int", BaseType::Int.to_string());
        Ok(())
    }

    #[test]
    fn test_long() -> Result<()> {
        assert_eq!(BaseType::Long.code(), 'J');
        assert_eq!(BaseType::Long, BaseType::parse('J')?);
        assert_eq!("long", BaseType::Long.class_name());
        assert_eq!("long", BaseType::Long.to_string());
        Ok(())
    }

    #[test]
    fn test_short() -> Result<()> {
        assert_eq!(BaseType::Short.code(), 'S');
        assert_eq!(BaseType::Short, BaseType::parse('S')?);
        assert_eq!("short", BaseType::Short.class_name());
        assert_eq!("short", BaseType::Short.to_string());
        Ok(())
    }
}
