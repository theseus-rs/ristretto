use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `com.apple.concurrent.LibDispatchNative`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "com/apple/concurrent/LibDispatchNative";
    registry.register(
        class_name,
        "nativeCreateConcurrentQueue",
        "(I)J",
        native_create_concurrent_queue,
    );
    registry.register(
        class_name,
        "nativeCreateSerialQueue",
        "(Ljava/lang/String;)J",
        native_create_serial_queue,
    );
    registry.register(
        class_name,
        "nativeExecuteAsync",
        "(JLjava/lang/Runnable;)V",
        native_execute_async,
    );
    registry.register(
        class_name,
        "nativeExecuteSync",
        "(JLjava/lang/Runnable;)V",
        native_execute_sync,
    );
    registry.register(
        class_name,
        "nativeGetMainQueue",
        "()J",
        native_get_main_queue,
    );
    registry.register(
        class_name,
        "nativeIsDispatchSupported",
        "()Z",
        native_is_dispatch_supported,
    );
    registry.register(
        class_name,
        "nativeReleaseQueue",
        "(J)V",
        native_release_queue,
    );
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn native_create_concurrent_queue(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn native_create_serial_queue(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn native_execute_async(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn native_execute_sync(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn native_get_main_queue(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn native_is_dispatch_supported(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn native_release_queue(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}
