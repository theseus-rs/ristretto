use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.net.sdp.SdpSupport`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/net/sdp/SdpSupport";
    registry.register(class_name, "convert0", "(I)V", convert_0);
    registry.register(class_name, "create0", "()I", create_0);
}

#[async_recursion(?Send)]
async fn convert_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn create_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
