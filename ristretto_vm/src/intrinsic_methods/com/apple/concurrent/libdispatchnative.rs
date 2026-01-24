use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "com/apple/concurrent/LibDispatchNative.nativeCreateConcurrentQueue(I)J",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn native_create_concurrent_queue(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.apple.concurrent.LibDispatchNative.nativeCreateConcurrentQueue(I)J")
}

#[intrinsic_method(
    "com/apple/concurrent/LibDispatchNative.nativeCreateSerialQueue(Ljava/lang/String;)J",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn native_create_serial_queue(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.apple.concurrent.LibDispatchNative.nativeCreateSerialQueue(Ljava/lang/String;)J")
}

#[intrinsic_method(
    "com/apple/concurrent/LibDispatchNative.nativeExecuteAsync(JLjava/lang/Runnable;)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn native_execute_async(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.apple.concurrent.LibDispatchNative.nativeExecuteAsync(JLjava/lang/Runnable;)V")
}

#[intrinsic_method(
    "com/apple/concurrent/LibDispatchNative.nativeExecuteSync(JLjava/lang/Runnable;)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn native_execute_sync(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.apple.concurrent.LibDispatchNative.nativeExecuteSync(JLjava/lang/Runnable;)V")
}

#[intrinsic_method(
    "com/apple/concurrent/LibDispatchNative.nativeGetMainQueue()J",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn native_get_main_queue(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.apple.concurrent.LibDispatchNative.nativeGetMainQueue()J")
}

#[intrinsic_method(
    "com/apple/concurrent/LibDispatchNative.nativeIsDispatchSupported()Z",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn native_is_dispatch_supported(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.apple.concurrent.LibDispatchNative.nativeIsDispatchSupported()Z")
}

#[intrinsic_method(
    "com/apple/concurrent/LibDispatchNative.nativeReleaseQueue(J)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn native_release_queue(
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
