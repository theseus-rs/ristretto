use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_20: Version = Version::Java20 { minor: 0 };

/// Register all native methods for `sun.nio.ch.SocketDispatcher`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/nio/ch/SocketDispatcher";
    let java_version = registry.java_version();

    if java_version >= &JAVA_20 {
        registry.register(
            class_name,
            "write0",
            "(Ljava/io/FileDescriptor;JI)I",
            write_0,
        );
        registry.register(
            class_name,
            "writev0",
            "(Ljava/io/FileDescriptor;JI)J",
            writev_0,
        );
    }

    registry.register(class_name, "read0", "(Ljava/io/FileDescriptor;JI)I", read_0);
    registry.register(
        class_name,
        "readv0",
        "(Ljava/io/FileDescriptor;JI)J",
        readv_0,
    );
}

#[async_recursion(?Send)]
async fn read_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.SocketDispatcher.read0(Ljava/io/FileDescriptor;JI)I")
}

#[async_recursion(?Send)]
async fn readv_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.SocketDispatcher.readv0(Ljava/io/FileDescriptor;JI)J")
}

#[async_recursion(?Send)]
async fn write_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.SocketDispatcher.write0(Ljava/io/FileDescriptor;JI)I")
}

#[async_recursion(?Send)]
async fn writev_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.SocketDispatcher.writev0(Ljava/io/FileDescriptor;JI)J")
}
