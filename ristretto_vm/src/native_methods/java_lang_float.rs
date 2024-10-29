use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for java.lang.Float.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/Float";
    registry.register(
        class_name,
        "floatToRawIntBits",
        "(F)I",
        float_to_raw_int_bits,
    );
}

#[expect(clippy::needless_pass_by_value)]
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
