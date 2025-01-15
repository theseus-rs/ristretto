use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/nio/ch/UnixDomainSockets";

/// Register all native methods for `sun.nio.ch.UnixDomainSockets`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "accept0",
        "(Ljava/io/FileDescriptor;Ljava/io/FileDescriptor;[Ljava/lang/Object;)I",
        accept_0,
    );
    registry.register(CLASS_NAME, "bind0", "(Ljava/io/FileDescriptor;[B)V", bind_0);
    registry.register(
        CLASS_NAME,
        "connect0",
        "(Ljava/io/FileDescriptor;[B)I",
        connect_0,
    );
    registry.register(CLASS_NAME, "init", "()Z", init);
    registry.register(
        CLASS_NAME,
        "localAddress0",
        "(Ljava/io/FileDescriptor;)[B",
        local_address_0,
    );
    registry.register(CLASS_NAME, "socket0", "()I", socket_0);
}

#[async_recursion(?Send)]
async fn accept_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.UnixDomainSockets.accept0(Ljava/io/FileDescriptor;Ljava/io/FileDescriptor;[Ljava/lang/Object;)I");
}

#[async_recursion(?Send)]
async fn bind_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.UnixDomainSockets.bind0(Ljava/io/FileDescriptor;[B)V");
}

#[async_recursion(?Send)]
async fn connect_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.UnixDomainSockets.connect0(Ljava/io/FileDescriptor;[B)I");
}

#[async_recursion(?Send)]
async fn init(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.UnixDomainSockets.init()Z");
}

#[async_recursion(?Send)]
async fn local_address_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.UnixDomainSockets.localAddress0(Ljava/io/FileDescriptor;)[B");
}

#[async_recursion(?Send)]
async fn socket_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.UnixDomainSockets.socket0()I");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.UnixDomainSockets.accept0(Ljava/io/FileDescriptor;Ljava/io/FileDescriptor;[Ljava/lang/Object;)I"
    )]
    async fn test_accept_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = accept_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.UnixDomainSockets.bind0(Ljava/io/FileDescriptor;[B)V"
    )]
    async fn test_bind_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = bind_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.UnixDomainSockets.connect0(Ljava/io/FileDescriptor;[B)I"
    )]
    async fn test_connect_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = connect_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.UnixDomainSockets.init()Z")]
    async fn test_init() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.UnixDomainSockets.localAddress0(Ljava/io/FileDescriptor;)[B"
    )]
    async fn test_local_address_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = local_address_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.UnixDomainSockets.socket0()I")]
    async fn test_socket_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = socket_0(thread, Arguments::default()).await;
    }
}
