use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("sun/awt/X11InputMethod.disposeXIC()V", LessThanOrEqual(JAVA_8))]
#[async_recursion(?Send)]
pub(crate) async fn dispose_xic(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.X11InputMethod.disposeXIC()V")
}

#[intrinsic_method("sun/awt/X11InputMethod.initIDs()V", LessThanOrEqual(JAVA_8))]
#[async_recursion(?Send)]
pub(crate) async fn init_ids(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "sun/awt/X11InputMethod.isCompositionEnabledNative()Z",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn is_composition_enabled_native(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.X11InputMethod.isCompositionEnabledNative()Z")
}

#[intrinsic_method(
    "sun/awt/X11InputMethod.resetXIC()Ljava/lang/String;",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn reset_xic(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.X11InputMethod.resetXIC()Ljava/lang/String;")
}

#[intrinsic_method(
    "sun/awt/X11InputMethod.setCompositionEnabledNative(Z)Z",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn set_composition_enabled_native(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.X11InputMethod.setCompositionEnabledNative(Z)Z")
}

#[intrinsic_method(
    "sun/awt/X11InputMethod.turnoffStatusWindow()V",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn turnoff_status_window(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.X11InputMethod.turnoffStatusWindow()V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.awt.X11InputMethod.disposeXIC()V")]
    async fn test_dispose_xic() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = dispose_xic(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.awt.X11InputMethod.isCompositionEnabledNative()Z"
    )]
    async fn test_is_composition_enabled_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_composition_enabled_native(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.awt.X11InputMethod.resetXIC()Ljava/lang/String;"
    )]
    async fn test_reset_xic() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = reset_xic(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.awt.X11InputMethod.setCompositionEnabledNative(Z)Z"
    )]
    async fn test_set_composition_enabled_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_composition_enabled_native(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.awt.X11InputMethod.turnoffStatusWindow()V")]
    async fn test_turnoff_status_window() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = turnoff_status_window(thread, Parameters::default()).await;
    }
}
