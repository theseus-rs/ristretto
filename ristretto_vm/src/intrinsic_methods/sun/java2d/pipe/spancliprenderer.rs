use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "sun/java2d/pipe/SpanClipRenderer.eraseTile(Lsun/java2d/pipe/RegionIterator;[BII[I)V",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn erase_tile(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.pipe.SpanClipRenderer.eraseTile(Lsun/java2d/pipe/RegionIterator;[BII[I)V")
}

#[intrinsic_method(
    "sun/java2d/pipe/SpanClipRenderer.fillTile(Lsun/java2d/pipe/RegionIterator;[BII[I)V",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn fill_tile(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.pipe.SpanClipRenderer.fillTile(Lsun/java2d/pipe/RegionIterator;[BII[I)V")
}

#[intrinsic_method(
    "sun/java2d/pipe/SpanClipRenderer.initIDs(Ljava/lang/Class;Ljava/lang/Class;)V",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn init_ids(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.pipe.SpanClipRenderer.eraseTile(Lsun/java2d/pipe/RegionIterator;[BII[I)V"
    )]
    async fn test_erase_tile() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = erase_tile(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.pipe.SpanClipRenderer.fillTile(Lsun/java2d/pipe/RegionIterator;[BII[I)V"
    )]
    async fn test_fill_tile() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = fill_tile(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }
}
