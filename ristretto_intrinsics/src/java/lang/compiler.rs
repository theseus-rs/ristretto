use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result, Thread};
use std::sync::Arc;

#[intrinsic_method(
    "java/lang/Compiler.command(Ljava/lang/Object;)Ljava/lang/Object;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn command<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Object(None)))
}

#[intrinsic_method(
    "java/lang/Compiler.compileClass(Ljava/lang/Class;)Z",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn compile_class<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::from(false)))
}

#[intrinsic_method(
    "java/lang/Compiler.compileClasses(Ljava/lang/String;)Z",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn compile_classes<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::from(false)))
}

#[intrinsic_method("java/lang/Compiler.disable()V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn disable<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("java/lang/Compiler.enable()V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn enable<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("java/lang/Compiler.initialize()V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn initialize<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("java/lang/Compiler.registerNatives()V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn register_natives<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_command() -> Result<()> {
        let (_vm, thread) = crate::test::java8_thread().await?;
        let result = command(thread, Parameters::default()).await?;
        assert_eq!(Some(Value::Object(None)), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_compile_class() -> Result<()> {
        let (_vm, thread) = crate::test::java8_thread().await?;
        let result = compile_class(thread, Parameters::default()).await?;
        assert_eq!(Some(Value::from(false)), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_compile_classes() -> Result<()> {
        let (_vm, thread) = crate::test::java8_thread().await?;
        let result = compile_classes(thread, Parameters::default()).await?;
        assert_eq!(Some(Value::from(false)), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_disable() -> Result<()> {
        let (_vm, thread) = crate::test::java8_thread().await?;
        let result = disable(thread, Parameters::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }

    #[tokio::test]
    async fn test_enable() -> Result<()> {
        let (_vm, thread) = crate::test::java8_thread().await?;
        let result = enable(thread, Parameters::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }

    #[tokio::test]
    async fn test_initialize() -> Result<()> {
        let (_vm, thread) = crate::test::java8_thread().await?;
        let result = initialize(thread, Parameters::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }

    #[tokio::test]
    async fn test_register_natives() -> Result<()> {
        let (_vm, thread) = crate::test::java8_thread().await?;
        let result = register_natives(thread, Parameters::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }
}
