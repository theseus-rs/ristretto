use ristretto_classfile::JAVA_8;
#[cfg(target_os = "linux")]
use ristretto_classfile::JAVA_11;
#[cfg(target_os = "linux")]
use ristretto_classfile::JAVA_17;
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
    "sun/awt/X11GraphicsEnvironment.checkShmExt()I",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn check_shm_ext<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.awt.X11GraphicsEnvironment.checkShmExt()I".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/awt/X11GraphicsEnvironment.getDefaultScreenNum()I",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_default_screen_num<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.awt.X11GraphicsEnvironment.getDefaultScreenNum()I".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/awt/X11GraphicsEnvironment.getDisplayString()Ljava/lang/String;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_display_string<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.awt.X11GraphicsEnvironment.getDisplayString()Ljava/lang/String;".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/awt/X11GraphicsEnvironment.getNumScreens()I",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_num_screens<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.awt.X11GraphicsEnvironment.getNumScreens()I".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/awt/X11GraphicsEnvironment.getXineramaCenterPoint()Ljava/awt/Point;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_xinerama_center_point<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.awt.X11GraphicsEnvironment.getXineramaCenterPoint()Ljava/awt/Point;".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/awt/X11GraphicsEnvironment.initDisplay(Z)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn init_display<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _glx_requested = parameters.pop_bool()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.awt.X11GraphicsEnvironment.initDisplay(Z)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/awt/X11GraphicsEnvironment.initGLX()Z", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn init_glx<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun.awt.X11GraphicsEnvironment.initGLX()Z".to_string())
            .into(),
    )
}

#[intrinsic_method(
    "sun/awt/X11GraphicsEnvironment.initXRender(ZZ)Z",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn init_x_render<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _ignore_linux_version = parameters.pop_bool()?;
    let _verbose = parameters.pop_bool()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.awt.X11GraphicsEnvironment.initXRender(ZZ)Z".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/awt/X11GraphicsEnvironment.pRunningXinerama()Z",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn p_running_xinerama<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.awt.X11GraphicsEnvironment.pRunningXinerama()Z".to_string(),
    )
    .into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/awt/X11GraphicsEnvironment.initNativeData()V",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn init_native_data<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11GraphicsEnvironment.initNativeData()V".to_string(),
    )
    .into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/awt/X11GraphicsEnvironment.checkShmExt()I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn check_shm_ext_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11GraphicsEnvironment.checkShmExt()I".to_string(),
    )
    .into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/awt/X11GraphicsEnvironment.getDefaultScreenNum()I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_default_screen_num_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11GraphicsEnvironment.getDefaultScreenNum()I".to_string(),
    )
    .into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/awt/X11GraphicsEnvironment.getDisplayString()Ljava/lang/String;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_display_string_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11GraphicsEnvironment.getDisplayString()Ljava/lang/String;".to_string(),
    )
    .into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/awt/X11GraphicsEnvironment.getNumScreens()I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_num_screens_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11GraphicsEnvironment.getNumScreens()I".to_string(),
    )
    .into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/awt/X11GraphicsEnvironment.getXineramaCenterPoint()Ljava/awt/Point;",
    Equal(JAVA_11)
)]
#[async_method]
pub async fn get_xinerama_center_point_linux_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11GraphicsEnvironment.getXineramaCenterPoint()Ljava/awt/Point;".to_string(),
    )
    .into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/awt/X11GraphicsEnvironment.initDisplay(Z)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn init_display_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _glx_requested = parameters.pop_bool()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11GraphicsEnvironment.initDisplay(Z)V".to_string(),
    )
    .into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/awt/X11GraphicsEnvironment.initGLX()Z",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn init_glx_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11GraphicsEnvironment.initGLX()Z".to_string())
            .into(),
    )
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/awt/X11GraphicsEnvironment.initNativeData()V",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn init_native_data_linux_ge_v17<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11GraphicsEnvironment.initNativeData()V".to_string(),
    )
    .into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/awt/X11GraphicsEnvironment.initXRender(ZZ)Z",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn init_xrender_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _ignore_linux_version = parameters.pop_bool()?;
    let _verbose = parameters.pop_bool()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11GraphicsEnvironment.initXRender(ZZ)Z".to_string(),
    )
    .into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/awt/X11GraphicsEnvironment.pRunningXinerama()Z",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn p_running_xinerama_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11GraphicsEnvironment.pRunningXinerama()Z".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_check_shm_ext() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = check_shm_ext(thread, Parameters::default()).await;
        assert_eq!(
            "sun.awt.X11GraphicsEnvironment.checkShmExt()I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_default_screen_num() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = get_default_screen_num(thread, Parameters::default()).await;
        assert_eq!(
            "sun.awt.X11GraphicsEnvironment.getDefaultScreenNum()I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_display_string() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = get_display_string(thread, Parameters::default()).await;
        assert_eq!(
            "sun.awt.X11GraphicsEnvironment.getDisplayString()Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_num_screens() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = get_num_screens(thread, Parameters::default()).await;
        assert_eq!(
            "sun.awt.X11GraphicsEnvironment.getNumScreens()I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_xinerama_center_point() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = get_xinerama_center_point(thread, Parameters::default()).await;
        assert_eq!(
            "sun.awt.X11GraphicsEnvironment.getXineramaCenterPoint()Ljava/awt/Point;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_init_display() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = init_display(thread, Parameters::new(vec![Value::from(false)])).await;
        assert_eq!(
            "sun.awt.X11GraphicsEnvironment.initDisplay(Z)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_init_glx() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = init_glx(thread, Parameters::default()).await;
        assert_eq!(
            "sun.awt.X11GraphicsEnvironment.initGLX()Z",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_init_x_render() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = init_x_render(
            thread,
            Parameters::new(vec![Value::from(false), Value::from(false)]),
        )
        .await;
        assert_eq!(
            "sun.awt.X11GraphicsEnvironment.initXRender(ZZ)Z",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_p_running_xinerama() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = p_running_xinerama(thread, Parameters::default()).await;
        assert_eq!(
            "sun.awt.X11GraphicsEnvironment.pRunningXinerama()Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_init_native_data() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_native_data(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/X11GraphicsEnvironment.initNativeData()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_check_shm_ext_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = check_shm_ext_linux_ge_v11(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/X11GraphicsEnvironment.checkShmExt()I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_get_default_screen_num_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_default_screen_num_linux_ge_v11(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/X11GraphicsEnvironment.getDefaultScreenNum()I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_get_display_string_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_display_string_linux_ge_v11(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/X11GraphicsEnvironment.getDisplayString()Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_get_num_screens_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_num_screens_linux_ge_v11(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/X11GraphicsEnvironment.getNumScreens()I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_get_xinerama_center_point_linux_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_xinerama_center_point_linux_v11(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/X11GraphicsEnvironment.getXineramaCenterPoint()Ljava/awt/Point;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_init_display_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            init_display_linux_ge_v11(thread, Parameters::new(vec![Value::from(false)])).await;
        assert_eq!(
            "sun/awt/X11GraphicsEnvironment.initDisplay(Z)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_init_glx_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_glx_linux_ge_v11(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/X11GraphicsEnvironment.initGLX()Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_init_native_data_linux_ge_v17() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_native_data_linux_ge_v17(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/X11GraphicsEnvironment.initNativeData()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_init_xrender_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_xrender_linux_ge_v11(
            thread,
            Parameters::new(vec![Value::from(false), Value::from(false)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11GraphicsEnvironment.initXRender(ZZ)Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_p_running_xinerama_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = p_running_xinerama_linux_ge_v11(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/X11GraphicsEnvironment.pRunningXinerama()Z",
            result.unwrap_err().to_string()
        );
    }
}
