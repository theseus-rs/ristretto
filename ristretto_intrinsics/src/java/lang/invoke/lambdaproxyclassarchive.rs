use ristretto_classfile::JAVA_17;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "java/lang/invoke/LambdaProxyClassArchive.addToArchive(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/invoke/MethodType;Ljava/lang/invoke/MethodType;Ljava/lang/invoke/MemberName;Ljava/lang/invoke/MethodType;Ljava/lang/Class;)V",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn add_to_archive<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("java.lang.invoke.LambdaProxyClassArchive.addToArchive(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/invoke/MethodType;Ljava/lang/invoke/MethodType;Ljava/lang/invoke/MemberName;Ljava/lang/invoke/MethodType;Ljava/lang/Class;)V".to_string()).into())
}

#[intrinsic_method(
    "java/lang/invoke/LambdaProxyClassArchive.findFromArchive(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/invoke/MethodType;Ljava/lang/invoke/MethodType;Ljava/lang/invoke/MemberName;Ljava/lang/invoke/MethodType;)Ljava/lang/Class;",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn find_from_archive<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("java.lang.invoke.LambdaProxyClassArchive.findFromArchive(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/invoke/MethodType;Ljava/lang/invoke/MethodType;Ljava/lang/invoke/MemberName;Ljava/lang/invoke/MethodType;)Ljava/lang/Class;".to_string()).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_to_archive() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = add_to_archive(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_find_from_archive() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = find_from_archive(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
