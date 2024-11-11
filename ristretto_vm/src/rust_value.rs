use crate::Value;

/// Trait for converting Rust values to `Value`.
pub trait RustValue {
    fn to_value(&self) -> Value;
}

impl RustValue for bool {
    fn to_value(&self) -> Value {
        Value::from(*self)
    }
}

impl RustValue for char {
    fn to_value(&self) -> Value {
        Value::from(*self)
    }
}

impl RustValue for i8 {
    fn to_value(&self) -> Value {
        Value::from(*self)
    }
}

impl RustValue for u8 {
    fn to_value(&self) -> Value {
        Value::from(*self)
    }
}

impl RustValue for i16 {
    fn to_value(&self) -> Value {
        Value::from(*self)
    }
}

impl RustValue for u16 {
    fn to_value(&self) -> Value {
        Value::from(*self)
    }
}

impl RustValue for i32 {
    fn to_value(&self) -> Value {
        Value::from(*self)
    }
}

impl RustValue for u32 {
    fn to_value(&self) -> Value {
        Value::from(*self)
    }
}

impl RustValue for i64 {
    fn to_value(&self) -> Value {
        Value::from(*self)
    }
}

impl RustValue for u64 {
    fn to_value(&self) -> Value {
        Value::from(*self)
    }
}

impl RustValue for isize {
    fn to_value(&self) -> Value {
        Value::from(*self)
    }
}

impl RustValue for usize {
    fn to_value(&self) -> Value {
        Value::from(*self)
    }
}

impl RustValue for f32 {
    fn to_value(&self) -> Value {
        Value::from(*self)
    }
}

impl RustValue for f64 {
    fn to_value(&self) -> Value {
        Value::from(*self)
    }
}

impl RustValue for Value {
    fn to_value(&self) -> Value {
        self.clone()
    }
}

impl RustValue for Vec<bool> {
    fn to_value(&self) -> Value {
        Value::from(self.clone())
    }
}

impl RustValue for Vec<char> {
    fn to_value(&self) -> Value {
        Value::from(self.clone())
    }
}

impl RustValue for Vec<i8> {
    fn to_value(&self) -> Value {
        Value::from(self.clone())
    }
}

impl RustValue for Vec<u8> {
    fn to_value(&self) -> Value {
        Value::from(self.clone())
    }
}

impl RustValue for Vec<i16> {
    fn to_value(&self) -> Value {
        Value::from(self.clone())
    }
}

impl RustValue for Vec<u16> {
    fn to_value(&self) -> Value {
        Value::from(self.clone())
    }
}

impl RustValue for Vec<i32> {
    fn to_value(&self) -> Value {
        Value::from(self.clone())
    }
}

impl RustValue for Vec<u32> {
    fn to_value(&self) -> Value {
        Value::from(self.clone())
    }
}

impl RustValue for Vec<i64> {
    fn to_value(&self) -> Value {
        Value::from(self.clone())
    }
}

impl RustValue for Vec<u64> {
    fn to_value(&self) -> Value {
        Value::from(self.clone())
    }
}

impl RustValue for Vec<isize> {
    fn to_value(&self) -> Value {
        Value::from(self.clone())
    }
}

impl RustValue for Vec<usize> {
    fn to_value(&self) -> Value {
        Value::from(self.clone())
    }
}

impl RustValue for Vec<f32> {
    fn to_value(&self) -> Value {
        Value::from(self.clone())
    }
}

impl RustValue for Vec<f64> {
    fn to_value(&self) -> Value {
        Value::from(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bool() {
        assert_eq!(true.to_value(), Value::from(true));
        assert_eq!(false.to_value(), Value::from(false));
    }

    #[test]
    fn test_char() {
        assert_eq!('a'.to_value(), Value::from('a'));
    }

    #[test]
    fn test_i8() {
        assert_eq!(42i8.to_value(), Value::from(42i8));
    }

    #[test]
    fn test_u8() {
        assert_eq!(42u8.to_value(), Value::from(42u8));
    }

    #[test]
    fn test_i16() {
        assert_eq!(42i16.to_value(), Value::from(42i16));
    }

    #[test]
    fn test_u16() {
        assert_eq!(42u16.to_value(), Value::from(42u16));
    }

    #[test]
    fn test_i32() {
        assert_eq!(42i32.to_value(), Value::from(42i32));
    }

    #[test]
    fn test_u32() {
        assert_eq!(42u32.to_value(), Value::from(42u32));
    }

    #[test]
    fn test_i64() {
        assert_eq!(42i64.to_value(), Value::from(42i64));
    }

    #[test]
    fn test_u64() {
        assert_eq!(42u64.to_value(), Value::from(42u64));
    }

    #[test]
    fn test_isize() {
        assert_eq!(42isize.to_value(), Value::from(42isize));
    }

    #[test]
    fn test_usize() {
        assert_eq!(42usize.to_value(), Value::from(42usize));
    }

    #[test]
    fn test_f32() {
        assert_eq!(42.1f32.to_value(), Value::from(42.1f32));
    }

    #[test]
    fn test_f64() {
        assert_eq!(42.1f64.to_value(), Value::from(42.1f64));
    }

    #[test]
    fn test_value() {
        let value = Value::from(42);
        assert_eq!(value.to_value(), value);
    }

    #[test]
    fn test_vec_bool() {
        assert_eq!(vec![true, false].to_value(), Value::from(vec![true, false]));
    }

    #[test]
    fn test_vec_char() {
        assert_eq!(vec!['a', 'b'].to_value(), Value::from(vec!['a', 'b']));
    }

    #[test]
    fn test_vec_i8() {
        assert_eq!(vec![42i8, 43i8].to_value(), Value::from(vec![42i8, 43i8]));
    }

    #[test]
    fn test_vec_u8() {
        assert_eq!(vec![42u8, 43u8].to_value(), Value::from(vec![42u8, 43u8]));
    }

    #[test]
    fn test_vec_i16() {
        assert_eq!(
            vec![42i16, 43i16].to_value(),
            Value::from(vec![42i16, 43i16])
        );
    }

    #[test]
    fn test_vec_u16() {
        assert_eq!(
            vec![42u16, 43u16].to_value(),
            Value::from(vec![42u16, 43u16])
        );
    }

    #[test]
    fn test_vec_i32() {
        assert_eq!(
            vec![42i32, 43i32].to_value(),
            Value::from(vec![42i32, 43i32])
        );
    }

    #[test]
    fn test_vec_u32() {
        assert_eq!(
            vec![42u32, 43u32].to_value(),
            Value::from(vec![42u32, 43u32])
        );
    }

    #[test]
    fn test_vec_i64() {
        assert_eq!(
            vec![42i64, 43i64].to_value(),
            Value::from(vec![42i64, 43i64])
        );
    }

    #[test]
    fn test_vec_u64() {
        assert_eq!(
            vec![42u64, 43u64].to_value(),
            Value::from(vec![42u64, 43u64])
        );
    }

    #[test]
    fn test_vec_isize() {
        assert_eq!(
            vec![42isize, 43isize].to_value(),
            Value::from(vec![42isize, 43isize])
        );
    }

    #[test]
    fn test_vec_usize() {
        assert_eq!(
            vec![42usize, 43usize].to_value(),
            Value::from(vec![42usize, 43usize])
        );
    }

    #[test]
    fn test_vec_f32() {
        assert_eq!(
            vec![42.1f32, 43.1f32].to_value(),
            Value::from(vec![42.1f32, 43.1f32])
        );
    }

    #[test]
    fn test_vec_f64() {
        assert_eq!(
            vec![42.1f64, 43.1f64].to_value(),
            Value::from(vec![42.1f64, 43.1f64])
        );
    }
}
