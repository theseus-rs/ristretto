use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.lwawt.macosx.CInputMethod`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/lwawt/macosx/CInputMethod";
    registry.register(
        class_name,
        "getNativeLocale",
        "()Ljava/util/Locale;",
        get_native_locale,
    );
    registry.register(
        class_name,
        "nativeEndComposition",
        "(J)V",
        native_end_composition,
    );
    registry.register(
        class_name,
        "nativeGetCurrentInputMethodInfo",
        "()Ljava/lang/String;",
        native_get_current_input_method_info,
    );
    registry.register(
        class_name,
        "nativeHandleEvent",
        "(Lsun/lwawt/LWComponentPeer;Ljava/awt/AWTEvent;)V",
        native_handle_event,
    );
    registry.register(class_name, "nativeInit", "()V", native_init);
    registry.register(
        class_name,
        "nativeNotifyPeer",
        "(JLsun/lwawt/macosx/CInputMethod;)V",
        native_notify_peer,
    );
    registry.register(
        class_name,
        "setNativeLocale",
        "(Ljava/lang/String;Z)Z",
        set_native_locale,
    );
}

#[async_recursion(?Send)]
async fn get_native_locale(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CInputMethod.getNativeLocale()Ljava/util/Locale;")
}

#[async_recursion(?Send)]
async fn native_end_composition(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CInputMethod.nativeEndComposition(J)V")
}

#[async_recursion(?Send)]
async fn native_get_current_input_method_info(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CInputMethod.nativeGetCurrentInputMethodInfo()Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn native_handle_event(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CInputMethod.nativeHandleEvent(Lsun/lwawt/LWComponentPeer;Ljava/awt/AWTEvent;)V")
}

#[async_recursion(?Send)]
async fn native_init(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CInputMethod.nativeInit()V")
}

#[async_recursion(?Send)]
async fn native_notify_peer(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CInputMethod.nativeNotifyPeer(JLsun/lwawt/macosx/CInputMethod;)V")
}

#[async_recursion(?Send)]
async fn set_native_locale(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CInputMethod.setNativeLocale(Ljava/lang/String;Z)Z")
}
