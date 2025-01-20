use crate::native_methods::registry::{MethodRegistry, JAVA_8};
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/lwawt/macosx/NSEvent";

/// Register all native methods for `sun.lwawt.macosx.NSEvent`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() <= JAVA_8 {
        registry.register(CLASS_NAME, "nsToJavaChar", "(CI)C", ns_to_java_char);
        registry.register(
            CLASS_NAME,
            "nsToJavaKeyModifiers",
            "(I)I",
            ns_to_java_key_modifiers,
        );
        registry.register(
            CLASS_NAME,
            "nsToJavaMouseModifiers",
            "(II)I",
            ns_to_java_mouse_modifiers,
        );
    } else {
        registry.register(CLASS_NAME, "nsToJavaChar", "(CIZ)C", ns_to_java_char);
        registry.register(
            CLASS_NAME,
            "nsToJavaModifiers",
            "(I)I",
            ns_to_java_modifiers,
        );
    }

    registry.register(
        CLASS_NAME,
        "nsKeyModifiersToJavaKeyInfo",
        "([I[I)V",
        ns_key_modifiers_to_java_key_info,
    );
    registry.register(
        CLASS_NAME,
        "nsToJavaKeyInfo",
        "([I[I)Z",
        ns_to_java_key_info,
    );
}

#[async_recursion(?Send)]
async fn ns_key_modifiers_to_java_key_info(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.NSEvent.nsKeyModifiersToJavaKeyInfo([I[I)V")
}

#[async_recursion(?Send)]
async fn ns_to_java_char(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.NSEvent.nsToJavaChar(CIZ)C")
}

#[async_recursion(?Send)]
async fn ns_to_java_key_info(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.NSEvent.nsToJavaKeyInfo([I[I)Z")
}

#[async_recursion(?Send)]
async fn ns_to_java_key_modifiers(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.NSEvent.nsToJavaKeyModifiers(I)I")
}

#[async_recursion(?Send)]
async fn ns_to_java_modifiers(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.NSEvent.nsToJavaModifiers(I)I")
}

#[async_recursion(?Send)]
async fn ns_to_java_mouse_modifiers(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.NSEvent.nsToJavaMouseModifiers(II)I")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.NSEvent.nsKeyModifiersToJavaKeyInfo([I[I)V"
    )]
    async fn test_ns_key_modifiers_to_java_key_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ns_key_modifiers_to_java_key_info(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.lwawt.macosx.NSEvent.nsToJavaChar(CIZ)C")]
    async fn test_ns_to_java_char() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ns_to_java_char(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.NSEvent.nsToJavaKeyInfo([I[I)Z"
    )]
    async fn test_ns_to_java_key_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ns_to_java_key_info(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.NSEvent.nsToJavaKeyModifiers(I)I"
    )]
    async fn test_ns_to_java_key_modifiers() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ns_to_java_key_modifiers(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.NSEvent.nsToJavaModifiers(I)I"
    )]
    async fn test_ns_to_java_modifiers() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ns_to_java_modifiers(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.NSEvent.nsToJavaMouseModifiers(II)I"
    )]
    async fn test_ns_to_java_mouse_modifiers() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ns_to_java_mouse_modifiers(thread, Parameters::default()).await;
    }
}
