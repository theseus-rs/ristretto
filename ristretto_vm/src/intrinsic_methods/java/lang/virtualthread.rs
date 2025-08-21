use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::{Equal, GreaterThan, GreaterThanOrEqual};
use ristretto_classfile::{JAVA_17, JAVA_21, JAVA_25};
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "java/lang/VirtualThread.notifyJvmtiDisableSuspend(Z)V",
    GreaterThanOrEqual(JAVA_25)
)]
#[async_recursion(?Send)]
pub(crate) async fn notify_jvmti_disable_suspend(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.VirtualThread.notifyJvmtiDisableSuspend(Z)V")
}

#[intrinsic_method("java/lang/VirtualThread.notifyJvmtiEnd()V", GreaterThan(JAVA_17))]
#[async_recursion(?Send)]
pub(crate) async fn notify_jvmti_end(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.VirtualThread.notifyJvmtiEnd()V")
}

#[intrinsic_method("java/lang/VirtualThread.notifyJvmtiHideFrames(Z)V", Equal(JAVA_21))]
#[async_recursion(?Send)]
pub(crate) async fn notify_jvmti_hide_frames(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.VirtualThread.notifyJvmtiHideFrames(Z)V")
}

#[intrinsic_method("java/lang/VirtualThread.notifyJvmtiMount(Z)V", GreaterThan(JAVA_17))]
#[async_recursion(?Send)]
pub(crate) async fn notify_jvmti_mount(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.VirtualThread.notifyJvmtiMount(Z)V")
}

#[intrinsic_method("java/lang/VirtualThread.notifyJvmtiStart()V", GreaterThan(JAVA_17))]
#[async_recursion(?Send)]
pub(crate) async fn notify_jvmti_start(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.VirtualThread.notifyJvmtiStart()V")
}

#[intrinsic_method("java/lang/VirtualThread.notifyJvmtiUnmount(Z)V", GreaterThan(JAVA_17))]
#[async_recursion(?Send)]
pub(crate) async fn notify_jvmti_unmount(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.VirtualThread.notifyJvmtiUnmount(Z)V")
}

#[intrinsic_method(
    "java/lang/VirtualThread.postPinnedEvent(Ljava/lang/String;)V",
    GreaterThanOrEqual(JAVA_25)
)]
#[async_recursion(?Send)]
pub(crate) async fn post_pinned_event(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.VirtualThread.postPinnedEvent(Ljava/lang/String;)V")
}

#[intrinsic_method(
    "java/lang/VirtualThread.registerNatives()V",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn register_natives(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "java/lang/VirtualThread.takeVirtualThreadListToUnblock()Ljava/lang/VirtualThread;",
    GreaterThanOrEqual(JAVA_25)
)]
#[async_recursion(?Send)]
pub(crate) async fn take_virtual_thread_list_to_unblock(
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
