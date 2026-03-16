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
    "com/sun/demo/jvmti/hprof/Tracker.nativeCallSite(Ljava/lang/Object;II)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_call_site<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.demo.jvmti.hprof.Tracker.nativeCallSite(Ljava/lang/Object;II)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/demo/jvmti/hprof/Tracker.nativeNewArray(Ljava/lang/Object;Ljava/lang/Object;)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_new_array<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.demo.jvmti.hprof.Tracker.nativeNewArray(Ljava/lang/Object;Ljava/lang/Object;)V"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/demo/jvmti/hprof/Tracker.nativeObjectInit(Ljava/lang/Object;Ljava/lang/Object;)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_object_init<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.demo.jvmti.hprof.Tracker.nativeObjectInit(Ljava/lang/Object;Ljava/lang/Object;)V"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/demo/jvmti/hprof/Tracker.nativeReturnSite(Ljava/lang/Object;II)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_return_site<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.demo.jvmti.hprof.Tracker.nativeReturnSite(Ljava/lang/Object;II)V".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_native_call_site() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_call_site(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_new_array() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_new_array(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_object_init() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_object_init(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_return_site() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_return_site(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
