use crate::concurrent_vec::ConcurrentVec;
use crate::Error::InvalidValueType;
use crate::{Class, Object, Result, Value};
use ristretto_classfile::{ClassFile, ConstantPool};
use std::fmt;
use std::fmt::Display;
use std::sync::Arc;

/// Represents a reference to an object in the Ristretto VM.
#[derive(Clone, Debug, PartialEq)]
pub enum Reference {
    ByteArray(ConcurrentVec<i8>),
    CharArray(ConcurrentVec<u16>),
    ShortArray(ConcurrentVec<i16>),
    IntArray(ConcurrentVec<i32>),
    LongArray(ConcurrentVec<i64>),
    FloatArray(ConcurrentVec<f32>),
    DoubleArray(ConcurrentVec<f64>),
    Array(Arc<Class>, ConcurrentVec<Option<Reference>>),
    Object(Object),
}

impl Reference {
    /// Get the class name of the reference
    #[must_use]
    pub fn class_name(&self) -> String {
        match self {
            Reference::ByteArray(_) => "[B".to_string(),
            Reference::CharArray(_) => "[C".to_string(),
            Reference::ShortArray(_) => "[S".to_string(),
            Reference::IntArray(_) => "[I".to_string(),
            Reference::LongArray(_) => "[J".to_string(),
            Reference::FloatArray(_) => "[F".to_string(),
            Reference::DoubleArray(_) => "[D".to_string(),
            Reference::Array(class, _) => class.name().to_string(),
            Reference::Object(value) => value.class().name().to_string(),
        }
    }

    /// Get the class of the reference
    ///
    /// # Errors
    /// if the class cannot be created
    pub fn class(&self) -> Result<Arc<Class>> {
        let class = if let Reference::Object(value) = self {
            value.class().clone()
        } else {
            let class_name = self.class_name();
            let mut constant_pool = ConstantPool::default();
            let class_index = constant_pool.add_class(class_name.as_str())?;
            let class_file = ClassFile {
                constant_pool,
                this_class: class_index,
                ..Default::default()
            };
            let class = Class::from(class_file)?;
            Arc::new(class)
        };
        Ok(class)
    }

    /// Returns the reference as a `Vec<i8>`.
    ///
    /// # Errors
    /// if the value is not a `ByteArray`.
    pub fn to_byte_vec(&self) -> Result<Vec<i8>> {
        match self {
            Reference::ByteArray(value) => Ok(value.to_vec()?),
            _ => Err(InvalidValueType("Expected byte array".to_string())),
        }
    }

    /// Returns the reference as a `Vec<char>`.
    ///
    /// # Errors
    /// if the value is not a `CharArray`.
    pub fn to_char_vec(&self) -> Result<Vec<u16>> {
        match self {
            Reference::CharArray(value) => Ok(value.to_vec()?),
            _ => Err(InvalidValueType("Expected char array".to_string())),
        }
    }

    /// Returns the reference as a `Vec<i16>`.
    ///
    /// # Errors
    /// if the value is not a `ShortArray`.
    pub fn to_short_vec(&self) -> Result<Vec<i16>> {
        match self {
            Reference::ShortArray(value) => Ok(value.to_vec()?),
            _ => Err(InvalidValueType("Expected short array".to_string())),
        }
    }

    /// Returns the reference as a `Vec<i32>`.
    ///
    /// # Errors
    /// if the value is not a `IntArray`.
    pub fn to_int_vec(&self) -> Result<Vec<i32>> {
        match self {
            Reference::IntArray(value) => Ok(value.to_vec()?),
            _ => Err(InvalidValueType("Expected int array".to_string())),
        }
    }

    /// Returns the reference as a `Vec<i64>`.
    ///
    /// # Errors
    /// if the value is not a `LongArray`.
    pub fn to_long_vec(&self) -> Result<Vec<i64>> {
        match self {
            Reference::LongArray(value) => Ok(value.to_vec()?),
            _ => Err(InvalidValueType("Expected long array".to_string())),
        }
    }

    /// Returns the reference as a `Vec<f32>`.
    ///
    /// # Errors
    /// if the value is not a `FloatArray`.
    pub fn to_float_vec(&self) -> Result<Vec<f32>> {
        match self {
            Reference::FloatArray(value) => Ok(value.to_vec()?),
            _ => Err(InvalidValueType("Expected float array".to_string())),
        }
    }

    /// Returns the reference as a `Vec<f64>`.
    ///
    /// # Errors
    /// if the value is not a `DoubleArray`.
    pub fn to_double_vec(&self) -> Result<Vec<f64>> {
        match self {
            Reference::DoubleArray(value) => Ok(value.to_vec()?),
            _ => Err(InvalidValueType("Expected double array".to_string())),
        }
    }

    /// Returns the reference as a `Vec<Option<Reference>>`.
    ///
    /// # Errors
    /// if the value is not an `Array`.
    pub fn to_class_vec(&self) -> Result<(Arc<Class>, Vec<Option<Reference>>)> {
        match self {
            Reference::Array(class, value) => Ok((class.clone(), value.to_vec()?)),
            _ => Err(InvalidValueType("Expected array".to_string())),
        }
    }

    /// Returns the reference as an `Object`.
    ///
    /// # Errors
    /// if the value is not an Object.
    pub fn to_object(&self) -> Result<Object> {
        match self {
            Reference::Object(object) => Ok(object.clone()),
            _ => Err(InvalidValueType("Expected array".to_string())),
        }
    }

    /// Returns a deep clone of the reference.
    ///
    /// # Errors
    /// if the reference cannot be cloned.
    pub fn deep_clone(&self) -> Result<Reference> {
        let value = match self {
            Reference::ByteArray(value) => Reference::ByteArray(value.deep_clone()?),
            Reference::CharArray(value) => Reference::CharArray(value.deep_clone()?),
            Reference::ShortArray(value) => Reference::ShortArray(value.deep_clone()?),
            Reference::IntArray(value) => Reference::IntArray(value.deep_clone()?),
            Reference::LongArray(value) => Reference::LongArray(value.deep_clone()?),
            Reference::FloatArray(value) => Reference::FloatArray(value.deep_clone()?),
            Reference::DoubleArray(value) => Reference::DoubleArray(value.deep_clone()?),
            Reference::Array(class, value) => {
                let values = value.to_vec()?;
                let array: ConcurrentVec<Option<Reference>> =
                    ConcurrentVec::with_capacity(values.len());
                for value in values {
                    if let Some(reference) = value {
                        array.push(Some(reference.deep_clone()?))?;
                    } else {
                        array.push(None)?;
                    }
                }
                Reference::Array(class.clone(), array)
            }
            Reference::Object(value) => Reference::Object(value.deep_clone()?),
        };
        Ok(value)
    }
}

impl Display for Reference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Reference::ByteArray(value) => write!(f, "byte{value}"),
            Reference::CharArray(value) => write!(f, "char{value}"),
            Reference::ShortArray(value) => write!(f, "short{value}"),
            Reference::IntArray(value) => write!(f, "int{value}"),
            Reference::LongArray(value) => write!(f, "long{value}"),
            Reference::FloatArray(value) => write!(f, "float{value}"),
            Reference::DoubleArray(value) => write!(f, "double{value}"),
            Reference::Array(class, value) => {
                let length = value.len().unwrap_or_default();
                write!(f, "{}[{length}]", class.array_component_type())
            }
            Reference::Object(value) => {
                write!(f, "{value}")
            }
        }
    }
}

impl From<Vec<bool>> for Reference {
    fn from(value: Vec<bool>) -> Self {
        let value: Vec<i8> = value.into_iter().map(i8::from).collect();
        Reference::ByteArray(ConcurrentVec::from(value))
    }
}

impl From<Vec<i8>> for Reference {
    fn from(value: Vec<i8>) -> Self {
        Reference::ByteArray(ConcurrentVec::from(value))
    }
}

impl From<Vec<u8>> for Reference {
    fn from(value: Vec<u8>) -> Self {
        #[expect(clippy::cast_possible_wrap)]
        let value: Vec<i8> = value.into_iter().map(|v| v as i8).collect();
        Reference::ByteArray(ConcurrentVec::from(value))
    }
}

impl From<Vec<char>> for Reference {
    fn from(value: Vec<char>) -> Self {
        let value: Vec<u16> = value.into_iter().map(|v| v as u16).collect();
        Reference::CharArray(ConcurrentVec::from(value))
    }
}

impl From<Vec<i16>> for Reference {
    fn from(value: Vec<i16>) -> Self {
        Reference::ShortArray(ConcurrentVec::from(value))
    }
}

impl From<Vec<u16>> for Reference {
    fn from(value: Vec<u16>) -> Self {
        #[expect(clippy::cast_possible_wrap)]
        let value: Vec<i16> = value.into_iter().map(|v| v as i16).collect();
        Reference::ShortArray(ConcurrentVec::from(value))
    }
}

impl From<Vec<i32>> for Reference {
    fn from(value: Vec<i32>) -> Self {
        Reference::IntArray(ConcurrentVec::from(value))
    }
}

impl From<Vec<u32>> for Reference {
    fn from(value: Vec<u32>) -> Self {
        #[expect(clippy::cast_possible_wrap)]
        let value: Vec<i32> = value.into_iter().map(|v| v as i32).collect();
        Reference::IntArray(ConcurrentVec::from(value))
    }
}

impl From<Vec<i64>> for Reference {
    fn from(value: Vec<i64>) -> Self {
        Reference::LongArray(ConcurrentVec::from(value))
    }
}

impl From<Vec<u64>> for Reference {
    fn from(value: Vec<u64>) -> Self {
        #[expect(clippy::cast_possible_wrap)]
        let value: Vec<i64> = value.into_iter().map(|v| v as i64).collect();
        Reference::LongArray(ConcurrentVec::from(value))
    }
}

impl From<Vec<isize>> for Reference {
    fn from(value: Vec<isize>) -> Self {
        let value: Vec<i64> = value.into_iter().map(|v| v as i64).collect();
        Reference::LongArray(ConcurrentVec::from(value))
    }
}

impl From<Vec<usize>> for Reference {
    fn from(value: Vec<usize>) -> Self {
        #[expect(clippy::cast_possible_wrap)]
        let value: Vec<i64> = value.into_iter().map(|v| v as i64).collect();
        Reference::LongArray(ConcurrentVec::from(value))
    }
}

impl From<Vec<f32>> for Reference {
    fn from(value: Vec<f32>) -> Self {
        Reference::FloatArray(ConcurrentVec::from(value))
    }
}

impl From<Vec<f64>> for Reference {
    fn from(value: Vec<f64>) -> Self {
        Reference::DoubleArray(ConcurrentVec::from(value))
    }
}

impl From<(Arc<Class>, Vec<Option<Reference>>)> for Reference {
    fn from(value: (Arc<Class>, Vec<Option<Reference>>)) -> Self {
        let (class, value) = value;
        Reference::Array(class, ConcurrentVec::from(value))
    }
}

impl TryFrom<(Arc<Class>, Vec<Value>)> for Reference {
    type Error = crate::Error;

    fn try_from(value: (Arc<Class>, Vec<Value>)) -> Result<Self> {
        let (class, values) = value;
        let mut references = Vec::new();

        for value in values {
            let Value::Object(reference) = value else {
                return Err(InvalidValueType("Expected object".to_string()));
            };
            references.push(reference);
        }

        Ok(Reference::Array(class, ConcurrentVec::from(references)))
    }
}

impl From<Object> for Reference {
    fn from(value: Object) -> Self {
        Reference::Object(value)
    }
}

impl TryInto<Vec<bool>> for Reference {
    type Error = crate::Error;

    fn try_into(self) -> Result<Vec<bool>> {
        let value = self.to_byte_vec()?;
        let value = value.into_iter().map(|v| v != 0).collect();
        Ok(value)
    }
}

impl TryInto<Vec<char>> for Reference {
    type Error = crate::Error;

    fn try_into(self) -> Result<Vec<char>> {
        let values = self.to_char_vec()?;
        let mut result = Vec::with_capacity(values.len());
        for value in values {
            let value = u32::from(value);
            let value = char::from_u32(value)
                .ok_or(InvalidValueType(format!("Invalid char value {value}")))?;
            result.push(value);
        }
        Ok(result)
    }
}

impl TryInto<Vec<i8>> for Reference {
    type Error = crate::Error;

    fn try_into(self) -> Result<Vec<i8>> {
        self.to_byte_vec()
    }
}

impl TryInto<Vec<u8>> for Reference {
    type Error = crate::Error;

    fn try_into(self) -> Result<Vec<u8>> {
        let value = self.to_byte_vec()?;
        #[expect(clippy::cast_sign_loss)]
        let value = value.into_iter().map(|v| v as u8).collect();
        Ok(value)
    }
}

impl TryInto<Vec<i16>> for Reference {
    type Error = crate::Error;

    fn try_into(self) -> Result<Vec<i16>> {
        self.to_short_vec()
    }
}

impl TryInto<Vec<u16>> for Reference {
    type Error = crate::Error;

    fn try_into(self) -> Result<Vec<u16>> {
        let value = self.to_short_vec()?;
        #[expect(clippy::cast_sign_loss)]
        let value = value.into_iter().map(|v| v as u16).collect();
        Ok(value)
    }
}

impl TryInto<Vec<i32>> for Reference {
    type Error = crate::Error;

    fn try_into(self) -> Result<Vec<i32>> {
        self.to_int_vec()
    }
}

impl TryInto<Vec<u32>> for Reference {
    type Error = crate::Error;

    fn try_into(self) -> Result<Vec<u32>> {
        let value = self.to_int_vec()?;
        #[expect(clippy::cast_sign_loss)]
        let value = value.into_iter().map(|v| v as u32).collect();
        Ok(value)
    }
}

impl TryInto<Vec<i64>> for Reference {
    type Error = crate::Error;

    fn try_into(self) -> Result<Vec<i64>> {
        self.to_long_vec()
    }
}

impl TryInto<Vec<u64>> for Reference {
    type Error = crate::Error;

    fn try_into(self) -> Result<Vec<u64>> {
        let value = self.to_long_vec()?;
        #[expect(clippy::cast_sign_loss)]
        let value = value.into_iter().map(|v| v as u64).collect();
        Ok(value)
    }
}

impl TryInto<Vec<isize>> for Reference {
    type Error = crate::Error;

    fn try_into(self) -> Result<Vec<isize>> {
        let value: Vec<i64> = self.try_into()?;
        #[expect(clippy::cast_possible_truncation)]
        let value = value.into_iter().map(|v| v as isize).collect();
        Ok(value)
    }
}

impl TryInto<Vec<usize>> for Reference {
    type Error = crate::Error;

    fn try_into(self) -> Result<Vec<usize>> {
        let value: Vec<u64> = self.try_into()?;
        #[expect(clippy::cast_possible_truncation)]
        let value = value.into_iter().map(|v| v as usize).collect();
        Ok(value)
    }
}

impl TryInto<Vec<f32>> for Reference {
    type Error = crate::Error;

    fn try_into(self) -> Result<Vec<f32>> {
        self.to_float_vec()
    }
}

impl TryInto<Vec<f64>> for Reference {
    type Error = crate::Error;

    fn try_into(self) -> Result<Vec<f64>> {
        self.to_double_vec()
    }
}

impl TryInto<bool> for Reference {
    type Error = crate::Error;

    fn try_into(self) -> Result<bool> {
        self.to_object()?.try_into()
    }
}

impl TryInto<char> for Reference {
    type Error = crate::Error;

    fn try_into(self) -> Result<char> {
        self.to_object()?.try_into()
    }
}

impl TryInto<i8> for Reference {
    type Error = crate::Error;

    fn try_into(self) -> Result<i8> {
        self.to_object()?.try_into()
    }
}

impl TryInto<u8> for Reference {
    type Error = crate::Error;

    fn try_into(self) -> Result<u8> {
        self.to_object()?.try_into()
    }
}

impl TryInto<i16> for Reference {
    type Error = crate::Error;

    fn try_into(self) -> Result<i16> {
        self.to_object()?.try_into()
    }
}

impl TryInto<u16> for Reference {
    type Error = crate::Error;

    fn try_into(self) -> Result<u16> {
        self.to_object()?.try_into()
    }
}

impl TryInto<i32> for Reference {
    type Error = crate::Error;

    fn try_into(self) -> Result<i32> {
        self.to_object()?.try_into()
    }
}

impl TryInto<u32> for Reference {
    type Error = crate::Error;

    fn try_into(self) -> Result<u32> {
        self.to_object()?.try_into()
    }
}

impl TryInto<i64> for Reference {
    type Error = crate::Error;

    fn try_into(self) -> Result<i64> {
        self.to_object()?.try_into()
    }
}

impl TryInto<u64> for Reference {
    type Error = crate::Error;

    fn try_into(self) -> Result<u64> {
        self.to_object()?.try_into()
    }
}

impl TryInto<isize> for Reference {
    type Error = crate::Error;

    fn try_into(self) -> Result<isize> {
        self.to_object()?.try_into()
    }
}

impl TryInto<usize> for Reference {
    type Error = crate::Error;

    fn try_into(self) -> Result<usize> {
        self.to_object()?.try_into()
    }
}

impl TryInto<f32> for Reference {
    type Error = crate::Error;

    fn try_into(self) -> Result<f32> {
        self.to_object()?.try_into()
    }
}

impl TryInto<f64> for Reference {
    type Error = crate::Error;

    fn try_into(self) -> Result<f64> {
        self.to_object()?.try_into()
    }
}

impl TryInto<Object> for Reference {
    type Error = crate::Error;

    fn try_into(self) -> Result<Object> {
        self.to_object()
    }
}

impl TryInto<String> for Reference {
    type Error = crate::Error;

    fn try_into(self) -> Result<String> {
        self.to_object()?.try_into()
    }
}

impl TryInto<Arc<Class>> for Reference {
    type Error = crate::Error;

    fn try_into(self) -> Result<Arc<Class>> {
        self.class()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{runtime, Class, Result, Value};
    use ristretto_classfile::ClassFile;
    use std::io::Cursor;
    use std::sync::Arc;

    async fn load_class(class: &str) -> Result<Arc<Class>> {
        let (_java_home, _java_version, class_loader) = runtime::default_class_loader().await?;
        class_loader.load(class).await
    }

    fn minimum_class() -> Result<Arc<Class>> {
        let bytes = include_bytes!("../../classes/Minimum.class").to_vec();
        let mut cursor = Cursor::new(bytes);
        let class_file = ClassFile::from_bytes(&mut cursor)?;
        let class = Arc::new(Class::from(class_file)?);
        Ok(class)
    }

    fn simple_class() -> Result<Arc<Class>> {
        let bytes = include_bytes!("../../classes/Simple.class").to_vec();
        let mut cursor = Cursor::new(bytes);
        let class_file = ClassFile::from_bytes(&mut cursor)?;
        let class = Arc::new(Class::from(class_file)?);
        Ok(class)
    }

    #[test]
    fn test_display_byte_array() -> Result<()> {
        let reference = Reference::from(vec![1i8, 2i8, 3i8]);
        assert_eq!(reference.class_name(), "[B");
        assert_eq!(reference.class()?.name(), "[B");
        assert_eq!(reference.to_string(), "byte[1, 2, 3]");
        Ok(())
    }

    #[test]
    fn test_to_byte_vec() -> Result<()> {
        let original_value = vec![42i8];
        let reference = Reference::from(original_value.clone());
        assert_eq!(original_value, reference.to_byte_vec()?);
        Ok(())
    }

    #[test]
    fn test_to_byte_vec_error() {
        let original_value = vec![42i32];
        let reference = Reference::from(original_value.clone());
        let result = reference.to_byte_vec();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[test]
    fn test_display_char_array() -> Result<()> {
        let reference = Reference::from(vec![1 as char, 2 as char, 3 as char]);
        assert_eq!(reference.class_name(), "[C");
        assert_eq!(reference.class()?.name(), "[C");
        assert_eq!(reference.to_string(), "char[1, 2, 3]");
        Ok(())
    }

    #[test]
    fn test_to_char_vec() -> Result<()> {
        let reference = Reference::from(vec!['*']);
        assert_eq!(vec![42u16], reference.to_char_vec()?);
        Ok(())
    }

    #[test]
    fn test_to_char_vec_error() {
        let original_value = vec![42i32];
        let reference = Reference::from(original_value.clone());
        let result = reference.to_char_vec();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[test]
    fn test_display_short_array() -> Result<()> {
        let reference = Reference::from(vec![1i16, 2i16, 3i16]);
        assert_eq!(reference.class_name(), "[S");
        assert_eq!(reference.class()?.name(), "[S");
        assert_eq!(reference.to_string(), "short[1, 2, 3]");
        Ok(())
    }

    #[test]
    fn test_to_short_vec() -> Result<()> {
        let original_value = vec![42i16];
        let reference = Reference::from(original_value.clone());
        assert_eq!(original_value, reference.to_short_vec()?);
        Ok(())
    }

    #[test]
    fn test_to_short_vec_error() {
        let original_value = vec![42i32];
        let reference = Reference::from(original_value.clone());
        let result = reference.to_short_vec();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[test]
    fn test_display_int_array() -> Result<()> {
        let reference = Reference::from(vec![1i32, 2i32, 3i32]);
        assert_eq!(reference.class_name(), "[I");
        assert_eq!(reference.class()?.name(), "[I");
        assert_eq!(reference.to_string(), "int[1, 2, 3]");
        Ok(())
    }

    #[test]
    fn test_to_int_vec() -> Result<()> {
        let original_value = vec![42i32];
        let reference = Reference::from(original_value.clone());
        assert_eq!(original_value, reference.to_int_vec()?);
        Ok(())
    }

    #[test]
    fn test_to_int_vec_error() {
        let original_value = vec![42i8];
        let reference = Reference::from(original_value.clone());
        let result = reference.to_int_vec();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[test]
    fn test_display_long_array() -> Result<()> {
        let reference = Reference::from(vec![1i64, 2i64, 3i64]);
        assert_eq!(reference.class_name(), "[J");
        assert_eq!(reference.class()?.name(), "[J");
        assert_eq!(reference.to_string(), "long[1, 2, 3]");
        Ok(())
    }

    #[test]
    fn test_to_long_vec() -> Result<()> {
        let original_value = vec![42i64];
        let reference = Reference::from(original_value.clone());
        assert_eq!(original_value, reference.to_long_vec()?);
        Ok(())
    }

    #[test]
    fn test_to_long_vec_error() {
        let original_value = vec![42i32];
        let reference = Reference::from(original_value.clone());
        let result = reference.to_long_vec();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[test]
    fn test_display_float_array() -> Result<()> {
        let reference = Reference::from(vec![1.0f32, 2.0f32, 3.0f32]);
        assert_eq!(reference.class_name(), "[F");
        assert_eq!(reference.class()?.name(), "[F");
        assert_eq!(reference.to_string(), "float[1.0, 2.0, 3.0]");
        Ok(())
    }

    #[test]
    fn test_to_float_vec() -> Result<()> {
        let original_value = vec![42.1f32];
        let reference = Reference::from(original_value.clone());
        assert_eq!(original_value, reference.to_float_vec()?);
        Ok(())
    }

    #[test]
    fn test_to_float_vec_error() {
        let original_value = vec![42i32];
        let reference = Reference::from(original_value.clone());
        let result = reference.to_float_vec();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[test]
    fn test_display_double_array() -> Result<()> {
        let reference = Reference::from(vec![1.0f64, 2.0f64, 3.0f64]);
        assert_eq!(reference.class_name(), "[D");
        assert_eq!(reference.class()?.name(), "[D");
        assert_eq!(reference.to_string(), "double[1.0, 2.0, 3.0]");
        Ok(())
    }

    #[test]
    fn test_to_double_vec() -> Result<()> {
        let original_value = vec![42.1f64];
        let reference = Reference::from(original_value.clone());
        assert_eq!(original_value, reference.to_double_vec()?);
        Ok(())
    }

    #[test]
    fn test_to_double_vec_error() {
        let original_value = vec![42i32];
        let reference = Reference::from(original_value.clone());
        let result = reference.to_double_vec();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[test]
    fn test_display_reference_array() -> Result<()> {
        let class = Arc::new(Class::new_named("[Ljava/lang/Object;")?);
        let reference = Reference::Array(class, ConcurrentVec::from(vec![None]));
        assert_eq!(reference.class_name(), "[Ljava/lang/Object;");
        assert_eq!(reference.class()?.name(), "[Ljava/lang/Object;");
        assert_eq!(reference.to_string(), "java/lang/Object[1]");
        Ok(())
    }

    #[test]
    fn test_to_class_vec() -> Result<()> {
        let original_class = Arc::new(Class::new_named("[Ljava/lang/Object;")?);
        let original_value = vec![None];
        let reference = Reference::from((original_class.clone(), original_value.clone()));
        let (class, value) = reference.to_class_vec()?;
        assert_eq!(original_class, class);
        assert_eq!(original_value, value);
        Ok(())
    }

    #[test]
    fn test_to_class_vec_error() {
        let original_value = vec![42i32];
        let reference = Reference::from(original_value.clone());
        let result = reference.to_class_vec();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[tokio::test]
    async fn test_display_reference() -> Result<()> {
        let class = minimum_class()?;
        let object = Object::new(class)?;
        let reference = Reference::from(object);
        assert_eq!(reference.class_name(), "Minimum");
        assert_eq!(reference.class()?.name(), "Minimum");
        assert!(reference.to_string().starts_with("Object(class Minimum)"));
        assert!(reference.to_string().contains("Minimum"));
        Ok(())
    }

    #[test]
    fn test_to_object() -> Result<()> {
        let class = minimum_class()?;
        let object = Object::new(class.clone())?;
        let reference = Reference::from(object);
        assert_eq!(class, reference.class()?);
        Ok(())
    }

    #[test]
    fn test_to_object_error() {
        let original_value = vec![42i32];
        let reference = Reference::from(original_value.clone());
        let result = reference.to_object();
        assert!(matches!(result, Err(InvalidValueType(_))));
    }

    #[test]
    fn test_clone_byte_array() -> Result<()> {
        let reference = Reference::from(vec![1i8]);
        let clone = reference.clone();
        assert_eq!(reference, clone);

        let Reference::ByteArray(ref array) = clone else {
            unreachable!("Expected byte array");
        };
        array.set(0, 2i8)?;
        assert_eq!(reference, clone);
        Ok(())
    }

    #[test]
    fn test_deep_clone_byte_array() -> Result<()> {
        let reference = Reference::from(vec![1i8]);
        let clone = reference.deep_clone()?;
        assert_eq!(reference, clone);

        let Reference::ByteArray(ref array) = clone else {
            unreachable!("Expected byte array");
        };
        array.set(0, 2i8)?;
        assert_ne!(reference, clone);
        Ok(())
    }

    #[test]
    fn test_clone_char_array() -> Result<()> {
        let reference = Reference::from(vec![1 as char]);
        let clone = reference.clone();
        assert_eq!(reference, clone);

        let Reference::CharArray(ref array) = clone else {
            unreachable!("Expected char array");
        };
        array.set(0, 2)?;
        assert_eq!(reference, clone);
        Ok(())
    }

    #[test]
    fn test_deep_clone_char_array() -> Result<()> {
        let reference = Reference::from(vec![1 as char]);
        let clone = reference.deep_clone()?;
        assert_eq!(reference, clone);

        let Reference::CharArray(ref array) = clone else {
            unreachable!("Expected char array");
        };
        array.set(0, 2)?;
        assert_ne!(reference, clone);
        Ok(())
    }

    #[test]
    fn test_clone_short_array() -> Result<()> {
        let reference = Reference::from(vec![1i16]);
        let clone = reference.clone();
        assert_eq!(reference, clone);

        let Reference::ShortArray(ref array) = clone else {
            unreachable!("Expected short array");
        };
        array.set(0, 2)?;
        assert_eq!(reference, clone);
        Ok(())
    }

    #[test]
    fn test_deep_clone_short_array() -> Result<()> {
        let reference = Reference::from(vec![1i16]);
        let clone = reference.deep_clone()?;
        assert_eq!(reference, clone);

        let Reference::ShortArray(ref array) = clone else {
            unreachable!("Expected short array");
        };
        array.set(0, 2)?;
        assert_ne!(reference, clone);
        Ok(())
    }

    #[test]
    fn test_clone_int_array() -> Result<()> {
        let reference = Reference::from(vec![1i32]);
        let clone = reference.clone();
        assert_eq!(reference, clone);

        let Reference::IntArray(ref array) = clone else {
            unreachable!("Expected int array");
        };
        array.set(0, 2)?;
        assert_eq!(reference, clone);
        Ok(())
    }

    #[test]
    fn test_deep_clone_int_array() -> Result<()> {
        let reference = Reference::from(vec![1i32]);
        let clone = reference.deep_clone()?;
        assert_eq!(reference, clone);

        let Reference::IntArray(ref array) = clone else {
            unreachable!("Expected int array");
        };
        array.set(0, 2)?;
        assert_ne!(reference, clone);
        Ok(())
    }

    #[test]
    fn test_clone_long_array() -> Result<()> {
        let reference = Reference::from(vec![1i64]);
        let clone = reference.clone();
        assert_eq!(reference, clone);

        let Reference::LongArray(ref array) = clone else {
            unreachable!("Expected long array");
        };
        array.set(0, 2)?;
        assert_eq!(reference, clone);
        Ok(())
    }

    #[test]
    fn test_deep_clone_long_array() -> Result<()> {
        let reference = Reference::from(vec![1i64]);
        let clone = reference.deep_clone()?;
        assert_eq!(reference, clone);

        let Reference::LongArray(ref array) = clone else {
            unreachable!("Expected long array");
        };
        array.set(0, 2)?;
        assert_ne!(reference, clone);
        Ok(())
    }

    #[test]
    fn test_clone_float_array() -> Result<()> {
        let reference = Reference::from(vec![1.0f32]);
        let clone = reference.clone();
        assert_eq!(reference, clone);

        let Reference::FloatArray(ref array) = clone else {
            unreachable!("Expected float array");
        };
        array.set(0, 2.0)?;
        assert_eq!(reference, clone);
        Ok(())
    }

    #[test]
    fn test_deep_clone_float_array() -> Result<()> {
        let reference = Reference::from(vec![1.0f32]);
        let clone = reference.deep_clone()?;
        assert_eq!(reference, clone);

        let Reference::FloatArray(ref array) = clone else {
            unreachable!("Expected float array");
        };
        array.set(0, 2.0)?;
        assert_ne!(reference, clone);
        Ok(())
    }

    #[test]
    fn test_clone_double_array() -> Result<()> {
        let reference = Reference::from(vec![1.0f64]);
        let clone = reference.clone();
        assert_eq!(reference, clone);

        let Reference::DoubleArray(ref array) = clone else {
            unreachable!("Expected double array");
        };
        array.set(0, 2.0)?;
        assert_eq!(reference, clone);
        Ok(())
    }

    #[test]
    fn test_deep_clone_double_array() -> Result<()> {
        let reference = Reference::from(vec![1.0f64]);
        let clone = reference.deep_clone()?;
        assert_eq!(reference, clone);

        let Reference::DoubleArray(ref array) = clone else {
            unreachable!("Expected double array");
        };
        array.set(0, 2.0)?;
        assert_ne!(reference, clone);
        Ok(())
    }

    #[test]
    fn test_clone_reference_array() -> Result<()> {
        let class = minimum_class()?;
        let reference = Reference::Array(class.clone(), ConcurrentVec::from(vec![None]));
        let clone = reference.clone();
        assert_eq!(reference, clone);

        let Reference::Array(ref _class, ref array) = clone else {
            unreachable!("Expected reference array");
        };
        array.set(0, Some(Reference::from(vec![1i8])))?;
        assert_eq!(reference, clone);
        Ok(())
    }

    #[test]
    fn test_deep_clone_reference_array() -> Result<()> {
        let class = minimum_class()?;
        let reference = Reference::Array(class.clone(), ConcurrentVec::from(vec![None]));
        let clone = reference.deep_clone()?;
        assert_eq!(reference, clone);

        let Reference::Array(ref _class, ref array) = clone else {
            unreachable!("Expected reference array");
        };
        array.set(0, Some(Reference::from(vec![1i8])))?;
        assert_ne!(reference, clone);
        Ok(())
    }

    #[tokio::test]
    async fn test_clone_object() -> Result<()> {
        let class_name = "java.lang.Integer";
        let class = load_class(class_name).await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Int(1))?;
        let reference = Reference::from(object);
        let clone = reference.clone();
        assert_eq!(reference, clone);

        let Reference::Object(ref cloned_object) = clone else {
            unreachable!("Expected object");
        };
        cloned_object.set_value("value", Value::Int(2))?;
        assert_eq!(reference, clone);
        Ok(())
    }

    #[tokio::test]
    async fn test_deep_clone_object() -> Result<()> {
        let class_name = "java.lang.Integer";
        let class = load_class(class_name).await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Int(1))?;
        let reference = Reference::from(object);
        let clone = reference.deep_clone()?;
        assert_eq!(reference, clone);

        let Reference::Object(ref cloned_object) = clone else {
            unreachable!("Expected object");
        };
        cloned_object.set_value("value", Value::Int(2))?;
        assert_ne!(reference, clone);
        Ok(())
    }

    #[test]
    fn test_array_eq() -> Result<()> {
        let class = minimum_class()?;
        let ref1 = Reference::Array(class.clone(), ConcurrentVec::from(vec![]));
        let ref2 = Reference::Array(class, ConcurrentVec::from(vec![]));
        assert_eq!(ref1, ref2);
        Ok(())
    }

    #[test]
    fn test_array_eq_class_ne() -> Result<()> {
        let minimum_class = minimum_class()?;
        let ref1 = Reference::Array(minimum_class, ConcurrentVec::from(vec![]));
        let simple_class = simple_class()?;
        let ref2 = Reference::Array(simple_class, ConcurrentVec::from(vec![]));
        assert_ne!(ref1, ref2);
        Ok(())
    }

    #[test]
    fn test_array_eq_value_ne() -> Result<()> {
        let minimum_class = minimum_class()?;
        let ref1 = Reference::Array(minimum_class.clone(), ConcurrentVec::from(vec![]));
        let ref2 = Reference::Array(minimum_class, ConcurrentVec::from(vec![None]));
        assert_ne!(ref1, ref2);
        Ok(())
    }

    #[test]
    fn test_byte_array_eq() {
        let ref1 = Reference::from(vec![42i8]);
        let ref2 = Reference::from(vec![42i8]);
        assert_eq!(ref1, ref2);
    }

    #[test]
    fn test_byte_array_ne() {
        let ref1 = Reference::from(vec![3i8]);
        let ref2 = Reference::from(vec![42i8]);
        assert_ne!(ref1, ref2);
    }

    #[test]
    fn test_char_array_eq() {
        let ref1 = Reference::from(vec![42 as char]);
        let ref2 = Reference::from(vec![42 as char]);
        assert_eq!(ref1, ref2);
    }

    #[test]
    fn test_char_array_ne() {
        let ref1 = Reference::from(vec![3 as char]);
        let ref2 = Reference::from(vec![42 as char]);
        assert_ne!(ref1, ref2);
    }

    #[test]
    fn test_double_array_eq() {
        let ref1 = Reference::from(vec![42.1f64]);
        let ref2 = Reference::from(vec![42.1f64]);
        assert_eq!(ref1, ref2);
    }

    #[test]
    fn test_double_array_ne() {
        let ref1 = Reference::from(vec![3.1f64]);
        let ref2 = Reference::from(vec![42.1f64]);
        assert_ne!(ref1, ref2);
    }

    #[test]
    fn test_float_array_eq() {
        let ref1 = Reference::from(vec![42.1f32]);
        let ref2 = Reference::from(vec![42.1f32]);
        assert_eq!(ref1, ref2);
    }

    #[test]
    fn test_float_array_ne() {
        let ref1 = Reference::from(vec![3.1f32]);
        let ref2 = Reference::from(vec![42.1f32]);
        assert_ne!(ref1, ref2);
    }

    #[test]
    fn test_int_array_eq() {
        let ref1 = Reference::from(vec![42i32]);
        let ref2 = Reference::from(vec![42i32]);
        assert_eq!(ref1, ref2);
    }

    #[test]
    fn test_int_array_ne() {
        let ref1 = Reference::from(vec![3i32]);
        let ref2 = Reference::from(vec![42i32]);
        assert_ne!(ref1, ref2);
    }

    #[test]
    fn test_long_array_eq() {
        let ref1 = Reference::from(vec![42i64]);
        let ref2 = Reference::from(vec![42i64]);
        assert_eq!(ref1, ref2);
    }

    #[test]
    fn test_long_array_ne() {
        let ref1 = Reference::from(vec![3i64]);
        let ref2 = Reference::from(vec![42i64]);
        assert_ne!(ref1, ref2);
    }

    #[test]
    fn test_short_array_eq() {
        let ref1 = Reference::from(vec![42i16]);
        let ref2 = Reference::from(vec![42i16]);
        assert_eq!(ref1, ref2);
    }

    #[test]
    fn test_short_array_ne() {
        let ref1 = Reference::from(vec![3i16]);
        let ref2 = Reference::from(vec![42i16]);
        assert_ne!(ref1, ref2);
    }

    #[test]
    fn test_object_eq() -> Result<()> {
        let minimum_class = minimum_class()?;
        let ref1 = Reference::from(Object::new(minimum_class.clone())?);
        let ref2 = Reference::from(Object::new(minimum_class)?);
        assert_eq!(ref1, ref2);
        Ok(())
    }

    #[test]
    fn test_object_ne() -> Result<()> {
        let minimum_class = minimum_class()?;
        let ref1 = Reference::from(Object::new(minimum_class)?);
        let simple_class = simple_class()?;
        let ref2 = Reference::from(Object::new(simple_class)?);
        assert_ne!(ref1, ref2);
        Ok(())
    }

    #[test]
    fn test_different_types_ne() {
        let ref1 = Reference::from(vec![42i32]);
        let ref2 = Reference::from(vec![42i64]);
        assert_ne!(ref1, ref2);
    }

    #[test]
    fn test_from_vec_bool() {
        let reference = Reference::from(vec![true]);
        assert!(matches!(reference, Reference::ByteArray(_)));
    }

    #[test]
    fn test_from_vec_i8() {
        let reference = Reference::from(vec![0i8]);
        assert!(matches!(reference, Reference::ByteArray(_)));
    }

    #[test]
    fn test_from_vec_u8() {
        let reference = Reference::from(vec![0u8]);
        assert!(matches!(reference, Reference::ByteArray(_)));
    }

    #[test]
    fn test_from_vec_char() {
        let reference = Reference::from(vec!['a']);
        assert!(matches!(reference, Reference::CharArray(_)));
    }

    #[test]
    fn test_from_vec_i16() {
        let reference = Reference::from(vec![0i16]);
        assert!(matches!(reference, Reference::ShortArray(_)));
    }

    #[test]
    fn test_from_vec_u16() {
        let reference = Reference::from(vec![0u16]);
        assert!(matches!(reference, Reference::ShortArray(_)));
    }

    #[test]
    fn test_from_vec_i32() {
        let reference = Reference::from(vec![0i32]);
        assert!(matches!(reference, Reference::IntArray(_)));
    }

    #[test]
    fn test_from_vec_u32() {
        let reference = Reference::from(vec![0u32]);
        assert!(matches!(reference, Reference::IntArray(_)));
    }

    #[test]
    fn test_from_vec_i64() {
        let reference = Reference::from(vec![0i64]);
        assert!(matches!(reference, Reference::LongArray(_)));
    }

    #[test]
    fn test_from_vec_u64() {
        let reference = Reference::from(vec![0u64]);
        assert!(matches!(reference, Reference::LongArray(_)));
    }

    #[test]
    fn test_from_vec_isize() {
        let reference = Reference::from(vec![0isize]);
        assert!(matches!(reference, Reference::LongArray(_)));
    }

    #[test]
    fn test_from_vec_usize() {
        let reference = Reference::from(vec![0usize]);
        assert!(matches!(reference, Reference::LongArray(_)));
    }

    #[test]
    fn test_from_vec_f32() {
        let reference = Reference::from(vec![0.0f32]);
        assert!(matches!(reference, Reference::FloatArray(_)));
    }

    #[test]
    fn test_from_vec_f64() {
        let reference = Reference::from(vec![0.0f64]);
        assert!(matches!(reference, Reference::DoubleArray(_)));
    }

    #[test]
    fn test_from_class_vec() -> Result<()> {
        let original_class = Arc::new(Class::new_named("[Ljava/lang/Object;")?);
        let original_value = vec![None];
        let reference = Reference::from((original_class.clone(), original_value.clone()));
        assert!(matches!(reference, Reference::Array(_, _)));
        Ok(())
    }

    #[tokio::test]
    async fn test_try_from_class_vec() -> Result<()> {
        let original_class = Arc::new(Class::new_named("[Ljava/lang/Object;")?);
        let class_name = "java/lang/Integer";
        let class = load_class(class_name).await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Int(42))?;
        let value = Value::from(object);
        let original_value = vec![value];
        let reference = Reference::try_from((original_class.clone(), original_value.clone()))?;
        assert!(matches!(reference, Reference::Array(_, _)));
        Ok(())
    }

    #[tokio::test]
    async fn test_try_from_class_vec_error() -> Result<()> {
        let original_class = Arc::new(Class::new_named("[Ljava/lang/Object;")?);
        let value = Value::from(42);
        let original_value = vec![value];
        let reference = Reference::try_from((original_class.clone(), original_value.clone()));
        assert!(matches!(reference, Err(InvalidValueType(_))));
        Ok(())
    }

    #[test]
    fn test_from_vec_object() -> Result<()> {
        let class = minimum_class()?;
        let object = Object::new(class)?;
        let reference = Reference::from(object);
        assert!(matches!(reference, Reference::Object(_)));
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_vec_bool() -> Result<()> {
        let original_value = vec![true];
        let reference = Reference::from(original_value.clone());
        let value: Vec<bool> = reference.try_into()?;
        assert_eq!(original_value, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_vec_char() -> Result<()> {
        let original_value = vec!['*'];
        let reference = Reference::from(original_value.clone());
        let value: Vec<char> = reference.try_into()?;
        assert_eq!(original_value, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_vec_i8() -> Result<()> {
        let original_value = vec![42i8];
        let reference = Reference::from(original_value.clone());
        let value: Vec<i8> = reference.try_into()?;
        assert_eq!(original_value, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_vec_u8() -> Result<()> {
        let original_value = vec![42u8];
        let reference = Reference::from(original_value.clone());
        let value: Vec<u8> = reference.try_into()?;
        assert_eq!(original_value, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_vec_i16() -> Result<()> {
        let original_value = vec![42i16];
        let reference = Reference::from(original_value.clone());
        let value: Vec<i16> = reference.try_into()?;
        assert_eq!(original_value, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_vec_u16() -> Result<()> {
        let original_value = vec![42u16];
        let reference = Reference::from(original_value.clone());
        let value: Vec<u16> = reference.try_into()?;
        assert_eq!(original_value, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_vec_i32() -> Result<()> {
        let original_value = vec![42i32];
        let reference = Reference::from(original_value.clone());
        let value: Vec<i32> = reference.try_into()?;
        assert_eq!(original_value, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_vec_u32() -> Result<()> {
        let original_value = vec![42u32];
        let reference = Reference::from(original_value.clone());
        let value: Vec<u32> = reference.try_into()?;
        assert_eq!(original_value, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_vec_i64() -> Result<()> {
        let original_value = vec![42i64];
        let reference = Reference::from(original_value.clone());
        let value: Vec<i64> = reference.try_into()?;
        assert_eq!(original_value, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_vec_u64() -> Result<()> {
        let original_value = vec![42u64];
        let reference = Reference::from(original_value.clone());
        let value: Vec<u64> = reference.try_into()?;
        assert_eq!(original_value, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_vec_isize() -> Result<()> {
        let original_value = vec![42isize];
        let reference = Reference::from(original_value.clone());
        let value: Vec<isize> = reference.try_into()?;
        assert_eq!(original_value, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_vec_usize() -> Result<()> {
        let original_value = vec![42usize];
        let reference = Reference::from(original_value.clone());
        let value: Vec<usize> = reference.try_into()?;
        assert_eq!(original_value, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_vec_f32() -> Result<()> {
        let original_value = vec![42.1f32];
        let reference = Reference::from(original_value.clone());
        let value: Vec<f32> = reference.try_into()?;
        assert_eq!(original_value, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_vec_f64() -> Result<()> {
        let original_value = vec![42.1f64];
        let reference = Reference::from(original_value.clone());
        let value: Vec<f64> = reference.try_into()?;
        assert_eq!(original_value, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_bool() -> Result<()> {
        let class = load_class("java/lang/Boolean").await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Int(1))?;
        let reference = Reference::from(object);
        let value: bool = reference.try_into()?;
        assert!(value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_char() -> Result<()> {
        let class = load_class("java/lang/Character").await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Int(42))?;
        let reference = Reference::from(object);
        let value: char = reference.try_into()?;
        assert_eq!('*', value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_i8() -> Result<()> {
        let class = load_class("java/lang/Byte").await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Int(42))?;
        let reference = Reference::from(object);
        let value: i8 = reference.try_into()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_u8() -> Result<()> {
        let class = load_class("java/lang/Byte").await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Int(42))?;
        let reference = Reference::from(object);
        let value: u8 = reference.try_into()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_i16() -> Result<()> {
        let class = load_class("java/lang/Short").await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Int(42))?;
        let reference = Reference::from(object);
        let value: i16 = reference.try_into()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_u16() -> Result<()> {
        let class = load_class("java/lang/Short").await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Int(42))?;
        let reference = Reference::from(object);
        let value: u16 = reference.try_into()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_i32() -> Result<()> {
        let class = load_class("java/lang/Integer").await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Int(42))?;
        let reference = Reference::from(object);
        let value: i32 = reference.try_into()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_u32() -> Result<()> {
        let class = load_class("java/lang/Integer").await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Int(42))?;
        let reference = Reference::from(object);
        let value: u32 = reference.try_into()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_i64() -> Result<()> {
        let class = load_class("java/lang/Long").await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Long(42))?;
        let reference = Reference::from(object);
        let value: i64 = reference.try_into()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_u64() -> Result<()> {
        let class = load_class("java/lang/Long").await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Long(42))?;
        let reference = Reference::from(object);
        let value: u64 = reference.try_into()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_isize() -> Result<()> {
        let class = load_class("java/lang/Long").await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Long(42))?;
        let reference = Reference::from(object);
        let value: isize = reference.try_into()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_usize() -> Result<()> {
        let class = load_class("java/lang/Long").await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Long(42))?;
        let reference = Reference::from(object);
        let value: usize = reference.try_into()?;
        assert_eq!(42, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_f32() -> Result<()> {
        let class = load_class("java/lang/Float").await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Float(42.1))?;
        let reference = Reference::from(object);
        let value: f32 = reference.try_into()?;
        let value = value - 42.1f32;
        assert!(value.abs() < 0.1f32);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_f64() -> Result<()> {
        let class = load_class("java/lang/Double").await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Double(42.1))?;
        let reference = Reference::from(object);
        let value: f64 = reference.try_into()?;
        let value = value - 42.1f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[tokio::test]
    async fn test_try_into_object() -> Result<()> {
        let class_name = "java.lang.Object";
        let class = load_class(class_name).await?;
        let reference = Reference::from(Object::new(class)?);
        let object: Object = reference.try_into()?;
        assert_eq!("java/lang/Object", object.class().name());
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

    #[tokio::test]
    async fn test_try_into_class() -> Result<()> {
        let class_name = "java.lang.Integer";
        let class = load_class(class_name).await?;
        let object = Object::new(class)?;
        object.set_value("value", Value::Int(42))?;
        let reference = Reference::from(object);
        let value: Arc<Class> = reference.try_into()?;
        assert_eq!("java/lang/Integer", value.name());
        Ok(())
    }
}
