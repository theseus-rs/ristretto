use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_8: Version = Version::Java8 { minor: 0 };

/// Register all native methods for `sun.lwawt.macosx.NSEvent`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/lwawt/macosx/NSEvent";
    let java_version = registry.java_version();

    if java_version <= &JAVA_8 {
        registry.register(class_name, "nsToJavaChar", "(CI)C", ns_to_java_char);
        registry.register(
            class_name,
            "nsToJavaKeyModifiers",
            "(I)I",
            ns_to_java_key_modifiers,
        );
        registry.register(
            class_name,
            "nsToJavaMouseModifiers",
            "(II)I",
            ns_to_java_mouse_modifiers,
        );
    } else {
        registry.register(class_name, "nsToJavaChar", "(CIZ)C", ns_to_java_char);
        registry.register(
            class_name,
            "nsToJavaModifiers",
            "(I)I",
            ns_to_java_modifiers,
        );
    }

    registry.register(
        class_name,
        "nsKeyModifiersToJavaKeyInfo",
        "([I[I)V",
        ns_key_modifiers_to_java_key_info,
    );
    registry.register(class_name, "nsToJavaChar", "(CIZ)C", ns_to_java_char);
    registry.register(
        class_name,
        "nsToJavaKeyInfo",
        "([I[I)Z",
        ns_to_java_key_info,
    );
    registry.register(
        class_name,
        "nsToJavaModifiers",
        "(I)I",
        ns_to_java_modifiers,
    );
}

#[async_recursion(?Send)]
async fn ns_key_modifiers_to_java_key_info(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.NSEvent.nsKeyModifiersToJavaKeyInfo([I[I)V")
}

#[async_recursion(?Send)]
async fn ns_to_java_char(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.NSEvent.nsToJavaChar(CIZ)C")
}

#[async_recursion(?Send)]
async fn ns_to_java_key_info(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.NSEvent.nsToJavaKeyInfo([I[I)Z")
}

#[async_recursion(?Send)]
async fn ns_to_java_key_modifiers(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.NSEvent.nsToJavaKeyModifiers(I)I")
}

#[async_recursion(?Send)]
async fn ns_to_java_modifiers(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.NSEvent.nsToJavaModifiers(I)I")
}

#[async_recursion(?Send)]
async fn ns_to_java_mouse_modifiers(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.NSEvent.nsToJavaMouseModifiers(II)I")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/lwawt/macosx/NSEvent";
        assert!(registry
            .method(class_name, "nsToJavaChar", "(CI)C")
            .is_some());
        assert!(registry
            .method(class_name, "nsToJavaKeyModifiers", "(I)I")
            .is_some());
        assert!(registry
            .method(class_name, "nsToJavaMouseModifiers", "(II)I")
            .is_some());
        assert!(registry
            .method(class_name, "nsKeyModifiersToJavaKeyInfo", "([I[I)V")
            .is_some());
        assert!(registry
            .method(class_name, "nsToJavaChar", "(CIZ)C")
            .is_some());
        assert!(registry
            .method(class_name, "nsToJavaKeyInfo", "([I[I)Z")
            .is_some());
        assert!(registry
            .method(class_name, "nsToJavaModifiers", "(I)I")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.NSEvent.nsKeyModifiersToJavaKeyInfo([I[I)V")]
    async fn test_ns_key_modifiers_to_java_key_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ns_key_modifiers_to_java_key_info(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.NSEvent.nsToJavaChar(CIZ)C")]
    async fn test_ns_to_java_char() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ns_to_java_char(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.NSEvent.nsToJavaKeyInfo([I[I)Z")]
    async fn test_ns_to_java_key_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ns_to_java_key_info(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.NSEvent.nsToJavaKeyModifiers(I)I")]
    async fn test_ns_to_java_key_modifiers() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ns_to_java_key_modifiers(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.NSEvent.nsToJavaModifiers(I)I")]
    async fn test_ns_to_java_modifiers() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ns_to_java_modifiers(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.NSEvent.nsToJavaMouseModifiers(II)I")]
    async fn test_ns_to_java_mouse_modifiers() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ns_to_java_mouse_modifiers(thread, Arguments::default()).await;
    }
}
