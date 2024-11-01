use crate::concurrent_vec::ConcurrentVec;
use crate::{Class, Object, Result};
use ristretto_classfile::{ClassFile, ConstantPool};
use std::fmt;
use std::fmt::Display;
use std::sync::Arc;

/// Represents a reference to an object in the Ristretto VM.
#[derive(Clone, Debug)]
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
                if value.class().name() == "java/lang/String" {
                    let string = value.as_string().unwrap_or_default();
                    write!(f, "string({string})")
                } else {
                    write!(f, "{value}")
                }
            }
        }
    }
}

impl PartialEq for Reference {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Reference::ByteArray(a), Reference::ByteArray(b)) => a == b,
            (Reference::CharArray(a), Reference::CharArray(b)) => a == b,
            (Reference::ShortArray(a), Reference::ShortArray(b)) => a == b,
            (Reference::IntArray(a), Reference::IntArray(b)) => a == b,
            (Reference::LongArray(a), Reference::LongArray(b)) => a == b,
            (Reference::FloatArray(a), Reference::FloatArray(b)) => a == b,
            (Reference::DoubleArray(a), Reference::DoubleArray(b)) => a == b,
            (Reference::Array(a_class, a_array), Reference::Array(b_class, b_array)) => {
                a_class.name() == b_class.name() && a_array == b_array
            }
            (Reference::Object(a), Reference::Object(b)) => {
                // Compare the references by pointer to determine if they are the same object in
                // order to avoid infinite recursion
                if std::ptr::eq(a, b) {
                    true
                } else {
                    a == b
                }
            }
            _ => false,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Class, Result};
    use ristretto_classfile::ClassFile;
    use std::io::Cursor;
    use std::sync::Arc;

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
    fn test_display_char_array() -> Result<()> {
        let reference = Reference::from(vec![1 as char, 2 as char, 3 as char]);
        assert_eq!(reference.class_name(), "[C");
        assert_eq!(reference.class()?.name(), "[C");
        assert_eq!(reference.to_string(), "char[1, 2, 3]");
        Ok(())
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
    fn test_display_int_array() -> Result<()> {
        let reference = Reference::from(vec![1i32, 2i32, 3i32]);
        assert_eq!(reference.class_name(), "[I");
        assert_eq!(reference.class()?.name(), "[I");
        assert_eq!(reference.to_string(), "int[1, 2, 3]");
        Ok(())
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
    fn test_display_float_array() -> Result<()> {
        let reference = Reference::from(vec![1.0f32, 2.0f32, 3.0f32]);
        assert_eq!(reference.class_name(), "[F");
        assert_eq!(reference.class()?.name(), "[F");
        assert_eq!(reference.to_string(), "float[1.0, 2.0, 3.0]");
        Ok(())
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
    fn test_display_reference_array() -> Result<()> {
        let class = Arc::new(Class::new_array("[Ljava/lang/Object;")?);
        let reference = Reference::Array(class, ConcurrentVec::from(vec![None]));
        assert_eq!(reference.class_name(), "[Ljava/lang/Object;");
        assert_eq!(reference.class()?.name(), "[Ljava/lang/Object;");
        assert_eq!(reference.to_string(), "java/lang/Object[1]");
        Ok(())
    }

    #[tokio::test]
    async fn test_display_reference() -> Result<()> {
        let class = minimum_class()?;
        let object = Object::new(class)?;
        let reference = Reference::Object(object);
        assert_eq!(reference.class_name(), "Minimum");
        assert_eq!(reference.class()?.name(), "Minimum");
        assert!(reference.to_string().starts_with("class Minimum"));
        assert!(reference.to_string().contains("Minimum"));
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
        let ref1 = Reference::Object(Object::new(minimum_class.clone())?);
        let ref2 = Reference::Object(Object::new(minimum_class)?);
        assert_eq!(ref1, ref2);
        Ok(())
    }

    #[test]
    fn test_object_ne() -> Result<()> {
        let minimum_class = minimum_class()?;
        let ref1 = Reference::Object(Object::new(minimum_class)?);
        let simple_class = simple_class()?;
        let ref2 = Reference::Object(Object::new(simple_class)?);
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
    fn test_from_bool() {
        let reference = Reference::from(vec![true]);
        assert!(matches!(reference, Reference::ByteArray(_)));
    }

    #[test]
    fn test_from_i8() {
        let reference = Reference::from(vec![0i8]);
        assert!(matches!(reference, Reference::ByteArray(_)));
    }

    #[test]
    fn test_from_u8() {
        let reference = Reference::from(vec![0u8]);
        assert!(matches!(reference, Reference::ByteArray(_)));
    }

    #[test]
    fn test_from_char() {
        let reference = Reference::from(vec!['a']);
        assert!(matches!(reference, Reference::CharArray(_)));
    }

    #[test]
    fn test_from_i16() {
        let reference = Reference::from(vec![0i16]);
        assert!(matches!(reference, Reference::ShortArray(_)));
    }

    #[test]
    fn test_from_u16() {
        let reference = Reference::from(vec![0u16]);
        assert!(matches!(reference, Reference::ShortArray(_)));
    }

    #[test]
    fn test_from_i32() {
        let reference = Reference::from(vec![0i32]);
        assert!(matches!(reference, Reference::IntArray(_)));
    }

    #[test]
    fn test_from_u32() {
        let reference = Reference::from(vec![0u32]);
        assert!(matches!(reference, Reference::IntArray(_)));
    }

    #[test]
    fn test_from_i64() {
        let reference = Reference::from(vec![0i64]);
        assert!(matches!(reference, Reference::LongArray(_)));
    }

    #[test]
    fn test_from_u64() {
        let reference = Reference::from(vec![0u64]);
        assert!(matches!(reference, Reference::LongArray(_)));
    }

    #[test]
    fn test_from_isize() {
        let reference = Reference::from(vec![0isize]);
        assert!(matches!(reference, Reference::LongArray(_)));
    }

    #[test]
    fn test_from_usize() {
        let reference = Reference::from(vec![0usize]);
        assert!(matches!(reference, Reference::LongArray(_)));
    }

    #[test]
    fn test_from_f32() {
        let reference = Reference::from(vec![0.0f32]);
        assert!(matches!(reference, Reference::FloatArray(_)));
    }

    #[test]
    fn test_from_f64() {
        let reference = Reference::from(vec![0.0f64]);
        assert!(matches!(reference, Reference::DoubleArray(_)));
    }
}
