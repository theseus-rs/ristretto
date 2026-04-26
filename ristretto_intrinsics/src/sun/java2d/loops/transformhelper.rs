use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/java2d/loops/TransformHelper.Transform(Lsun/java2d/loops/MaskBlit;Lsun/java2d/SurfaceData;Lsun/java2d/SurfaceData;Ljava/awt/Composite;Lsun/java2d/pipe/Region;Ljava/awt/geom/AffineTransform;IIIIIIIII[III)V",
    Any
)]
#[async_method]
pub async fn transform<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _dyoff = parameters.pop_int()?;
    let _dxoff = parameters.pop_int()?;
    let _edges = parameters.pop_reference()?;
    let _dy2 = parameters.pop_int()?;
    let _dx2 = parameters.pop_int()?;
    let _dy1 = parameters.pop_int()?;
    let _dx1 = parameters.pop_int()?;
    let _sy2 = parameters.pop_int()?;
    let _sx2 = parameters.pop_int()?;
    let _sy1 = parameters.pop_int()?;
    let _sx1 = parameters.pop_int()?;
    let _txtype = parameters.pop_int()?;
    let _itx = parameters.pop_reference()?;
    let _clip = parameters.pop_reference()?;
    let _comp = parameters.pop_reference()?;
    let _dst = parameters.pop_reference()?;
    let _src = parameters.pop_reference()?;
    let _output = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun.java2d.loops.TransformHelper.Transform(Lsun/java2d/loops/MaskBlit;Lsun/java2d/SurfaceData;Lsun/java2d/SurfaceData;Ljava/awt/Composite;Lsun/java2d/pipe/Region;Ljava/awt/geom/AffineTransform;IIIIIIIII[III)V".to_string()).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_transform() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = transform(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
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
            "sun.java2d.loops.TransformHelper.Transform(Lsun/java2d/loops/MaskBlit;Lsun/java2d/SurfaceData;Lsun/java2d/SurfaceData;Ljava/awt/Composite;Lsun/java2d/pipe/Region;Ljava/awt/geom/AffineTransform;IIIIIIIII[III)V",
            result.unwrap_err().to_string()
        );
    }
}
