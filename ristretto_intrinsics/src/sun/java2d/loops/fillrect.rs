use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/java2d/loops/FillRect.FillRect(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;IIII)V",
    Any
)]
#[async_method]
pub async fn fill_rect<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _dest = parameters.pop_reference()?;
    let _sg2d = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun.java2d.loops.FillRect.FillRect(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;IIII)V".to_string()).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fill_rect() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = fill_rect(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.java2d.loops.FillRect.FillRect(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;IIII)V",
            result.unwrap_err().to_string()
        );
    }
}
