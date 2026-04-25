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
    "sun/java2d/x11/X11PMBlitBgLoops.nativeBlitBg(JJJIIIIIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_blit_bg<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _dy = parameters.pop_int()?;
    let _dx = parameters.pop_int()?;
    let _sy = parameters.pop_int()?;
    let _sx = parameters.pop_int()?;
    let _pixel = parameters.pop_int()?;
    let _xgc = parameters.pop_long()?;
    let _dst_data = parameters.pop_long()?;
    let _src_data = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.x11.X11PMBlitBgLoops.nativeBlitBg(JJJIIIIIII)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/java2d/x11/X11PMBlitBgLoops.nativeBlitBg(JJJIIIIIII)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn native_blit_bg_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _dy = parameters.pop_int()?;
    let _dx = parameters.pop_int()?;
    let _sy = parameters.pop_int()?;
    let _sx = parameters.pop_int()?;
    let _pixel = parameters.pop_int()?;
    let _xgc = parameters.pop_long()?;
    let _dst_data = parameters.pop_long()?;
    let _src_data = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/x11/X11PMBlitBgLoops.nativeBlitBg(JJJIIIIIII)V".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_native_blit_bg() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = native_blit_bg(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.java2d.x11.X11PMBlitBgLoops.nativeBlitBg(JJJIIIIIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_native_blit_bg_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_blit_bg_linux_ge_v11(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/java2d/x11/X11PMBlitBgLoops.nativeBlitBg(JJJIIIIIII)V",
            result.unwrap_err().to_string()
        );
    }
}
