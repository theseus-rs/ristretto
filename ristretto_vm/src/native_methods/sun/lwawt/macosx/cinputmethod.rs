use crate::Result;
use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/lwawt/macosx/CInputMethod";

/// Register all native methods for `sun.lwawt.macosx.CInputMethod`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "getNativeLocale",
        "()Ljava/util/Locale;",
        get_native_locale,
    );
    registry.register(
        CLASS_NAME,
        "nativeEndComposition",
        "(J)V",
        native_end_composition,
    );
    registry.register(
        CLASS_NAME,
        "nativeGetCurrentInputMethodInfo",
        "()Ljava/lang/String;",
        native_get_current_input_method_info,
    );
    registry.register(
        CLASS_NAME,
        "nativeHandleEvent",
        "(Lsun/lwawt/LWComponentPeer;Ljava/awt/AWTEvent;)V",
        native_handle_event,
    );
    registry.register(CLASS_NAME, "nativeInit", "()V", native_init);
    registry.register(
        CLASS_NAME,
        "nativeNotifyPeer",
        "(JLsun/lwawt/macosx/CInputMethod;)V",
        native_notify_peer,
    );
    registry.register(
        CLASS_NAME,
        "setNativeLocale",
        "(Ljava/lang/String;Z)Z",
        set_native_locale,
    );
}

#[async_recursion(?Send)]
async fn get_native_locale(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CInputMethod.getNativeLocale()Ljava/util/Locale;")
}

#[async_recursion(?Send)]
async fn native_end_composition(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CInputMethod.nativeEndComposition(J)V")
}

#[async_recursion(?Send)]
async fn native_get_current_input_method_info(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CInputMethod.nativeGetCurrentInputMethodInfo()Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn native_handle_event(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.lwawt.macosx.CInputMethod.nativeHandleEvent(Lsun/lwawt/LWComponentPeer;Ljava/awt/AWTEvent;)V"
    )
}

#[async_recursion(?Send)]
async fn native_init(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CInputMethod.nativeInit()V")
}

#[async_recursion(?Send)]
async fn native_notify_peer(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CInputMethod.nativeNotifyPeer(JLsun/lwawt/macosx/CInputMethod;)V")
}

#[async_recursion(?Send)]
async fn set_native_locale(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
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
