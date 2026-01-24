use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("apple/laf/JRSUIConstants.getPtrForConstant(I)J", Any)]
#[async_method]
pub(crate) async fn get_ptr_for_constant(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("apple.laf.JRSUIConstants.getPtrForConstant(I)J")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: apple.laf.JRSUIConstants.getPtrForConstant(I)J"
    )]
    async fn test_get_ptr_for_constant() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_ptr_for_constant(thread, Parameters::default()).await;
    }
}
