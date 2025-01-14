use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.misc.Signal`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/misc/Signal";
    registry.register(
        class_name,
        "findSignal",
        "(Ljava/lang/String;)I",
        find_signal,
    );
    registry.register(class_name, "handle0", "(IJ)J", handle_0);
    registry.register(class_name, "raise0", "(I)V", raise_0);
}

#[async_recursion(?Send)]
async fn find_signal(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.misc.Signal.findSignal(Ljava/lang/String;)I")
}

#[async_recursion(?Send)]
async fn handle_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.misc.Signal.handle0(IJ)J")
}

#[async_recursion(?Send)]
async fn raise_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.misc.Signal.raise0(I)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/misc/Signal";
        assert!(registry
            .method(class_name, "findSignal", "(Ljava/lang/String;)I")
            .is_some());
        assert!(registry.method(class_name, "handle0", "(IJ)J").is_some());
        assert!(registry.method(class_name, "raise0", "(I)V").is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.misc.Signal.findSignal(Ljava/lang/String;)I")]
    async fn test_find_signal() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = find_signal(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.misc.Signal.handle0(IJ)J")]
    async fn test_handle_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = handle_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.misc.Signal.raise0(I)V")]
    async fn test_raise_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = raise_0(thread, Arguments::default()).await;
    }
}
