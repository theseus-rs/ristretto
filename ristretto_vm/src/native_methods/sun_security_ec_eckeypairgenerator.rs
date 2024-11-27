use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.security.ec.ECKeyPairGenerator`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/security/ec/ECKeyPairGenerator";
    registry.register(
        class_name,
        "generateECKeyPair",
        "(I[B[B)[Ljava/lang/Object;",
        generate_ec_key_pair,
    );
    registry.register(class_name, "isCurveSupported", "([B)Z", is_curve_supported);
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn generate_ec_key_pair(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn is_curve_supported(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
