use crate::Result;
use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/lwawt/macosx/CDataTransferer";

/// Register all native methods for `sun.lwawt.macosx.CDataTransferer`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "formatForIndex",
        "(J)Ljava/lang/String;",
        format_for_index,
    );
    registry.register(
        CLASS_NAME,
        "nativeDragQueryFile",
        "([B)[Ljava/lang/String;",
        native_drag_query_file,
    );
    registry.register(
        CLASS_NAME,
        "registerFormatWithPasteboard",
        "(Ljava/lang/String;)J",
        register_format_with_pasteboard,
    );
}

#[async_recursion(?Send)]
async fn format_for_index(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CDataTransferer.formatForIndex(J)Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn native_drag_query_file(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CDataTransferer.nativeDragQueryFile([B)[Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn register_format_with_pasteboard(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CDataTransferer.registerFormatWithPasteboard(Ljava/lang/String;)J")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CDataTransferer.formatForIndex(J)Ljava/lang/String;"
    )]
    async fn test_format_for_index() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = format_for_index(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CDataTransferer.nativeDragQueryFile([B)[Ljava/lang/String;"
    )]
    async fn test_native_drag_query_file() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_drag_query_file(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CDataTransferer.registerFormatWithPasteboard(Ljava/lang/String;)J"
    )]
    async fn test_register_format_with_pasteboard() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = register_format_with_pasteboard(thread, Parameters::default()).await;
    }
}
