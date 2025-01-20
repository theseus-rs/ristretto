use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "com/apple/concurrent/LibDispatchNative";

/// Register all native methods for `com.apple.concurrent.LibDispatchNative`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "nativeCreateConcurrentQueue",
        "(I)J",
        native_create_concurrent_queue,
    );
    registry.register(
        CLASS_NAME,
        "nativeCreateSerialQueue",
        "(Ljava/lang/String;)J",
        native_create_serial_queue,
    );
    registry.register(
        CLASS_NAME,
        "nativeExecuteAsync",
        "(JLjava/lang/Runnable;)V",
        native_execute_async,
    );
    registry.register(
        CLASS_NAME,
        "nativeExecuteSync",
        "(JLjava/lang/Runnable;)V",
        native_execute_sync,
    );
    registry.register(
        CLASS_NAME,
        "nativeGetMainQueue",
        "()J",
        native_get_main_queue,
    );
    registry.register(
        CLASS_NAME,
        "nativeIsDispatchSupported",
        "()Z",
        native_is_dispatch_supported,
    );
    registry.register(
        CLASS_NAME,
        "nativeReleaseQueue",
        "(J)V",
        native_release_queue,
    );
}

#[async_recursion(?Send)]
async fn native_create_concurrent_queue(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.apple.concurrent.LibDispatchNative.nativeCreateConcurrentQueue(I)J")
}

#[async_recursion(?Send)]
async fn native_create_serial_queue(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.apple.concurrent.LibDispatchNative.nativeCreateSerialQueue(Ljava/lang/String;)J")
}

#[async_recursion(?Send)]
async fn native_execute_async(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.apple.concurrent.LibDispatchNative.nativeExecuteAsync(JLjava/lang/Runnable;)V")
}

#[async_recursion(?Send)]
async fn native_execute_sync(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.apple.concurrent.LibDispatchNative.nativeExecuteSync(JLjava/lang/Runnable;)V")
}

#[async_recursion(?Send)]
async fn native_get_main_queue(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.apple.concurrent.LibDispatchNative.nativeGetMainQueue()J")
}

#[async_recursion(?Send)]
async fn native_is_dispatch_supported(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.apple.concurrent.LibDispatchNative.nativeIsDispatchSupported()Z")
}

#[async_recursion(?Send)]
async fn native_release_queue(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.apple.concurrent.LibDispatchNative.nativeReleaseQueue(J)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.apple.concurrent.LibDispatchNative.nativeCreateConcurrentQueue(I)J"
    )]
    async fn test_native_create_concurrent_queue() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_create_concurrent_queue(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.apple.concurrent.LibDispatchNative.nativeCreateSerialQueue(Ljava/lang/String;)J"
    )]
    async fn test_native_create_serial_queue() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_create_serial_queue(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.apple.concurrent.LibDispatchNative.nativeExecuteAsync(JLjava/lang/Runnable;)V"
    )]
    async fn test_native_execute_async() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_execute_async(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.apple.concurrent.LibDispatchNative.nativeExecuteSync(JLjava/lang/Runnable;)V"
    )]
    async fn test_native_execute_sync() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_execute_sync(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.apple.concurrent.LibDispatchNative.nativeGetMainQueue()J"
    )]
    async fn test_native_get_main_queue() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_get_main_queue(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.apple.concurrent.LibDispatchNative.nativeIsDispatchSupported()Z"
    )]
    async fn test_native_is_dispatch_supported() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_is_dispatch_supported(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.apple.concurrent.LibDispatchNative.nativeReleaseQueue(J)V"
    )]
    async fn test_native_release_queue() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_release_queue(thread, Parameters::default()).await;
    }
}
