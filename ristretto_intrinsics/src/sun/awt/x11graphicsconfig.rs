use ristretto_classfile::JAVA_8;
#[cfg(target_os = "linux")]
use ristretto_classfile::JAVA_11;
#[cfg(target_os = "linux")]
use ristretto_classfile::VersionSpecification::Equal;
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
    "sun/awt/X11GraphicsConfig.createBackBuffer(JI)J",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn create_back_buffer<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _swap_action = parameters.pop_int()?;
    let _window = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.awt.X11GraphicsConfig.createBackBuffer(JI)J".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/awt/X11GraphicsConfig.destroyBackBuffer(J)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn destroy_back_buffer<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _back_buffer = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.awt.X11GraphicsConfig.destroyBackBuffer(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/awt/X11GraphicsConfig.dispose(J)V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn dispose<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _x11_config_data = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun.awt.X11GraphicsConfig.dispose(J)V".to_string()).into())
}

#[intrinsic_method("sun/awt/X11GraphicsConfig.getNumColors()I", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn get_num_colors<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun.awt.X11GraphicsConfig.getNumColors()I".to_string())
            .into(),
    )
}

#[intrinsic_method(
    "sun/awt/X11GraphicsConfig.getXResolution(I)D",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_x_resolution<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _screen = parameters.pop_int()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun.awt.X11GraphicsConfig.getXResolution(I)D".to_string())
            .into(),
    )
}

#[intrinsic_method(
    "sun/awt/X11GraphicsConfig.getYResolution(I)D",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_y_resolution<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _screen = parameters.pop_int()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun.awt.X11GraphicsConfig.getYResolution(I)D".to_string())
            .into(),
    )
}

#[intrinsic_method("sun/awt/X11GraphicsConfig.init(II)V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn init<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _screen = parameters.pop_int()?;
    let _visual_num = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError("sun.awt.X11GraphicsConfig.init(II)V".to_string()).into())
}

#[intrinsic_method("sun/awt/X11GraphicsConfig.initIDs()V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "sun/awt/X11GraphicsConfig.isTranslucencyCapable(J)Z",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn is_translucency_capable<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _x11_config_data = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.awt.X11GraphicsConfig.isTranslucencyCapable(J)Z".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/awt/X11GraphicsConfig.makeColorModel()Ljava/awt/image/ColorModel;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn make_color_model<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.awt.X11GraphicsConfig.makeColorModel()Ljava/awt/image/ColorModel;".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/awt/X11GraphicsConfig.pGetBounds(I)Ljava/awt/Rectangle;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn p_get_bounds<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _screen_num = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.awt.X11GraphicsConfig.pGetBounds(I)Ljava/awt/Rectangle;".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/awt/X11GraphicsConfig.swapBuffers(JI)V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn swap_buffers<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _swap_action = parameters.pop_int()?;
    let _window = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun.awt.X11GraphicsConfig.swapBuffers(JI)V".to_string())
            .into(),
    )
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/awt/X11GraphicsConfig.createBackBuffer(JI)J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn create_back_buffer_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _swap_action = parameters.pop_int()?;
    let _window = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11GraphicsConfig.createBackBuffer(JI)J".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/awt/X11GraphicsConfig.destroyBackBuffer(J)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn destroy_back_buffer_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _back_buffer = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11GraphicsConfig.destroyBackBuffer(J)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method("sun/awt/X11GraphicsConfig.dispose(J)V", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn dispose_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _x11_config_data = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun/awt/X11GraphicsConfig.dispose(J)V".to_string()).into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/awt/X11GraphicsConfig.getNumColors()I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_num_colors_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11GraphicsConfig.getNumColors()I".to_string())
            .into(),
    )
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/awt/X11GraphicsConfig.getXResolution(I)D",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_xresolution_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _screen = parameters.pop_int()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11GraphicsConfig.getXResolution(I)D".to_string())
            .into(),
    )
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/awt/X11GraphicsConfig.getYResolution(I)D",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_yresolution_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _screen = parameters.pop_int()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11GraphicsConfig.getYResolution(I)D".to_string())
            .into(),
    )
}
#[cfg(target_os = "linux")]
#[intrinsic_method("sun/awt/X11GraphicsConfig.init(II)V", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn init_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _screen = parameters.pop_int()?;
    let _visual_num = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError("sun/awt/X11GraphicsConfig.init(II)V".to_string()).into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method("sun/awt/X11GraphicsConfig.initIDs()V", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn init_ids_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun/awt/X11GraphicsConfig.initIDs()V".to_string()).into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/awt/X11GraphicsConfig.isTranslucencyCapable(J)Z",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn is_translucency_capable_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _x11_config_data = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11GraphicsConfig.isTranslucencyCapable(J)Z".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/awt/X11GraphicsConfig.makeColorModel()Ljava/awt/image/ColorModel;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn make_color_model_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11GraphicsConfig.makeColorModel()Ljava/awt/image/ColorModel;".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/awt/X11GraphicsConfig.pGetBounds(I)Ljava/awt/Rectangle;",
    Equal(JAVA_11)
)]
#[async_method]
pub async fn p_get_bounds_linux_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _screen_num = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11GraphicsConfig.pGetBounds(I)Ljava/awt/Rectangle;".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/awt/X11GraphicsConfig.swapBuffers(JI)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn swap_buffers_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _swap_action = parameters.pop_int()?;
    let _window = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11GraphicsConfig.swapBuffers(JI)V".to_string())
            .into(),
    )
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_back_buffer() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result =
            create_back_buffer(thread, Parameters::new(vec![Value::Long(0), Value::Int(0)])).await;
        assert_eq!(
            "sun.awt.X11GraphicsConfig.createBackBuffer(JI)J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_destroy_back_buffer() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = destroy_back_buffer(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.awt.X11GraphicsConfig.destroyBackBuffer(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_dispose() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = dispose(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.awt.X11GraphicsConfig.dispose(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_num_colors() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = get_num_colors(thread, Parameters::default()).await;
        assert_eq!(
            "sun.awt.X11GraphicsConfig.getNumColors()I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_x_resolution() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = get_x_resolution(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun.awt.X11GraphicsConfig.getXResolution(I)D",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_y_resolution() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = get_y_resolution(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun.awt.X11GraphicsConfig.getYResolution(I)D",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_init() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = init(thread, Parameters::new(vec![Value::Int(0), Value::Int(0)])).await;
        assert_eq!(
            "sun.awt.X11GraphicsConfig.init(II)V",
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
    async fn test_is_translucency_capable() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = is_translucency_capable(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.awt.X11GraphicsConfig.isTranslucencyCapable(J)Z",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_make_color_model() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = make_color_model(thread, Parameters::default()).await;
        assert_eq!(
            "sun.awt.X11GraphicsConfig.makeColorModel()Ljava/awt/image/ColorModel;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_p_get_bounds() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = p_get_bounds(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun.awt.X11GraphicsConfig.pGetBounds(I)Ljava/awt/Rectangle;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_swap_buffers() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result =
            swap_buffers(thread, Parameters::new(vec![Value::Long(0), Value::Int(0)])).await;
        assert_eq!(
            "sun.awt.X11GraphicsConfig.swapBuffers(JI)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_create_back_buffer_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create_back_buffer_linux_ge_v11(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11GraphicsConfig.createBackBuffer(JI)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_destroy_back_buffer_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            destroy_back_buffer_linux_ge_v11(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/awt/X11GraphicsConfig.destroyBackBuffer(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_dispose_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = dispose_linux_ge_v11(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/awt/X11GraphicsConfig.dispose(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_get_num_colors_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_num_colors_linux_ge_v11(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/X11GraphicsConfig.getNumColors()I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_get_xresolution_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            get_xresolution_linux_ge_v11(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/awt/X11GraphicsConfig.getXResolution(I)D",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_get_yresolution_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            get_yresolution_linux_ge_v11(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/awt/X11GraphicsConfig.getYResolution(I)D",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_init_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            init_linux_ge_v11(thread, Parameters::new(vec![Value::Int(0), Value::Int(0)])).await;
        assert_eq!(
            "sun/awt/X11GraphicsConfig.init(II)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_init_ids_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_ids_linux_ge_v11(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/X11GraphicsConfig.initIDs()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_is_translucency_capable_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            is_translucency_capable_linux_ge_v11(thread, Parameters::new(vec![Value::Long(0)]))
                .await;
        assert_eq!(
            "sun/awt/X11GraphicsConfig.isTranslucencyCapable(J)Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_make_color_model_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = make_color_model_linux_ge_v11(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/X11GraphicsConfig.makeColorModel()Ljava/awt/image/ColorModel;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_p_get_bounds_linux_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = p_get_bounds_linux_v11(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/awt/X11GraphicsConfig.pGetBounds(I)Ljava/awt/Rectangle;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_swap_buffers_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            swap_buffers_linux_ge_v11(thread, Parameters::new(vec![Value::Long(0), Value::Int(0)]))
                .await;
        assert_eq!(
            "sun/awt/X11GraphicsConfig.swapBuffers(JI)V",
            result.unwrap_err().to_string()
        );
    }
}
