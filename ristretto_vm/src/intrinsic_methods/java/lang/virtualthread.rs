use crate::Result;
use crate::intrinsic_methods::registry::{JAVA_17, JAVA_21, JAVA_24, MethodRegistry};
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "java/lang/VirtualThread";

/// Register all intrinsic methods for `java.lang.VirtualThread`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() <= JAVA_17 {
        registry.register(
            CLASS_NAME,
            "notifyJvmtiMountBegin",
            "(Z)V",
            notify_jvmti_mount_begin,
        );
        registry.register(
            CLASS_NAME,
            "notifyJvmtiMountEnd",
            "(Z)V",
            notify_jvmti_mount_end,
        );
        registry.register(
            CLASS_NAME,
            "notifyJvmtiUnmountBegin",
            "(Z)V",
            notify_jvmti_unmount_begin,
        );
        registry.register(
            CLASS_NAME,
            "notifyJvmtiUnmountEnd",
            "(Z)V",
            notify_jvmti_unmount_end,
        );
    } else {
        registry.register(CLASS_NAME, "notifyJvmtiEnd", "()V", notify_jvmti_end);
        registry.register(CLASS_NAME, "notifyJvmtiMount", "(Z)V", notify_jvmti_mount);
        registry.register(CLASS_NAME, "notifyJvmtiStart", "()V", notify_jvmti_start);
        registry.register(
            CLASS_NAME,
            "notifyJvmtiUnmount",
            "(Z)V",
            notify_jvmti_unmount,
        );
    }
    if registry.java_major_version() == JAVA_21 {
        registry.register(
            CLASS_NAME,
            "notifyJvmtiHideFrames",
            "(Z)V",
            notify_jvmti_hide_frames,
        );
    }

    if registry.java_major_version() >= JAVA_24 {
        registry.register(
            CLASS_NAME,
            "notifyJvmtiDisableSuspend",
            "(Z)V",
            notify_jvmti_disable_suspend,
        );
        registry.register(
            CLASS_NAME,
            "postPinnedEvent",
            "(Ljava/lang/String;)V",
            post_pinned_event,
        );
        registry.register(
            CLASS_NAME,
            "takeVirtualThreadListToUnblock",
            "()Ljava/lang/VirtualThread;",
            take_virtual_thread_list_to_unblock,
        );
    }

    registry.register(CLASS_NAME, "registerNatives", "()V", register_natives);
}

#[async_recursion(?Send)]
async fn notify_jvmti_disable_suspend(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.VirtualThread.notifyJvmtiDisableSuspend(Z)V")
}

#[async_recursion(?Send)]
async fn notify_jvmti_end(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.VirtualThread.notifyJvmtiEnd()V")
}

#[async_recursion(?Send)]
async fn notify_jvmti_hide_frames(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.VirtualThread.notifyJvmtiHideFrames(Z)V")
}

#[async_recursion(?Send)]
async fn notify_jvmti_mount(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.VirtualThread.notifyJvmtiMount(Z)V")
}

#[async_recursion(?Send)]
async fn notify_jvmti_mount_begin(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.VirtualThread.notifyJvmtiMountBegin(Z)V")
}

#[async_recursion(?Send)]
async fn notify_jvmti_mount_end(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.VirtualThread.notifyJvmtiMountEnd(Z)V")
}

#[async_recursion(?Send)]
async fn notify_jvmti_start(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.VirtualThread.notifyJvmtiStart()V")
}

#[async_recursion(?Send)]
async fn notify_jvmti_unmount(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.VirtualThread.notifyJvmtiUnmount(Z)V")
}

#[async_recursion(?Send)]
async fn notify_jvmti_unmount_begin(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.VirtualThread.notifyJvmtiUnmountBegin(Z)V")
}

#[async_recursion(?Send)]
async fn notify_jvmti_unmount_end(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.VirtualThread.notifyJvmtiUnmountEnd(Z)V")
}

#[async_recursion(?Send)]
async fn post_pinned_event(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.VirtualThread.postPinnedEvent(Ljava/lang/String;)V")
}

#[async_recursion(?Send)]
async fn register_natives(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn take_virtual_thread_list_to_unblock(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.VirtualThread.takeVirtualThreadListToUnblock()Ljava/lang/VirtualThread;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.VirtualThread.notifyJvmtiDisableSuspend(Z)V"
    )]
    async fn test_notify_jvmti_disable_suspend() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = notify_jvmti_disable_suspend(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.lang.VirtualThread.notifyJvmtiEnd()V")]
    async fn test_notify_jvmti_end() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = notify_jvmti_end(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.VirtualThread.notifyJvmtiHideFrames(Z)V"
    )]
    async fn test_notify_jvmti_hide_frames() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = notify_jvmti_hide_frames(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.lang.VirtualThread.notifyJvmtiMount(Z)V")]
    async fn test_notify_jvmti_mount() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = notify_jvmti_mount(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.VirtualThread.notifyJvmtiMountBegin(Z)V"
    )]
    async fn test_notify_jvmti_mount_begin() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = notify_jvmti_mount_begin(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.VirtualThread.notifyJvmtiMountEnd(Z)V"
    )]
    async fn test_notify_jvmti_mount_end() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = notify_jvmti_mount_end(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.lang.VirtualThread.notifyJvmtiStart()V")]
    async fn test_notify_jvmti_start() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = notify_jvmti_start(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.VirtualThread.notifyJvmtiUnmount(Z)V"
    )]
    async fn test_notify_jvmti_unmount() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = notify_jvmti_unmount(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.VirtualThread.notifyJvmtiUnmountBegin(Z)V"
    )]
    async fn test_notify_jvmti_unmount_begin() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = notify_jvmti_unmount_begin(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.VirtualThread.notifyJvmtiUnmountEnd(Z)V"
    )]
    async fn test_notify_jvmti_unmount_end() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = notify_jvmti_unmount_end(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.VirtualThread.postPinnedEvent(Ljava/lang/String;)V"
    )]
    async fn test_post_pinned_event() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = post_pinned_event(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_register_natives() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = register_natives(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.VirtualThread.takeVirtualThreadListToUnblock()Ljava/lang/VirtualThread;"
    )]
    async fn test_take_virtual_thread_list_to_unblock() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = take_virtual_thread_list_to_unblock(thread, Parameters::default()).await;
    }
}
