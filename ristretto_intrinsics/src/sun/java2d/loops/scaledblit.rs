use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/java2d/loops/ScaledBlit.Scale(Lsun/java2d/SurfaceData;Lsun/java2d/SurfaceData;Ljava/awt/Composite;Lsun/java2d/pipe/Region;IIIIDDDD)V",
    Any
)]
#[async_method]
pub async fn scale<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _dy2 = parameters.pop_double()?;
    let _dx2 = parameters.pop_double()?;
    let _dy1 = parameters.pop_double()?;
    let _dx1 = parameters.pop_double()?;
    let _sy2 = parameters.pop_int()?;
    let _sx2 = parameters.pop_int()?;
    let _sy1 = parameters.pop_int()?;
    let _sx1 = parameters.pop_int()?;
    let _clip = parameters.pop_reference()?;
    let _comp = parameters.pop_reference()?;
    let _dst = parameters.pop_reference()?;
    let _src = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun.java2d.loops.ScaledBlit.Scale(Lsun/java2d/SurfaceData;Lsun/java2d/SurfaceData;Ljava/awt/Composite;Lsun/java2d/pipe/Region;IIIIDDDD)V".to_string()).into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_scale() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = scale(
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
                Value::Double(0.0),
                Value::Double(0.0),
                Value::Double(0.0),
                Value::Double(0.0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.java2d.loops.ScaledBlit.Scale(Lsun/java2d/SurfaceData;Lsun/java2d/SurfaceData;Ljava/awt/Composite;Lsun/java2d/pipe/Region;IIIIDDDD)V",
            result.unwrap_err().to_string()
        );
    }
}
