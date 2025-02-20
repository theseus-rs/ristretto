use crate::Error::InvalidBaseTypeCode;
use crate::error::Result;
use std::fmt;

/// Implementation of `BaseType`.
///
/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.3.2>
#[derive(Clone, Debug, PartialEq)]
pub enum BaseType {
    Boolean,
    Byte,
    Char,
    Double,
    Float,
    Int,
    Long,
    Short,
}

impl BaseType {
    /// Return the code for the `BaseType`.
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

    /// Return the class name for the `BaseType`.
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

    /// Return the `BaseType` for a given code.
    ///
    /// # Errors
    /// Returns an error if the code is invalid.
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
