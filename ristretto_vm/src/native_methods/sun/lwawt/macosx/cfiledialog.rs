use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.lwawt.macosx.CFileDialog`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/lwawt/macosx/CFileDialog";
    registry.register(
        class_name,
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

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/lwawt/macosx/CFileDialog";
        assert!(registry
            .method(
                class_name,
                "nativeRunFileDialog",
                "(Ljava/lang/String;IZZZZLjava/lang/String;Ljava/lang/String;)[Ljava/lang/String;"
            )
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.lwawt.macosx.CFileDialog.nativeRunFileDialog(Ljava/lang/String;IZZZZLjava/lang/String;Ljava/lang/String;)[Ljava/lang/String;"
    )]
    async fn test_native_run_file_dialog() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_run_file_dialog(thread, Arguments::default()).await;
    }
}
