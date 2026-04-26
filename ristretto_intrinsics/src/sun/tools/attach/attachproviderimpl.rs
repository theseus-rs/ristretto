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
    "sun/tools/attach/AttachProviderImpl.enumProcesses([II)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn enum_processes<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _max = parameters.pop_int()?;
    let _arr = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/tools/attach/AttachProviderImpl.enumProcesses([II)I".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/tools/attach/AttachProviderImpl.isLibraryLoadedByProcess(Ljava/lang/String;I)Z",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn is_library_loaded_by_process<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _process_id = parameters.pop_int()?;
    let _str = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/tools/attach/AttachProviderImpl.isLibraryLoadedByProcess(Ljava/lang/String;I)Z"
            .to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/tools/attach/AttachProviderImpl.tempPath()Ljava/lang/String;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn temp_path<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/tools/attach/AttachProviderImpl.tempPath()Ljava/lang/String;".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/tools/attach/AttachProviderImpl.volumeFlags(Ljava/lang/String;)J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn volume_flags<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _str = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/tools/attach/AttachProviderImpl.volumeFlags(Ljava/lang/String;)J".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_enum_processes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = enum_processes(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/tools/attach/AttachProviderImpl.enumProcesses([II)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_is_library_loaded_by_process() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = is_library_loaded_by_process(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/tools/attach/AttachProviderImpl.isLibraryLoadedByProcess(Ljava/lang/String;I)Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_temp_path() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = temp_path(thread, Parameters::default()).await;
        assert_eq!(
            "sun/tools/attach/AttachProviderImpl.tempPath()Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_volume_flags() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = volume_flags(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/tools/attach/AttachProviderImpl.volumeFlags(Ljava/lang/String;)J",
            result.unwrap_err().to_string()
        );
    }
}
