use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_21;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("jdk/internal/vm/Continuation.doYield()I", GreaterThanOrEqual(JAVA_21))]
#[async_recursion(?Send)]
pub(crate) async fn do_yield(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.vm.Continuation.doYield()I")
}

#[intrinsic_method(
    "jdk/internal/vm/Continuation.enterSpecial(Ljdk/internal/vm/Continuation;ZZ)V",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn enter_special(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.vm.Continuation.enterSpecial(Ljdk/internal/vm/Continuation;ZZ)V")
}

#[intrinsic_method(
    "jdk/internal/vm/Continuation.isPinned0(Ljdk/internal/vm/ContinuationScope;)I",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn is_pinned_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.vm.Continuation.isPinned0(Ljdk/internal/vm/ContinuationScope;)I")
}

#[intrinsic_method("jdk/internal/vm/Continuation.pin()V", GreaterThanOrEqual(JAVA_21))]
#[async_recursion(?Send)]
pub(crate) async fn pin(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("jdk.internal.vm.Continuation.pin()V")
}

#[intrinsic_method(
    "jdk/internal/vm/Continuation.registerNatives()V",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn register_natives(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("jdk/internal/vm/Continuation.unpin()V", GreaterThanOrEqual(JAVA_21))]
#[async_recursion(?Send)]
pub(crate) async fn unpin(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("jdk.internal.vm.Continuation.unpin()V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.internal.vm.Continuation.doYield()I")]
    async fn test_do_yield() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = do_yield(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.vm.Continuation.enterSpecial(Ljdk/internal/vm/Continuation;ZZ)V"
    )]
    async fn test_enter_special() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = enter_special(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.vm.Continuation.isPinned0(Ljdk/internal/vm/ContinuationScope;)I"
    )]
    async fn test_is_pinned_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_pinned_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.internal.vm.Continuation.pin()V")]
    async fn test_pin() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = pin(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_register_natives() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = register_natives(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.internal.vm.Continuation.unpin()V")]
    async fn test_unpin() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = unpin(thread, Parameters::default()).await;
    }
}
