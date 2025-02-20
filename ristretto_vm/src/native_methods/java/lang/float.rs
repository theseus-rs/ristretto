use crate::Result;
use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "java/lang/Float";

/// Register all native methods for `java.lang.Float`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "floatToRawIntBits",
        "(F)I",
        float_to_raw_int_bits,
    );
    registry.register(CLASS_NAME, "intBitsToFloat", "(I)F", int_bits_to_float);
}

#[async_recursion(?Send)]
async fn float_to_raw_int_bits(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let float = parameters.pop_float()?;
    #[expect(clippy::cast_possible_wrap)]
    let bits = float.to_bits() as i32;
    Ok(Some(Value::Int(bits)))
}

#[async_recursion(?Send)]
async fn int_bits_to_float(
    _thread: Arc<Thread>,
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
