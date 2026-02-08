use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/lwawt/macosx/CInputMethodDescriptor.nativeGetAvailableLocales()Ljava/util/List;",
    Any
)]
#[async_method]
pub async fn native_get_available_locales<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CInputMethodDescriptor.nativeGetAvailableLocales()Ljava/util/List;")
}

#[intrinsic_method("sun/lwawt/macosx/CInputMethodDescriptor.nativeInit()V", Any)]
#[async_method]
pub async fn native_init<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CInputMethodDescriptor.nativeInit()V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CInputMethodDescriptor.nativeGetAvailableLocales()Ljava/util/List;"
    )]
    async fn test_native_get_available_locales() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_get_available_locales(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CInputMethodDescriptor.nativeInit()V"
    )]
    async fn test_native_init() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_init(thread, Parameters::default()).await;
    }
}
