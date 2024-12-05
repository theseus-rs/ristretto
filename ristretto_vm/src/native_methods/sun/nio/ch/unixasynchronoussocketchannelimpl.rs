use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.nio.ch.UnixAsynchronousSocketChannelImpl`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/nio/ch/UnixAsynchronousSocketChannelImpl";
    registry.register(class_name, "checkConnect", "(I)V", check_connect);
}

#[async_recursion(?Send)]
async fn check_connect(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.UnixAsynchronousSocketChannelImpl.checkConnect(I)V")
}
