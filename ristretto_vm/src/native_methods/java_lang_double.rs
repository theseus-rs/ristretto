use crate::arguments::Arguments;
use crate::call_stack::CallStack;
use crate::native_methods::registry::MethodRegistry;
use crate::{Result, VM};
use ristretto_classloader::Value;

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

fn double_to_raw_long_bits(
    _vm: &VM,
    _call_stack: &CallStack,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let double = arguments.pop_double()?;
    #[expect(clippy::cast_possible_wrap)]
    let bits = double.to_bits() as i64;
    Ok(Some(Value::Long(bits)))
}

fn long_bits_to_double(
    _vm: &VM,
    _call_stack: &CallStack,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let long = arguments.pop_long()?;
    #[expect(clippy::cast_sign_loss)]
    let bits = long as u64;
    let double = f64::from_bits(bits);
    Ok(Some(Value::Double(double)))
}
