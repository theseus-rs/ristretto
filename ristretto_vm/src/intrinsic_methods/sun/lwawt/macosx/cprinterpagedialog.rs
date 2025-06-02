use crate::Result;
use crate::intrinsic_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/lwawt/macosx/CPrinterPageDialog";

/// Register all intrinsic methods for `sun.lwawt.macosx.CPrinterPageDialog`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "showDialog", "()Z", show_dialog);
}

#[async_recursion(?Send)]
async fn show_dialog(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPrinterPageDialog.showDialog()Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CPrinterPageDialog.showDialog()Z"
    )]
    async fn test_show_dialog() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = show_dialog(thread, Parameters::default()).await;
    }
}
