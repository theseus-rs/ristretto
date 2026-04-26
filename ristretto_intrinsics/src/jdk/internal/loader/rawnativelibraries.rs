use ristretto_classfile::JAVA_21;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "jdk/internal/loader/RawNativeLibraries.load0(Ljdk/internal/loader/RawNativeLibraries$RawNativeLibraryImpl;Ljava/lang/String;)Z",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn load_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _name = parameters.pop_reference()?;
    let _lib = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("jdk.internal.loader.RawNativeLibraries.load0(Ljdk/internal/loader/RawNativeLibraries$RawNativeLibraryImpl;Ljava/lang/String;)Z".to_string()).into())
}

#[intrinsic_method(
    "jdk/internal/loader/RawNativeLibraries.unload0(Ljava/lang/String;J)V",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn unload_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _address = parameters.pop_long()?;
    let _name = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.internal.loader.RawNativeLibraries.unload0(Ljava/lang/String;J)V".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_load_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = load_0(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "jdk.internal.loader.RawNativeLibraries.load0(Ljdk/internal/loader/RawNativeLibraries$RawNativeLibraryImpl;Ljava/lang/String;)Z",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_unload_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = unload_0(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "jdk.internal.loader.RawNativeLibraries.unload0(Ljava/lang/String;J)V",
            result.unwrap_err().to_string()
        );
    }
}
