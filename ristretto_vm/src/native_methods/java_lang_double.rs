use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for java.lang.Double.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/Double";
    registry.register(
        class_name,
        "doubleToRawLongBits",
        "(D)J",
        double_to_raw_long_bits,
    );
    registry.register(class_name, "longBitsToDouble", "(J)D", long_bits_to_double);
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn double_to_raw_long_bits(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let double = arguments.pop_double()?;
    #[expect(clippy::cast_possible_wrap)]
    let bits = double.to_bits() as i64;
    Ok(Some(Value::Long(bits)))
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn long_bits_to_double(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let long = arguments.pop_long()?;
    #[expect(clippy::cast_sign_loss)]
    let bits = long as u64;
    let double = f64::from_bits(bits);
    Ok(Some(Value::Double(double)))
}
