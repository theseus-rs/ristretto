use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.nio.ch.PollSelectorImpl`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/nio/ch/PollSelectorImpl";
    registry.register(class_name, "poll", "(JII)I", poll);
}

#[async_recursion(?Send)]
async fn poll(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.PollSelectorImpl.poll(JII)I")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/nio/ch/PollSelectorImpl";
        assert!(registry.method(class_name, "poll", "(JII)I").is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.PollSelectorImpl.poll(JII)I")]
    async fn test_poll() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = poll(thread, Arguments::default()).await;
    }
}
