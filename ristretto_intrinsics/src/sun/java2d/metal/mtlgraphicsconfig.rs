use ristretto_classfile::VersionSpecification::{Between, GreaterThanOrEqual};
use ristretto_classfile::{JAVA_17, JAVA_21};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/java2d/metal/MTLGraphicsConfig.getMTLConfigInfo(ILjava/lang/String;)J",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn get_mtl_config_info<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.metal.MTLGraphicsConfig.getMTLConfigInfo(ILjava/lang/String;)J".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/metal/MTLGraphicsConfig.isMetalFrameworkAvailable()Z",
    Between(JAVA_17, JAVA_21)
)]
#[async_method]
pub async fn is_metal_framework_available<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.metal.MTLGraphicsConfig.isMetalFrameworkAvailable()Z".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/metal/MTLGraphicsConfig.nativeGetMaxTextureSize()I",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn native_get_max_texture_size<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.metal.MTLGraphicsConfig.nativeGetMaxTextureSize()I".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/metal/MTLGraphicsConfig.tryLoadMetalLibrary(ILjava/lang/String;)Z",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn try_load_metal_library<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.metal.MTLGraphicsConfig.tryLoadMetalLibrary(ILjava/lang/String;)Z".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_mtl_config_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_mtl_config_info(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_is_metal_framework_available() {
        let (_vm, thread) = crate::test::java21_thread().await.expect("thread");
        let result = is_metal_framework_available(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_get_max_texture_size() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_get_max_texture_size(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_try_load_metal_library() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = try_load_metal_library(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
