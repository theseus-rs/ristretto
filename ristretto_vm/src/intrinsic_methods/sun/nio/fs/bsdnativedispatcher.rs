use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_21;
use ristretto_classfile::VersionSpecification::{Any, GreaterThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "sun/nio/fs/BsdNativeDispatcher.clonefile0(JJI)I",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn clonefile_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.fs.BsdNativeDispatcher.clonefile0(JJI)I");
}

#[intrinsic_method("sun/nio/fs/BsdNativeDispatcher.endfsstat(J)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn endfsstat(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.fs.BsdNativeDispatcher.endfsstat(J)V");
}

#[intrinsic_method(
    "sun/nio/fs/BsdNativeDispatcher.fsetattrlist0(IIJJJJ)V",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn fsetattrlist_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.fs.BsdNativeDispatcher.fsetattrlist0(IIJJJJ)V");
}

#[intrinsic_method(
    "sun/nio/fs/BsdNativeDispatcher.fsstatEntry(JLsun/nio/fs/UnixMountEntry;)I",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn fsstat_entry(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.fs.BsdNativeDispatcher.fsstatEntry(JLsun/nio/fs/UnixMountEntry;)I");
}

#[intrinsic_method("sun/nio/fs/BsdNativeDispatcher.getfsstat()J", Any)]
#[async_recursion(?Send)]
pub(crate) async fn getfsstat(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.fs.BsdNativeDispatcher.getfsstat()J");
}

#[intrinsic_method("sun/nio/fs/BsdNativeDispatcher.getmntonname0(J)[B", Any)]
#[async_recursion(?Send)]
pub(crate) async fn getmntonname_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.fs.BsdNativeDispatcher.getmntonname0(J)[B");
}

#[intrinsic_method("sun/nio/fs/BsdNativeDispatcher.initIDs()V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn init_ids(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "sun/nio/fs/BsdNativeDispatcher.setattrlist0(JIJJJJ)V",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn setattrlist_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.fs.BsdNativeDispatcher.setattrlist0(JIJJJJ)V");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.fs.BsdNativeDispatcher.clonefile0(JJI)I"
    )]
    async fn test_clonefile_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = clonefile_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.fs.BsdNativeDispatcher.endfsstat(J)V")]
    async fn test_endfsstat() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = endfsstat(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.fs.BsdNativeDispatcher.fsetattrlist0(IIJJJJ)V"
    )]
    async fn test_fsetattrlist_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = fsetattrlist_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.fs.BsdNativeDispatcher.fsstatEntry(JLsun/nio/fs/UnixMountEntry;)I"
    )]
    async fn test_fsstat_entry() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = fsstat_entry(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.fs.BsdNativeDispatcher.getfsstat()J")]
    async fn test_getfsstat() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = getfsstat(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.fs.BsdNativeDispatcher.getmntonname0(J)[B"
    )]
    async fn test_getmntonname_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = getmntonname_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.fs.BsdNativeDispatcher.setattrlist0(JIJJJJ)V"
    )]
    async fn test_setattrlist_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = setattrlist_0(thread, Parameters::default()).await;
    }
}
