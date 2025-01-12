use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.lwawt.macosx.CMenuItem`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/lwawt/macosx/CMenuItem";
    registry.register(class_name, "nativeCreate", "(JZ)J", native_create);
    registry.register(class_name, "nativeSetEnabled", "(JZ)V", native_set_enabled);
    registry.register(class_name, "nativeSetImage", "(JJ)V", native_set_image);
    registry.register(
        class_name,
        "nativeSetLabel",
        "(JLjava/lang/String;CII)V",
        native_set_label,
    );
    registry.register(
        class_name,
        "nativeSetTooltip",
        "(JLjava/lang/String;)V",
        native_set_tooltip,
    );
}

#[async_recursion(?Send)]
async fn native_create(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CMenuItem.nativeCreate(JZ)J")
}

#[async_recursion(?Send)]
async fn native_set_enabled(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CMenuItem.nativeSetEnabled(JZ)V")
}

#[async_recursion(?Send)]
async fn native_set_image(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CMenuItem.nativeSetImage(JJ)V")
}

#[async_recursion(?Send)]
async fn native_set_label(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CMenuItem.nativeSetLabel(JLjava/lang/String;CII)V")
}

#[async_recursion(?Send)]
async fn native_set_tooltip(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CMenuItem.nativeSetTooltip(JLjava/lang/String;)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/lwawt/macosx/CMenuItem";
        assert!(registry
            .method(class_name, "nativeCreate", "(JZ)J")
            .is_some());
        assert!(registry
            .method(class_name, "nativeSetEnabled", "(JZ)V")
            .is_some());
        assert!(registry
            .method(class_name, "nativeSetImage", "(JJ)V")
            .is_some());
        assert!(registry
            .method(class_name, "nativeSetLabel", "(JLjava/lang/String;CII)V")
            .is_some());
        assert!(registry
            .method(class_name, "nativeSetTooltip", "(JLjava/lang/String;)V")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.CMenuItem.nativeCreate(JZ)J")]
    async fn test_native_create() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_create(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.CMenuItem.nativeSetEnabled(JZ)V")]
    async fn test_native_set_enabled() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_enabled(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.CMenuItem.nativeSetImage(JJ)V")]
    async fn test_native_set_image() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_image(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.CMenuItem.nativeSetLabel(JLjava/lang/String;CII)V")]
    async fn test_native_set_label() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_label(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.CMenuItem.nativeSetTooltip(JLjava/lang/String;)V")]
    async fn test_native_set_tooltip() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_tooltip(thread, Arguments::default()).await;
    }
}
