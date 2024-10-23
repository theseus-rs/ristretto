use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use ristretto_classloader::Value;
use std::future::Future;
use std::pin::Pin;
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
fn float_to_raw_int_bits(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Pin<Box<dyn Future<Output = Result<Option<Value>>>>> {
    Box::pin(async move {
        let float = arguments.pop_float()?;
        #[expect(clippy::cast_possible_wrap)]
        let bits = float.to_bits() as i32;
        Ok(Some(Value::Int(bits)))
    })
}
