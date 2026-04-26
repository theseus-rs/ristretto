use ristretto_classfile::JAVA_21;
#[cfg(not(target_family = "wasm"))]
use ristretto_classfile::JAVA_25;
#[cfg(not(target_family = "wasm"))]
use ristretto_classfile::VersionSpecification::Equal;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/nio/ch/FileKey.init(Ljava/io/FileDescriptor;)V",
    LessThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn init_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _fd = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.nio.ch.FileKey.init(Ljava/io/FileDescriptor;)V".to_string(),
    )
    .into())
}

#[cfg(target_family = "unix")]
#[intrinsic_method("sun/nio/ch/FileKey.init(Ljava/io/FileDescriptor;[J)V", Equal(JAVA_25))]
#[async_method]
pub async fn init_1<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _finfo = parameters.pop_reference()?;
    let _fd = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.nio.ch.FileKey.init(Ljava/io/FileDescriptor;[J)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/nio/ch/FileKey.initIDs()V", LessThanOrEqual(JAVA_21))]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/ch/FileKey.init(Ljava/io/FileDescriptor;[I)V", Equal(JAVA_25))]
#[async_method]
pub async fn init<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _finfo = parameters.pop_reference()?;
    let _fdo = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/ch/FileKey.init(Ljava/io/FileDescriptor;[I)V".to_string(),
    )
    .into())
}

#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/ch/FileKey.init(Ljava/io/FileDescriptor;[I)V", Equal(JAVA_25))]
#[async_method]
pub async fn init_windows_v25<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _finfo = parameters.pop_reference()?;
    let _fdo = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/ch/FileKey.init(Ljava/io/FileDescriptor;[I)V".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_init_0() {
        let (_vm, thread) = crate::test::java21_thread().await.expect("thread");
        let result = init_0(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun.nio.ch.FileKey.init(Ljava/io/FileDescriptor;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_family = "unix")]
    #[tokio::test]
    async fn test_init_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_1(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun.nio.ch.FileKey.init(Ljava/io/FileDescriptor;[J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::java21_thread().await?;
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_init() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun/nio/ch/FileKey.init(Ljava/io/FileDescriptor;[I)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_init_windows_v25() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_windows_v25(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun/nio/ch/FileKey.init(Ljava/io/FileDescriptor;[I)V",
            result.unwrap_err().to_string()
        );
    }
}
