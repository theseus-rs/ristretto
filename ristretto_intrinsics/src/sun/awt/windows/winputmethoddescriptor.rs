use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/awt/windows/WInputMethodDescriptor.getNativeAvailableLocales()[Ljava/util/Locale;",
    Any
)]
#[async_method]
pub async fn get_native_available_locales<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WInputMethodDescriptor.getNativeAvailableLocales()[Ljava/util/Locale;"
            .to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_native_available_locales() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_native_available_locales(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WInputMethodDescriptor.getNativeAvailableLocales()[Ljava/util/Locale;",
            result.unwrap_err().to_string()
        );
    }
}
