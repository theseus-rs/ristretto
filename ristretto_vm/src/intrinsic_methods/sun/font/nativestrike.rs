use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "sun/font/NativeStrike.createNullScalerContext()J",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn create_null_scaler_context(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.NativeStrike.createNullScalerContext()J")
}

#[intrinsic_method(
    "sun/font/NativeStrike.createScalerContext([BID)J",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn create_scaler_context(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.NativeStrike.createScalerContext([BID)J")
}

#[intrinsic_method("sun/font/NativeStrike.getMaxGlyph(J)I", LessThanOrEqual(JAVA_8))]
#[async_recursion(?Send)]
pub(crate) async fn get_max_glyph(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.NativeStrike.getMaxGlyph(J)I")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.font.NativeStrike.createNullScalerContext()J"
    )]
    async fn test_create_null_scaler_context() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = create_null_scaler_context(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.font.NativeStrike.createScalerContext([BID)J"
    )]
    async fn test_create_scaler_context() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = create_scaler_context(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.font.NativeStrike.getMaxGlyph(J)I")]
    async fn test_get_max_glyph() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_max_glyph(thread, Parameters::default()).await;
    }
}
