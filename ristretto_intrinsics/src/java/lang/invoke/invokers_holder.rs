//! Intrinsic methods for `java/lang/invoke/Invokers$Holder`.
//!
//! These methods are used for dynamic method invocation in the JVM.

use super::methodhandle::dispatch_holder_method;
use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

/// Helper function for holder method implementations.
/// Dispatches the method call through the `LambdaForm` interpreter.
async fn holder_method_stub<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    method_name: &str,
    parameters: Parameters,
) -> Result<Option<Value>> {
    let arguments: Vec<Value> = parameters.into_vec();
    let result = dispatch_holder_method(thread, method_name, arguments).await?;
    Ok(Some(result))
}

#[intrinsic_method(
    "java/lang/invoke/Invokers$Holder.invoker([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn invoker<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    holder_method_stub(thread, "invoker", parameters).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use ristretto_types::Error;

    #[tokio::test]
    async fn test_invoker_requires_method_handle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = invoker(thread, Parameters::default()).await;
        // With no arguments, dispatch_holder_method should return an error
        assert!(matches!(result, Err(Error::InternalError(_))));
    }
}
