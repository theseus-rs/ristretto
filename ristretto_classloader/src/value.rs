use crate::reference::Reference;
use crate::Error::InvalidValueType;
use crate::Result;
use std::fmt;
use std::fmt::Display;

/// Represents a value in the Ristretto VM.
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    Object(Option<Reference>),
    Unused,
}

impl Value {
    /// Returns the value as an `i32`.
    ///
    /// # Errors
    /// if the value is not an Int.
    pub fn as_int(&self) -> Result<i32> {
        match self {
            Value::Int(value) => Ok(*value),
            _ => Err(InvalidValueType("Expected an int value".to_string())),
        }
    }

    /// Returns the value as an `i64`.
    ///
    /// # Errors
    /// if the value is not a Long
    pub fn as_long(&self) -> Result<i64> {
        match self {
            Value::Long(value) => Ok(*value),
            _ => Err(InvalidValueType("Expected a long value".to_string())),
        }
    }

    /// Returns the value as an `f32`.
    ///
    /// # Errors
    /// if the value is not a Float
    pub fn as_float(&self) -> Result<f32> {
        match self {
            Value::Float(value) => Ok(*value),
            _ => Err(InvalidValueType("Expected a float value".to_string())),
        }
    }

    /// Returns the value as an `f64`.
    ///
    /// # Errors
    /// if the value is not a Double
    pub fn as_double(&self) -> Result<f64> {
        match self {
            Value::Double(value) => Ok(*value),
            _ => Err(InvalidValueType("Expected a double value".to_string())),
        }
    }

    /// Returns the value as an `Option<Reference>`.
    ///
    /// # Errors
    /// if the value is not an Object
    pub fn as_object(&self) -> Result<Option<&Reference>> {
        match self {
            Value::Object(value) => Ok(value.as_ref()),
            _ => Err(InvalidValueType("Expected an object value".to_string())),
        }
    }

    /// Returns a string value for a java.lang.String object.
    ///
    /// # Errors
    /// if the value is not a string Object
    pub fn as_string(&self) -> Result<String> {
        match self {
            Value::Object(Some(Reference::Object(object))) => object.as_string(),
            _ => Err(InvalidValueType("Expected an object value".to_string())),
        }
    }

    /// Returns true if the value is a category 1 value.
    #[must_use]
    pub fn is_category_1(&self) -> bool {
        !self.is_category_2()
    }

    /// Returns true if the value is a category 2 value.
    #[must_use]
    pub fn is_category_2(&self) -> bool {
        matches!(self, Value::Long(_) | Value::Double(_))
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(value) => write!(f, "int({value})"),
            Value::Long(value) => write!(f, "long({value})"),
            Value::Float(value) => write!(f, "float({value})"),
            Value::Double(value) => write!(f, "double({value})"),
            Value::Object(value) => {
                if let Some(value) = value {
                    if let Ok(class) = value.class() {
                        if class.name() == "java/lang/String" {
                            let string = self.as_string().unwrap_or_default();
                            return write!(f, "string({string})");
                        }
                    }
                    write!(f, "object({value})")
                } else {
                    write!(f, "object(null)")
                }
            }
            Value::Unused => write!(f, "unused"),
        }
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Int(i32::from(value))
    }
}

impl From<i8> for Value {
    fn from(value: i8) -> Self {
        Value::Int(i32::from(value))
    }
}

impl From<u8> for Value {
    fn from(value: u8) -> Self {
        Value::Int(i32::from(value))
    }
}

impl From<char> for Value {
    fn from(value: char) -> Self {
        Value::Int(value as i32)
    }
}

impl From<i16> for Value {
    fn from(value: i16) -> Self {
        Value::Int(i32::from(value))
    }
}

impl From<u16> for Value {
    fn from(value: u16) -> Self {
        Value::Int(i32::from(value))
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Value::Int(value)
    }
}

impl From<u32> for Value {
    fn from(value: u32) -> Self {
        #[expect(clippy::cast_possible_wrap)]
        Value::Int(value as i32)
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::Long(value)
    }
}

impl From<u64> for Value {
    fn from(value: u64) -> Self {
        #[expect(clippy::cast_possible_wrap)]
        Value::Long(value as i64)
    }
}

impl From<isize> for Value {
    fn from(value: isize) -> Self {
        Value::Long(value as i64)
    }
}

impl From<usize> for Value {
    fn from(value: usize) -> Self {
        #[expect(clippy::cast_possible_wrap)]
        Value::Long(value as i64)
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Value::Float(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Double(value)
    }
}

impl From<Vec<bool>> for Value {
    fn from(value: Vec<bool>) -> Self {
        Value::Object(Some(Reference::from(value)))
    }
}

impl From<Vec<i8>> for Value {
    fn from(value: Vec<i8>) -> Self {
        Value::Object(Some(Reference::from(value)))
    }
}

impl From<Vec<u8>> for Value {
    fn from(value: Vec<u8>) -> Self {
        Value::Object(Some(Reference::from(value)))
    }
}

impl From<Vec<char>> for Value {
    fn from(value: Vec<char>) -> Self {
        Value::Object(Some(Reference::from(value)))
    }
}

impl From<Vec<i16>> for Value {
    fn from(value: Vec<i16>) -> Self {
        Value::Object(Some(Reference::from(value)))
    }
}

impl From<Vec<u16>> for Value {
    fn from(value: Vec<u16>) -> Self {
        Value::Object(Some(Reference::from(value)))
    }
}

impl From<Vec<i32>> for Value {
    fn from(value: Vec<i32>) -> Self {
        Value::Object(Some(Reference::from(value)))
    }
}

impl From<Vec<u32>> for Value {
    fn from(value: Vec<u32>) -> Self {
        Value::Object(Some(Reference::from(value)))
    }
}

impl From<Vec<i64>> for Value {
    fn from(value: Vec<i64>) -> Self {
        Value::Object(Some(Reference::from(value)))
    }
}

impl From<Vec<u64>> for Value {
    fn from(value: Vec<u64>) -> Self {
        Value::Object(Some(Reference::from(value)))
    }
}

impl From<Vec<isize>> for Value {
    fn from(value: Vec<isize>) -> Self {
        Value::Object(Some(Reference::from(value)))
    }
}

impl From<Vec<usize>> for Value {
    fn from(value: Vec<usize>) -> Self {
        Value::Object(Some(Reference::from(value)))
    }
}

impl From<Vec<f32>> for Value {
    fn from(value: Vec<f32>) -> Self {
        Value::Object(Some(Reference::from(value)))
    }
}

impl From<Vec<f64>> for Value {
    fn from(value: Vec<f64>) -> Self {
        Value::Object(Some(Reference::from(value)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{runtime, Object};

    #[test]
    fn test_int_format() -> Result<()> {
        let value = Value::Int(42);
        assert_eq!(42, value.as_int()?);
        assert_eq!("int(42)", value.to_string());
        assert!(value.is_category_1());
        assert!(!value.is_category_2());
        Ok(())
    }

    #[test]
    fn test_as_int_error() {
        let result = Value::Long(42).as_int();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[test]
    fn test_long_format() -> Result<()> {
        let value = Value::Long(42);
        assert_eq!(42, value.as_long()?);
        assert_eq!("long(42)", value.to_string());
        assert!(!value.is_category_1());
        assert!(value.is_category_2());
        Ok(())
    }

    #[test]
    fn test_as_long_error() {
        let result = Value::Int(42).as_long();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[test]
    fn test_float_format() -> Result<()> {
        let value = Value::Float(42.1);
        let compare_value = value.as_float()? - 42.1f32;
        assert!(compare_value.abs() < 0.1f32);
        assert_eq!("float(42.1)", value.to_string());
        assert!(value.is_category_1());
        assert!(!value.is_category_2());
        Ok(())
    }

    #[test]
    fn test_as_float_error() {
        let result = Value::Int(42).as_float();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[test]
    fn test_double_format() -> Result<()> {
        let value = Value::Double(42.1);
        let compare_value = value.as_double()? - 42.1f64;
        assert!(compare_value.abs() < 0.1f64);
        assert_eq!("double(42.1)", value.to_string());
        assert!(!value.is_category_1());
        assert!(value.is_category_2());
        Ok(())
    }

    #[test]
    fn test_as_double_error() {
        let result = Value::Int(42).as_double();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[test]
    fn test_object_format() -> Result<()> {
        let value = Value::Object(None);
        assert_eq!(None, value.as_object()?);
        assert_eq!("object(null)", value.to_string());
        assert!(value.is_category_1());
        assert!(!value.is_category_2());
        assert_eq!(
            "object(byte[1, 2, 3])",
            format!("{}", Value::from(vec![1i8, 2i8, 3i8]))
        );
        Ok(())
    }

    #[test]
    fn test_as_object_error() {
        let result = Value::Int(42).as_object();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[expect(clippy::cast_possible_wrap)]
    #[tokio::test]
    async fn test_string_format() -> Result<()> {
        let (_java_home, _java_version, class_loader) = runtime::default_class_loader().await?;
        let class = class_loader.load("java/lang/String").await?;
        let object = Object::new(class)?;
        let string_bytes: Vec<i8> = "foo".as_bytes().to_vec().iter().map(|&b| b as i8).collect();
        let string_value = Value::from(string_bytes);
        object.set_value("value", string_value)?;
        let value = Value::Object(Some(Reference::from(object)));
        assert_eq!("string(foo)", value.to_string());
        assert_eq!("foo".to_string(), value.as_string()?);
        Ok(())
    }

    #[test]
    fn test_as_string_error() {
        let result = Value::Int(42).as_string();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[test]
    fn test_unused_format() {
        let value = Value::Unused;
        assert_eq!("unused", value.to_string());
        assert!(value.is_category_1());
        assert!(!value.is_category_2());
    }

    #[test]
    fn test_from_bool() {
        let value: Value = true.into();
        assert_eq!(Value::Int(1), value);
    }

    #[test]
    fn test_from_i8() {
        let value: Value = 42i8.into();
        assert_eq!(Value::Int(42), value);
    }

    #[test]
    fn test_from_u8() {
        let value: Value = 42u8.into();
        assert_eq!(Value::Int(42), value);
    }

    #[test]
    fn test_from_char() {
        let value: Value = 'a'.into();
        assert_eq!(Value::Int(97), value);
    }

    #[test]
    fn test_from_i16() {
        let value: Value = 42i16.into();
        assert_eq!(Value::Int(42), value);
    }

    #[test]
    fn test_from_u16() {
        let value: Value = 42u16.into();
        assert_eq!(Value::Int(42), value);
    }

    #[test]
    fn test_from_i32() {
        let value: Value = 42i32.into();
        assert_eq!(Value::Int(42), value);
    }

    #[test]
    fn test_from_u32() {
        let value: Value = 42u32.into();
        assert_eq!(Value::Int(42), value);
    }

    #[test]
    fn test_from_i64() {
        let value: Value = 42i64.into();
        assert_eq!(Value::Long(42), value);
    }

    #[test]
    fn test_from_u64() {
        let value: Value = 42u64.into();
        assert_eq!(Value::Long(42), value);
    }

    #[test]
    fn test_from_isize() {
        let value: Value = 42isize.into();
        assert_eq!(Value::Long(42), value);
    }

    #[test]
    fn test_from_usize() {
        let value: Value = 42usize.into();
        assert_eq!(Value::Long(42), value);
    }

    #[test]
    fn test_from_f32() {
        let value: Value = 42.1f32.into();
        assert_eq!(Value::Float(42.1), value);
    }

    #[test]
    fn test_from_f64() {
        let value: Value = 42.1f64.into();
        assert_eq!(Value::Double(42.1), value);
    }

    #[test]
    fn test_from_vec_bool() {
        let value: Value = vec![true, false].into();
        assert_eq!(Value::Object(Some(Reference::from(vec![1i8, 0i8]))), value);
    }

    #[test]
    fn test_from_vec_i8() {
        let value: Value = vec![1i8, 2i8].into();
        assert_eq!(Value::Object(Some(Reference::from(vec![1i8, 2i8]))), value);
    }

    #[test]
    fn test_from_vec_u8() {
        let value: Value = vec![1u8, 2u8].into();
        assert_eq!(Value::Object(Some(Reference::from(vec![1i8, 2i8]))), value);
    }

    #[test]
    fn test_from_vec_char() {
        let value: Value = vec!['a', 'b'].into();
        assert_eq!(Value::Object(Some(Reference::from(vec!['a', 'b']))), value);
    }

    #[test]
    fn test_from_vec_i16() {
        let value: Value = vec![1i16, 2i16].into();
        assert_eq!(
            Value::Object(Some(Reference::from(vec![1i16, 2i16]))),
            value
        );
    }

    #[test]
    fn test_from_vec_u16() {
        let value: Value = vec![1u16, 2u16].into();
        assert_eq!(
            Value::Object(Some(Reference::from(vec![1i16, 2i16]))),
            value
        );
    }

    #[test]
    fn test_from_vec_i32() {
        let value: Value = vec![1i32, 2i32].into();
        assert_eq!(
            Value::Object(Some(Reference::from(vec![1i32, 2i32]))),
            value
        );
    }

    #[test]
    fn test_from_vec_u32() {
        let value: Value = vec![1u32, 2u32].into();
        assert_eq!(
            Value::Object(Some(Reference::from(vec![1i32, 2i32]))),
            value
        );
    }

    #[test]
    fn test_from_vec_i64() {
        let value: Value = vec![1i64, 2i64].into();
        assert_eq!(
            Value::Object(Some(Reference::from(vec![1i64, 2i64]))),
            value
        );
    }

    #[test]
    fn test_from_vec_u64() {
        let value: Value = vec![1u64, 2u64].into();
        assert_eq!(
            Value::Object(Some(Reference::from(vec![1i64, 2i64]))),
            value
        );
    }

    #[test]
    fn test_from_vec_isize() {
        let value: Value = vec![1isize, 2isize].into();
        assert_eq!(
            Value::Object(Some(Reference::from(vec![1i64, 2i64]))),
            value
        );
    }

    #[test]
    fn test_from_vec_usize() {
        let value: Value = vec![1usize, 2usize].into();
        assert_eq!(
            Value::Object(Some(Reference::from(vec![1i64, 2i64]))),
            value
        );
    }

    #[test]
    fn test_from_vec_f32() {
        let value: Value = vec![1.1f32, 2.2f32].into();
        assert_eq!(
            Value::Object(Some(Reference::from(vec![1.1f32, 2.2f32]))),
            value
        );
    }

    #[test]
    fn test_from_vec_f64() {
        let value: Value = vec![1.1f64, 2.2f64].into();
        assert_eq!(
            Value::Object(Some(Reference::from(vec![1.1f64, 2.2f64]))),
            value
        );
    }
}
