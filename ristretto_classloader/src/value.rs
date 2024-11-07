use crate::reference::Reference;
use crate::Error::InvalidValueType;
use crate::{Class, Object, Result};
use std::fmt;
use std::fmt::Display;
use std::sync::Arc;

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
    /// if the value is not an `Int`.
    pub fn to_int(&self) -> Result<i32> {
        match self {
            Value::Int(value) => Ok(*value),
            _ => Err(InvalidValueType("Expected an int value".to_string())),
        }
    }

    /// Returns the value as an `i64`.
    ///
    /// # Errors
    /// if the value is not a `Long`
    pub fn to_long(&self) -> Result<i64> {
        match self {
            Value::Long(value) => Ok(*value),
            _ => Err(InvalidValueType("Expected a long value".to_string())),
        }
    }

    /// Returns the value as an `f32`.
    ///
    /// # Errors
    /// if the value is not a `Float`
    pub fn to_float(&self) -> Result<f32> {
        match self {
            Value::Float(value) => Ok(*value),
            _ => Err(InvalidValueType("Expected a float value".to_string())),
        }
    }

    /// Returns the value as an `f64`.
    ///
    /// # Errors
    /// if the value is not a `Double`
    pub fn to_double(&self) -> Result<f64> {
        match self {
            Value::Double(value) => Ok(*value),
            _ => Err(InvalidValueType("Expected a double value".to_string())),
        }
    }

    /// Returns the value as an `Option<Reference>`.
    ///
    /// # Errors
    /// if the value is not an `Object`
    pub fn to_object(&self) -> Result<Option<Reference>> {
        match self {
            Value::Object(value) => Ok(value.clone()),
            _ => Err(InvalidValueType("Expected an object value".to_string())),
        }
    }

    /// Returns the value as an `Option<Reference>`.
    ///
    /// # Errors
    /// if the value is not an `Object`
    pub fn try_to_object(&self) -> Result<Reference> {
        let Some(reference) = self.to_object()? else {
            return Err(InvalidValueType("Expected an object value".to_string()));
        };
        Ok(reference)
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
                    write!(f, "{value}")
                } else {
                    write!(f, "Object(null)")
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

impl From<(Arc<Class>, Vec<Option<Reference>>)> for Value {
    fn from(value: (Arc<Class>, Vec<Option<Reference>>)) -> Self {
        Value::Object(Some(Reference::from(value)))
    }
}

impl From<Object> for Value {
    fn from(value: Object) -> Self {
        Value::Object(Some(Reference::from(value)))
    }
}

impl TryInto<Vec<bool>> for Value {
    type Error = crate::Error;

    fn try_into(self) -> Result<Vec<bool>> {
        self.try_to_object()?.try_into()
    }
}

impl TryInto<Vec<char>> for Value {
    type Error = crate::Error;

    fn try_into(self) -> Result<Vec<char>> {
        self.try_to_object()?.try_into()
    }
}

impl TryInto<Vec<i8>> for Value {
    type Error = crate::Error;

    fn try_into(self) -> Result<Vec<i8>> {
        self.try_to_object()?.try_into()
    }
}

impl TryInto<Vec<u8>> for Value {
    type Error = crate::Error;

    fn try_into(self) -> Result<Vec<u8>> {
        self.try_to_object()?.try_into()
    }
}

impl TryInto<Vec<i16>> for Value {
    type Error = crate::Error;

    fn try_into(self) -> Result<Vec<i16>> {
        self.try_to_object()?.try_into()
    }
}

impl TryInto<Vec<u16>> for Value {
    type Error = crate::Error;

    fn try_into(self) -> Result<Vec<u16>> {
        self.try_to_object()?.try_into()
    }
}

impl TryInto<Vec<i32>> for Value {
    type Error = crate::Error;

    fn try_into(self) -> Result<Vec<i32>> {
        self.try_to_object()?.try_into()
    }
}

impl TryInto<Vec<u32>> for Value {
    type Error = crate::Error;

    fn try_into(self) -> Result<Vec<u32>> {
        self.try_to_object()?.try_into()
    }
}

impl TryInto<Vec<i64>> for Value {
    type Error = crate::Error;

    fn try_into(self) -> Result<Vec<i64>> {
        self.try_to_object()?.try_into()
    }
}

impl TryInto<Vec<u64>> for Value {
    type Error = crate::Error;

    fn try_into(self) -> Result<Vec<u64>> {
        self.try_to_object()?.try_into()
    }
}

impl TryInto<Vec<isize>> for Value {
    type Error = crate::Error;

    fn try_into(self) -> Result<Vec<isize>> {
        self.try_to_object()?.try_into()
    }
}

impl TryInto<Vec<usize>> for Value {
    type Error = crate::Error;

    fn try_into(self) -> Result<Vec<usize>> {
        self.try_to_object()?.try_into()
    }
}

impl TryInto<Vec<f32>> for Value {
    type Error = crate::Error;

    fn try_into(self) -> Result<Vec<f32>> {
        self.try_to_object()?.try_into()
    }
}

impl TryInto<Vec<f64>> for Value {
    type Error = crate::Error;

    fn try_into(self) -> Result<Vec<f64>> {
        self.try_to_object()?.try_into()
    }
}

impl TryInto<bool> for Value {
    type Error = crate::Error;

    fn try_into(self) -> Result<bool> {
        match self {
            Value::Int(value) => Ok(value != 0),
            _ => self.try_to_object()?.try_into(),
        }
    }
}

impl TryInto<char> for Value {
    type Error = crate::Error;

    fn try_into(self) -> Result<char> {
        match self {
            Value::Int(value) => {
                #[expect(clippy::cast_sign_loss)]
                let value = value as u32;
                let value = char::from_u32(value)
                    .ok_or(InvalidValueType("Invalid char value".to_string()))?;
                Ok(value)
            }
            _ => self.try_to_object()?.try_into(),
        }
    }
}

impl TryInto<i8> for Value {
    type Error = crate::Error;

    fn try_into(self) -> Result<i8> {
        match self {
            Value::Int(value) => Ok(i8::try_from(value)?),
            _ => self.try_to_object()?.try_into(),
        }
    }
}

impl TryInto<u8> for Value {
    type Error = crate::Error;

    fn try_into(self) -> Result<u8> {
        match self {
            Value::Int(value) => Ok(u8::try_from(value)?),
            _ => self.try_to_object()?.try_into(),
        }
    }
}

impl TryInto<i16> for Value {
    type Error = crate::Error;

    fn try_into(self) -> Result<i16> {
        match self {
            Value::Int(value) => Ok(i16::try_from(value)?),
            _ => self.try_to_object()?.try_into(),
        }
    }
}

impl TryInto<u16> for Value {
    type Error = crate::Error;

    fn try_into(self) -> Result<u16> {
        match self {
            Value::Int(value) => Ok(u16::try_from(value)?),
            _ => self.try_to_object()?.try_into(),
        }
    }
}

impl TryInto<i32> for Value {
    type Error = crate::Error;

    fn try_into(self) -> Result<i32> {
        match self {
            Value::Int(value) => Ok(value),
            _ => self.try_to_object()?.try_into(),
        }
    }
}

impl TryInto<u32> for Value {
    type Error = crate::Error;

    fn try_into(self) -> Result<u32> {
        match self {
            Value::Int(value) => Ok(u32::try_from(value)?),
            _ => self.try_to_object()?.try_into(),
        }
    }
}

impl TryInto<i64> for Value {
    type Error = crate::Error;

    fn try_into(self) -> Result<i64> {
        match self {
            Value::Long(value) => Ok(value),
            _ => self.try_to_object()?.try_into(),
        }
    }
}

impl TryInto<u64> for Value {
    type Error = crate::Error;

    fn try_into(self) -> Result<u64> {
        match self {
            Value::Long(value) => Ok(u64::try_from(value)?),
            _ => self.try_to_object()?.try_into(),
        }
    }
}

impl TryInto<isize> for Value {
    type Error = crate::Error;

    fn try_into(self) -> Result<isize> {
        match self {
            Value::Long(value) => Ok(isize::try_from(value)?),
            _ => self.try_to_object()?.try_into(),
        }
    }
}

impl TryInto<usize> for Value {
    type Error = crate::Error;

    fn try_into(self) -> Result<usize> {
        match self {
            Value::Long(value) => Ok(usize::try_from(value)?),
            _ => self.try_to_object()?.try_into(),
        }
    }
}

impl TryInto<f32> for Value {
    type Error = crate::Error;

    fn try_into(self) -> Result<f32> {
        match self {
            Value::Float(value) => Ok(value),
            _ => self.try_to_object()?.try_into(),
        }
    }
}

impl TryInto<f64> for Value {
    type Error = crate::Error;

    fn try_into(self) -> Result<f64> {
        match self {
            Value::Double(value) => Ok(value),
            _ => self.try_to_object()?.try_into(),
        }
    }
}

impl TryInto<String> for Value {
    type Error = crate::Error;

    fn try_into(self) -> Result<String> {
        self.try_to_object()?.try_into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{runtime, Class, Object};
    use std::sync::Arc;

    async fn load_class(class: &str) -> Result<Arc<Class>> {
        let (_java_home, _java_version, class_loader) = runtime::default_class_loader().await?;
        class_loader.load(class).await
    }

    #[test]
    fn test_int_format() -> Result<()> {
        let value = Value::Int(42);
        assert_eq!(42, value.to_int()?);
        assert_eq!("int(42)", value.to_string());
        assert!(value.is_category_1());
        assert!(!value.is_category_2());
        Ok(())
    }

    #[test]
    fn test_to_int_error() {
        let result = Value::Long(42).to_int();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[test]
    fn test_long_format() -> Result<()> {
        let value = Value::Long(42);
        assert_eq!(42, value.to_long()?);
        assert_eq!("long(42)", value.to_string());
        assert!(!value.is_category_1());
        assert!(value.is_category_2());
        Ok(())
    }

    #[test]
    fn test_to_long_error() {
        let result = Value::Int(42).to_long();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[test]
    fn test_float_format() -> Result<()> {
        let value = Value::Float(42.1);
        let compare_value = value.to_float()? - 42.1f32;
        assert!(compare_value.abs() < 0.1f32);
        assert_eq!("float(42.1)", value.to_string());
        assert!(value.is_category_1());
        assert!(!value.is_category_2());
        Ok(())
    }

    #[test]
    fn test_to_float_error() {
        let result = Value::Int(42).to_float();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[test]
    fn test_double_format() -> Result<()> {
        let value = Value::Double(42.1);
        let compare_value = value.to_double()? - 42.1f64;
        assert!(compare_value.abs() < 0.1f64);
        assert_eq!("double(42.1)", value.to_string());
        assert!(!value.is_category_1());
        assert!(value.is_category_2());
        Ok(())
    }

    #[test]
    fn test_to_double_error() {
        let result = Value::Int(42).to_double();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[test]
    fn test_object_format() -> Result<()> {
        let value = Value::Object(None);
        assert_eq!(None, value.to_object()?);
        assert_eq!("Object(null)", value.to_string());
        assert!(value.is_category_1());
        assert!(!value.is_category_2());
        assert_eq!(
            "byte[1, 2, 3]",
            format!("{}", Value::from(vec![1i8, 2i8, 3i8]))
        );
        Ok(())
    }

    #[test]
    fn test_to_object_error() {
        let result = Value::Int(42).to_object();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[test]
    fn test_try_to_object_error() {
        let result = Value::Int(42).try_to_object();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[test]
    fn test_try_to_object_null_error() {
        let result = Value::Object(None).try_to_object();
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
        let value = Value::from(object);
        assert_eq!("String(\"foo\")", value.to_string());
        let value: String = value.try_into()?;
        assert_eq!("foo".to_string(), value);
        Ok(())
    }

    #[test]
    fn test_as_string_error() {
        let result: Result<String> = Value::Int(42).try_into();
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

    #[test]
    fn test_from_class_vec() -> Result<()> {
        let original_class = Arc::new(Class::new_array("[Ljava/lang/Object;")?);
        let original_value = vec![None];
        let value = Value::from((original_class.clone(), original_value.clone()));
        assert!(matches!(value, Value::Object(Some(Reference::Array(_, _)))));
        Ok(())
    }

    #[test]
    fn test_from_object() -> Result<()> {
        let class = Arc::new(Class::new_array("[Ljava/lang/Object;")?);
        let object = Object::new(class)?;
        let value = Value::from(object);
        assert!(matches!(value, Value::Object(Some(Reference::Object(_)))));
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_vec_bool() -> Result<()> {
        let original_value = vec![true];
        let value = Value::from(original_value.clone());
        let value: Vec<bool> = value.try_into()?;
        assert_eq!(original_value, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_vec_char() -> Result<()> {
        let original_value = vec!['*'];
        let value = Value::from(original_value.clone());
        let value: Vec<char> = value.try_into()?;
        assert_eq!(original_value, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_vec_i8() -> Result<()> {
        let original_value = vec![42i8];
        let value = Value::from(original_value.clone());
        let value: Vec<i8> = value.try_into()?;
        assert_eq!(original_value, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_vec_u8() -> Result<()> {
        let original_value = vec![42u8];
        let value = Value::from(original_value.clone());
        let value: Vec<u8> = value.try_into()?;
        assert_eq!(original_value, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_vec_i16() -> Result<()> {
        let original_value = vec![42i16];
        let value = Value::from(original_value.clone());
        let value: Vec<i16> = value.try_into()?;
        assert_eq!(original_value, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_vec_u16() -> Result<()> {
        let original_value = vec![42u16];
        let value = Value::from(original_value.clone());
        let value: Vec<u16> = value.try_into()?;
        assert_eq!(original_value, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_vec_i32() -> Result<()> {
        let original_value = vec![42i32];
        let value = Value::from(original_value.clone());
        let value: Vec<i32> = value.try_into()?;
        assert_eq!(original_value, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_vec_u32() -> Result<()> {
        let original_value = vec![42u32];
        let value = Value::from(original_value.clone());
        let value: Vec<u32> = value.try_into()?;
        assert_eq!(original_value, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_vec_i64() -> Result<()> {
        let original_value = vec![42i64];
        let value = Value::from(original_value.clone());
        let value: Vec<i64> = value.try_into()?;
        assert_eq!(original_value, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_vec_u64() -> Result<()> {
        let original_value = vec![42u64];
        let value = Value::from(original_value.clone());
        let value: Vec<u64> = value.try_into()?;
        assert_eq!(original_value, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_vec_isize() -> Result<()> {
        let original_value = vec![42isize];
        let value = Value::from(original_value.clone());
        let value: Vec<isize> = value.try_into()?;
        assert_eq!(original_value, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_vec_usize() -> Result<()> {
        let original_value = vec![42usize];
        let value = Value::from(original_value.clone());
        let value: Vec<usize> = value.try_into()?;
        assert_eq!(original_value, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_vec_f32() -> Result<()> {
        let original_value = vec![42.1f32];
        let value = Value::from(original_value.clone());
        let value: Vec<f32> = value.try_into()?;
        assert_eq!(original_value, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_vec_f64() -> Result<()> {
        let original_value = vec![42.1f64];
        let value = Value::from(original_value.clone());
        let value: Vec<f64> = value.try_into()?;
        assert_eq!(original_value, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_bool() -> Result<()> {
        let value: bool = Value::Int(1).try_into()?;
        assert!(value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_bool_object() -> Result<()> {
        let class = load_class("java/lang/Boolean").await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Int(1))?;
        let value = Value::from(object);
        let value: bool = value.try_into()?;
        assert!(value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_char() -> Result<()> {
        let value: char = Value::Int(42).try_into()?;
        assert_eq!('*', value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_char_object() -> Result<()> {
        let class = load_class("java/lang/Character").await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Int(42))?;
        let value = Value::from(object);
        let value: char = value.try_into()?;
        assert_eq!('*', value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_i8() -> Result<()> {
        let value: i8 = Value::Int(42).try_into()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_i8_object() -> Result<()> {
        let class = load_class("java/lang/Byte").await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Int(42))?;
        let value = Value::from(object);
        let value: i8 = value.try_into()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_u8() -> Result<()> {
        let value: u8 = Value::Int(42).try_into()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_u8_object() -> Result<()> {
        let class = load_class("java/lang/Byte").await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Int(42))?;
        let value = Value::from(object);
        let value: u8 = value.try_into()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_i16() -> Result<()> {
        let value: i16 = Value::Int(42).try_into()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_i16_object() -> Result<()> {
        let class = load_class("java/lang/Short").await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Int(42))?;
        let value = Value::from(object);
        let value: i16 = value.try_into()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_u16() -> Result<()> {
        let value: u16 = Value::Int(42).try_into()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_u16_object() -> Result<()> {
        let class = load_class("java/lang/Short").await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Int(42))?;
        let value = Value::from(object);
        let value: u16 = value.try_into()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_i32() -> Result<()> {
        let value: i32 = Value::Int(42).try_into()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_i32_object() -> Result<()> {
        let class = load_class("java/lang/Integer").await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Int(42))?;
        let value = Value::from(object);
        let value: i32 = value.try_into()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_u32() -> Result<()> {
        let value: u32 = Value::Int(42).try_into()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_u32_object() -> Result<()> {
        let class = load_class("java/lang/Integer").await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Int(42))?;
        let value = Value::from(object);
        let value: u32 = value.try_into()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_i64() -> Result<()> {
        let value: i64 = Value::Long(42).try_into()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_i64_object() -> Result<()> {
        let class = load_class("java/lang/Long").await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Long(42))?;
        let value = Value::from(object);
        let value: i64 = value.try_into()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_u64() -> Result<()> {
        let value: u64 = Value::Long(42).try_into()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_u64_object() -> Result<()> {
        let class = load_class("java/lang/Long").await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Long(42))?;
        let value = Value::from(object);
        let value: u64 = value.try_into()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_isize() -> Result<()> {
        let value: isize = Value::Long(42).try_into()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_isize_object() -> Result<()> {
        let class = load_class("java/lang/Long").await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Long(42))?;
        let value = Value::from(object);
        let value: isize = value.try_into()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_usize() -> Result<()> {
        let value: usize = Value::Long(42).try_into()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_usize_object() -> Result<()> {
        let class = load_class("java/lang/Long").await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Long(42))?;
        let value = Value::from(object);
        let value: usize = value.try_into()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_f32() -> Result<()> {
        let class = load_class("java/lang/Float").await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Float(42.1))?;
        let value = Value::from(object);
        let value: f32 = value.try_into()?;
        let value = value - 42.1f32;
        assert!(value.abs() < 0.1f32);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_f32_object() -> Result<()> {
        let value: f32 = Value::Float(42.1).try_into()?;
        let value = value - 42.1f32;
        assert!(value.abs() < 0.1f32);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_f64() -> Result<()> {
        let value: f64 = Value::Double(42.1).try_into()?;
        let value = value - 42.1f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_f64_object() -> Result<()> {
        let class = load_class("java/lang/Double").await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Double(42.1))?;
        let value = Value::from(object);
        let value: f64 = value.try_into()?;
        let value = value - 42.1f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[expect(clippy::cast_possible_wrap)]
    #[tokio::test]
    async fn test_try_into_string() -> Result<()> {
        let class = load_class("java/lang/String").await?;
        let object = Object::new(class)?;
        let string_bytes: Vec<i8> = "foo".as_bytes().to_vec().iter().map(|&b| b as i8).collect();
        let string_value = Value::from(string_bytes);
        object.set_value("value", string_value)?;
        let result: String = object.try_into()?;
        assert_eq!("foo".to_string(), result);
        Ok(())
    }
}
