use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/awt/PlatformGraphicsInfo";

/// Register all native methods for `sun.awt.PlatformGraphicsInfo`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "isInAquaSession", "()Z", is_in_aqua_session);
}

#[async_recursion(?Send)]
async fn is_in_aqua_session(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.PlatformGraphicsInfo.isInAquaSession()Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.awt.PlatformGraphicsInfo.isInAquaSession()Z"
    )]
    async fn test_is_in_aqua_session() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_in_aqua_session(thread, Parameters::default()).await;
    }
}
