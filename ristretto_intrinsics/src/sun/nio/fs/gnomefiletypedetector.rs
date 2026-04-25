use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::Equal;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/nio/fs/GnomeFileTypeDetector.initializeGio()Z", Equal(JAVA_8))]
#[async_method]
pub async fn initialize_gio<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/GnomeFileTypeDetector.initializeGio()Z".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/nio/fs/GnomeFileTypeDetector.initializeGnomeVfs()Z",
    Equal(JAVA_8)
)]
#[async_method]
pub async fn initialize_gnome_vfs<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/GnomeFileTypeDetector.initializeGnomeVfs()Z".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/nio/fs/GnomeFileTypeDetector.probeUsingGio(J)[B", Equal(JAVA_8))]
#[async_method]
pub async fn probe_using_gio<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _path_address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/GnomeFileTypeDetector.probeUsingGio(J)[B".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/nio/fs/GnomeFileTypeDetector.probeUsingGnomeVfs(J)[B",
    Equal(JAVA_8)
)]
#[async_method]
pub async fn probe_using_gnome_vfs<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _path_address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/GnomeFileTypeDetector.probeUsingGnomeVfs(J)[B".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_initialize_gio() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = initialize_gio(thread, Parameters::default()).await;
        assert_eq!(
            "sun/nio/fs/GnomeFileTypeDetector.initializeGio()Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_initialize_gnome_vfs() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = initialize_gnome_vfs(thread, Parameters::default()).await;
        assert_eq!(
            "sun/nio/fs/GnomeFileTypeDetector.initializeGnomeVfs()Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_probe_using_gio() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = probe_using_gio(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/nio/fs/GnomeFileTypeDetector.probeUsingGio(J)[B",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_probe_using_gnome_vfs() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = probe_using_gnome_vfs(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/nio/fs/GnomeFileTypeDetector.probeUsingGnomeVfs(J)[B",
            result.unwrap_err().to_string()
        );
    }
}
