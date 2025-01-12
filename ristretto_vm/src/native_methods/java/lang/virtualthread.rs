use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_20: Version = Version::Java20 { minor: 0 };
const JAVA_22: Version = Version::Java22 { minor: 0 };

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

    if java_version >= JAVA_22 {
        registry.register(
            class_name,
            "notifyJvmtiDisableSuspend",
            "(Z)V",
            notify_jvmti_disable_suspend,
        );
    }

    registry.register(class_name, "registerNatives", "()V", register_natives);
}

#[async_recursion(?Send)]
async fn notify_jvmti_disable_suspend(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.lang.VirtualThread.notifyJvmtiDisableSuspend(Z)V")
}

#[async_recursion(?Send)]
async fn notify_jvmti_end(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.VirtualThread.notifyJvmtiEnd()V")
}

#[async_recursion(?Send)]
async fn notify_jvmti_hide_frames(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.lang.VirtualThread.notifyJvmtiHideFrames(Z)V")
}

#[async_recursion(?Send)]
async fn notify_jvmti_mount(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.VirtualThread.notifyJvmtiMount(Z)V")
}

#[async_recursion(?Send)]
async fn notify_jvmti_mount_begin(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.lang.VirtualThread.notifyJvmtiMountBegin(Z)V")
}

#[async_recursion(?Send)]
async fn notify_jvmti_mount_end(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.lang.VirtualThread.notifyJvmtiMountEnd(Z)V")
}

#[async_recursion(?Send)]
async fn notify_jvmti_start(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.VirtualThread.notifyJvmtiStart()V")
}

#[async_recursion(?Send)]
async fn notify_jvmti_unmount(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.lang.VirtualThread.notifyJvmtiUnmount(Z)V")
}

#[async_recursion(?Send)]
async fn notify_jvmti_unmount_begin(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.lang.VirtualThread.notifyJvmtiUnmountBegin(Z)V")
}

#[async_recursion(?Send)]
async fn notify_jvmti_unmount_end(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.lang.VirtualThread.notifyJvmtiUnmountEnd(Z)V")
}

#[async_recursion(?Send)]
async fn register_natives(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::new(&Version::Java21 { minor: 0 }, true);
        register(&mut registry);
        let class_name = "java/lang/VirtualThread";
        assert!(registry
            .method(class_name, "notifyJvmtiEnd", "()V")
            .is_some());
        assert!(registry
            .method(class_name, "notifyJvmtiMount", "(Z)V")
            .is_some());
        assert!(registry
            .method(class_name, "notifyJvmtiStart", "()V")
            .is_some());
        assert!(registry
            .method(class_name, "notifyJvmtiUnmount", "(Z)V")
            .is_some());
        assert!(registry
            .method(class_name, "notifyJvmtiHideFrames", "(Z)V")
            .is_some());
        assert!(registry
            .method(class_name, "registerNatives", "()V")
            .is_some());
    }

    #[test]
    fn test_register_java_20() {
        let mut registry = MethodRegistry::new(&Version::Java20 { minor: 0 }, true);
        register(&mut registry);
        let class_name = "java/lang/VirtualThread";
        assert!(registry
            .method(class_name, "notifyJvmtiMountBegin", "(Z)V")
            .is_some());
        assert!(registry
            .method(class_name, "notifyJvmtiMountEnd", "(Z)V")
            .is_some());
        assert!(registry
            .method(class_name, "notifyJvmtiUnmountBegin", "(Z)V")
            .is_some());
        assert!(registry
            .method(class_name, "notifyJvmtiUnmountEnd", "(Z)V")
            .is_some());
    }

    #[test]
    fn test_register_java_22() {
        let mut registry = MethodRegistry::new(&Version::Java22 { minor: 0 }, true);
        register(&mut registry);
        let class_name = "java/lang/VirtualThread";
        assert!(registry
            .method(class_name, "notifyJvmtiDisableSuspend", "(Z)V")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.VirtualThread.notifyJvmtiDisableSuspend(Z)V"
    )]
    async fn test_notify_jvmti_disable_suspend() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = notify_jvmti_disable_suspend(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.lang.VirtualThread.notifyJvmtiEnd()V")]
    async fn test_notify_jvmti_end() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = notify_jvmti_end(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.VirtualThread.notifyJvmtiHideFrames(Z)V"
    )]
    async fn test_notify_jvmti_hide_frames() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = notify_jvmti_hide_frames(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.lang.VirtualThread.notifyJvmtiMount(Z)V")]
    async fn test_notify_jvmti_mount() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = notify_jvmti_mount(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.VirtualThread.notifyJvmtiMountBegin(Z)V"
    )]
    async fn test_notify_jvmti_mount_begin() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = notify_jvmti_mount_begin(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.VirtualThread.notifyJvmtiMountEnd(Z)V"
    )]
    async fn test_notify_jvmti_mount_end() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = notify_jvmti_mount_end(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.lang.VirtualThread.notifyJvmtiStart()V")]
    async fn test_notify_jvmti_start() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = notify_jvmti_start(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.VirtualThread.notifyJvmtiUnmount(Z)V"
    )]
    async fn test_notify_jvmti_unmount() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = notify_jvmti_unmount(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.VirtualThread.notifyJvmtiUnmountBegin(Z)V"
    )]
    async fn test_notify_jvmti_unmount_begin() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = notify_jvmti_unmount_begin(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.VirtualThread.notifyJvmtiUnmountEnd(Z)V"
    )]
    async fn test_notify_jvmti_unmount_end() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = notify_jvmti_unmount_end(thread, Arguments::default()).await;
    }

    #[tokio::test]
    async fn test_register_natives() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = register_natives(thread, Arguments::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }
}
