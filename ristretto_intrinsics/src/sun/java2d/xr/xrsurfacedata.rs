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
    "sun/java2d/xr/XRSurfaceData.XRInitSurface(IIIJI)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn xr_init_surface<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _pict_format = parameters.pop_int()?;
    let _drawable = parameters.pop_long()?;
    let _height = parameters.pop_int()?;
    let _width = parameters.pop_int()?;
    let _depth = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.xr.XRSurfaceData.XRInitSurface(IIIJI)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/xr/XRSurfaceData.freeXSDOPicture(J)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn free_xsdo_picture<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _xsdo = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.xr.XRSurfaceData.freeXSDOPicture(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/java2d/xr/XRSurfaceData.initIDs()V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "sun/java2d/xr/XRSurfaceData.initXRPicture(JI)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn init_xr_picture<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _pict_form = parameters.pop_int()?;
    let _xsdo = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.xr.XRSurfaceData.initXRPicture(JI)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/java2d/xr/XRSurfaceData.XRInitSurface(IIIJI)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn xrinit_surface_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _pict_format = parameters.pop_int()?;
    let _drawable = parameters.pop_long()?;
    let _height = parameters.pop_int()?;
    let _width = parameters.pop_int()?;
    let _depth = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/xr/XRSurfaceData.XRInitSurface(IIIJI)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/java2d/xr/XRSurfaceData.freeXSDOPicture(J)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn free_xsdopicture_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _xsdo = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/xr/XRSurfaceData.freeXSDOPicture(J)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method("sun/java2d/xr/XRSurfaceData.initIDs()V", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn init_ids_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/java2d/xr/XRSurfaceData.initIDs()V".to_string())
            .into(),
    )
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/java2d/xr/XRSurfaceData.initXRPicture(JI)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn init_xrpicture_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _pict_form = parameters.pop_int()?;
    let _xsdo = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/xr/XRSurfaceData.initXRPicture(JI)V".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_xr_init_surface() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = xr_init_surface(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Long(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.java2d.xr.XRSurfaceData.XRInitSurface(IIIJI)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_free_xsdo_picture() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = free_xsdo_picture(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.java2d.xr.XRSurfaceData.freeXSDOPicture(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::java8_thread().await?;
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_init_xr_picture() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result =
            init_xr_picture(thread, Parameters::new(vec![Value::Long(0), Value::Int(0)])).await;
        assert_eq!(
            "sun.java2d.xr.XRSurfaceData.initXRPicture(JI)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xrinit_surface_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xrinit_surface_linux_ge_v11(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Long(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/java2d/xr/XRSurfaceData.XRInitSurface(IIIJI)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_free_xsdopicture_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            free_xsdopicture_linux_ge_v11(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/java2d/xr/XRSurfaceData.freeXSDOPicture(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_init_ids_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_ids_linux_ge_v11(thread, Parameters::default()).await;
        assert_eq!(
            "sun/java2d/xr/XRSurfaceData.initIDs()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_init_xrpicture_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_xrpicture_linux_ge_v11(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/java2d/xr/XRSurfaceData.initXRPicture(JI)V",
            result.unwrap_err().to_string()
        );
    }
}
