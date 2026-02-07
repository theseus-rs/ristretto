use ristretto_classfile::VersionSpecification::{Any, GreaterThanOrEqual, LessThanOrEqual};
use ristretto_classfile::{JAVA_11, JAVA_17};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/java2d/opengl/CGLGraphicsConfig.getCGLConfigInfo()J",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn get_cgl_config_info_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.opengl.CGLGraphicsConfig.getCGLConfigInfo()J")
}

#[intrinsic_method(
    "sun/java2d/opengl/CGLGraphicsConfig.getCGLConfigInfo(III)J",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_cgl_config_info_1<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.opengl.CGLGraphicsConfig.getCGLConfigInfo(III)J")
}

#[intrinsic_method("sun/java2d/opengl/CGLGraphicsConfig.getOGLCapabilities(J)I", Any)]
#[async_method]
pub async fn get_ogl_capabilities<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.opengl.CGLGraphicsConfig.getOGLCapabilities(J)I")
}

#[intrinsic_method("sun/java2d/opengl/CGLGraphicsConfig.initCGL()Z", Any)]
#[async_method]
pub async fn init_cgl<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.opengl.CGLGraphicsConfig.initCGL()Z")
}

#[intrinsic_method("sun/java2d/opengl/CGLGraphicsConfig.nativeGetMaxTextureSize()I", Any)]
#[async_method]
pub async fn native_get_max_texture_size<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.opengl.CGLGraphicsConfig.nativeGetMaxTextureSize()I")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.opengl.CGLGraphicsConfig.getCGLConfigInfo()J"
    )]
    async fn test_get_cgl_config_info_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_cgl_config_info_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.opengl.CGLGraphicsConfig.getCGLConfigInfo(III)J"
    )]
    async fn test_get_cgl_config_info_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_cgl_config_info_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.opengl.CGLGraphicsConfig.getOGLCapabilities(J)I"
    )]
    async fn test_get_ogl_capabilities() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_ogl_capabilities(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.opengl.CGLGraphicsConfig.initCGL()Z"
    )]
    async fn test_init_cgl() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_cgl(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.opengl.CGLGraphicsConfig.nativeGetMaxTextureSize()I"
    )]
    async fn test_native_get_max_texture_size() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_get_max_texture_size(thread, Parameters::default()).await;
    }
}
