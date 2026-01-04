//! Intrinsic methods for `java/lang/invoke/DelegatingMethodHandle$Holder`.
//!
//! These methods are used for dynamic method invocation in the JVM.

use super::methodhandle::dispatch_holder_method;
use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

/// Helper function for holder method implementations.
/// Dispatches the method call through the `LambdaForm` interpreter.
async fn holder_method_stub(
    thread: Arc<Thread>,
    method_name: &str,
    parameters: Parameters,
) -> Result<Option<Value>> {
    let arguments: Vec<Value> = parameters.into_vec();
    let result = dispatch_holder_method(thread, method_name, arguments).await?;
    Ok(Some(result))
}

#[intrinsic_method(
    "java/lang/invoke/DelegatingMethodHandle$Holder.delegate([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn delegate(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    holder_method_stub(thread, "delegate", parameters).await
}

#[intrinsic_method(
    "java/lang/invoke/DelegatingMethodHandle$Holder.reinvoke_L([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn reinvoke_l(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    holder_method_stub(thread, "reinvoke_L", parameters).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error;

    #[tokio::test]
    async fn test_delegate_requires_method_handle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = delegate(thread, Parameters::default()).await;
        // With no arguments, dispatch_holder_method should return an error
        assert!(matches!(result, Err(Error::InternalError(_))));
    }

    #[tokio::test]
    async fn test_reinvoke_l_requires_method_handle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = reinvoke_l(thread, Parameters::default()).await;
        // With no arguments, dispatch_holder_method should return an error
        assert!(matches!(result, Err(Error::InternalError(_))));
    }
}
