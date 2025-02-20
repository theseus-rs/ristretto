use crate::Result;
use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "java/lang/Compiler";

/// Register all native methods for `java.lang.Compiler`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "command",
        "(Ljava/lang/Object;)Ljava/lang/Object;",
        command,
    );
    registry.register(
        CLASS_NAME,
        "compileClass",
        "(Ljava/lang/Class;)Z",
        compile_class,
    );
    registry.register(
        CLASS_NAME,
        "compileClasses",
        "(Ljava/lang/String;)Z",
        compile_classes,
    );
    registry.register(CLASS_NAME, "disable", "()V", disable);
    registry.register(CLASS_NAME, "enable", "()V", enable);
    registry.register(CLASS_NAME, "initialize", "()V", initialize);
    registry.register(CLASS_NAME, "registerNatives", "()V", register_natives);
}

#[async_recursion(?Send)]
async fn command(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.Compiler.command(Ljava/lang/Object;)Ljava/lang/Object;")
}

#[async_recursion(?Send)]
async fn compile_class(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.Compiler.compileClass(Ljava/lang/Class;)Z")
}

#[async_recursion(?Send)]
async fn compile_classes(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.Compiler.compileClasses(Ljava/lang/String;)Z")
}

#[async_recursion(?Send)]
async fn disable(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.Compiler.disable()V")
}

#[async_recursion(?Send)]
async fn enable(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.Compiler.enable()V")
}

#[async_recursion(?Send)]
async fn initialize(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn register_natives(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.Compiler.command(Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_command() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = command(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.Compiler.compileClass(Ljava/lang/Class;)Z"
    )]
    async fn test_compile_class() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = compile_class(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.Compiler.compileClasses(Ljava/lang/String;)Z"
    )]
    async fn test_compile_classes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = compile_classes(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.lang.Compiler.disable()V")]
    async fn test_disable() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = disable(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.lang.Compiler.enable()V")]
    async fn test_enable() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = enable(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_initialize() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = initialize(thread, Parameters::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }

    #[tokio::test]
    async fn test_register_natives() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = register_natives(thread, Parameters::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }
}
