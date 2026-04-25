use ristretto_classfile::JAVA_8;
#[cfg(target_os = "linux")]
use ristretto_classfile::JAVA_11;
#[cfg(target_os = "linux")]
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/java2d/xr/XRMaskBlit.maskBlit(JJIIIIIIIII[B)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn mask_blit<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _mask = parameters.pop_reference()?;
    let _masklen = parameters.pop_int()?;
    let _maskscan = parameters.pop_int()?;
    let _maskoff = parameters.pop_int()?;
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _dsty = parameters.pop_int()?;
    let _dstx = parameters.pop_int()?;
    let _srcy = parameters.pop_int()?;
    let _srcx = parameters.pop_int()?;
    let _dstxsdo = parameters.pop_long()?;
    let _src_xsdo = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.xr.XRMaskBlit.maskBlit(JJIIIIIIIII[B)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/java2d/xr/XRMaskBlit.maskBlit(JJIIIIIIIII[B)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn mask_blit_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _mask = parameters.pop_reference()?;
    let _masklen = parameters.pop_int()?;
    let _maskscan = parameters.pop_int()?;
    let _maskoff = parameters.pop_int()?;
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _dsty = parameters.pop_int()?;
    let _dstx = parameters.pop_int()?;
    let _srcy = parameters.pop_int()?;
    let _srcx = parameters.pop_int()?;
    let _dstxsdo = parameters.pop_long()?;
    let _src_xsdo = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/xr/XRMaskBlit.maskBlit(JJIIIIIIIII[B)V".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mask_blit() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = mask_blit(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
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
            ]),
        )
        .await;
        assert_eq!(
            "sun.java2d.xr.XRMaskBlit.maskBlit(JJIIIIIIIII[B)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_mask_blit_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = mask_blit_linux_ge_v11(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
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
            ]),
        )
        .await;
        assert_eq!(
            "sun/java2d/xr/XRMaskBlit.maskBlit(JJIIIIIIIII[B)V",
            result.unwrap_err().to_string()
        );
    }
}
