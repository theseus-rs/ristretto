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
        let reference = Reference::ByteArray(ConcurrentVec::from(vec![1, 2, 3]));
        assert_eq!(reference.class_name(), "[B");
        assert_eq!(reference.class()?.name(), "[B");
        assert_eq!(reference.to_string(), "byte[1, 2, 3]");
        Ok(())
    }

    #[test]
    fn test_display_char_array() -> Result<()> {
        let reference = Reference::CharArray(ConcurrentVec::from(vec![1, 2, 3]));
        assert_eq!(reference.class_name(), "[C");
        assert_eq!(reference.class()?.name(), "[C");
        assert_eq!(reference.to_string(), "char[1, 2, 3]");
        Ok(())
    }

    #[test]
    fn test_display_short_array() -> Result<()> {
        let reference = Reference::ShortArray(ConcurrentVec::from(vec![1, 2, 3]));
        assert_eq!(reference.class_name(), "[S");
        assert_eq!(reference.class()?.name(), "[S");
        assert_eq!(reference.to_string(), "short[1, 2, 3]");
        Ok(())
    }

    #[test]
    fn test_display_int_array() -> Result<()> {
        let reference = Reference::IntArray(ConcurrentVec::from(vec![1, 2, 3]));
        assert_eq!(reference.class_name(), "[I");
        assert_eq!(reference.class()?.name(), "[I");
        assert_eq!(reference.to_string(), "int[1, 2, 3]");
        Ok(())
    }

    #[test]
    fn test_display_long_array() -> Result<()> {
        let reference = Reference::LongArray(ConcurrentVec::from(vec![1, 2, 3]));
        assert_eq!(reference.class_name(), "[J");
        assert_eq!(reference.class()?.name(), "[J");
        assert_eq!(reference.to_string(), "long[1, 2, 3]");
        Ok(())
    }

    #[test]
    fn test_display_float_array() -> Result<()> {
        let reference = Reference::FloatArray(ConcurrentVec::from(vec![1.0, 2.0, 3.0]));
        assert_eq!(reference.class_name(), "[F");
        assert_eq!(reference.class()?.name(), "[F");
        assert_eq!(reference.to_string(), "float[1.0, 2.0, 3.0]");
        Ok(())
    }

    #[test]
    fn test_display_double_array() -> Result<()> {
        let reference = Reference::DoubleArray(ConcurrentVec::from(vec![1.0, 2.0, 3.0]));
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
        let ref1 = Reference::ByteArray(ConcurrentVec::from(vec![42]));
        let ref2 = Reference::ByteArray(ConcurrentVec::from(vec![42]));
        assert_eq!(ref1, ref2);
    }

    #[test]
    fn test_byte_array_ne() {
        let ref1 = Reference::ByteArray(ConcurrentVec::from(vec![3]));
        let ref2 = Reference::ByteArray(ConcurrentVec::from(vec![42]));
        assert_ne!(ref1, ref2);
    }

    #[test]
    fn test_char_array_eq() {
        let ref1 = Reference::CharArray(ConcurrentVec::from(vec![42]));
        let ref2 = Reference::CharArray(ConcurrentVec::from(vec![42]));
        assert_eq!(ref1, ref2);
    }

    #[test]
    fn test_char_array_ne() {
        let ref1 = Reference::CharArray(ConcurrentVec::from(vec![3]));
        let ref2 = Reference::CharArray(ConcurrentVec::from(vec![42]));
        assert_ne!(ref1, ref2);
    }

    #[test]
    fn test_double_array_eq() {
        let ref1 = Reference::DoubleArray(ConcurrentVec::from(vec![42.1]));
        let ref2 = Reference::DoubleArray(ConcurrentVec::from(vec![42.1]));
        assert_eq!(ref1, ref2);
    }

    #[test]
    fn test_double_array_ne() {
        let ref1 = Reference::DoubleArray(ConcurrentVec::from(vec![3.1]));
        let ref2 = Reference::DoubleArray(ConcurrentVec::from(vec![42.1]));
        assert_ne!(ref1, ref2);
    }

    #[test]
    fn test_float_array_eq() {
        let ref1 = Reference::FloatArray(ConcurrentVec::from(vec![42.1]));
        let ref2 = Reference::FloatArray(ConcurrentVec::from(vec![42.1]));
        assert_eq!(ref1, ref2);
    }

    #[test]
    fn test_float_array_ne() {
        let ref1 = Reference::FloatArray(ConcurrentVec::from(vec![3.1]));
        let ref2 = Reference::FloatArray(ConcurrentVec::from(vec![42.1]));
        assert_ne!(ref1, ref2);
    }

    #[test]
    fn test_int_array_eq() {
        let ref1 = Reference::IntArray(ConcurrentVec::from(vec![42]));
        let ref2 = Reference::IntArray(ConcurrentVec::from(vec![42]));
        assert_eq!(ref1, ref2);
    }

    #[test]
    fn test_int_array_ne() {
        let ref1 = Reference::IntArray(ConcurrentVec::from(vec![3]));
        let ref2 = Reference::IntArray(ConcurrentVec::from(vec![42]));
        assert_ne!(ref1, ref2);
    }

    #[test]
    fn test_long_array_eq() {
        let ref1 = Reference::LongArray(ConcurrentVec::from(vec![42]));
        let ref2 = Reference::LongArray(ConcurrentVec::from(vec![42]));
        assert_eq!(ref1, ref2);
    }

    #[test]
    fn test_long_array_ne() {
        let ref1 = Reference::LongArray(ConcurrentVec::from(vec![3]));
        let ref2 = Reference::LongArray(ConcurrentVec::from(vec![42]));
        assert_ne!(ref1, ref2);
    }

    #[test]
    fn test_short_array_eq() {
        let ref1 = Reference::ShortArray(ConcurrentVec::from(vec![42]));
        let ref2 = Reference::ShortArray(ConcurrentVec::from(vec![42]));
        assert_eq!(ref1, ref2);
    }

    #[test]
    fn test_short_array_ne() {
        let ref1 = Reference::ShortArray(ConcurrentVec::from(vec![3]));
        let ref2 = Reference::ShortArray(ConcurrentVec::from(vec![42]));
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
        let ref1 = Reference::IntArray(ConcurrentVec::from(vec![42]));
        let ref2 = Reference::LongArray(ConcurrentVec::from(vec![42]));
        assert_ne!(ref1, ref2);
    }
}
