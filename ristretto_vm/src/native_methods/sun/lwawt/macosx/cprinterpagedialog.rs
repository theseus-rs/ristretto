use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.lwawt.macosx.CPrinterPageDialog`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/lwawt/macosx/CPrinterPageDialog";
    registry.register(class_name, "showDialog", "()Z", show_dialog);
}

#[async_recursion(?Send)]
async fn show_dialog(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPrinterPageDialog.showDialog()Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/lwawt/macosx/CPrinterPageDialog";
        assert!(registry.method(class_name, "showDialog", "()Z").is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.CPrinterPageDialog.showDialog()Z")]
    async fn test_show_dialog() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = show_dialog(thread, Arguments::default()).await;
    }
}
