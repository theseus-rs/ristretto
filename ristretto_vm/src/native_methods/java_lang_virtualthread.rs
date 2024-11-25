use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_20: Version = Version::Java20 { minor: 0 };

/// Register all native methods for `java.lang.VirtualThread`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/VirtualThread";
    let java_version = registry.java_version().clone();

    if java_version <= JAVA_20 {
        registry.register(
            class_name,
            "notifyJvmtiMountBegin",
            "(Z)V",
            notify_jvmti_mount_begin,
        );
        registry.register(
            class_name,
            "notifyJvmtiMountEnd",
            "(Z)V",
            notify_jvmti_mount_end,
        );
        registry.register(
            class_name,
            "notifyJvmtiUnmountBegin",
            "(Z)V",
            notify_jvmti_unmount_begin,
        );
        registry.register(
            class_name,
            "notifyJvmtiUnmountEnd",
            "(Z)V",
            notify_jvmti_unmount_end,
        );
    } else {
        registry.register(class_name, "notifyJvmtiEnd", "()V", notify_jvmti_end);
        // java/lang/VirtualThread.notifyJvmtiMount(Z)V
        // java/lang/VirtualThread.notifyJvmtiStart()V
        // java/lang/VirtualThread.notifyJvmtiUnmount(Z)V
        registry.register(class_name, "notifyJvmtiMount", "(Z)V", notify_jvmti_mount);
        registry.register(class_name, "notifyJvmtiStart", "()V", notify_jvmti_start);
        registry.register(
            class_name,
            "notifyJvmtiUnmount",
            "(Z)V",
            notify_jvmti_unmount,
        );
    }
    if java_version >= JAVA_20 {
        registry.register(
            class_name,
            "notifyJvmtiHideFrames",
            "(Z)V",
            notify_jvmti_hide_frames,
        );
    }

    registry.register(class_name, "registerNatives", "()V", register_natives);
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn notify_jvmti_end(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn notify_jvmti_hide_frames(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn notify_jvmti_mount(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn notify_jvmti_mount_begin(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn notify_jvmti_mount_end(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn notify_jvmti_start(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn notify_jvmti_unmount(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn notify_jvmti_unmount_begin(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn notify_jvmti_unmount_end(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn register_natives(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}
