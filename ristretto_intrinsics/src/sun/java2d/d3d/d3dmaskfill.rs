use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/java2d/d3d/D3DMaskFill.maskFill(IIIIIII[B)V", Any)]
#[async_method]
pub async fn mask_fill<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _mask_array = parameters.pop_reference()?;
    let _masklen = parameters.pop_int()?;
    let _maskscan = parameters.pop_int()?;
    let _maskoff = parameters.pop_int()?;
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/d3d/D3DMaskFill.maskFill(IIIIIII[B)V".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mask_fill() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = mask_fill(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "sun/java2d/d3d/D3DMaskFill.maskFill(IIIIIII[B)V",
            result.unwrap_err().to_string()
        );
    }
}
