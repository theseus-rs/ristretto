use ristretto_classloader::{Reference, Value};
use ristretto_gc::GarbageCollector;
use std::fmt::Debug;

/// Trait for converting Rust values to `Value`.
pub trait RustValue: Debug {
    fn to_value(&self, collector: &GarbageCollector) -> Value;
}

impl RustValue for bool {
    fn to_value(&self, _collector: &GarbageCollector) -> Value {
        Value::from(*self)
    }
}

impl RustValue for char {
    fn to_value(&self, _collector: &GarbageCollector) -> Value {
        Value::from(*self)
    }
}

impl RustValue for i8 {
    fn to_value(&self, _collector: &GarbageCollector) -> Value {
        Value::from(*self)
    }
}

impl RustValue for u8 {
    fn to_value(&self, _collector: &GarbageCollector) -> Value {
        Value::from(*self)
    }
}

impl RustValue for i16 {
    fn to_value(&self, _collector: &GarbageCollector) -> Value {
        Value::from(*self)
    }
}

impl RustValue for u16 {
    fn to_value(&self, _collector: &GarbageCollector) -> Value {
        Value::from(*self)
    }
}

impl RustValue for i32 {
    fn to_value(&self, _collector: &GarbageCollector) -> Value {
        Value::from(*self)
    }
}

impl RustValue for u32 {
    fn to_value(&self, _collector: &GarbageCollector) -> Value {
        Value::from(*self)
    }
}

impl RustValue for i64 {
    fn to_value(&self, _collector: &GarbageCollector) -> Value {
        Value::from(*self)
    }
}

impl RustValue for u64 {
    fn to_value(&self, _collector: &GarbageCollector) -> Value {
        Value::from(*self)
    }
}

impl RustValue for isize {
    fn to_value(&self, _collector: &GarbageCollector) -> Value {
        Value::from(*self)
    }
}

impl RustValue for usize {
    fn to_value(&self, _collector: &GarbageCollector) -> Value {
        Value::from(*self)
    }
}

impl RustValue for f32 {
    fn to_value(&self, _collector: &GarbageCollector) -> Value {
        Value::from(*self)
    }
}

impl RustValue for f64 {
    fn to_value(&self, _collector: &GarbageCollector) -> Value {
        Value::from(*self)
    }
}

impl RustValue for &str {
    fn to_value(&self, collector: &GarbageCollector) -> Value {
        use ristretto_classfile::{ClassFile, ConstantPool};
        use ristretto_classloader::{Class, Object};

        const STRING_PREFIX: &str = "str:";
        let class_name = format!("{STRING_PREFIX}{self}");
        let mut constant_pool = ConstantPool::new();
        let Ok(class_index) = constant_pool.add_class(class_name) else {
            return Value::Object(None);
        };
        let class_file = ClassFile {
            constant_pool,
            this_class: class_index,
            ..Default::default()
        };
        let Ok(class) = Class::from(None, class_file) else {
            return Value::Object(None);
        };
        let Ok(object) = Object::new(class) else {
            return Value::Object(None);
        };
        Value::from_object(collector, object)
    }
}

impl RustValue for String {
    fn to_value(&self, collector: &GarbageCollector) -> Value {
        self.as_str().to_value(collector)
    }
}

impl RustValue for Value {
    fn to_value(&self, _collector: &GarbageCollector) -> Value {
        self.clone()
    }
}

impl RustValue for Vec<bool> {
    fn to_value(&self, collector: &GarbageCollector) -> Value {
        Value::new_object(collector, Reference::from(self.clone()))
    }
}

impl RustValue for Vec<char> {
    fn to_value(&self, collector: &GarbageCollector) -> Value {
        Value::new_object(collector, Reference::from(self.clone()))
    }
}

impl RustValue for Vec<i8> {
    fn to_value(&self, collector: &GarbageCollector) -> Value {
        Value::new_object(collector, Reference::from(self.clone()))
    }
}

impl RustValue for Vec<u8> {
    fn to_value(&self, collector: &GarbageCollector) -> Value {
        let values: &[i8] = zerocopy::transmute_ref!(self.as_slice());
        Value::new_object(collector, Reference::from(values.to_vec()))
    }
}

impl RustValue for Vec<i16> {
    fn to_value(&self, collector: &GarbageCollector) -> Value {
        Value::new_object(collector, Reference::from(self.clone()))
    }
}

impl RustValue for Vec<u16> {
    fn to_value(&self, collector: &GarbageCollector) -> Value {
        let values: Vec<i64> = self.iter().map(|&x| i64::from(x)).collect();

        Value::new_object(collector, Reference::from(values))
    }
}

impl RustValue for Vec<i32> {
    fn to_value(&self, collector: &GarbageCollector) -> Value {
        Value::new_object(collector, Reference::from(self.clone()))
    }
}

impl RustValue for Vec<u32> {
    fn to_value(&self, collector: &GarbageCollector) -> Value {
        let values: Vec<i64> = self.iter().map(|&x| i64::from(x)).collect();
        Value::new_object(collector, Reference::from(values))
    }
}

impl RustValue for Vec<i64> {
    fn to_value(&self, collector: &GarbageCollector) -> Value {
        Value::new_object(collector, Reference::from(self.clone()))
    }
}

impl RustValue for Vec<u64> {
    fn to_value(&self, collector: &GarbageCollector) -> Value {
        let values: &[i64] = zerocopy::transmute_ref!(self.as_slice());
        Value::new_object(collector, Reference::from(values.to_vec()))
    }
}

impl RustValue for Vec<isize> {
    fn to_value(&self, collector: &GarbageCollector) -> Value {
        let values: Vec<i64> = self.iter().map(|&x| x as i64).collect();
        Value::new_object(collector, Reference::from(values))
    }
}

impl RustValue for Vec<usize> {
    #[expect(clippy::cast_possible_wrap)]
    fn to_value(&self, collector: &GarbageCollector) -> Value {
        let values: Vec<i64> = self.iter().map(|&x| x as i64).collect();
        Value::new_object(collector, Reference::from(values))
    }
}

impl RustValue for Vec<f32> {
    fn to_value(&self, collector: &GarbageCollector) -> Value {
        Value::new_object(collector, Reference::from(self.clone()))
    }
}

impl RustValue for Vec<f64> {
    fn to_value(&self, collector: &GarbageCollector) -> Value {
        Value::new_object(collector, Reference::from(self.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ristretto_classloader::Reference;

    #[test]
    fn test_bool() {
        let collector = GarbageCollector::new();
        assert_eq!(true.to_value(&collector), Value::from(true));
        assert_eq!(false.to_value(&collector), Value::from(false));
    }

    #[test]
    fn test_char() {
        let collector = GarbageCollector::new();
        assert_eq!('a'.to_value(&collector), Value::from('a'));
    }

    #[test]
    fn test_i8() {
        let collector = GarbageCollector::new();
        assert_eq!(42i8.to_value(&collector), Value::from(42i8));
    }

    #[test]
    fn test_u8() {
        let collector = GarbageCollector::new();
        assert_eq!(42u8.to_value(&collector), Value::from(42u8));
    }

    #[test]
    fn test_i16() {
        let collector = GarbageCollector::new();
        assert_eq!(42i16.to_value(&collector), Value::from(42i16));
    }

    #[test]
    fn test_u16() {
        let collector = GarbageCollector::new();
        assert_eq!(42u16.to_value(&collector), Value::from(42u16));
    }

    #[test]
    fn test_i32() {
        let collector = GarbageCollector::new();
        assert_eq!(42i32.to_value(&collector), Value::from(42i32));
    }

    #[test]
    fn test_u32() {
        let collector = GarbageCollector::new();
        assert_eq!(42u32.to_value(&collector), Value::from(42u32));
    }

    #[test]
    fn test_i64() {
        let collector = GarbageCollector::new();
        assert_eq!(42i64.to_value(&collector), Value::from(42i64));
    }

    #[test]
    fn test_u64() {
        let collector = GarbageCollector::new();
        assert_eq!(42u64.to_value(&collector), Value::from(42u64));
    }

    #[test]
    fn test_isize() {
        let collector = GarbageCollector::new();
        assert_eq!(42isize.to_value(&collector), Value::from(42isize));
    }

    #[test]
    fn test_usize() {
        let collector = GarbageCollector::new();
        assert_eq!(42usize.to_value(&collector), Value::from(42usize));
    }

    #[test]
    fn test_f32() {
        let collector = GarbageCollector::new();
        assert_eq!(42.1f32.to_value(&collector), Value::from(42.1f32));
    }

    #[test]
    fn test_f64() {
        let collector = GarbageCollector::new();
        assert_eq!(42.1f64.to_value(&collector), Value::from(42.1f64));
    }

    #[test]
    fn test_str() {
        let collector = GarbageCollector::new();
        let value = "foo".to_value(&collector);
        let object = value.as_object_ref().expect("object ref");
        let class_name = object.class().name();
        assert_eq!("str:foo", class_name);
    }

    #[test]
    fn test_string() {
        let collector = GarbageCollector::new();
        let value = "foo".to_string().to_value(&collector);
        let object = value.as_object_ref().expect("object ref");
        let class_name = object.class().name();
        assert_eq!("str:foo", class_name);
    }

    #[test]
    fn test_value() {
        let collector = GarbageCollector::new();
        let value = Value::from(42);
        assert_eq!(value.to_value(&collector), value);
    }

    #[test]
    fn test_vec_bool() {
        let collector = GarbageCollector::new();
        let value = vec![true, false].to_value(&collector);
        let bytes = value.as_bytes().expect("bytes");
        assert_eq!(bytes.len(), 2);
    }

    #[test]
    fn test_vec_char() {
        let collector = GarbageCollector::new();
        let value = vec!['a', 'b'].to_value(&collector);
        let chars = value.as_char_vec_ref().expect("chars");
        assert_eq!(chars.len(), 2);
    }

    #[test]
    fn test_vec_i8() {
        let collector = GarbageCollector::new();
        let value = vec![42i8, 43i8].to_value(&collector);
        let bytes = value.as_bytes().expect("bytes");
        assert_eq!(bytes.len(), 2);
    }

    #[test]
    fn test_vec_u8() {
        let collector = GarbageCollector::new();
        let value = vec![42u8, 43u8].to_value(&collector);
        let bytes = value.as_bytes().expect("bytes");
        assert_eq!(bytes.len(), 2);
    }

    #[test]
    fn test_vec_i16() {
        let collector = GarbageCollector::new();
        let value = vec![42i16, 43i16].to_value(&collector);
        let reference = value.as_reference().expect("reference");
        assert!(matches!(&*reference, Reference::ShortArray(_)));
    }

    #[test]
    fn test_vec_u16() {
        let collector = GarbageCollector::new();
        let value = vec![42u16, 43u16].to_value(&collector);
        // mapped to long array
        let reference = value.as_reference().expect("reference");
        assert!(matches!(&*reference, Reference::LongArray(_)));
    }

    #[test]
    fn test_vec_i32() {
        let collector = GarbageCollector::new();
        let value = vec![42i32, 43i32].to_value(&collector);
        let ints = value.as_int_vec_ref().expect("ints");
        assert_eq!(ints.len(), 2);
    }

    #[test]
    fn test_vec_u32() {
        let collector = GarbageCollector::new();
        let value = vec![42u32, 43u32].to_value(&collector);
        // mapped to long array
        let longs = value.as_long_vec_ref().expect("longs");
        assert_eq!(longs.len(), 2);
    }

    #[test]
    fn test_vec_i64() {
        let collector = GarbageCollector::new();
        let value = vec![42i64, 43i64].to_value(&collector);
        let longs = value.as_long_vec_ref().expect("longs");
        assert_eq!(longs.len(), 2);
    }

    #[test]
    fn test_vec_u64() {
        let collector = GarbageCollector::new();
        let value = vec![42u64, 43u64].to_value(&collector);
        let longs = value.as_long_vec_ref().expect("longs");
        assert_eq!(longs.len(), 2);
    }

    #[test]
    fn test_vec_isize() {
        let collector = GarbageCollector::new();
        let value = vec![42isize, 43isize].to_value(&collector);
        let longs = value.as_long_vec_ref().expect("longs");
        assert_eq!(longs.len(), 2);
    }

    #[test]
    fn test_vec_usize() {
        let collector = GarbageCollector::new();
        let value = vec![42usize, 43usize].to_value(&collector);
        let longs = value.as_long_vec_ref().expect("longs");
        assert_eq!(longs.len(), 2);
    }

    #[test]
    fn test_vec_f32() {
        let collector = GarbageCollector::new();
        let value = vec![42.1f32, 43.1f32].to_value(&collector);
        let floats = value.as_float_vec_ref().expect("floats");
        assert_eq!(floats.len(), 2);
    }

    #[test]
    fn test_vec_f64() {
        let collector = GarbageCollector::new();
        let value = vec![42.1f64, 43.1f64].to_value(&collector);
        let doubles = value.as_double_vec_ref().expect("doubles");
        assert_eq!(doubles.len(), 2);
    }
}
