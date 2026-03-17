use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "com/sun/management/internal/Flag.getAllFlagNames()[Ljava/lang/String;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_all_flag_names<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.management.internal.Flag.getAllFlagNames()[Ljava/lang/String;".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/management/internal/Flag.getFlags([Ljava/lang/String;[Lcom/sun/management/internal/Flag;I)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_flags<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("com.sun.management.internal.Flag.getFlags([Ljava/lang/String;[Lcom/sun/management/internal/Flag;I)I".to_string()).into())
}

#[intrinsic_method(
    "com/sun/management/internal/Flag.getInternalFlagCount()I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_internal_flag_count<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.management.internal.Flag.getInternalFlagCount()I".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/management/internal/Flag.initialize()V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn initialize<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "com/sun/management/internal/Flag.setBooleanValue(Ljava/lang/String;Z)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn set_boolean_value<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.management.internal.Flag.setBooleanValue(Ljava/lang/String;Z)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/management/internal/Flag.setDoubleValue(Ljava/lang/String;D)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn set_double_value<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.management.internal.Flag.setDoubleValue(Ljava/lang/String;D)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/management/internal/Flag.setLongValue(Ljava/lang/String;J)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn set_long_value<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.management.internal.Flag.setLongValue(Ljava/lang/String;J)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/management/internal/Flag.setStringValue(Ljava/lang/String;Ljava/lang/String;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn set_string_value<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.management.internal.Flag.setStringValue(Ljava/lang/String;Ljava/lang/String;)V"
            .to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_all_flag_names() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_all_flag_names(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_flags() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_flags(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_internal_flag_count() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_internal_flag_count(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_initialize() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = initialize(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_set_boolean_value() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_boolean_value(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_double_value() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_double_value(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_long_value() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_long_value(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_string_value() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_string_value(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
