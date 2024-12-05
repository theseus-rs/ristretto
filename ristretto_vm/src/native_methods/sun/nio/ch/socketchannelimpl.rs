use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_8: Version = Version::Java8 { minor: 0 };

/// Register all native methods for `sun.nio.ch.SocketChannelImpl`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/nio/ch/SocketChannelImpl";
    let java_version = registry.java_version();

    if java_version <= &JAVA_8 {
        registry.register(
            class_name,
            "checkConnect",
            "(Ljava/io/FileDescriptor;ZZ)I",
            check_connect,
        );
    } else {
        registry.register(
            class_name,
            "checkConnect",
            "(Ljava/io/FileDescriptor;Z)I",
            check_connect,
        );
    }

    registry.register(
        class_name,
        "sendOutOfBandData",
        "(Ljava/io/FileDescriptor;B)I",
        send_out_of_band_data,
    );
}

#[async_recursion(?Send)]
async fn check_connect(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.SocketChannelImpl.checkConnect(Ljava/io/FileDescriptor;ZZ)I")
}

#[async_recursion(?Send)]
async fn send_out_of_band_data(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.SocketChannelImpl.sendOutOfBandData(Ljava/io/FileDescriptor;B)I")
}
