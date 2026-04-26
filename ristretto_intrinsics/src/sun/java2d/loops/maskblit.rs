use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/java2d/loops/MaskBlit.MaskBlit(Lsun/java2d/SurfaceData;Lsun/java2d/SurfaceData;Ljava/awt/Composite;Lsun/java2d/pipe/Region;IIIIII[BII)V",
    Any
)]
#[async_method]
pub async fn mask_blit<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _maskscan = parameters.pop_int()?;
    let _maskoff = parameters.pop_int()?;
    let _mask = parameters.pop_reference()?;
    let _height = parameters.pop_int()?;
    let _width = parameters.pop_int()?;
    let _dsty = parameters.pop_int()?;
    let _dstx = parameters.pop_int()?;
    let _srcy = parameters.pop_int()?;
    let _srcx = parameters.pop_int()?;
    let _clip = parameters.pop_reference()?;
    let _comp = parameters.pop_reference()?;
    let _dst = parameters.pop_reference()?;
    let _src = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun.java2d.loops.MaskBlit.MaskBlit(Lsun/java2d/SurfaceData;Lsun/java2d/SurfaceData;Ljava/awt/Composite;Lsun/java2d/pipe/Region;IIIIII[BII)V".to_string()).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mask_blit() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = mask_blit(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.java2d.loops.MaskBlit.MaskBlit(Lsun/java2d/SurfaceData;Lsun/java2d/SurfaceData;Ljava/awt/Composite;Lsun/java2d/pipe/Region;IIIIII[BII)V",
            result.unwrap_err().to_string()
        );
    }
}
