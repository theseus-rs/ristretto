use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.lang.VirtualThread`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/VirtualThread";
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
    registry.register(class_name, "registerNatives", "()V", register_natives);
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
