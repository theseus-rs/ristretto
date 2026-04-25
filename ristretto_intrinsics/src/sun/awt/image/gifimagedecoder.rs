use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/awt/image/GifImageDecoder.initIDs()V", Any)]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "sun/awt/image/GifImageDecoder.parseImage(IIIIZI[B[BLjava/awt/image/IndexColorModel;)Z",
    Any
)]
#[async_method]
pub async fn parse_image<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _cmh = parameters.pop_reference()?;
    let _raslineh = parameters.pop_reference()?;
    let _blockh = parameters.pop_reference()?;
    let _init_code_size = parameters.pop_int()?;
    let _interlace = parameters.pop_bool()?;
    let _height = parameters.pop_int()?;
    let _width = parameters.pop_int()?;
    let _rely = parameters.pop_int()?;
    let _relx = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.awt.image.GifImageDecoder.parseImage(IIIIZI[B[BLjava/awt/image/IndexColorModel;)Z"
            .to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_parse_image() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = parse_image(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::from(false),
                Value::Int(0),
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "sun.awt.image.GifImageDecoder.parseImage(IIIIZI[B[BLjava/awt/image/IndexColorModel;)Z",
            result.unwrap_err().to_string()
        );
    }
}
