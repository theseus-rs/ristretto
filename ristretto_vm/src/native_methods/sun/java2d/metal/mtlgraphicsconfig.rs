use crate::native_methods::registry::{MethodRegistry, JAVA_21};
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/java2d/metal/MTLGraphicsConfig";

/// Register all native methods for `sun.java2d.metal.MTLGraphicsConfig`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() <= JAVA_21 {
        registry.register(
            CLASS_NAME,
            "isMetalFrameworkAvailable",
            "()Z",
            is_metal_framework_available,
        );
    }

    registry.register(
        CLASS_NAME,
        "getMTLConfigInfo",
        "(ILjava/lang/String;)J",
        get_mtl_config_info,
    );
    registry.register(
        CLASS_NAME,
        "nativeGetMaxTextureSize",
        "()I",
        native_get_max_texture_size,
    );
    registry.register(
        CLASS_NAME,
        "tryLoadMetalLibrary",
        "(ILjava/lang/String;)Z",
        try_load_metal_library,
    );
}

#[async_recursion(?Send)]
async fn get_mtl_config_info(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.metal.MTLGraphicsConfig.getMTLConfigInfo(ILjava/lang/String;)J")
}

#[async_recursion(?Send)]
async fn is_metal_framework_available(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.metal.MTLGraphicsConfig.isMetalFrameworkAvailable()Z")
}

#[async_recursion(?Send)]
async fn native_get_max_texture_size(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.metal.MTLGraphicsConfig.nativeGetMaxTextureSize()I")
}

#[async_recursion(?Send)]
async fn try_load_metal_library(
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
