use crate::error::Result;
use crate::Error::InvalidBaseTypeCode;
use std::fmt;

/// Implementation of `BaseType`.
///
/// See: <https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.3.2>
#[derive(Clone, Debug, PartialEq)]
pub enum BaseType {
    Byte,
    Char,
    Double,
    Float,
    Int,
    Long,
    Short,
    Boolean,
}

impl BaseType {
    /// Return the code for the `BaseType`.
    #[must_use]
    pub fn code(&self) -> char {
        match self {
            BaseType::Byte => 'B',
            BaseType::Char => 'C',
            BaseType::Double => 'D',
            BaseType::Float => 'F',
            BaseType::Int => 'I',
            BaseType::Long => 'J',
            BaseType::Short => 'S',
            BaseType::Boolean => 'Z',
        }
    }

    /// Return the `BaseType` for a given code.
    ///
    /// # Errors
    /// Returns an error if the code is invalid.
    pub fn parse(code: char) -> Result<BaseType> {
        let base_type = match code {
            'B' => BaseType::Byte,
            'C' => BaseType::Char,
            'D' => BaseType::Double,
            'F' => BaseType::Float,
            'I' => BaseType::Int,
            'J' => BaseType::Long,
            'S' => BaseType::Short,
            'Z' => BaseType::Boolean,
            _ => return Err(InvalidBaseTypeCode(code)),
        };

        Ok(base_type)
    }
}

impl fmt::Display for BaseType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BaseType::Byte => write!(f, "byte"),
            BaseType::Char => write!(f, "char"),
            BaseType::Double => write!(f, "double"),
            BaseType::Float => write!(f, "float"),
            BaseType::Int => write!(f, "int"),
            BaseType::Long => write!(f, "long"),
            BaseType::Short => write!(f, "short"),
            BaseType::Boolean => write!(f, "boolean"),
        }
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
    fn test_byte() -> Result<()> {
        assert_eq!(BaseType::Byte.code(), 'B');
        assert_eq!(BaseType::Byte, BaseType::parse('B')?);
        assert_eq!("byte", BaseType::Byte.to_string());
        Ok(())
    }

    #[test]
    fn test_char() -> Result<()> {
        assert_eq!(BaseType::Char.code(), 'C');
        assert_eq!(BaseType::Char, BaseType::parse('C')?);
        assert_eq!("char", BaseType::Char.to_string());
        Ok(())
    }

    #[test]
    fn test_double() -> Result<()> {
        assert_eq!(BaseType::Double.code(), 'D');
        assert_eq!(BaseType::Double, BaseType::parse('D')?);
        assert_eq!("double", BaseType::Double.to_string());
        Ok(())
    }

    #[test]
    fn test_float() -> Result<()> {
        assert_eq!(BaseType::Float.code(), 'F');
        assert_eq!(BaseType::Float, BaseType::parse('F')?);
        assert_eq!("float", BaseType::Float.to_string());
        Ok(())
    }

    #[test]
    fn test_int() -> Result<()> {
        assert_eq!(BaseType::Int.code(), 'I');
        assert_eq!(BaseType::Int, BaseType::parse('I')?);
        assert_eq!("int", BaseType::Int.to_string());
        Ok(())
    }

    #[test]
    fn test_long() -> Result<()> {
        assert_eq!(BaseType::Long.code(), 'J');
        assert_eq!(BaseType::Long, BaseType::parse('J')?);
        assert_eq!("long", BaseType::Long.to_string());
        Ok(())
    }

    #[test]
    fn test_short() -> Result<()> {
        assert_eq!(BaseType::Short.code(), 'S');
        assert_eq!(BaseType::Short, BaseType::parse('S')?);
        assert_eq!("short", BaseType::Short.to_string());
        Ok(())
    }

    #[test]
    fn test_boolean() -> Result<()> {
        assert_eq!(BaseType::Boolean.code(), 'Z');
        assert_eq!(BaseType::Boolean, BaseType::parse('Z')?);
        assert_eq!("boolean", BaseType::Boolean.to_string());
        Ok(())
    }
}
