use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/java2d/pipe/SpanClipRenderer.eraseTile(Lsun/java2d/pipe/RegionIterator;[BII[I)V",
    Any
)]
#[async_method]
pub async fn erase_tile<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _band = parameters.pop_reference()?;
    let _tsize = parameters.pop_int()?;
    let _offset = parameters.pop_int()?;
    let _alpha = parameters.pop_reference()?;
    let _ri = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.pipe.SpanClipRenderer.eraseTile(Lsun/java2d/pipe/RegionIterator;[BII[I)V"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/pipe/SpanClipRenderer.fillTile(Lsun/java2d/pipe/RegionIterator;[BII[I)V",
    Any
)]
#[async_method]
pub async fn fill_tile<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _band = parameters.pop_reference()?;
    let _tsize = parameters.pop_int()?;
    let _offset = parameters.pop_int()?;
    let _alpha = parameters.pop_reference()?;
    let _ri = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.pipe.SpanClipRenderer.fillTile(Lsun/java2d/pipe/RegionIterator;[BII[I)V"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/pipe/SpanClipRenderer.initIDs(Ljava/lang/Class;Ljava/lang/Class;)V",
    Any
)]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_erase_tile() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = erase_tile(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "sun.java2d.pipe.SpanClipRenderer.eraseTile(Lsun/java2d/pipe/RegionIterator;[BII[I)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_fill_tile() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = fill_tile(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "sun.java2d.pipe.SpanClipRenderer.fillTile(Lsun/java2d/pipe/RegionIterator;[BII[I)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }
}
