use crate::Result;
use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/lwawt/macosx/CAccessibility";

/// Register all native methods for `sun.lwawt.macosx.CAccessibility`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "focusChanged", "()V", focus_changed);
    registry.register(
        CLASS_NAME,
        "roleKey",
        "(Ljavax/accessibility/AccessibleRole;)Ljava/lang/String;",
        role_key,
    );
}

#[async_recursion(?Send)]
async fn focus_changed(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CAccessibility.focusChanged()V")
}

#[async_recursion(?Send)]
async fn role_key(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
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
