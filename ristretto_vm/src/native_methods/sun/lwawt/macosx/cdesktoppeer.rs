use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/lwawt/macosx/CDesktopPeer";

/// Register all native methods for `sun.lwawt.macosx.CDesktopPeer`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "_lsOpenFile",
        "(Ljava/lang/String;Z)I",
        ls_open_file,
    );
    registry.register(
        CLASS_NAME,
        "_lsOpenURI",
        "(Ljava/lang/String;)I",
        ls_open_uri,
    );
}

#[async_recursion(?Send)]
async fn ls_open_file(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CDesktopPeer._lsOpenFile(Ljava/lang/String;Z)I")
}

#[async_recursion(?Send)]
async fn ls_open_uri(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CDesktopPeer._lsOpenURI(Ljava/lang/String;)I")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CDesktopPeer._lsOpenFile(Ljava/lang/String;Z)I"
    )]
    async fn test_ls_open_file() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ls_open_file(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CDesktopPeer._lsOpenURI(Ljava/lang/String;)I"
    )]
    async fn test_ls_open_uri() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ls_open_uri(thread, Parameters::default()).await;
    }
}
