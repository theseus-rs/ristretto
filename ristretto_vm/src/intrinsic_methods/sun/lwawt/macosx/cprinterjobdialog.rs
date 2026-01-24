use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("sun/lwawt/macosx/CPrinterJobDialog.showDialog()Z", Any)]
#[async_method]
pub(crate) async fn show_dialog(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
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
