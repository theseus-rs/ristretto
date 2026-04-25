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
    "sun/security/mscapi/PRNG.generateSeed(JI[B)[B",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn generate_seed<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _seed = parameters.pop_reference()?;
    let _length = parameters.pop_int()?;
    let _ctxt = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/security/mscapi/PRNG.generateSeed(JI[B)[B".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/security/mscapi/PRNG.getContext()J", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn get_context<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/security/mscapi/PRNG.getContext()J".to_string())
            .into(),
    )
}
#[intrinsic_method(
    "sun/security/mscapi/PRNG.releaseContext(J)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn release_context<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _ctxt = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/security/mscapi/PRNG.releaseContext(J)V".to_string())
            .into(),
    )
}
#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_generate_seed() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = generate_seed(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Int(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun/security/mscapi/PRNG.generateSeed(JI[B)[B",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_context() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_context(thread, Parameters::default()).await;
        assert_eq!(
            "sun/security/mscapi/PRNG.getContext()J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_release_context() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = release_context(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/security/mscapi/PRNG.releaseContext(J)V",
            result.unwrap_err().to_string()
        );
    }
}
