use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.lang.Float`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/Float";
    registry.register(
        class_name,
        "floatToRawIntBits",
        "(F)I",
        float_to_raw_int_bits,
    );
    registry.register(class_name, "intBitsToFloat", "(I)F", int_bits_to_float);
}

#[async_recursion(?Send)]
async fn float_to_raw_int_bits(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let float = arguments.pop_float()?;
    #[expect(clippy::cast_possible_wrap)]
    let bits = float.to_bits() as i32;
    Ok(Some(Value::Int(bits)))
}

#[async_recursion(?Send)]
async fn int_bits_to_float(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let integer = arguments.pop_int()?;
    #[expect(clippy::cast_sign_loss)]
    let float = f32::from_bits(integer as u32);
    Ok(Some(Value::Float(float)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "java/lang/Float";
        assert!(registry
            .method(class_name, "floatToRawIntBits", "(F)I")
            .is_some());
        assert!(registry
            .method(class_name, "intBitsToFloat", "(I)F")
            .is_some());
    }

    #[tokio::test]
    async fn test_float_to_raw_int_bits() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let mut arguments = Arguments::default();
        arguments.push_float(42.0);
        let result = float_to_raw_int_bits(thread, arguments).await?;
        assert_eq!(result, Some(Value::Int(1_109_917_696)));
        Ok(())
    }

    #[tokio::test]
    async fn test_int_bits_to_float() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let mut arguments = Arguments::default();
        arguments.push_int(1_109_917_696);
        let result = int_bits_to_float(thread, arguments).await?;
        assert_eq!(result, Some(Value::Float(42.0)));
        Ok(())
    }
}
