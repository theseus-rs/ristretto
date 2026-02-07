use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/java2d/pipe/BufferedMaskBlit.enqueueTile(JILsun/java2d/SurfaceData;JI[BIIIIIIIII)I",
    Any
)]
#[async_method]
pub async fn enqueue_tile<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.pipe.BufferedMaskBlit.enqueueTile(JILsun/java2d/SurfaceData;JI[BIIIIIIIII)I");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.pipe.BufferedMaskBlit.enqueueTile(JILsun/java2d/SurfaceData;JI[BIIIIIIIII)I"
    )]
    async fn test_enqueue_tile() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = enqueue_tile(thread, Parameters::default()).await;
    }
}
