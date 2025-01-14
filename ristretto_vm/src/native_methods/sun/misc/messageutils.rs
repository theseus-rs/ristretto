use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.misc.MessageUtils`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/misc/MessageUtils";
    registry.register(class_name, "toStderr", "(Ljava/lang/String;)V", to_stderr);
    registry.register(class_name, "toStdout", "(Ljava/lang/String;)V", to_stdout);
}

#[async_recursion(?Send)]
async fn to_stderr(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.misc.MessageUtils.toStderr(Ljava/lang/String;)V")
}

#[async_recursion(?Send)]
async fn to_stdout(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.misc.MessageUtils.toStdout(Ljava/lang/String;)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/misc/MessageUtils";
        assert!(registry
            .method(class_name, "toStderr", "(Ljava/lang/String;)V")
            .is_some());
        assert!(registry
            .method(class_name, "toStdout", "(Ljava/lang/String;)V")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.misc.MessageUtils.toStderr(Ljava/lang/String;)V")]
    async fn test_to_stderr() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = to_stderr(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.misc.MessageUtils.toStdout(Ljava/lang/String;)V")]
    async fn test_to_stdout() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = to_stdout(thread, Arguments::default()).await;
    }
}
