use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/awt/X11InputMethod.disposeXIC()V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn dispose_xic<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.awt.X11InputMethod.disposeXIC()V".to_string()).into())
}

#[intrinsic_method("sun/awt/X11InputMethod.initIDs()V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "sun/awt/X11InputMethod.isCompositionEnabledNative()Z",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn is_composition_enabled_native<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.awt.X11InputMethod.isCompositionEnabledNative()Z".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/awt/X11InputMethod.resetXIC()Ljava/lang/String;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn reset_xic<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.awt.X11InputMethod.resetXIC()Ljava/lang/String;".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/awt/X11InputMethod.setCompositionEnabledNative(Z)Z",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn set_composition_enabled_native<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _enable = parameters.pop_bool()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.awt.X11InputMethod.setCompositionEnabledNative(Z)Z".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/awt/X11InputMethod.turnoffStatusWindow()V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn turnoff_status_window<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.awt.X11InputMethod.turnoffStatusWindow()V".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_dispose_xic() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = dispose_xic(thread, Parameters::default()).await;
        assert_eq!(
            "sun.awt.X11InputMethod.disposeXIC()V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_is_composition_enabled_native() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = is_composition_enabled_native(thread, Parameters::default()).await;
        assert_eq!(
            "sun.awt.X11InputMethod.isCompositionEnabledNative()Z",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_reset_xic() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = reset_xic(thread, Parameters::default()).await;
        assert_eq!(
            "sun.awt.X11InputMethod.resetXIC()Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_set_composition_enabled_native() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result =
            set_composition_enabled_native(thread, Parameters::new(vec![Value::from(false)])).await;
        assert_eq!(
            "sun.awt.X11InputMethod.setCompositionEnabledNative(Z)Z",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_turnoff_status_window() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = turnoff_status_window(thread, Parameters::default()).await;
        assert_eq!(
            "sun.awt.X11InputMethod.turnoffStatusWindow()V",
            result.unwrap_err().to_string()
        );
    }
}
