use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classfile::{JAVA_8, JAVA_21};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "java/lang/SecurityManager.classDepth(Ljava/lang/String;)I",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn class_depth<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.SecurityManager.classDepth(Ljava/lang/String;)I")
}

#[intrinsic_method(
    "java/lang/SecurityManager.classLoaderDepth0()I",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn class_loader_depth_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.SecurityManager.classLoaderDepth0()I")
}

#[intrinsic_method(
    "java/lang/SecurityManager.currentClassLoader0()Ljava/lang/ClassLoader;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn current_class_loader_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.SecurityManager.currentClassLoader0()Ljava/lang/ClassLoader;")
}

#[intrinsic_method(
    "java/lang/SecurityManager.currentLoadedClass0()Ljava/lang/Class;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn current_loaded_class_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.SecurityManager.currentLoadedClass0()Ljava/lang/Class;")
}

#[intrinsic_method(
    "java/lang/SecurityManager.getClassContext()[Ljava/lang/Class;",
    LessThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn get_class_context<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.SecurityManager.getClassContext()[Ljava/lang/Class;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.SecurityManager.classDepth(Ljava/lang/String;)I"
    )]
    async fn test_class_depth() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = class_depth(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.SecurityManager.classLoaderDepth0()I"
    )]
    async fn test_class_loader_depth_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = class_loader_depth_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.SecurityManager.currentClassLoader0()Ljava/lang/ClassLoader;"
    )]
    async fn test_current_class_loader_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = current_class_loader_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.SecurityManager.currentLoadedClass0()Ljava/lang/Class;"
    )]
    async fn test_current_loaded_class_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = current_loaded_class_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.SecurityManager.getClassContext()[Ljava/lang/Class;"
    )]
    async fn test_get_class_context() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_class_context(thread, Parameters::default()).await;
    }
}
