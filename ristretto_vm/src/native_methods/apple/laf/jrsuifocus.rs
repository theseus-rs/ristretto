use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `apple.laf.JRSUIFocus`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "apple/laf/JRSUIFocus";
    registry.register(class_name, "beginNativeFocus", "(JI)I", begin_native_focus);
    registry.register(class_name, "endNativeFocus", "(J)I", end_native_focus);
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

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "apple/laf/JRSUIFocus";
        assert!(registry
            .method(class_name, "beginNativeFocus", "(JI)I")
            .is_some());
        assert!(registry
            .method(class_name, "endNativeFocus", "(J)I")
            .is_some());
    }

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
