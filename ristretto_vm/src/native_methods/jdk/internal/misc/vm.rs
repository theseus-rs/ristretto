use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_11: Version = Version::Java11 { minor: 0 };

/// Register all native methods for `jdk.internal.misc.VM`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "jdk/internal/misc/VM";
    let java_version = registry.java_version();

    if java_version <= &JAVA_11 {
        registry.register(
            class_name,
            "initializeFromArchive",
            "(Ljava/lang/Class;)V",
            initialize_from_archive,
        );
    }

    registry.register(
        class_name,
        "getNanoTimeAdjustment",
        "(J)J",
        get_nano_time_adjustment,
    );
    registry.register(
        class_name,
        "getRuntimeArguments",
        "()[Ljava/lang/String;",
        get_runtime_arguments,
    );
    registry.register(class_name, "getegid", "()J", getegid);
    registry.register(class_name, "geteuid", "()J", geteuid);
    registry.register(class_name, "getgid", "()J", getgid);
    registry.register(class_name, "getuid", "()J", getuid);
    registry.register(class_name, "initialize", "()V", initialize);
    registry.register(
        class_name,
        "latestUserDefinedLoader0",
        "()Ljava/lang/ClassLoader;",
        latest_user_defined_loader_0,
    );
}

#[async_recursion(?Send)]
async fn get_nano_time_adjustment(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.VM.getNanoTimeAdjustment(J)J")
}

#[async_recursion(?Send)]
async fn get_runtime_arguments(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.VM.getRuntimeArguments()[Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn getegid(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.VM.getegid()J")
}

#[async_recursion(?Send)]
async fn geteuid(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.VM.geteuid()J")
}

#[async_recursion(?Send)]
async fn getgid(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.VM.getgid()J")
}

#[async_recursion(?Send)]
async fn getuid(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.VM.getuid()J")
}

#[async_recursion(?Send)]
async fn initialize(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn initialize_from_archive(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn latest_user_defined_loader_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.VM.latestUserDefinedLoader0()Ljava/lang/ClassLoader;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::new(&Version::Java11 { minor: 0 }, true);
        register(&mut registry);
        let class_name = "jdk/internal/misc/VM";
        assert!(registry
            .method(class_name, "getNanoTimeAdjustment", "(J)J")
            .is_some());
        assert!(registry
            .method(class_name, "getRuntimeArguments", "()[Ljava/lang/String;")
            .is_some());
        assert!(registry.method(class_name, "getegid", "()J").is_some());
        assert!(registry.method(class_name, "geteuid", "()J").is_some());
        assert!(registry.method(class_name, "getgid", "()J").is_some());
        assert!(registry.method(class_name, "getuid", "()J").is_some());
        assert!(registry.method(class_name, "initialize", "()V").is_some());
        assert!(registry
            .method(class_name, "initializeFromArchive", "(Ljava/lang/Class;)V")
            .is_some());
        assert!(registry
            .method(
                class_name,
                "latestUserDefinedLoader0",
                "()Ljava/lang/ClassLoader;"
            )
            .is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "jdk.internal.misc.VM.getNanoTimeAdjustment(J)J")]
    async fn test_get_nano_time_adjustment() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_nano_time_adjustment(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "jdk.internal.misc.VM.getRuntimeArguments()[Ljava/lang/String;")]
    async fn test_get_runtime_arguments() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_runtime_arguments(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "jdk.internal.misc.VM.getegid()J")]
    async fn test_getegid() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = getegid(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "jdk.internal.misc.VM.geteuid()J")]
    async fn test_geteuid() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = geteuid(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "jdk.internal.misc.VM.getgid()J")]
    async fn test_getgid() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = getgid(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "jdk.internal.misc.VM.getuid()J")]
    async fn test_getuid() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = getuid(thread, Arguments::default()).await;
    }

    #[tokio::test]
    async fn test_initialize() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let value = initialize(thread, Arguments::default()).await?;
        assert_eq!(value, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_initialize_from_archive() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let value = initialize_from_archive(thread, Arguments::default()).await?;
        assert_eq!(value, None);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "jdk.internal.misc.VM.latestUserDefinedLoader0()Ljava/lang/ClassLoader;"
    )]
    async fn test_latest_user_defined_loader_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = latest_user_defined_loader_0(thread, Arguments::default()).await;
    }
}
