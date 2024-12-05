use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_18: Version = Version::Java18 { minor: 0 };

/// Register all native methods for `sun.java2d.metal.MTLLayer`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/java2d/metal/MTLLayer";
    let java_version = registry.java_version();

    if java_version >= &JAVA_18 {
        registry.register(class_name, "nativeSetOpaque", "(JZ)V", native_set_opaque);
    }

    registry.register(class_name, "blitTexture", "(J)V", blit_texture);
    registry.register(class_name, "nativeCreateLayer", "()J", native_create_layer);
    registry.register(class_name, "nativeSetInsets", "(JII)V", native_set_insets);
    registry.register(class_name, "nativeSetScale", "(JD)V", native_set_scale);
    registry.register(
        class_name,
        "validate",
        "(JLsun/java2d/metal/MTLSurfaceData;)V",
        validate,
    );
}

#[async_recursion(?Send)]
async fn blit_texture(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.metal.MTLLayer.blitTexture(J)V")
}

#[async_recursion(?Send)]
async fn native_create_layer(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.metal.MTLLayer.nativeCreateLayer()J")
}

#[async_recursion(?Send)]
async fn native_set_insets(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.metal.MTLLayer.nativeSetInsets(JII)V")
}

#[async_recursion(?Send)]
async fn native_set_opaque(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.metal.MTLLayer.nativeSetOpaque(JZ)V")
}

#[async_recursion(?Send)]
async fn native_set_scale(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.metal.MTLLayer.nativeSetScale(JD)V")
}

#[async_recursion(?Send)]
async fn validate(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.metal.MTLLayer.validate(JLsun/java2d/metal/MTLSurfaceData;)V")
}
