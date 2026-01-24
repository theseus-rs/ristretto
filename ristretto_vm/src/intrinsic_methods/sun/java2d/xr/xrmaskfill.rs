use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "sun/java2d/xr/XRMaskFill.maskFill(JIIIIIII[B)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn mask_fill(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.xr.XRMaskFill.maskFill(JIIIIIII[B)V");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.xr.XRMaskFill.maskFill(JIIIIIII[B)V"
    )]
    async fn test_mask_fill() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = mask_fill(thread, Parameters::default()).await;
    }
}
