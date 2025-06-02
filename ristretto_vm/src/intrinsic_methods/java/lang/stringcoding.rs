use crate::Result;
use crate::intrinsic_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "java/lang/StringCoding";

/// Register all intrinsic methods for `java.lang.StringCoding`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "err", "(Ljava/lang/String;)V", err);
}

#[async_recursion(?Send)]
async fn err(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.StringCoding.err(Ljava/lang/String;)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.StringCoding.err(Ljava/lang/String;)V"
    )]
    async fn test_err() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = err(thread, Parameters::default()).await;
    }
}
