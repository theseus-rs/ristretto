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
