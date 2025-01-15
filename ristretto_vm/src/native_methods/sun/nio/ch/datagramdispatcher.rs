use crate::arguments::Arguments;
use crate::native_methods::registry::{MethodRegistry, JAVA_20};
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/nio/ch/DatagramDispatcher";

/// Register all native methods for `sun.nio.ch.DatagramDispatcher`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() >= JAVA_20 {
        registry.register(
            CLASS_NAME,
            "dup0",
            "(Ljava/io/FileDescriptor;Ljava/io/FileDescriptor;)V",
            dup_0,
        );
    }

    registry.register(CLASS_NAME, "read0", "(Ljava/io/FileDescriptor;JI)I", read_0);
    registry.register(
        CLASS_NAME,
        "readv0",
        "(Ljava/io/FileDescriptor;JI)J",
        readv_0,
    );
    registry.register(
        CLASS_NAME,
        "write0",
        "(Ljava/io/FileDescriptor;JI)I",
        write_0,
    );
    registry.register(
        CLASS_NAME,
        "writev0",
        "(Ljava/io/FileDescriptor;JI)J",
        writev_0,
    );
}

#[async_recursion(?Send)]
async fn dup_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.DatagramDispatcher.dup0(Ljava/io/FileDescriptor;Ljava/io/FileDescriptor;)V");
}

#[async_recursion(?Send)]
async fn read_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.DatagramDispatcher.read0(Ljava/io/FileDescriptor;JI)I");
}

#[async_recursion(?Send)]
async fn readv_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.DatagramDispatcher.readv0(Ljava/io/FileDescriptor;JI)J");
}

#[async_recursion(?Send)]
async fn write_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.DatagramDispatcher.write0(Ljava/io/FileDescriptor;JI)I");
}

#[async_recursion(?Send)]
async fn writev_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.DatagramDispatcher.writev0(Ljava/io/FileDescriptor;JI)J");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.DatagramDispatcher.dup0(Ljava/io/FileDescriptor;Ljava/io/FileDescriptor;)V"
    )]
    async fn test_dup_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = dup_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.DatagramDispatcher.read0(Ljava/io/FileDescriptor;JI)I"
    )]
    async fn test_read_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = read_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.DatagramDispatcher.readv0(Ljava/io/FileDescriptor;JI)J"
    )]
    async fn test_readv_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = readv_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.DatagramDispatcher.write0(Ljava/io/FileDescriptor;JI)I"
    )]
    async fn test_write_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = write_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.DatagramDispatcher.writev0")]
    async fn test_writev_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = writev_0(thread, Arguments::default()).await;
    }
}
