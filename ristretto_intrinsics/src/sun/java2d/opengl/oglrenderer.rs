use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/java2d/opengl/OGLRenderer.drawPoly([I[IIZII)V", Any)]
#[async_method]
pub async fn draw_poly<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _trans_y = parameters.pop_int()?;
    let _trans_x = parameters.pop_int()?;
    let _is_closed = parameters.pop_bool()?;
    let _n_points = parameters.pop_int()?;
    let _y_points = parameters.pop_reference()?;
    let _x_points = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.opengl.OGLRenderer.drawPoly([I[IIZII)V".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_draw_poly() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = draw_poly(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Int(0),
                Value::from(false),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.java2d.opengl.OGLRenderer.drawPoly([I[IIZII)V",
            result.unwrap_err().to_string()
        );
    }
}
