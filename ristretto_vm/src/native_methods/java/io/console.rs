use crate::arguments::Arguments;
use crate::native_methods::registry::{MethodRegistry, JAVA_20};
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "java/io/Console";

/// Register all native methods for `java.io.Console`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() <= JAVA_20 {
        registry.register(CLASS_NAME, "echo", "(Z)Z", echo);
    }

    registry.register(CLASS_NAME, "encoding", "()Ljava/lang/String;", encoding);
    registry.register(CLASS_NAME, "istty", "()Z", istty);
}

#[async_recursion(?Send)]
async fn echo(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.io.Console.echo(Z)Z")
}

#[async_recursion(?Send)]
async fn encoding(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.io.Console.encoding()Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn istty(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.io.Console.istty()Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.io.Console.echo(Z)Z")]
    async fn test_echo() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = echo(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.io.Console.encoding()Ljava/lang/String;")]
    async fn test_encoding() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = encoding(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.io.Console.istty()Z")]
    async fn test_istty() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = istty(thread, Arguments::default()).await;
    }
}
