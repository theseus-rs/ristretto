use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("apple/laf/JRSUIConstants.getPtrForConstant(I)J", Any)]
#[async_method]
pub async fn get_ptr_for_constant<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _constant = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "apple.laf.JRSUIConstants.getPtrForConstant(I)J".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_ptr_for_constant() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_ptr_for_constant(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "apple.laf.JRSUIConstants.getPtrForConstant(I)J",
            result.unwrap_err().to_string()
        );
    }
}
