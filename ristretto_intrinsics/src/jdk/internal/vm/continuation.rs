use ristretto_classfile::JAVA_21;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("jdk/internal/vm/Continuation.doYield()I", GreaterThanOrEqual(JAVA_21))]
#[async_method]
pub async fn do_yield<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("jdk.internal.vm.Continuation.doYield()I".to_string())
            .into(),
    )
}

#[intrinsic_method(
    "jdk/internal/vm/Continuation.enterSpecial(Ljdk/internal/vm/Continuation;ZZ)V",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn enter_special<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.internal.vm.Continuation.enterSpecial(Ljdk/internal/vm/Continuation;ZZ)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "jdk/internal/vm/Continuation.isPinned0(Ljdk/internal/vm/ContinuationScope;)I",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn is_pinned_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.internal.vm.Continuation.isPinned0(Ljdk/internal/vm/ContinuationScope;)I".to_string(),
    )
    .into())
}

#[intrinsic_method("jdk/internal/vm/Continuation.pin()V", GreaterThanOrEqual(JAVA_21))]
#[async_method]
pub async fn pin<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("jdk.internal.vm.Continuation.pin()V".to_string()).into())
}

#[intrinsic_method(
    "jdk/internal/vm/Continuation.registerNatives()V",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn register_natives<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("jdk/internal/vm/Continuation.unpin()V", GreaterThanOrEqual(JAVA_21))]
#[async_method]
pub async fn unpin<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("jdk.internal.vm.Continuation.unpin()V".to_string()).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_do_yield() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = do_yield(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_enter_special() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = enter_special(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_is_pinned_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = is_pinned_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_pin() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = pin(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_register_natives() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = register_natives(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_unpin() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = unpin(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
