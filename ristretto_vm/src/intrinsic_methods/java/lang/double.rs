use crate::Result;
use crate::intrinsic_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "java/lang/Double";

/// Register all intrinsic methods for `java.lang.Double`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "doubleToRawLongBits",
        "(D)J",
        double_to_raw_long_bits,
    );
    registry.register(CLASS_NAME, "longBitsToDouble", "(J)D", long_bits_to_double);
}

#[async_recursion(?Send)]
async fn double_to_raw_long_bits(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let double = parameters.pop_double()?;
    #[expect(clippy::cast_possible_wrap)]
    let bits = double.to_bits() as i64;
    Ok(Some(Value::Long(bits)))
}

#[async_recursion(?Send)]
async fn long_bits_to_double(
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
