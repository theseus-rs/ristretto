use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.lwawt.macosx.CDesktopPeer`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/lwawt/macosx/CDesktopPeer";
    registry.register(
        class_name,
        "_lsOpenFile",
        "(Ljava/lang/String;Z)I",
        ls_open_file,
    );
    registry.register(
        class_name,
        "_lsOpenURI",
        "(Ljava/lang/String;)I",
        ls_open_uri,
    );
}

#[async_recursion(?Send)]
async fn ls_open_file(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CDesktopPeer._lsOpenFile(Ljava/lang/String;Z)I")
}

#[async_recursion(?Send)]
async fn ls_open_uri(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CDesktopPeer._lsOpenURI(Ljava/lang/String;)I")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/lwawt/macosx/CDesktopPeer";
        assert!(registry
            .method(class_name, "_lsOpenFile", "(Ljava/lang/String;Z)I")
            .is_some());
        assert!(registry
            .method(class_name, "_lsOpenURI", "(Ljava/lang/String;)I")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.CDesktopPeer._lsOpenFile(Ljava/lang/String;Z)I")]
    async fn test_ls_open_file() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ls_open_file(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.CDesktopPeer._lsOpenURI(Ljava/lang/String;)I")]
    async fn test_ls_open_uri() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ls_open_uri(thread, Arguments::default()).await;
    }
}
