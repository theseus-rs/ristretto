use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/java2d/xr/XRSurfaceData.XRInitSurface(IIIJI)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn xr_init_surface<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.xr.XRSurfaceData.XRInitSurface(IIIJI)V");
}

#[intrinsic_method(
    "sun/java2d/xr/XRSurfaceData.freeXSDOPicture(J)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn free_xsdo_picture<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.xr.XRSurfaceData.freeXSDOPicture(J)V");
}

#[intrinsic_method("sun/java2d/xr/XRSurfaceData.initIDs()V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn init_ids<T: ristretto_types::Thread + 'static>(
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
pub async fn init_xr_picture<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.xr.XRSurfaceData.initXRPicture(JI)V");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.xr.XRSurfaceData.XRInitSurface(IIIJI)V"
    )]
    async fn test_xr_init_surface() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = xr_init_surface(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.xr.XRSurfaceData.freeXSDOPicture(J)V"
    )]
    async fn test_free_xsdo_picture() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = free_xsdo_picture(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.xr.XRSurfaceData.initXRPicture(JI)V"
    )]
    async fn test_init_xr_picture() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_xr_picture(thread, Parameters::default()).await;
    }
}
