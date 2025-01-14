use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_20: Version = Version::Java20 { minor: 0 };
const JAVA_21: Version = Version::Java21 { minor: 0 };

/// Register all native methods for `sun.nio.fs.BsdNativeDispatcher`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/nio/fs/BsdNativeDispatcher";
    let java_version = registry.java_version().clone();

    if java_version >= JAVA_20 {
        registry.register(class_name, "clonefile0", "(JJI)I", clonefile_0);
        registry.register(class_name, "setattrlist0", "(JIJJJJ)V", setattrlist_0);
    }

    if java_version >= JAVA_21 {
        registry.register(class_name, "fsetattrlist0", "(IIJJJJ)V", fsetattrlist_0);
    }

    registry.register(class_name, "endfsstat", "(J)V", endfsstat);
    registry.register(
        class_name,
        "fsstatEntry",
        "(JLsun/nio/fs/UnixMountEntry;)I",
        fsstat_entry,
    );
    registry.register(class_name, "getfsstat", "()J", getfsstat);
    registry.register(class_name, "getmntonname0", "(J)[B", getmntonname_0);
    registry.register(class_name, "initIDs", "()V", init_ids);
}

#[async_recursion(?Send)]
async fn clonefile_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.fs.BsdNativeDispatcher.clonefile0(JJI)I");
}

#[async_recursion(?Send)]
async fn endfsstat(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.fs.BsdNativeDispatcher.endfsstat(J)V");
}

#[async_recursion(?Send)]
async fn fsetattrlist_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.fs.BsdNativeDispatcher.fsetattrlist0(IIJJJJ)V");
}

#[async_recursion(?Send)]
async fn fsstat_entry(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.fs.BsdNativeDispatcher.fsstatEntry(JLsun/nio/fs/UnixMountEntry;)I");
}

#[async_recursion(?Send)]
async fn getfsstat(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.fs.BsdNativeDispatcher.getfsstat()J");
}

#[async_recursion(?Send)]
async fn getmntonname_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.fs.BsdNativeDispatcher.getmntonname0(J)[B");
}

#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn setattrlist_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.fs.BsdNativeDispatcher.setattrlist0(JIJJJJ)V");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::new(&Version::Java21 { minor: 0 }, true);
        register(&mut registry);
        let class_name = "sun/nio/fs/BsdNativeDispatcher";
        assert!(registry
            .method(class_name, "clonefile0", "(JJI)I")
            .is_some());
        assert!(registry
            .method(class_name, "setattrlist0", "(JIJJJJ)V")
            .is_some());
        assert!(registry.method(class_name, "endfsstat", "(J)V").is_some());
        assert!(registry
            .method(class_name, "fsstatEntry", "(JLsun/nio/fs/UnixMountEntry;)I")
            .is_some());
        assert!(registry.method(class_name, "getfsstat", "()J").is_some());
        assert!(registry
            .method(class_name, "getmntonname0", "(J)[B")
            .is_some());
        assert!(registry.method(class_name, "initIDs", "()V").is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.fs.BsdNativeDispatcher.clonefile0(JJI)I")]
    async fn test_clonefile_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = clonefile_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.fs.BsdNativeDispatcher.endfsstat(J)V")]
    async fn test_endfsstat() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = endfsstat(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.fs.BsdNativeDispatcher.fsetattrlist0(IIJJJJ)V")]
    async fn test_fsetattrlist_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = fsetattrlist_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.nio.fs.BsdNativeDispatcher.fsstatEntry(JLsun/nio/fs/UnixMountEntry;)I"
    )]
    async fn test_fsstat_entry() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = fsstat_entry(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.fs.BsdNativeDispatcher.getfsstat()J")]
    async fn test_getfsstat() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = getfsstat(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.fs.BsdNativeDispatcher.getmntonname0(J)[B")]
    async fn test_getmntonname_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = getmntonname_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init_ids(thread, Arguments::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.fs.BsdNativeDispatcher.setattrlist0(JIJJJJ)V")]
    async fn test_setattrlist_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = setattrlist_0(thread, Arguments::default()).await;
    }
}
