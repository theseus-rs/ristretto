use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.net.InetAddressImplFactory`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/net/InetAddressImplFactory";
    registry.register(class_name, "isIPv6Supported", "()Z", is_i_pv_6_supported);
}

#[async_recursion(?Send)]
async fn is_i_pv_6_supported(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
