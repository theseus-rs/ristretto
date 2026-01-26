use crate::Error;
use crate::Error::{InternalError, InvalidValue};

pub(crate) const NONE: i8 = 0;
pub(crate) const I32: i8 = 1;
pub(crate) const I64: i8 = 2;
pub(crate) const F32: i8 = 3;
pub(crate) const F64: i8 = 4;

/// A struct that can be used as a union type that can hold different types of values.  This is used
/// to represent values passed to / from native JIT compiled functions.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JitValue {
    discriminant: i8,
    value: i64,
}

impl JitValue {
    /// Creates a new `Value` equivalent to `Option::None`.
    #[must_use]
    pub fn new() -> JitValue {
        JitValue {
            discriminant: NONE,
            value: 0,
        }
    }
}

impl From<Option<crate::Value>> for JitValue {
    fn from(value: Option<crate::Value>) -> JitValue {
        match value {
            None => JitValue::new(),
            Some(value) => JitValue::from(value),
        }
    }
}

impl From<crate::Value> for JitValue {
    fn from(value: crate::Value) -> JitValue {
        match value {
            crate::Value::I32(value) => JitValue::from(value),
            crate::Value::I64(value) => JitValue::from(value),
            crate::Value::F32(value) => JitValue::from(value),
            crate::Value::F64(value) => JitValue::from(value),
        }
    }
}

impl TryInto<Option<crate::Value>> for JitValue {
    type Error = Error;

    fn try_into(self) -> Result<Option<crate::Value>, Self::Error> {
        let value = match self.discriminant {
            NONE => None,
            _ => Some(self.try_into()?),
        };
        Ok(value)
    }
}

impl TryInto<crate::Value> for JitValue {
    type Error = Error;

    fn try_into(self) -> Result<crate::Value, Self::Error> {
        let value = match self.discriminant {
            I32 => crate::Value::I32(self.try_into()?),
            I64 => crate::Value::I64(self.try_into()?),
            F32 => crate::Value::F32(self.try_into()?),
            F64 => crate::Value::F64(self.try_into()?),
            _ => {
                return Err(InternalError(format!(
                    "Invalid discriminant {}",
                    self.discriminant
                )));
            }
        };
        Ok(value)
    }
}

impl From<i32> for JitValue {
    fn from(value: i32) -> JitValue {
        JitValue {
            discriminant: I32,
            value: i64::from(value),
        }
    }
}

impl TryInto<i32> for JitValue {
    type Error = Error;

    #[expect(clippy::cast_possible_truncation)]
    fn try_into(self) -> Result<i32, Self::Error> {
        if self.discriminant != I32 {
            return Err(InvalidValue {
                expected: I32,
                actual: self.discriminant,
            });
        }
        Ok(self.value as i32)
    }
}

impl From<i64> for JitValue {
    fn from(value: i64) -> JitValue {
        JitValue {
            discriminant: I64,
            value,
        }
    }
}

impl TryInto<i64> for JitValue {
    type Error = Error;

    fn try_into(self) -> Result<i64, Self::Error> {
        if self.discriminant != I64 {
            return Err(InvalidValue {
                expected: I64,
                actual: self.discriminant,
            });
        }
        Ok(self.value)
    }
}

impl From<f32> for JitValue {
    fn from(value: f32) -> JitValue {
        let value = value.to_bits();
        let value = i64::from(value);
        JitValue {
            discriminant: F32,
            value,
        }
    }
}

impl TryInto<f32> for JitValue {
    type Error = Error;

    fn try_into(self) -> Result<f32, Self::Error> {
        if self.discriminant != F32 {
            return Err(InvalidValue {
                expected: F32,
                actual: self.discriminant,
            });
        }
        let value = u32::try_from(self.value)?;
        let value = f32::from_bits(value);
        Ok(value)
    }
}

impl From<f64> for JitValue {
    fn from(value: f64) -> JitValue {
        let value: i64 = zerocopy::transmute!(value.to_bits());
        JitValue {
            discriminant: F64,
            value,
        }
    }
}

impl TryInto<f64> for JitValue {
    type Error = Error;

    #[expect(clippy::cast_sign_loss)]
    fn try_into(self) -> Result<f64, Self::Error> {
        if self.discriminant != F64 {
            return Err(InvalidValue {
                expected: F64,
                actual: self.discriminant,
            });
        }
        Ok(f64::from_bits(self.value as u64))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;

    #[test]
    fn test_jit_value_from_none() {
        let value: Option<crate::Value> = None;
        let value = JitValue::from(value);
        assert_eq!(value.discriminant, NONE);
        assert_eq!(value.value, 0);
    }

    #[test]
    fn test_jit_value_from_some_value_i32() -> Result<()> {
        let value = Some(crate::Value::I32(42));
        let value = JitValue::from(value);
        let value: i32 = value.try_into()?;
        assert_eq!(value, 42i32);
        Ok(())
    }

    #[test]
    fn test_jit_value_from_some_value_i64() -> Result<()> {
        let value = Some(crate::Value::I64(42));
        let value = JitValue::from(value);
        let value: i64 = value.try_into()?;
        assert_eq!(value, 42i64);
        Ok(())
    }

    #[test]
    fn test_jit_value_from_some_value_f32() -> Result<()> {
        let value = Some(crate::Value::F32(42.1f32));
        let value = JitValue::from(value);
        let value: f32 = value.try_into()?;
        let value = value - 42.1f32;
        assert!(value.abs() < 0.1f32);
        Ok(())
    }

    #[test]
    fn test_jit_value_from_some_value_f64() -> Result<()> {
        let value = Some(crate::Value::F64(42.1f64));
        let value = JitValue::from(value);
        let value: f64 = value.try_into()?;
        let value = value - 42.1f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[test]
    fn test_jit_value_try_into_value_i32() -> Result<()> {
        let value = JitValue::from(42i32);
        let value: crate::Value = value.try_into()?;
        assert_eq!(value, crate::Value::I32(42));
        Ok(())
    }

    #[test]
    fn test_jit_value_try_into_value_i64() -> Result<()> {
        let value = JitValue::from(42i64);
        let value: crate::Value = value.try_into()?;
        assert_eq!(value, crate::Value::I64(42));
        Ok(())
    }

    #[test]
    fn test_jit_value_try_into_value_if2() -> Result<()> {
        let value = JitValue::from(42.1f32);
        let value: crate::Value = value.try_into()?;
        assert_eq!(value, crate::Value::F32(42.1f32));
        Ok(())
    }

    #[test]
    fn test_jit_value_try_into_value_f64() -> Result<()> {
        let value = JitValue::from(42.1f64);
        let value: crate::Value = value.try_into()?;
        assert_eq!(value, crate::Value::F64(42.1f64));
        Ok(())
    }

    #[test]
    fn test_jit_value_try_into_value_error() {
        let value = JitValue {
            discriminant: NONE,
            value: 0,
        };
        let result: Result<crate::Value> = value.try_into();
        assert!(result.is_err());
    }

    #[test]
    fn test_jit_value_try_into_i32() -> Result<()> {
        let value = JitValue::from(42i32);
        let value: i32 = value.try_into()?;
        assert_eq!(value, 42i32);
        Ok(())
    }

    #[test]
    fn test_jit_value_try_into_i32_error() {
        let value = JitValue::from(42i64);
        let result: Result<i32> = value.try_into();
        assert!(matches!(
            result,
            Err(InvalidValue {
                expected,
                actual
            }) if expected == I32 && actual == I64
        ));
    }

    #[test]
    fn test_jit_value_try_into_i64() -> Result<()> {
        let value = JitValue::from(42i64);
        let value: i64 = value.try_into()?;
        assert_eq!(value, 42i64);
        Ok(())
    }

    #[test]
    fn test_jit_value_try_into_i64_error() {
        let value = JitValue::from(42i32);
        let result: Result<i64> = value.try_into();
        assert!(matches!(
            result,
            Err(InvalidValue {
                expected,
                actual
            }) if expected == I64 && actual == I32
        ));
    }

    #[test]
    fn test_jit_value_try_into_f32() -> Result<()> {
        let value = JitValue::from(42.1f32);
        let value: f32 = value.try_into()?;
        let value = value - 42.1f32;
        assert!(value.abs() < 0.1f32);
        Ok(())
    }

    #[test]
    fn test_jit_value_try_into_f32_error() {
        let value = JitValue::from(42.1f64);
        let result: Result<f32> = value.try_into();
        assert!(matches!(
            result,
            Err(InvalidValue {
                expected,
                actual
            }) if expected == F32 && actual == F64
        ));
    }

    #[test]
    fn test_jit_value_try_into_f64() -> Result<()> {
        let value = JitValue::from(42.1f64);
        let value: f64 = value.try_into()?;
        let value = value - 42.1f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[test]
    fn test_jit_value_try_into_f64_error() {
        let value = JitValue::from(42.1f32);
        let result: Result<f64> = value.try_into();
        assert!(matches!(
            result,
            Err(InvalidValue {
                expected,
                actual
            }) if expected == F64 && actual == F32
        ));
    }
}
