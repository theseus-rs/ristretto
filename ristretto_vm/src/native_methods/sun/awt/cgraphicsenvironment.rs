use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_8: Version = Version::Java8 { minor: 0 };

/// Register all native methods for `sun.awt.CGraphicsEnvironment`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/awt/CGraphicsEnvironment";
    let java_version = registry.java_version();

    if java_version <= &JAVA_8 {
        registry.register(class_name, "initCocoa", "()V", init_cocoa);
    }

    registry.register(
        class_name,
        "deregisterDisplayReconfiguration",
        "(J)V",
        deregister_display_reconfiguration,
    );
    registry.register(class_name, "getDisplayIDs", "()[I", get_display_i_ds);
    registry.register(class_name, "getMainDisplayID", "()I", get_main_display_id);
    registry.register(
        class_name,
        "registerDisplayReconfiguration",
        "()J",
        register_display_reconfiguration,
    );
}

#[async_recursion(?Send)]
async fn deregister_display_reconfiguration(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.awt.CGraphicsEnvironment.deregisterDisplayReconfiguration(J)V")
}

#[async_recursion(?Send)]
async fn get_display_i_ds(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.awt.CGraphicsEnvironment.getDisplayIDs()[I")
}

#[async_recursion(?Send)]
async fn get_main_display_id(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.awt.CGraphicsEnvironment.getMainDisplayID()I")
}

#[async_recursion(?Send)]
async fn init_cocoa(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.awt.CGraphicsEnvironment.initCocoa()V")
}

#[async_recursion(?Send)]
async fn register_display_reconfiguration(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.awt.CGraphicsEnvironment.registerDisplayReconfiguration()J")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::new(&Version::Java8 { minor: 0 }, true);
        register(&mut registry);
        let class_name = "sun/awt/CGraphicsEnvironment";
        assert!(registry
            .method(class_name, "deregisterDisplayReconfiguration", "(J)V")
            .is_some());
        assert!(registry
            .method(class_name, "getDisplayIDs", "()[I")
            .is_some());
        assert!(registry
            .method(class_name, "getMainDisplayID", "()I")
            .is_some());
        assert!(registry
            .method(class_name, "registerDisplayReconfiguration", "()J")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.awt.CGraphicsEnvironment.deregisterDisplayReconfiguration(J)V")]
    async fn test_deregister_display_reconfiguration() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = deregister_display_reconfiguration(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.awt.CGraphicsEnvironment.getDisplayIDs()[I")]
    async fn test_get_display_i_ds() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_display_i_ds(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.awt.CGraphicsEnvironment.getMainDisplayID()I")]
    async fn test_get_main_display_id() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_main_display_id(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.awt.CGraphicsEnvironment.initCocoa()V")]
    async fn test_init_cocoa() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_cocoa(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.awt.CGraphicsEnvironment.registerDisplayReconfiguration()J")]
    async fn test_register_display_reconfiguration() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = register_display_reconfiguration(thread, Arguments::default()).await;
    }
}
