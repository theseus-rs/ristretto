use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/lwawt/macosx/CMenuItem";

/// Register all native methods for `sun.lwawt.macosx.CMenuItem`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "nativeCreate", "(JZ)J", native_create);
    registry.register(CLASS_NAME, "nativeSetEnabled", "(JZ)V", native_set_enabled);
    registry.register(CLASS_NAME, "nativeSetImage", "(JJ)V", native_set_image);
    registry.register(
        CLASS_NAME,
        "nativeSetLabel",
        "(JLjava/lang/String;CII)V",
        native_set_label,
    );
    registry.register(
        CLASS_NAME,
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

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.lwawt.macosx.CMenuItem.nativeCreate(JZ)J")]
    async fn test_native_create() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_create(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CMenuItem.nativeSetEnabled(JZ)V"
    )]
    async fn test_native_set_enabled() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_enabled(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CMenuItem.nativeSetImage(JJ)V"
    )]
    async fn test_native_set_image() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_image(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CMenuItem.nativeSetLabel(JLjava/lang/String;CII)V"
    )]
    async fn test_native_set_label() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_label(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CMenuItem.nativeSetTooltip(JLjava/lang/String;)V"
    )]
    async fn test_native_set_tooltip() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_tooltip(thread, Arguments::default()).await;
    }
}
