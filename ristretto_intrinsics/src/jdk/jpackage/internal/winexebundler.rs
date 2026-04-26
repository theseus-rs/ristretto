use ristretto_classfile::JAVA_17;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "jdk/jpackage/internal/WinExeBundler.embedMSI(JLjava/lang/String;)I",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn embed_msi<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _jmsi_path = parameters.pop_reference()?;
    let _j_resource_lock = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "jdk/jpackage/internal/WinExeBundler.embedMSI(JLjava/lang/String;)I".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_embed_msi() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = embed_msi(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "jdk/jpackage/internal/WinExeBundler.embedMSI(JLjava/lang/String;)I",
            result.unwrap_err().to_string()
        );
    }
}
