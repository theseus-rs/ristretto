//! Intrinsic methods for `java/lang/invoke/Invokers$Holder`.
//!
//! These methods are used for dynamic method invocation in the JVM.

use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "java/lang/invoke/Invokers$Holder.invoker([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn invoker(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.Invokers$Holder.invoker([Ljava/lang/Object;)Ljava/lang/Object;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.Invokers$Holder.invoker([Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_invoker() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoker(thread, Parameters::default()).await;
    }
}
