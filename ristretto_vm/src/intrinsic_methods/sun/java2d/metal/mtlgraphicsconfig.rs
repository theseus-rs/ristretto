use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::{Between, GreaterThanOrEqual};
use ristretto_classfile::{JAVA_17, JAVA_21};
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "sun/java2d/metal/MTLGraphicsConfig.getMTLConfigInfo(ILjava/lang/String;)J",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_mtl_config_info(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.metal.MTLGraphicsConfig.getMTLConfigInfo(ILjava/lang/String;)J")
}

#[intrinsic_method(
    "sun/java2d/metal/MTLGraphicsConfig.isMetalFrameworkAvailable()Z",
    Between(JAVA_17, JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn is_metal_framework_available(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.metal.MTLGraphicsConfig.isMetalFrameworkAvailable()Z")
}

#[intrinsic_method(
    "sun/java2d/metal/MTLGraphicsConfig.nativeGetMaxTextureSize()I",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn native_get_max_texture_size(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.metal.MTLGraphicsConfig.nativeGetMaxTextureSize()I")
}

#[intrinsic_method(
    "sun/java2d/metal/MTLGraphicsConfig.tryLoadMetalLibrary(ILjava/lang/String;)Z",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn try_load_metal_library(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.metal.MTLGraphicsConfig.tryLoadMetalLibrary(ILjava/lang/String;)Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.metal.MTLGraphicsConfig.getMTLConfigInfo(ILjava/lang/String;)J"
    )]
    async fn test_get_mtl_config_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_mtl_config_info(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.metal.MTLGraphicsConfig.isMetalFrameworkAvailable()Z"
    )]
    async fn test_is_metal_framework_available() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_metal_framework_available(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.metal.MTLGraphicsConfig.nativeGetMaxTextureSize()I"
    )]
    async fn test_native_get_max_texture_size() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_get_max_texture_size(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.metal.MTLGraphicsConfig.tryLoadMetalLibrary(ILjava/lang/String;)Z"
    )]
    async fn test_try_load_metal_library() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = try_load_metal_library(thread, Parameters::default()).await;
    }
}
