use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/lwawt/macosx/CInputMethod.getNativeLocale()Ljava/util/Locale;",
    Any
)]
#[async_method]
pub async fn get_native_locale<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CInputMethod.getNativeLocale()Ljava/util/Locale;")
}

#[intrinsic_method("sun/lwawt/macosx/CInputMethod.nativeEndComposition(J)V", Any)]
#[async_method]
pub async fn native_end_composition<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CInputMethod.nativeEndComposition(J)V")
}

#[intrinsic_method(
    "sun/lwawt/macosx/CInputMethod.nativeGetCurrentInputMethodInfo()Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn native_get_current_input_method_info<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CInputMethod.nativeGetCurrentInputMethodInfo()Ljava/lang/String;")
}

#[intrinsic_method(
    "sun/lwawt/macosx/CInputMethod.nativeHandleEvent(Lsun/lwawt/LWComponentPeer;Ljava/awt/AWTEvent;)V",
    Any
)]
#[async_method]
pub async fn native_handle_event<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.lwawt.macosx.CInputMethod.nativeHandleEvent(Lsun/lwawt/LWComponentPeer;Ljava/awt/AWTEvent;)V"
    )
}

#[intrinsic_method("sun/lwawt/macosx/CInputMethod.nativeInit()V", Any)]
#[async_method]
pub async fn native_init<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CInputMethod.nativeInit()V")
}

#[intrinsic_method(
    "sun/lwawt/macosx/CInputMethod.nativeNotifyPeer(JLsun/lwawt/macosx/CInputMethod;)V",
    Any
)]
#[async_method]
pub async fn native_notify_peer<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CInputMethod.nativeNotifyPeer(JLsun/lwawt/macosx/CInputMethod;)V")
}

#[intrinsic_method(
    "sun/lwawt/macosx/CInputMethod.setNativeLocale(Ljava/lang/String;Z)Z",
    Any
)]
#[async_method]
pub async fn set_native_locale<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CInputMethod.setNativeLocale(Ljava/lang/String;Z)Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CInputMethod.getNativeLocale()Ljava/util/Locale;"
    )]
    async fn test_get_native_locale() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_native_locale(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CInputMethod.nativeEndComposition(J)V"
    )]
    async fn test_native_end_composition() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_end_composition(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CInputMethod.nativeGetCurrentInputMethodInfo()Ljava/lang/String;"
    )]
    async fn test_native_get_current_input_method_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_get_current_input_method_info(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CInputMethod.nativeHandleEvent(Lsun/lwawt/LWComponentPeer;Ljava/awt/AWTEvent;)V"
    )]
    async fn test_native_handle_event() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_handle_event(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.lwawt.macosx.CInputMethod.nativeInit()V")]
    async fn test_native_init() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_init(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CInputMethod.nativeNotifyPeer(JLsun/lwawt/macosx/CInputMethod;)V"
    )]
    async fn test_native_notify_peer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_notify_peer(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CInputMethod.setNativeLocale(Ljava/lang/String;Z)Z"
    )]
    async fn test_set_native_locale() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_native_locale(thread, Parameters::default()).await;
    }
}
