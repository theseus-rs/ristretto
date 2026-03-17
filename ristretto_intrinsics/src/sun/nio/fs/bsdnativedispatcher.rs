use ristretto_classfile::JAVA_21;
use ristretto_classfile::VersionSpecification::{Any, GreaterThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/nio/fs/BsdNativeDispatcher.clonefile0(JJI)I",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn clonefile_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.nio.fs.BsdNativeDispatcher.clonefile0(JJI)I".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/nio/fs/BsdNativeDispatcher.endfsstat(J)V", Any)]
#[async_method]
pub async fn endfsstat<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun.nio.fs.BsdNativeDispatcher.endfsstat(J)V".to_string())
            .into(),
    )
}

#[intrinsic_method(
    "sun/nio/fs/BsdNativeDispatcher.fsetattrlist0(IIJJJJ)V",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn fsetattrlist_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.nio.fs.BsdNativeDispatcher.fsetattrlist0(IIJJJJ)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/nio/fs/BsdNativeDispatcher.fsstatEntry(JLsun/nio/fs/UnixMountEntry;)I",
    Any
)]
#[async_method]
pub async fn fsstat_entry<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.nio.fs.BsdNativeDispatcher.fsstatEntry(JLsun/nio/fs/UnixMountEntry;)I".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/nio/fs/BsdNativeDispatcher.getfsstat()J", Any)]
#[async_method]
pub async fn getfsstat<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun.nio.fs.BsdNativeDispatcher.getfsstat()J".to_string())
            .into(),
    )
}

#[intrinsic_method("sun/nio/fs/BsdNativeDispatcher.getmntonname0(J)[B", Any)]
#[async_method]
pub async fn getmntonname_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.nio.fs.BsdNativeDispatcher.getmntonname0(J)[B".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/nio/fs/BsdNativeDispatcher.initIDs()V", Any)]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "sun/nio/fs/BsdNativeDispatcher.setattrlist0(JIJJJJ)V",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn setattrlist_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.nio.fs.BsdNativeDispatcher.setattrlist0(JIJJJJ)V".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_clonefile_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = clonefile_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_endfsstat() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = endfsstat(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_fsetattrlist_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = fsetattrlist_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_fsstat_entry() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = fsstat_entry(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_getfsstat() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = getfsstat(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_getmntonname_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = getmntonname_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_setattrlist_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = setattrlist_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
