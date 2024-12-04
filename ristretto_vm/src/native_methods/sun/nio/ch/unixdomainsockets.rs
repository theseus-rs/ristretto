use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.nio.ch.UnixDomainSockets`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/nio/ch/UnixDomainSockets";
    registry.register(
        class_name,
        "accept0",
        "(Ljava/io/FileDescriptor;Ljava/io/FileDescriptor;[Ljava/lang/Object;)I",
        accept_0,
    );
    registry.register(class_name, "bind0", "(Ljava/io/FileDescriptor;[B)V", bind_0);
    registry.register(
        class_name,
        "connect0",
        "(Ljava/io/FileDescriptor;[B)I",
        connect_0,
    );
    registry.register(class_name, "init", "()Z", init);
    registry.register(
        class_name,
        "localAddress0",
        "(Ljava/io/FileDescriptor;)[B",
        local_address_0,
    );
    registry.register(class_name, "socket0", "()I", socket_0);
}

#[async_recursion(?Send)]
async fn accept_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn bind_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn connect_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn init(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn local_address_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn socket_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
