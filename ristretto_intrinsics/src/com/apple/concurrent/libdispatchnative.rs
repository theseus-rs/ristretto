use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "com/apple/concurrent/LibDispatchNative.nativeCreateConcurrentQueue(I)J",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_create_concurrent_queue<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.apple.concurrent.LibDispatchNative.nativeCreateConcurrentQueue(I)J".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/apple/concurrent/LibDispatchNative.nativeCreateSerialQueue(Ljava/lang/String;)J",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_create_serial_queue<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.apple.concurrent.LibDispatchNative.nativeCreateSerialQueue(Ljava/lang/String;)J"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/apple/concurrent/LibDispatchNative.nativeExecuteAsync(JLjava/lang/Runnable;)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_execute_async<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.apple.concurrent.LibDispatchNative.nativeExecuteAsync(JLjava/lang/Runnable;)V"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/apple/concurrent/LibDispatchNative.nativeExecuteSync(JLjava/lang/Runnable;)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_execute_sync<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.apple.concurrent.LibDispatchNative.nativeExecuteSync(JLjava/lang/Runnable;)V"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/apple/concurrent/LibDispatchNative.nativeGetMainQueue()J",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_get_main_queue<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.apple.concurrent.LibDispatchNative.nativeGetMainQueue()J".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/apple/concurrent/LibDispatchNative.nativeIsDispatchSupported()Z",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_is_dispatch_supported<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.apple.concurrent.LibDispatchNative.nativeIsDispatchSupported()Z".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/apple/concurrent/LibDispatchNative.nativeReleaseQueue(J)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_release_queue<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.apple.concurrent.LibDispatchNative.nativeReleaseQueue(J)V".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_native_create_concurrent_queue() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = native_create_concurrent_queue(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_create_serial_queue() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = native_create_serial_queue(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_execute_async() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = native_execute_async(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_execute_sync() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = native_execute_sync(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_get_main_queue() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = native_get_main_queue(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_is_dispatch_supported() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = native_is_dispatch_supported(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_release_queue() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = native_release_queue(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
