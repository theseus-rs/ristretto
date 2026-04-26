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
    "jdk/internal/loader/NativeLibrary.findEntry0(JLjava/lang/String;)J",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn find_entry_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _name = parameters.pop_reference()?;
    let _handle = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.internal.loader.NativeLibrary.findEntry0(JLjava/lang/String;)J".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_find_entry_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = find_entry_0(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "jdk.internal.loader.NativeLibrary.findEntry0(JLjava/lang/String;)J",
            result.unwrap_err().to_string()
        );
    }
}
