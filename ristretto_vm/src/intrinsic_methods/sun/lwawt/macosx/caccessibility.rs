use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("sun/lwawt/macosx/CAccessibility.focusChanged()V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn focus_changed(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CAccessibility.focusChanged()V")
}

#[intrinsic_method(
    "sun/lwawt/macosx/CAccessibility.roleKey(Ljavax/accessibility/AccessibleRole;)Ljava/lang/String;",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn role_key(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.lwawt.macosx.CAccessibility.roleKey(Ljavax/accessibility/AccessibleRole;)Ljava/lang/String;"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CAccessibility.focusChanged()V"
    )]
    async fn test_focus_changed() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = focus_changed(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CAccessibility.roleKey(Ljavax/accessibility/AccessibleRole;)Ljava/lang/String;"
    )]
    async fn test_role_key() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = role_key(thread, Parameters::default()).await;
    }
}
