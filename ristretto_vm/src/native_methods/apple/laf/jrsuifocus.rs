use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "apple/laf/JRSUIFocus";

/// Register all native methods for `apple.laf.JRSUIFocus`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "beginNativeFocus", "(JI)I", begin_native_focus);
    registry.register(CLASS_NAME, "endNativeFocus", "(J)I", end_native_focus);
}

#[async_recursion(?Send)]
async fn begin_native_focus(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("apple.laf.JRSUIFocus.beginNativeFocus(JI)I")
}

#[async_recursion(?Send)]
async fn end_native_focus(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("apple.laf.JRSUIFocus.endNativeFocus(J)I")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: apple.laf.JRSUIFocus.beginNativeFocus(JI)I")]
    async fn test_begin_native_focus() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = begin_native_focus(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: apple.laf.JRSUIFocus.endNativeFocus(J)I")]
    async fn test_end_native_focus() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = end_native_focus(thread, Arguments::default()).await;
    }
}
