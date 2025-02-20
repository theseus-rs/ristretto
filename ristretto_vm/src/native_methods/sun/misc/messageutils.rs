use crate::Result;
use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/misc/MessageUtils";

/// Register all native methods for `sun.misc.MessageUtils`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "toStderr", "(Ljava/lang/String;)V", to_stderr);
    registry.register(CLASS_NAME, "toStdout", "(Ljava/lang/String;)V", to_stdout);
}

#[async_recursion(?Send)]
async fn to_stderr(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.misc.MessageUtils.toStderr(Ljava/lang/String;)V")
}

#[async_recursion(?Send)]
async fn to_stdout(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.misc.MessageUtils.toStdout(Ljava/lang/String;)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.misc.MessageUtils.toStderr(Ljava/lang/String;)V"
    )]
    async fn test_to_stderr() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = to_stderr(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.misc.MessageUtils.toStdout(Ljava/lang/String;)V"
    )]
    async fn test_to_stdout() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = to_stdout(thread, Parameters::default()).await;
    }
}
