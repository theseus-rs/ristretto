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
    todo!()
}

#[async_recursion(?Send)]
async fn role_key(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
