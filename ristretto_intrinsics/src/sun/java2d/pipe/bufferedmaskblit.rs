use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/java2d/pipe/BufferedMaskBlit.enqueueTile(JILsun/java2d/SurfaceData;JI[BIIIIIIIII)I",
    Any
)]
#[async_method]
pub async fn enqueue_tile<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _height = parameters.pop_int()?;
    let _width = parameters.pop_int()?;
    let _dsty = parameters.pop_int()?;
    let _dstx = parameters.pop_int()?;
    let _srcy = parameters.pop_int()?;
    let _srcx = parameters.pop_int()?;
    let _maskscan = parameters.pop_int()?;
    let _maskoff = parameters.pop_int()?;
    let _masklen = parameters.pop_int()?;
    let _mask = parameters.pop_reference()?;
    let _src_type = parameters.pop_int()?;
    let _p_src_ops = parameters.pop_long()?;
    let _src_data = parameters.pop_reference()?;
    let _bpos = parameters.pop_int()?;
    let _buf = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.pipe.BufferedMaskBlit.enqueueTile(JILsun/java2d/SurfaceData;JI[BIIIIIIIII)I"
            .to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_enqueue_tile() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = enqueue_tile(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Int(0),
                Value::Object(None),
                Value::Long(0),
                Value::Int(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.java2d.pipe.BufferedMaskBlit.enqueueTile(JILsun/java2d/SurfaceData;JI[BIIIIIIIII)I",
            result.unwrap_err().to_string()
        );
    }
}
