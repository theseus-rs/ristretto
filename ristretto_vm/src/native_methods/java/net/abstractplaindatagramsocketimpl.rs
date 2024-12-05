use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.net.AbstractPlainDatagramSocketImpl`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/net/AbstractPlainDatagramSocketImpl";
    registry.register(
        class_name,
        "isReusePortAvailable0",
        "()Z",
        is_reuse_port_available_0,
    );
}

#[async_recursion(?Send)]
async fn is_reuse_port_available_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.net.AbstractPlainDatagramSocketImpl.isReusePortAvailable0()Z")
}
