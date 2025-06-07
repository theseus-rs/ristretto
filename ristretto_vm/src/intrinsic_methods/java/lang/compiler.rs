use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "java/lang/Compiler.command(Ljava/lang/Object;)Ljava/lang/Object;",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn command(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.Compiler.command(Ljava/lang/Object;)Ljava/lang/Object;")
}

#[intrinsic_method(
    "java/lang/Compiler.compileClass(Ljava/lang/Class;)Z",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn compile_class(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.Compiler.compileClass(Ljava/lang/Class;)Z")
}

#[intrinsic_method(
    "java/lang/Compiler.compileClasses(Ljava/lang/String;)Z",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn compile_classes(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.Compiler.compileClasses(Ljava/lang/String;)Z")
}

#[intrinsic_method("java/lang/Compiler.disable()V", LessThanOrEqual(JAVA_8))]
#[async_recursion(?Send)]
pub(crate) async fn disable(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.Compiler.disable()V")
}

#[intrinsic_method("java/lang/Compiler.enable()V", LessThanOrEqual(JAVA_8))]
#[async_recursion(?Send)]
pub(crate) async fn enable(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.Compiler.enable()V")
}

#[intrinsic_method("java/lang/Compiler.initialize()V", LessThanOrEqual(JAVA_8))]
#[async_recursion(?Send)]
pub(crate) async fn initialize(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("java/lang/Compiler.registerNatives()V", LessThanOrEqual(JAVA_8))]
#[async_recursion(?Send)]
pub(crate) async fn register_natives(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
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
