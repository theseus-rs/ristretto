use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_20: Version = Version::Java20 { minor: 0 };

/// Register all native methods for `java.io.Console`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/io/Console";
    let java_version = registry.java_version();

    if java_version <= &JAVA_20 {
        registry.register(class_name, "echo", "(Z)Z", echo);
    }

    registry.register(class_name, "encoding", "()Ljava/lang/String;", encoding);
    registry.register(class_name, "istty", "()Z", istty);
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

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::new(&Version::Java20 { minor: 0 }, true);
        register(&mut registry);
        let class_name = "java/io/Console";
        assert!(registry.method(class_name, "echo", "(Z)Z").is_some());
        assert!(registry
            .method(class_name, "encoding", "()Ljava/lang/String;")
            .is_some());
        assert!(registry.method(class_name, "istty", "()Z").is_some());
    }

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
