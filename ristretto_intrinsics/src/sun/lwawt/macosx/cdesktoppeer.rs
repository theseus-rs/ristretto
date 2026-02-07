use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/lwawt/macosx/CDesktopPeer._lsOpenFile(Ljava/lang/String;ILjava/lang/String;)I",
    Any
)]
#[async_method]
pub async fn ls_open_file<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CDesktopPeer._lsOpenFile(Ljava/lang/String;ILjava/lang/String;)I")
}

#[intrinsic_method("sun/lwawt/macosx/CDesktopPeer._lsOpenURI(Ljava/lang/String;I)I", Any)]
#[async_method]
pub async fn ls_open_uri<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CDesktopPeer._lsOpenURI(Ljava/lang/String;I)I")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CDesktopPeer._lsOpenFile(Ljava/lang/String;ILjava/lang/String;)I"
    )]
    async fn test_ls_open_file() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ls_open_file(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CDesktopPeer._lsOpenURI(Ljava/lang/String;I)I"
    )]
    async fn test_ls_open_uri() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ls_open_uri(thread, Parameters::default()).await;
    }
}
