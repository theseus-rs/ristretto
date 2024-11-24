use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.net.SocketOutputStream`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/net/SocketOutputStream";
    registry.register(class_name, "init", "()V", init);
    registry.register(
        class_name,
        "socketWrite0",
        "(Ljava/io/FileDescriptor;[BII)V",
        socket_write_0,
    );
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn init(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn socket_write_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
