use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/lwawt/macosx/CMenuItem.nativeCreate(JZ)J", Any)]
#[async_method]
pub async fn native_create<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CMenuItem.nativeCreate(JZ)J")
}

#[intrinsic_method("sun/lwawt/macosx/CMenuItem.nativeSetEnabled(JZ)V", Any)]
#[async_method]
pub async fn native_set_enabled<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CMenuItem.nativeSetEnabled(JZ)V")
}

#[intrinsic_method("sun/lwawt/macosx/CMenuItem.nativeSetImage(JJ)V", Any)]
#[async_method]
pub async fn native_set_image<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CMenuItem.nativeSetImage(JJ)V")
}

#[intrinsic_method(
    "sun/lwawt/macosx/CMenuItem.nativeSetLabel(JLjava/lang/String;CII)V",
    Any
)]
#[async_method]
pub async fn native_set_label<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CMenuItem.nativeSetLabel(JLjava/lang/String;CII)V")
}

#[intrinsic_method(
    "sun/lwawt/macosx/CMenuItem.nativeSetTooltip(JLjava/lang/String;)V",
    Any
)]
#[async_method]
pub async fn native_set_tooltip<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CMenuItem.nativeSetTooltip(JLjava/lang/String;)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.lwawt.macosx.CMenuItem.nativeCreate(JZ)J")]
    async fn test_native_create() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_create(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CMenuItem.nativeSetEnabled(JZ)V"
    )]
    async fn test_native_set_enabled() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_enabled(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CMenuItem.nativeSetImage(JJ)V"
    )]
    async fn test_native_set_image() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_image(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CMenuItem.nativeSetLabel(JLjava/lang/String;CII)V"
    )]
    async fn test_native_set_label() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_label(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CMenuItem.nativeSetTooltip(JLjava/lang/String;)V"
    )]
    async fn test_native_set_tooltip() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_tooltip(thread, Parameters::default()).await;
    }
}
