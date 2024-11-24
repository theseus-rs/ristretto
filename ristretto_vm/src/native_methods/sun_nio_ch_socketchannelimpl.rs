use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.nio.ch.SocketChannelImpl`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/nio/ch/SocketChannelImpl";
    registry.register(
        class_name,
        "checkConnect",
        "(Ljava/io/FileDescriptor;ZZ)I",
        check_connect,
    );
    registry.register(
        class_name,
        "sendOutOfBandData",
        "(Ljava/io/FileDescriptor;B)I",
        send_out_of_band_data,
    );
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn check_connect(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn send_out_of_band_data(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}
