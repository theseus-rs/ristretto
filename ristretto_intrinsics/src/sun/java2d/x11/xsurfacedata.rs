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

#[intrinsic_method("sun/java2d/x11/XSurfaceData.XCreateGC(J)J", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn x_create_gc<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _p_xs_data = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun.java2d.x11.XSurfaceData.XCreateGC(J)J".to_string())
            .into(),
    )
}

#[intrinsic_method("sun/java2d/x11/XSurfaceData.XResetClip(J)V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn x_reset_clip<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _xgc = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun.java2d.x11.XSurfaceData.XResetClip(J)V".to_string())
            .into(),
    )
}

#[intrinsic_method(
    "sun/java2d/x11/XSurfaceData.XSetClip(JIIIILsun/java2d/pipe/Region;)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn x_set_clip<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _complexclip = parameters.pop_reference()?;
    let _y2 = parameters.pop_int()?;
    let _x2 = parameters.pop_int()?;
    let _y1 = parameters.pop_int()?;
    let _x1 = parameters.pop_int()?;
    let _xgc = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.x11.XSurfaceData.XSetClip(JIIIILsun/java2d/pipe/Region;)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/x11/XSurfaceData.XSetGraphicsExposures(JZ)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn x_set_graphics_exposures<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _need_exposures = parameters.pop_bool()?;
    let _xgc = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.x11.XSurfaceData.XSetGraphicsExposures(JZ)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/x11/XSurfaceData.flushNativeSurface()V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn flush_native_surface<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.x11.XSurfaceData.flushNativeSurface()V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/x11/XSurfaceData.initOps(Lsun/awt/X11ComponentPeer;Lsun/awt/X11GraphicsConfig;I)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn init_ops<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _depth = parameters.pop_int()?;
    let _graphics_config = parameters.pop_reference()?;
    let _peer = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun.java2d.x11.XSurfaceData.initOps(Lsun/awt/X11ComponentPeer;Lsun/awt/X11GraphicsConfig;I)V".to_string()).into())
}

#[intrinsic_method(
    "sun/java2d/x11/XSurfaceData.isDrawableValid()Z",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn is_drawable_valid<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.x11.XSurfaceData.isDrawableValid()Z".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/java2d/x11/XSurfaceData.setInvalid()V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn set_invalid<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun.java2d.x11.XSurfaceData.setInvalid()V".to_string())
            .into(),
    )
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/java2d/x11/XSurfaceData.XCreateGC(J)J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn xcreate_gc_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _p_xs_data = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/java2d/x11/XSurfaceData.XCreateGC(J)J".to_string())
            .into(),
    )
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/java2d/x11/XSurfaceData.XResetClip(J)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn xreset_clip_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _xgc = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/java2d/x11/XSurfaceData.XResetClip(J)V".to_string())
            .into(),
    )
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/java2d/x11/XSurfaceData.XSetClip(JIIIILsun/java2d/pipe/Region;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn xset_clip_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _complexclip = parameters.pop_reference()?;
    let _y2 = parameters.pop_int()?;
    let _x2 = parameters.pop_int()?;
    let _y1 = parameters.pop_int()?;
    let _x1 = parameters.pop_int()?;
    let _xgc = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/x11/XSurfaceData.XSetClip(JIIIILsun/java2d/pipe/Region;)V".to_string(),
    )
    .into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/java2d/x11/XSurfaceData.XSetGraphicsExposures(JZ)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn xset_graphics_exposures_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _need_exposures = parameters.pop_bool()?;
    let _xgc = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/x11/XSurfaceData.XSetGraphicsExposures(JZ)V".to_string(),
    )
    .into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/java2d/x11/XSurfaceData.flushNativeSurface()V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn flush_native_surface_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/x11/XSurfaceData.flushNativeSurface()V".to_string(),
    )
    .into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/java2d/x11/XSurfaceData.initOps(Lsun/awt/X11ComponentPeer;Lsun/awt/X11GraphicsConfig;I)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn init_ops_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _depth = parameters.pop_int()?;
    let _graphics_config = parameters.pop_reference()?;
    let _peer = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun/java2d/x11/XSurfaceData.initOps(Lsun/awt/X11ComponentPeer;Lsun/awt/X11GraphicsConfig;I)V".to_string()).into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/java2d/x11/XSurfaceData.isDrawableValid()Z",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn is_drawable_valid_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/x11/XSurfaceData.isDrawableValid()Z".to_string(),
    )
    .into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/java2d/x11/XSurfaceData.setInvalid()V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn set_invalid_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/java2d/x11/XSurfaceData.setInvalid()V".to_string())
            .into(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_x_create_gc() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = x_create_gc(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.java2d.x11.XSurfaceData.XCreateGC(J)J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_x_reset_clip() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = x_reset_clip(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.java2d.x11.XSurfaceData.XResetClip(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_x_set_clip() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = x_set_clip(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "sun.java2d.x11.XSurfaceData.XSetClip(JIIIILsun/java2d/pipe/Region;)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_x_set_graphics_exposures() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = x_set_graphics_exposures(
            thread,
            Parameters::new(vec![Value::Long(0), Value::from(false)]),
        )
        .await;
        assert_eq!(
            "sun.java2d.x11.XSurfaceData.XSetGraphicsExposures(JZ)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_flush_native_surface() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = flush_native_surface(thread, Parameters::default()).await;
        assert_eq!(
            "sun.java2d.x11.XSurfaceData.flushNativeSurface()V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_init_ops() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = init_ops(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.java2d.x11.XSurfaceData.initOps(Lsun/awt/X11ComponentPeer;Lsun/awt/X11GraphicsConfig;I)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_is_drawable_valid() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = is_drawable_valid(thread, Parameters::default()).await;
        assert_eq!(
            "sun.java2d.x11.XSurfaceData.isDrawableValid()Z",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_set_invalid() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = set_invalid(thread, Parameters::default()).await;
        assert_eq!(
            "sun.java2d.x11.XSurfaceData.setInvalid()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xcreate_gc_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xcreate_gc_linux_ge_v11(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/java2d/x11/XSurfaceData.XCreateGC(J)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xreset_clip_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xreset_clip_linux_ge_v11(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/java2d/x11/XSurfaceData.XResetClip(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xset_clip_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xset_clip_linux_ge_v11(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "sun/java2d/x11/XSurfaceData.XSetClip(JIIIILsun/java2d/pipe/Region;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xset_graphics_exposures_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xset_graphics_exposures_linux_ge_v11(
            thread,
            Parameters::new(vec![Value::Long(0), Value::from(false)]),
        )
        .await;
        assert_eq!(
            "sun/java2d/x11/XSurfaceData.XSetGraphicsExposures(JZ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_flush_native_surface_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = flush_native_surface_linux_ge_v11(thread, Parameters::default()).await;
        assert_eq!(
            "sun/java2d/x11/XSurfaceData.flushNativeSurface()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_init_ops_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_ops_linux_ge_v11(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/java2d/x11/XSurfaceData.initOps(Lsun/awt/X11ComponentPeer;Lsun/awt/X11GraphicsConfig;I)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_is_drawable_valid_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = is_drawable_valid_linux_ge_v11(thread, Parameters::default()).await;
        assert_eq!(
            "sun/java2d/x11/XSurfaceData.isDrawableValid()Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_set_invalid_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_invalid_linux_ge_v11(thread, Parameters::default()).await;
        assert_eq!(
            "sun/java2d/x11/XSurfaceData.setInvalid()V",
            result.unwrap_err().to_string()
        );
    }
}
