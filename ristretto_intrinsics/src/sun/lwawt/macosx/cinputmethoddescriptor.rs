use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/lwawt/macosx/CInputMethodDescriptor.nativeGetAvailableLocales()Ljava/util/List;",
    Any
)]
#[async_method]
pub async fn native_get_available_locales<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CInputMethodDescriptor.nativeGetAvailableLocales()Ljava/util/List;"
            .to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/CInputMethodDescriptor.nativeInit()V", Any)]
#[async_method]
pub async fn native_init<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CInputMethodDescriptor.nativeInit()V".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_native_get_available_locales() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_get_available_locales(thread, Parameters::default()).await;
        assert_eq!(
            "sun.lwawt.macosx.CInputMethodDescriptor.nativeGetAvailableLocales()Ljava/util/List;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_init() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_init(thread, Parameters::default()).await;
        assert_eq!(
            "sun.lwawt.macosx.CInputMethodDescriptor.nativeInit()V",
            result.unwrap_err().to_string()
        );
    }
}
