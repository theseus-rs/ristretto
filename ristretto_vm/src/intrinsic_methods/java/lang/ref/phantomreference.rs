use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_24;
use ristretto_classfile::VersionSpecification::{Any, GreaterThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "java/lang/ref/PhantomReference.clear0()V",
    GreaterThanOrEqual(JAVA_24)
)]
#[async_recursion(?Send)]
pub(crate) async fn clear_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.ref.PhantomReference.clear0()V")
}

#[intrinsic_method("java/lang/ref/PhantomReference.refersTo0(Ljava/lang/Object;)Z", Any)]
#[async_recursion(?Send)]
pub(crate) async fn refers_to_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.ref.PhantomReference.refersTo0(Ljava/lang/Object;)Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.lang.ref.PhantomReference.clear0()V")]
    async fn test_clear_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = clear_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.ref.PhantomReference.refersTo0(Ljava/lang/Object;)Z"
    )]
    async fn test_refers_to_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = refers_to_0(thread, Parameters::default()).await;
    }
}
