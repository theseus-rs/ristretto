use crate::Result;
use crate::native_methods::registry::{JAVA_11, MethodRegistry};
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "jdk/internal/misc/VM";

/// Register all native methods for `jdk.internal.misc.VM`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() <= JAVA_11 {
        registry.register(
            CLASS_NAME,
            "initializeFromArchive",
            "(Ljava/lang/Class;)V",
            initialize_from_archive,
        );
    }

    registry.register(
        CLASS_NAME,
        "getNanoTimeAdjustment",
        "(J)J",
        get_nano_time_adjustment,
    );
    registry.register(
        CLASS_NAME,
        "getRuntimeArguments",
        "()[Ljava/lang/String;",
        get_runtime_arguments,
    );
    registry.register(CLASS_NAME, "getegid", "()J", getegid);
    registry.register(CLASS_NAME, "geteuid", "()J", geteuid);
    registry.register(CLASS_NAME, "getgid", "()J", getgid);
    registry.register(CLASS_NAME, "getuid", "()J", getuid);
    registry.register(CLASS_NAME, "initialize", "()V", initialize);
    registry.register(
        CLASS_NAME,
        "latestUserDefinedLoader0",
        "()Ljava/lang/ClassLoader;",
        latest_user_defined_loader_0,
    );
}

#[async_recursion(?Send)]
async fn get_nano_time_adjustment(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.VM.getNanoTimeAdjustment(J)J")
}

#[async_recursion(?Send)]
async fn get_runtime_arguments(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.VM.getRuntimeArguments()[Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn getegid(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.VM.getegid()J")
}

#[async_recursion(?Send)]
async fn geteuid(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.VM.geteuid()J")
}

#[async_recursion(?Send)]
async fn getgid(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.VM.getgid()J")
}

#[async_recursion(?Send)]
async fn getuid(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.VM.getuid()J")
}

#[async_recursion(?Send)]
async fn initialize(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn initialize_from_archive(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn latest_user_defined_loader_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.VM.latestUserDefinedLoader0()Ljava/lang/ClassLoader;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.misc.VM.getNanoTimeAdjustment(J)J"
    )]
    async fn test_get_nano_time_adjustment() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_nano_time_adjustment(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.misc.VM.getRuntimeArguments()[Ljava/lang/String;"
    )]
    async fn test_get_runtime_arguments() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_runtime_arguments(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.internal.misc.VM.getegid()J")]
    async fn test_getegid() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = getegid(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.internal.misc.VM.geteuid()J")]
    async fn test_geteuid() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = geteuid(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.internal.misc.VM.getgid()J")]
    async fn test_getgid() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = getgid(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.internal.misc.VM.getuid()J")]
    async fn test_getuid() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = getuid(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_initialize() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let value = initialize(thread, Parameters::default()).await?;
        assert_eq!(value, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_initialize_from_archive() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let value = initialize_from_archive(thread, Parameters::default()).await?;
        assert_eq!(value, None);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.misc.VM.latestUserDefinedLoader0()Ljava/lang/ClassLoader;"
    )]
    async fn test_latest_user_defined_loader_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = latest_user_defined_loader_0(thread, Parameters::default()).await;
    }
}
