use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.security.smartcardio.PlatformPCSC`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/security/smartcardio/PlatformPCSC";
    registry.register(
        class_name,
        "initialize",
        "(Ljava/lang/String;)V",
        initialize,
    );
}

#[async_recursion(?Send)]
async fn initialize(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.smartcardio.PlatformPCSC.initialize(Ljava/lang/String;)V")
}
