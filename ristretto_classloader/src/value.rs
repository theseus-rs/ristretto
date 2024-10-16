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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{runtime, ConcurrentVec, Object};

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
            format!(
                "{}",
                Value::Object(Some(Reference::ByteArray(ConcurrentVec::from(vec![
                    1, 2, 3
                ]))))
            )
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
        let (_runtime_version, class_loader) = runtime::class_loader("21.0.4.7.1").await?;
        let class = class_loader.load("java/lang/String").await?;
        let object = Object::new(class)?;
        let string_bytes = "foo".as_bytes().to_vec().iter().map(|&b| b as i8).collect();
        let string_value = Value::Object(Some(Reference::ByteArray(ConcurrentVec::from(
            string_bytes,
        ))));
        object.field("value")?.set_value(string_value)?;
        let value = Value::Object(Some(Reference::Object(object)));
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
}
