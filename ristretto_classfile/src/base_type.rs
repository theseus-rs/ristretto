use crate::error::Result;
use crate::Error::InvalidBaseTypeCode;

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_invalid_code() {
        assert_eq!(Err(InvalidBaseTypeCode('0')), BaseType::parse('0'));
    }

    #[test]
    fn test_byte() {
        assert_eq!(BaseType::Byte.code(), 'B');
    }

    #[test]
    fn test_char() {
        assert_eq!(BaseType::Char.code(), 'C');
    }

    #[test]
    fn test_double() {
        assert_eq!(BaseType::Double.code(), 'D');
    }

    #[test]
    fn test_float() {
        assert_eq!(BaseType::Float.code(), 'F');
    }

    #[test]
    fn test_int() {
        assert_eq!(BaseType::Int.code(), 'I');
    }

    #[test]
    fn test_long() {
        assert_eq!(BaseType::Long.code(), 'J');
    }

    #[test]
    fn test_short() {
        assert_eq!(BaseType::Short.code(), 'S');
    }

    #[test]
    fn test_boolean() {
        assert_eq!(BaseType::Boolean.code(), 'Z');
    }
}
