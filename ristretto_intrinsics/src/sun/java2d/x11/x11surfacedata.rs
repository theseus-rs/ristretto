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
    "sun/java2d/x11/X11SurfaceData.XSetCopyMode(J)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn x_set_copy_mode<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _xgc = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.x11.X11SurfaceData.XSetCopyMode(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/x11/X11SurfaceData.XSetForeground(JI)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn x_set_foreground<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _pixel = parameters.pop_int()?;
    let _xgc = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.x11.X11SurfaceData.XSetForeground(JI)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/x11/X11SurfaceData.XSetXorMode(J)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn x_set_xor_mode<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _xgc = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.x11.X11SurfaceData.XSetXorMode(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/x11/X11SurfaceData.initIDs(Ljava/lang/Class;Z)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "sun/java2d/x11/X11SurfaceData.initSurface(IIIJ)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn init_surface<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _drawable = parameters.pop_long()?;
    let _height = parameters.pop_int()?;
    let _width = parameters.pop_int()?;
    let _depth = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.x11.X11SurfaceData.initSurface(IIIJ)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/x11/X11SurfaceData.isDgaAvailable()Z",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn is_dga_available<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.x11.X11SurfaceData.isDgaAvailable()Z".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/x11/X11SurfaceData.isShmPMAvailable()Z",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn is_shm_pm_available<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.x11.X11SurfaceData.isShmPMAvailable()Z".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/java2d/x11/X11SurfaceData.initIDs(Ljava/lang/Class;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn init_ids_linux_ge_v11_v1<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _xor_comp = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/x11/X11SurfaceData.initIDs(Ljava/lang/Class;)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/java2d/x11/X11SurfaceData.XSetCopyMode(J)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn xset_copy_mode_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _xgc = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/x11/X11SurfaceData.XSetCopyMode(J)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/java2d/x11/X11SurfaceData.XSetForeground(JI)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn xset_foreground_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _pixel = parameters.pop_int()?;
    let _xgc = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/x11/X11SurfaceData.XSetForeground(JI)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/java2d/x11/X11SurfaceData.XSetXorMode(J)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn xset_xor_mode_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _xgc = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/x11/X11SurfaceData.XSetXorMode(J)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/java2d/x11/X11SurfaceData.initIDs(Ljava/lang/Class;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn init_ids_linux_ge_v11_v2<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _xor_comp = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/x11/X11SurfaceData.initIDs(Ljava/lang/Class;)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/java2d/x11/X11SurfaceData.initSurface(IIIJ)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn init_surface_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _drawable = parameters.pop_long()?;
    let _height = parameters.pop_int()?;
    let _width = parameters.pop_int()?;
    let _depth = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/x11/X11SurfaceData.initSurface(IIIJ)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/java2d/x11/X11SurfaceData.isShmPMAvailable()Z",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn is_shm_pmavailable_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/x11/X11SurfaceData.isShmPMAvailable()Z".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_x_set_copy_mode() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = x_set_copy_mode(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.java2d.x11.X11SurfaceData.XSetCopyMode(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_x_set_foreground() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result =
            x_set_foreground(thread, Parameters::new(vec![Value::Long(0), Value::Int(0)])).await;
        assert_eq!(
            "sun.java2d.x11.X11SurfaceData.XSetForeground(JI)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_x_set_xor_mode() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = x_set_xor_mode(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.java2d.x11.X11SurfaceData.XSetXorMode(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_init_surface() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = init_surface(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Long(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.java2d.x11.X11SurfaceData.initSurface(IIIJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_is_dga_available() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = is_dga_available(thread, Parameters::default()).await;
        assert_eq!(
            "sun.java2d.x11.X11SurfaceData.isDgaAvailable()Z",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_is_shm_pm_available() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = is_shm_pm_available(thread, Parameters::default()).await;
        assert_eq!(
            "sun.java2d.x11.X11SurfaceData.isShmPMAvailable()Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_init_ids_linux_ge_v11_v1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            init_ids_linux_ge_v11_v1(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/java2d/x11/X11SurfaceData.initIDs(Ljava/lang/Class;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xset_copy_mode_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            xset_copy_mode_linux_ge_v11(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/java2d/x11/X11SurfaceData.XSetCopyMode(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xset_foreground_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xset_foreground_linux_ge_v11(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/java2d/x11/X11SurfaceData.XSetForeground(JI)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xset_xor_mode_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            xset_xor_mode_linux_ge_v11(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/java2d/x11/X11SurfaceData.XSetXorMode(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_init_ids_linux_ge_v11_v2() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            init_ids_linux_ge_v11_v2(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/java2d/x11/X11SurfaceData.initIDs(Ljava/lang/Class;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_init_surface_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_surface_linux_ge_v11(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Long(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/java2d/x11/X11SurfaceData.initSurface(IIIJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_is_shm_pmavailable_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = is_shm_pmavailable_linux_ge_v11(thread, Parameters::default()).await;
        assert_eq!(
            "sun/java2d/x11/X11SurfaceData.isShmPMAvailable()Z",
            result.unwrap_err().to_string()
        );
    }
}
