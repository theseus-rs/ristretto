use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;
use zerocopy::transmute;

#[intrinsic_method("java/lang/Float.floatToRawIntBits(F)I", Any)]
#[async_method]
pub async fn float_to_raw_int_bits<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let float = parameters.pop_float()?;
    let bits: i32 = transmute!(float.to_bits());
    Ok(Some(Value::Int(bits)))
}

#[intrinsic_method("java/lang/Float.intBitsToFloat(I)F", Any)]
#[async_method]
pub async fn int_bits_to_float<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let integer = parameters.pop_int()?;
    #[expect(clippy::cast_sign_loss)]
    let float = f32::from_bits(integer as u32);
    Ok(Some(Value::Float(float)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_float_to_raw_int_bits() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let mut parameters = Parameters::default();
        parameters.push_float(42.0);
        let result = float_to_raw_int_bits(thread, parameters).await?;
        assert_eq!(result, Some(Value::Int(1_109_917_696)));
        Ok(())
    }

    #[tokio::test]
    async fn test_int_bits_to_float() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let mut parameters = Parameters::default();
        parameters.push_int(1_109_917_696);
        let result = int_bits_to_float(thread, parameters).await?;
        assert_eq!(result, Some(Value::Float(42.0)));
        Ok(())
    }
}
