use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/nio/ch/PollSelectorImpl";

/// Register all native methods for `sun.nio.ch.PollSelectorImpl`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "poll", "(JII)I", poll);
}

#[async_recursion(?Send)]
async fn poll(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.PollSelectorImpl.poll(JII)I")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.PollSelectorImpl.poll(JII)I")]
    async fn test_poll() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = poll(thread, Parameters::default()).await;
    }
}
