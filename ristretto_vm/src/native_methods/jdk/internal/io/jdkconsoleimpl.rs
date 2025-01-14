use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `jdk.internal.io.JdkConsoleImpl`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "jdk/internal/io/JdkConsoleImpl";
    registry.register(class_name, "echo", "(Z)Z", echo);
}

#[async_recursion(?Send)]
async fn echo(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.internal.io.JdkConsoleImpl.echo(Z)Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "jdk/internal/io/JdkConsoleImpl";
        assert!(registry.method(class_name, "echo", "(Z)Z").is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "jdk.internal.io.JdkConsoleImpl.echo(Z)Z")]
    async fn test_echo() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = echo(thread, Arguments::default()).await;
    }
}
