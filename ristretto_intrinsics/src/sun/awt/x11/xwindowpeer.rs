use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/awt/X11/XWindowPeer.getJvmPID()I", Any)]
#[async_method]
pub async fn get_jvm_pid<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun/awt/X11/XWindowPeer.getJvmPID()I".to_string()).into())
}
#[intrinsic_method("sun/awt/X11/XWindowPeer.getLocalHostname()Ljava/lang/String;", Any)]
#[async_method]
pub async fn get_local_hostname<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XWindowPeer.getLocalHostname()Ljava/lang/String;".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_jvm_pid() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_jvm_pid(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/X11/XWindowPeer.getJvmPID()I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_local_hostname() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_local_hostname(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/X11/XWindowPeer.getLocalHostname()Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }
}
