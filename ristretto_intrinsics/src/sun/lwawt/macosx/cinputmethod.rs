use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/lwawt/macosx/CInputMethod.getNativeLocale()Ljava/util/Locale;",
    Any
)]
#[async_method]
pub async fn get_native_locale<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CInputMethod.getNativeLocale()Ljava/util/Locale;".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/CInputMethod.nativeEndComposition(J)V", Any)]
#[async_method]
pub async fn native_end_composition<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _native_peer = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CInputMethod.nativeEndComposition(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/lwawt/macosx/CInputMethod.nativeGetCurrentInputMethodInfo()Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn native_get_current_input_method_info<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CInputMethod.nativeGetCurrentInputMethodInfo()Ljava/lang/String;"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/lwawt/macosx/CInputMethod.nativeHandleEvent(Lsun/lwawt/LWComponentPeer;Ljava/awt/AWTEvent;)V",
    Any
)]
#[async_method]
pub async fn native_handle_event<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg1 = parameters.pop_reference()?;
    let _arg0 = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun.lwawt.macosx.CInputMethod.nativeHandleEvent(Lsun/lwawt/LWComponentPeer;Ljava/awt/AWTEvent;)V".to_string()).into())
}

#[intrinsic_method("sun/lwawt/macosx/CInputMethod.nativeInit()V", Any)]
#[async_method]
pub async fn native_init<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun.lwawt.macosx.CInputMethod.nativeInit()V".to_string())
            .into(),
    )
}

#[intrinsic_method(
    "sun/lwawt/macosx/CInputMethod.nativeNotifyPeer(JLsun/lwawt/macosx/CInputMethod;)V",
    Any
)]
#[async_method]
pub async fn native_notify_peer<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _im_instance = parameters.pop_reference()?;
    let _native_peer = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CInputMethod.nativeNotifyPeer(JLsun/lwawt/macosx/CInputMethod;)V"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/lwawt/macosx/CInputMethod.setNativeLocale(Ljava/lang/String;Z)Z",
    Any
)]
#[async_method]
pub async fn set_native_locale<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _is_activating = parameters.pop_bool()?;
    let _locale = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CInputMethod.setNativeLocale(Ljava/lang/String;Z)Z".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_native_locale() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_native_locale(thread, Parameters::default()).await;
        assert_eq!(
            "sun.lwawt.macosx.CInputMethod.getNativeLocale()Ljava/util/Locale;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_end_composition() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_end_composition(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.lwawt.macosx.CInputMethod.nativeEndComposition(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_get_current_input_method_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_get_current_input_method_info(thread, Parameters::default()).await;
        assert_eq!(
            "sun.lwawt.macosx.CInputMethod.nativeGetCurrentInputMethodInfo()Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_handle_event() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_handle_event(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun.lwawt.macosx.CInputMethod.nativeHandleEvent(Lsun/lwawt/LWComponentPeer;Ljava/awt/AWTEvent;)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_init() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_init(thread, Parameters::default()).await;
        assert_eq!(
            "sun.lwawt.macosx.CInputMethod.nativeInit()V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_notify_peer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_notify_peer(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun.lwawt.macosx.CInputMethod.nativeNotifyPeer(JLsun/lwawt/macosx/CInputMethod;)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_set_native_locale() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_native_locale(
            thread,
            Parameters::new(vec![Value::Object(None), Value::from(false)]),
        )
        .await;
        assert_eq!(
            "sun.lwawt.macosx.CInputMethod.setNativeLocale(Ljava/lang/String;Z)Z",
            result.unwrap_err().to_string()
        );
    }
}
