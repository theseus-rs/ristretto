use crate::Result;
use crate::intrinsic_methods::registry::{JAVA_8, MethodRegistry};
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/awt/CGraphicsEnvironment";

/// Register all intrinsic methods for `sun.awt.CGraphicsEnvironment`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() <= JAVA_8 {
        registry.register(CLASS_NAME, "initCocoa", "()V", init_cocoa);
    }

    registry.register(
        CLASS_NAME,
        "deregisterDisplayReconfiguration",
        "(J)V",
        deregister_display_reconfiguration,
    );
    registry.register(CLASS_NAME, "getDisplayIDs", "()[I", get_display_i_ds);
    registry.register(CLASS_NAME, "getMainDisplayID", "()I", get_main_display_id);
    registry.register(
        CLASS_NAME,
        "registerDisplayReconfiguration",
        "()J",
        register_display_reconfiguration,
    );
}

#[async_recursion(?Send)]
async fn deregister_display_reconfiguration(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.CGraphicsEnvironment.deregisterDisplayReconfiguration(J)V")
}

#[async_recursion(?Send)]
async fn get_display_i_ds(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.awt.CGraphicsEnvironment.getDisplayIDs()[I")
}

#[async_recursion(?Send)]
async fn get_main_display_id(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.CGraphicsEnvironment.getMainDisplayID()I")
}

#[async_recursion(?Send)]
async fn init_cocoa(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.awt.CGraphicsEnvironment.initCocoa()V")
}

#[async_recursion(?Send)]
async fn register_display_reconfiguration(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.CGraphicsEnvironment.registerDisplayReconfiguration()J")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.awt.CGraphicsEnvironment.deregisterDisplayReconfiguration(J)V"
    )]
    async fn test_deregister_display_reconfiguration() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = deregister_display_reconfiguration(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.awt.CGraphicsEnvironment.getDisplayIDs()[I"
    )]
    async fn test_get_display_i_ds() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_display_i_ds(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.awt.CGraphicsEnvironment.getMainDisplayID()I"
    )]
    async fn test_get_main_display_id() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_main_display_id(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.awt.CGraphicsEnvironment.initCocoa()V")]
    async fn test_init_cocoa() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_cocoa(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.awt.CGraphicsEnvironment.registerDisplayReconfiguration()J"
    )]
    async fn test_register_display_reconfiguration() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = register_display_reconfiguration(thread, Parameters::default()).await;
    }
}
