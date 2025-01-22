use crate::native_methods::registry::{MethodRegistry, JAVA_21};
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/nio/fs/BsdNativeDispatcher";

/// Register all native methods for `sun.nio.fs.BsdNativeDispatcher`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() >= JAVA_21 {
        registry.register(CLASS_NAME, "clonefile0", "(JJI)I", clonefile_0);
        registry.register(CLASS_NAME, "fsetattrlist0", "(IIJJJJ)V", fsetattrlist_0);
        registry.register(CLASS_NAME, "setattrlist0", "(JIJJJJ)V", setattrlist_0);
    }

    registry.register(CLASS_NAME, "endfsstat", "(J)V", endfsstat);
    registry.register(
        CLASS_NAME,
        "fsstatEntry",
        "(JLsun/nio/fs/UnixMountEntry;)I",
        fsstat_entry,
    );
    registry.register(CLASS_NAME, "getfsstat", "()J", getfsstat);
    registry.register(CLASS_NAME, "getmntonname0", "(J)[B", getmntonname_0);
    registry.register(CLASS_NAME, "initIDs", "()V", init_ids);
}

#[async_recursion(?Send)]
async fn clonefile_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.fs.BsdNativeDispatcher.clonefile0(JJI)I");
}

#[async_recursion(?Send)]
async fn endfsstat(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.fs.BsdNativeDispatcher.endfsstat(J)V");
}

#[async_recursion(?Send)]
async fn fsetattrlist_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.fs.BsdNativeDispatcher.fsetattrlist0(IIJJJJ)V");
}

#[async_recursion(?Send)]
async fn fsstat_entry(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.fs.BsdNativeDispatcher.fsstatEntry(JLsun/nio/fs/UnixMountEntry;)I");
}

#[async_recursion(?Send)]
async fn getfsstat(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.fs.BsdNativeDispatcher.getfsstat()J");
}

#[async_recursion(?Send)]
async fn getmntonname_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.fs.BsdNativeDispatcher.getmntonname0(J)[B");
}

#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn setattrlist_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
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
