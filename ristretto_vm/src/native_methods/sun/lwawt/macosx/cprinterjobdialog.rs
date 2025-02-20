use crate::Result;
use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/lwawt/macosx/CPrinterJobDialog";

/// Register all native methods for `sun.lwawt.macosx.CPrinterJobDialog`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "showDialog", "()Z", show_dialog);
}

#[async_recursion(?Send)]
async fn show_dialog(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPrinterJobDialog.showDialog()Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CPrinterJobDialog.showDialog()Z"
    )]
    async fn test_show_dialog() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = show_dialog(thread, Parameters::default()).await;
    }
}
