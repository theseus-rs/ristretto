use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/lwawt/macosx/CAccessibility.focusChanged()V", Any)]
#[async_method]
pub async fn focus_changed<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CAccessibility.focusChanged()V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/lwawt/macosx/CAccessibility.roleKey(Ljavax/accessibility/AccessibleRole;)Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn role_key<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _a_role = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun.lwawt.macosx.CAccessibility.roleKey(Ljavax/accessibility/AccessibleRole;)Ljava/lang/String;".to_string()).into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_focus_changed() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = focus_changed(thread, Parameters::default()).await;
        assert_eq!(
            "sun.lwawt.macosx.CAccessibility.focusChanged()V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_role_key() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = role_key(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun.lwawt.macosx.CAccessibility.roleKey(Ljavax/accessibility/AccessibleRole;)Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }
}
