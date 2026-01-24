use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::{Any, LessThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "sun/awt/CGraphicsEnvironment.deregisterDisplayReconfiguration(J)V",
    Any
)]
#[async_method]
pub(crate) async fn deregister_display_reconfiguration(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.CGraphicsEnvironment.deregisterDisplayReconfiguration(J)V")
}

#[intrinsic_method("sun/awt/CGraphicsEnvironment.getDisplayIDs()[I", Any)]
#[async_method]
pub(crate) async fn get_display_i_ds(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.CGraphicsEnvironment.getDisplayIDs()[I")
}

#[intrinsic_method("sun/awt/CGraphicsEnvironment.getMainDisplayID()I", Any)]
#[async_method]
pub(crate) async fn get_main_display_id(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.CGraphicsEnvironment.getMainDisplayID()I")
}

#[intrinsic_method("sun/awt/CGraphicsEnvironment.initCocoa()V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub(crate) async fn init_cocoa(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.CGraphicsEnvironment.initCocoa()V")
}

#[intrinsic_method("sun/awt/CGraphicsEnvironment.registerDisplayReconfiguration()J", Any)]
#[async_method]
pub(crate) async fn register_display_reconfiguration(
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
