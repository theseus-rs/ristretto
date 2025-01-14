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

#[async_recursion(?Send)]
async fn native_create_concurrent_queue(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("com.apple.concurrent.LibDispatchNative.nativeCreateConcurrentQueue(I)J")
}

#[async_recursion(?Send)]
async fn native_create_serial_queue(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("com.apple.concurrent.LibDispatchNative.nativeCreateSerialQueue(Ljava/lang/String;)J")
}

#[async_recursion(?Send)]
async fn native_execute_async(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("com.apple.concurrent.LibDispatchNative.nativeExecuteAsync(JLjava/lang/Runnable;)V")
}

#[async_recursion(?Send)]
async fn native_execute_sync(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.apple.concurrent.LibDispatchNative.nativeExecuteSync(JLjava/lang/Runnable;)V")
}

#[async_recursion(?Send)]
async fn native_get_main_queue(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("com.apple.concurrent.LibDispatchNative.nativeGetMainQueue()J")
}

#[async_recursion(?Send)]
async fn native_is_dispatch_supported(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("com.apple.concurrent.LibDispatchNative.nativeIsDispatchSupported()Z")
}

#[async_recursion(?Send)]
async fn native_release_queue(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("com.apple.concurrent.LibDispatchNative.nativeReleaseQueue(J)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "com/apple/concurrent/LibDispatchNative";
        assert!(registry
            .method(class_name, "nativeCreateConcurrentQueue", "(I)J")
            .is_some());
        assert!(registry
            .method(
                class_name,
                "nativeCreateSerialQueue",
                "(Ljava/lang/String;)J"
            )
            .is_some());
        assert!(registry
            .method(class_name, "nativeExecuteAsync", "(JLjava/lang/Runnable;)V")
            .is_some());
        assert!(registry
            .method(class_name, "nativeExecuteSync", "(JLjava/lang/Runnable;)V")
            .is_some());
        assert!(registry
            .method(class_name, "nativeGetMainQueue", "()J")
            .is_some());
        assert!(registry
            .method(class_name, "nativeIsDispatchSupported", "()Z")
            .is_some());
        assert!(registry
            .method(class_name, "nativeReleaseQueue", "(J)V")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.apple.concurrent.LibDispatchNative.nativeCreateConcurrentQueue(I)J"
    )]
    async fn test_native_create_concurrent_queue() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_create_concurrent_queue(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.apple.concurrent.LibDispatchNative.nativeCreateSerialQueue(Ljava/lang/String;)J"
    )]
    async fn test_native_create_serial_queue() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_create_serial_queue(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.apple.concurrent.LibDispatchNative.nativeExecuteAsync(JLjava/lang/Runnable;)V"
    )]
    async fn test_native_execute_async() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_execute_async(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.apple.concurrent.LibDispatchNative.nativeExecuteSync(JLjava/lang/Runnable;)V"
    )]
    async fn test_native_execute_sync() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_execute_sync(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.apple.concurrent.LibDispatchNative.nativeGetMainQueue()J"
    )]
    async fn test_native_get_main_queue() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_get_main_queue(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.apple.concurrent.LibDispatchNative.nativeIsDispatchSupported()Z"
    )]
    async fn test_native_is_dispatch_supported() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_is_dispatch_supported(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.apple.concurrent.LibDispatchNative.nativeReleaseQueue(J)V"
    )]
    async fn test_native_release_queue() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_release_queue(thread, Arguments::default()).await;
    }
}
