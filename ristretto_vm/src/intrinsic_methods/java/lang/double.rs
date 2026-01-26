use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;
use zerocopy::transmute;

#[intrinsic_method("java/lang/Double.doubleToRawLongBits(D)J", Any)]
#[async_method]
pub(crate) async fn double_to_raw_long_bits(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let double = parameters.pop_double()?;
    let bits: i64 = transmute!(double.to_bits());
    Ok(Some(Value::Long(bits)))
}

#[intrinsic_method("java/lang/Double.longBitsToDouble(J)D", Any)]
#[async_method]
pub(crate) async fn long_bits_to_double(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let long = parameters.pop_long()?;
    #[expect(clippy::cast_sign_loss)]
    let bits = long as u64;
    let double = f64::from_bits(bits);
    Ok(Some(Value::Double(double)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_double_to_raw_long_bits() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let mut parameters = Parameters::default();
        parameters.push_double(42.0);
        let value = double_to_raw_long_bits(thread, parameters).await?;
        assert_eq!(Some(Value::Long(4_631_107_791_820_423_168)), value);
        Ok(())
    }

    #[tokio::test]
    async fn test_long_bits_to_double() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let mut parameters = Parameters::default();
        parameters.push_long(4_631_107_791_820_423_168);
        let value = long_bits_to_double(thread, parameters).await?;
        assert_eq!(Some(Value::Double(42.0)), value);
        Ok(())
    }
}
