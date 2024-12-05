use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_21: Version = Version::Java21 { minor: 0 };

/// Register all native methods for `sun.java2d.metal.MTLGraphicsConfig`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/java2d/metal/MTLGraphicsConfig";
    let java_version = registry.java_version();

    if java_version <= &JAVA_21 {
        registry.register(
            class_name,
            "isMetalFrameworkAvailable",
            "()Z",
            is_metal_framework_available,
        );
    }

    registry.register(
        class_name,
        "getMTLConfigInfo",
        "(ILjava/lang/String;)J",
        get_mtl_config_info,
    );
    registry.register(
        class_name,
        "nativeGetMaxTextureSize",
        "()I",
        native_get_max_texture_size,
    );
    registry.register(
        class_name,
        "tryLoadMetalLibrary",
        "(ILjava/lang/String;)Z",
        try_load_metal_library,
    );
}

#[async_recursion(?Send)]
async fn get_mtl_config_info(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.metal.MTLGraphicsConfig.getMTLConfigInfo(ILjava/lang/String;)J")
}

#[async_recursion(?Send)]
async fn is_metal_framework_available(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.java2d.metal.MTLGraphicsConfig.isMetalFrameworkAvailable()Z")
}

#[async_recursion(?Send)]
async fn native_get_max_texture_size(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.java2d.metal.MTLGraphicsConfig.nativeGetMaxTextureSize()I")
}

#[async_recursion(?Send)]
async fn try_load_metal_library(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.java2d.metal.MTLGraphicsConfig.tryLoadMetalLibrary(ILjava/lang/String;)Z")
}
