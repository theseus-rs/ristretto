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
    "sun/java2d/x11/X11PMBlitLoops.nativeBlit(JJJLsun/java2d/pipe/Region;IIIIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_blit<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _dy = parameters.pop_int()?;
    let _dx = parameters.pop_int()?;
    let _sy = parameters.pop_int()?;
    let _sx = parameters.pop_int()?;
    let _clip = parameters.pop_reference()?;
    let _xgc = parameters.pop_long()?;
    let _dst_data = parameters.pop_long()?;
    let _src_data = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.x11.X11PMBlitLoops.nativeBlit(JJJLsun/java2d/pipe/Region;IIIIII)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/x11/X11PMBlitLoops.updateBitmask(Lsun/java2d/SurfaceData;Lsun/java2d/SurfaceData;Z)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn update_bitmask<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _is_icm = parameters.pop_bool()?;
    let _dst = parameters.pop_reference()?;
    let _src = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun.java2d.x11.X11PMBlitLoops.updateBitmask(Lsun/java2d/SurfaceData;Lsun/java2d/SurfaceData;Z)V".to_string()).into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/java2d/x11/X11PMBlitLoops.nativeBlit(JJJLsun/java2d/pipe/Region;IIIIII)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn native_blit_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _dy = parameters.pop_int()?;
    let _dx = parameters.pop_int()?;
    let _sy = parameters.pop_int()?;
    let _sx = parameters.pop_int()?;
    let _clip = parameters.pop_reference()?;
    let _xgc = parameters.pop_long()?;
    let _dst_data = parameters.pop_long()?;
    let _src_data = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/x11/X11PMBlitLoops.nativeBlit(JJJLsun/java2d/pipe/Region;IIIIII)V".to_string(),
    )
    .into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/java2d/x11/X11PMBlitLoops.updateBitmask(Lsun/java2d/SurfaceData;Lsun/java2d/SurfaceData;Z)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn update_bitmask_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _is_icm = parameters.pop_bool()?;
    let _dst = parameters.pop_reference()?;
    let _src = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun/java2d/x11/X11PMBlitLoops.updateBitmask(Lsun/java2d/SurfaceData;Lsun/java2d/SurfaceData;Z)V".to_string()).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_native_blit() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = native_blit(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Object(None),
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
            "sun.java2d.x11.X11PMBlitLoops.nativeBlit(JJJLsun/java2d/pipe/Region;IIIIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_update_bitmask() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = update_bitmask(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::from(false),
            ]),
        )
        .await;
        assert_eq!(
            "sun.java2d.x11.X11PMBlitLoops.updateBitmask(Lsun/java2d/SurfaceData;Lsun/java2d/SurfaceData;Z)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_native_blit_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_blit_linux_ge_v11(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Object(None),
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
            "sun/java2d/x11/X11PMBlitLoops.nativeBlit(JJJLsun/java2d/pipe/Region;IIIIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_update_bitmask_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = update_bitmask_linux_ge_v11(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::from(false),
            ]),
        )
        .await;
        assert_eq!(
            "sun/java2d/x11/X11PMBlitLoops.updateBitmask(Lsun/java2d/SurfaceData;Lsun/java2d/SurfaceData;Z)V",
            result.unwrap_err().to_string()
        );
    }
}
