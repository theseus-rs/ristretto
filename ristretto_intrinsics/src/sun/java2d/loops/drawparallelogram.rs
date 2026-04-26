use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/java2d/loops/DrawParallelogram.DrawParallelogram(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;DDDDDDDD)V",
    Any
)]
#[async_method]
pub async fn draw_parallelogram<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _lw2 = parameters.pop_double()?;
    let _lw1 = parameters.pop_double()?;
    let _dy2 = parameters.pop_double()?;
    let _dx2 = parameters.pop_double()?;
    let _dy1 = parameters.pop_double()?;
    let _dx1 = parameters.pop_double()?;
    let _y = parameters.pop_double()?;
    let _x = parameters.pop_double()?;
    let _dest = parameters.pop_reference()?;
    let _sg = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun.java2d.loops.DrawParallelogram.DrawParallelogram(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;DDDDDDDD)V".to_string()).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_draw_parallelogram() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = draw_parallelogram(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Double(0.0),
                Value::Double(0.0),
                Value::Double(0.0),
                Value::Double(0.0),
                Value::Double(0.0),
                Value::Double(0.0),
                Value::Double(0.0),
                Value::Double(0.0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.java2d.loops.DrawParallelogram.DrawParallelogram(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;DDDDDDDD)V",
            result.unwrap_err().to_string()
        );
    }
}
