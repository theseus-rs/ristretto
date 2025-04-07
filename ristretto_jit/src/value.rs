#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Value::I32(value)
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::I64(value)
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Value::F32(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::F64(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_i32() {
        let value: Value = 42i32.into();
        assert_eq!(value, Value::I32(42));
    }

    #[test]
    fn test_from_i64() {
        let value: Value = 42i64.into();
        assert_eq!(value, Value::I64(42));
    }

    #[test]
    fn test_from_f32() {
        let value: Value = 42.1f32.into();
        assert_eq!(value, Value::F32(42.1));
    }

    #[test]
    fn test_from_f64() {
        let value: Value = 42.1f64.into();
        assert_eq!(value, Value::F64(42.1));
    }
}
