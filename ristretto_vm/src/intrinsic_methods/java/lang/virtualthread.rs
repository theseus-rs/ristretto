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
    Ok(None)
}

#[intrinsic_method("java/lang/VirtualThread.notifyJvmtiEnd()V", GreaterThan(JAVA_17))]
#[async_recursion(?Send)]
pub(crate) async fn notify_jvmti_end(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("java/lang/VirtualThread.notifyJvmtiHideFrames(Z)V", Equal(JAVA_21))]
#[async_recursion(?Send)]
pub(crate) async fn notify_jvmti_hide_frames(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("java/lang/VirtualThread.notifyJvmtiMount(Z)V", GreaterThan(JAVA_17))]
#[async_recursion(?Send)]
pub(crate) async fn notify_jvmti_mount(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("java/lang/VirtualThread.notifyJvmtiStart()V", GreaterThan(JAVA_17))]
#[async_recursion(?Send)]
pub(crate) async fn notify_jvmti_start(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("java/lang/VirtualThread.notifyJvmtiUnmount(Z)V", GreaterThan(JAVA_17))]
#[async_recursion(?Send)]
pub(crate) async fn notify_jvmti_unmount(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
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
    Ok(None)
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
    // Return null to indicate no virtual threads to unblock
    Ok(Some(Value::Object(None)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_notify_jvmti_disable_suspend() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = notify_jvmti_disable_suspend(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_notify_jvmti_end() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = notify_jvmti_end(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_notify_jvmti_hide_frames() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = notify_jvmti_hide_frames(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_notify_jvmti_mount() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = notify_jvmti_mount(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_notify_jvmti_start() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = notify_jvmti_start(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_notify_jvmti_unmount() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = notify_jvmti_unmount(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_post_pinned_event() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = post_pinned_event(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_register_natives() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = register_natives(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_take_virtual_thread_list_to_unblock() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = take_virtual_thread_list_to_unblock(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::Object(None)));
        Ok(())
    }
}
