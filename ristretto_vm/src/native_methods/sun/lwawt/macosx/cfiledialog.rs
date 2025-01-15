use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/lwawt/macosx/CFileDialog";

/// Register all native methods for `sun.lwawt.macosx.CFileDialog`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "nativeRunFileDialog",
        "(Ljava/lang/String;IZZZZLjava/lang/String;Ljava/lang/String;)[Ljava/lang/String;",
        native_run_file_dialog,
    );
}

#[async_recursion(?Send)]
async fn native_run_file_dialog(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CFileDialog.nativeRunFileDialog(Ljava/lang/String;IZZZZLjava/lang/String;Ljava/lang/String;)[Ljava/lang/String;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CFileDialog.nativeRunFileDialog(Ljava/lang/String;IZZZZLjava/lang/String;Ljava/lang/String;)[Ljava/lang/String;"
    )]
    async fn test_native_run_file_dialog() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_run_file_dialog(thread, Arguments::default()).await;
    }
}
