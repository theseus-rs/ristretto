use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.lang.Compiler`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/Compiler";
    registry.register(
        class_name,
        "command",
        "(Ljava/lang/Object;)Ljava/lang/Object;",
        command,
    );
    registry.register(
        class_name,
        "compileClass",
        "(Ljava/lang/Class;)Z",
        compile_class,
    );
    registry.register(
        class_name,
        "compileClasses",
        "(Ljava/lang/String;)Z",
        compile_classes,
    );
    registry.register(class_name, "disable", "()V", disable);
    registry.register(class_name, "enable", "()V", enable);
    registry.register(class_name, "initialize", "()V", initialize);
    registry.register(class_name, "registerNatives", "()V", register_natives);
}

#[async_recursion(?Send)]
async fn command(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.Compiler.command(Ljava/lang/Object;)Ljava/lang/Object;")
}

#[async_recursion(?Send)]
async fn compile_class(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.Compiler.compileClass(Ljava/lang/Class;)Z")
}

#[async_recursion(?Send)]
async fn compile_classes(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.Compiler.compileClasses(Ljava/lang/String;)Z")
}

#[async_recursion(?Send)]
async fn disable(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.Compiler.disable()V")
}

#[async_recursion(?Send)]
async fn enable(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.Compiler.enable()V")
}

#[async_recursion(?Send)]
async fn initialize(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn register_natives(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "java/lang/Compiler";
        assert!(registry
            .method(
                class_name,
                "command",
                "(Ljava/lang/Object;)Ljava/lang/Object;"
            )
            .is_some());
        assert!(registry
            .method(class_name, "compileClass", "(Ljava/lang/Class;)Z")
            .is_some());
        assert!(registry
            .method(class_name, "compileClasses", "(Ljava/lang/String;)Z")
            .is_some());
        assert!(registry.method(class_name, "disable", "()V").is_some());
        assert!(registry.method(class_name, "enable", "()V").is_some());
        assert!(registry.method(class_name, "initialize", "()V").is_some());
        assert!(registry
            .method(class_name, "registerNatives", "()V")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.Compiler.command(Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_command() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = command(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.Compiler.compileClass(Ljava/lang/Class;)Z"
    )]
    async fn test_compile_class() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = compile_class(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.Compiler.compileClasses(Ljava/lang/String;)Z"
    )]
    async fn test_compile_classes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = compile_classes(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.lang.Compiler.disable()V")]
    async fn test_disable() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = disable(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.lang.Compiler.enable()V")]
    async fn test_enable() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = enable(thread, Arguments::default()).await;
    }

    #[tokio::test]
    async fn test_initialize() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = initialize(thread, Arguments::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }

    #[tokio::test]
    async fn test_register_natives() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = register_natives(thread, Arguments::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }
}
