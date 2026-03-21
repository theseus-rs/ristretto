use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classfile::{JAVA_8, JAVA_21};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "java/lang/SecurityManager.classDepth(Ljava/lang/String;)I",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn class_depth<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "java.lang.SecurityManager.classDepth(Ljava/lang/String;)I".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "java/lang/SecurityManager.classLoaderDepth0()I",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn class_loader_depth_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "java.lang.SecurityManager.classLoaderDepth0()I".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "java/lang/SecurityManager.currentClassLoader0()Ljava/lang/ClassLoader;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn current_class_loader_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "java.lang.SecurityManager.currentClassLoader0()Ljava/lang/ClassLoader;".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "java/lang/SecurityManager.currentLoadedClass0()Ljava/lang/Class;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn current_loaded_class_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "java.lang.SecurityManager.currentLoadedClass0()Ljava/lang/Class;".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "java/lang/SecurityManager.getClassContext()[Ljava/lang/Class;",
    LessThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn get_class_context<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "java.lang.SecurityManager.getClassContext()[Ljava/lang/Class;".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_class_depth() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = class_depth(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_class_loader_depth_0() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = class_loader_depth_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_current_class_loader_0() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = current_class_loader_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_current_loaded_class_0() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = current_loaded_class_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_class_context() {
        let (_vm, thread) = crate::test::java21_thread().await.expect("thread");
        let result = get_class_context(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
