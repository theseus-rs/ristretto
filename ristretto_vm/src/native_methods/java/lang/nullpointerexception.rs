use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.lang.NullPointerException`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/NullPointerException";
    registry.register(
        class_name,
        "getExtendedNPEMessage",
        "()Ljava/lang/String;",
        get_extended_npe_message,
    );
}

#[async_recursion(?Send)]
async fn get_extended_npe_message(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.lang.NullPointerException.getExtendedNPEMessage()Ljava/lang/String;")
}
