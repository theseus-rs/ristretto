use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.lwawt.macosx.CAccessibility`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/lwawt/macosx/CAccessibility";
    registry.register(class_name, "focusChanged", "()V", focus_changed);
    registry.register(
        class_name,
        "roleKey",
        "(Ljavax/accessibility/AccessibleRole;)Ljava/lang/String;",
        role_key,
    );
}

#[async_recursion(?Send)]
async fn focus_changed(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CAccessibility.focusChanged()V")
}

#[async_recursion(?Send)]
async fn role_key(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CAccessibility.roleKey(Ljavax/accessibility/AccessibleRole;)Ljava/lang/String;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/lwawt/macosx/CAccessibility";
        assert!(registry.method(class_name, "focusChanged", "()V").is_some());
        assert!(registry
            .method(
                class_name,
                "roleKey",
                "(Ljavax/accessibility/AccessibleRole;)Ljava/lang/String;"
            )
            .is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.CAccessibility.focusChanged()V")]
    async fn test_focus_changed() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = focus_changed(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.lwawt.macosx.CAccessibility.roleKey(Ljavax/accessibility/AccessibleRole;)Ljava/lang/String;"
    )]
    async fn test_role_key() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = role_key(thread, Arguments::default()).await;
    }
}
