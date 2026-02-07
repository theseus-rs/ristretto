use ristretto_classfile::VersionSpecification::{Equal, GreaterThan, GreaterThanOrEqual};
use ristretto_classfile::{JAVA_17, JAVA_21, JAVA_25};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "java/lang/VirtualThread.notifyJvmtiDisableSuspend(Z)V",
    GreaterThanOrEqual(JAVA_25)
)]
#[async_method]
pub async fn notify_jvmti_disable_suspend<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("java/lang/VirtualThread.notifyJvmtiEnd()V", GreaterThan(JAVA_17))]
#[async_method]
pub async fn notify_jvmti_end<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("java/lang/VirtualThread.notifyJvmtiHideFrames(Z)V", Equal(JAVA_21))]
#[async_method]
pub async fn notify_jvmti_hide_frames<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("java/lang/VirtualThread.notifyJvmtiMount(Z)V", GreaterThan(JAVA_17))]
#[async_method]
pub async fn notify_jvmti_mount<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("java/lang/VirtualThread.notifyJvmtiStart()V", GreaterThan(JAVA_17))]
#[async_method]
pub async fn notify_jvmti_start<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("java/lang/VirtualThread.notifyJvmtiUnmount(Z)V", GreaterThan(JAVA_17))]
#[async_method]
pub async fn notify_jvmti_unmount<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "java/lang/VirtualThread.postPinnedEvent(Ljava/lang/String;)V",
    GreaterThanOrEqual(JAVA_25)
)]
#[async_method]
pub async fn post_pinned_event<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "java/lang/VirtualThread.registerNatives()V",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn register_natives<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "java/lang/VirtualThread.takeVirtualThreadListToUnblock()Ljava/lang/VirtualThread;",
    GreaterThanOrEqual(JAVA_25)
)]
#[async_method]
pub async fn take_virtual_thread_list_to_unblock<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
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
