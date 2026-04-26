use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/java2d/windows/GDIBlitLoops.nativeBlit(Lsun/java2d/SurfaceData;Lsun/java2d/SurfaceData;Lsun/java2d/pipe/Region;IIIIIIIIIZ)V",
    Any
)]
#[async_method]
pub async fn native_blit<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _need_lut = parameters.pop_bool()?;
    let _bmask = parameters.pop_int()?;
    let _gmask = parameters.pop_int()?;
    let _rmask = parameters.pop_int()?;
    let _height = parameters.pop_int()?;
    let _width = parameters.pop_int()?;
    let _dsty = parameters.pop_int()?;
    let _dstx = parameters.pop_int()?;
    let _srcy = parameters.pop_int()?;
    let _srcx = parameters.pop_int()?;
    let _clip = parameters.pop_reference()?;
    let _dst_data = parameters.pop_reference()?;
    let _src_data = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun/java2d/windows/GDIBlitLoops.nativeBlit(Lsun/java2d/SurfaceData;Lsun/java2d/SurfaceData;Lsun/java2d/pipe/Region;IIIIIIIIIZ)V".to_string()).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_native_blit() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_blit(
            thread,
            Parameters::new(vec![
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
                Value::from(false),
            ]),
        )
        .await;
        assert_eq!(
            "sun/java2d/windows/GDIBlitLoops.nativeBlit(Lsun/java2d/SurfaceData;Lsun/java2d/SurfaceData;Lsun/java2d/pipe/Region;IIIIIIIIIZ)V",
            result.unwrap_err().to_string()
        );
    }
}
