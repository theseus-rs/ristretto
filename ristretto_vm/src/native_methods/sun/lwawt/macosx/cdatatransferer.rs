use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.lwawt.macosx.CDataTransferer`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/lwawt/macosx/CDataTransferer";
    registry.register(
        class_name,
        "formatForIndex",
        "(J)Ljava/lang/String;",
        format_for_index,
    );
    registry.register(
        class_name,
        "nativeDragQueryFile",
        "([B)[Ljava/lang/String;",
        native_drag_query_file,
    );
    registry.register(
        class_name,
        "registerFormatWithPasteboard",
        "(Ljava/lang/String;)J",
        register_format_with_pasteboard,
    );
}

#[async_recursion(?Send)]
async fn format_for_index(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CDataTransferer.formatForIndex(J)Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn native_drag_query_file(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CDataTransferer.nativeDragQueryFile([B)[Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn register_format_with_pasteboard(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CDataTransferer.registerFormatWithPasteboard(Ljava/lang/String;)J")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/lwawt/macosx/CDataTransferer";
        assert!(registry
            .method(class_name, "formatForIndex", "(J)Ljava/lang/String;")
            .is_some());
        assert!(registry
            .method(class_name, "nativeDragQueryFile", "([B)[Ljava/lang/String;")
            .is_some());
        assert!(registry
            .method(
                class_name,
                "registerFormatWithPasteboard",
                "(Ljava/lang/String;)J"
            )
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.lwawt.macosx.CDataTransferer.formatForIndex(J)Ljava/lang/String;"
    )]
    async fn test_format_for_index() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = format_for_index(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.lwawt.macosx.CDataTransferer.nativeDragQueryFile([B)[Ljava/lang/String;"
    )]
    async fn test_native_drag_query_file() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_drag_query_file(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.lwawt.macosx.CDataTransferer.registerFormatWithPasteboard(Ljava/lang/String;)J"
    )]
    async fn test_register_format_with_pasteboard() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = register_format_with_pasteboard(thread, Arguments::default()).await;
    }
}
