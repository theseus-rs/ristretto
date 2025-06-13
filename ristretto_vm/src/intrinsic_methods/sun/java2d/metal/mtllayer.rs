use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classfile::{JAVA_17, JAVA_21};
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "sun/java2d/metal/MTLLayer.blitTexture(J)V",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn blit_texture(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.metal.MTLLayer.blitTexture(J)V")
}

#[intrinsic_method(
    "sun/java2d/metal/MTLLayer.nativeCreateLayer()J",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn native_create_layer(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.metal.MTLLayer.nativeCreateLayer()J")
}

#[intrinsic_method(
    "sun/java2d/metal/MTLLayer.nativeSetInsets(JII)V",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn native_set_insets(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.metal.MTLLayer.nativeSetInsets(JII)V")
}

#[intrinsic_method(
    "sun/java2d/metal/MTLLayer.nativeSetOpaque(JZ)V",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn native_set_opaque(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.metal.MTLLayer.nativeSetOpaque(JZ)V")
}

#[intrinsic_method(
    "sun/java2d/metal/MTLLayer.nativeSetScale(JD)V",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn native_set_scale(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.metal.MTLLayer.nativeSetScale(JD)V")
}

#[intrinsic_method(
    "sun/java2d/metal/MTLLayer.validate(JLsun/java2d/metal/MTLSurfaceData;)V",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn validate(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.metal.MTLLayer.validate(JLsun/java2d/metal/MTLSurfaceData;)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.java2d.metal.MTLLayer.blitTexture(J)V")]
    async fn test_blit_texture() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = blit_texture(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.metal.MTLLayer.nativeCreateLayer()J"
    )]
    async fn test_native_create_layer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_create_layer(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.metal.MTLLayer.nativeSetInsets(JII)V"
    )]
    async fn test_native_set_insets() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_insets(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.metal.MTLLayer.nativeSetOpaque(JZ)V"
    )]
    async fn test_native_set_opaque() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_opaque(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.java2d.metal.MTLLayer.nativeSetScale(JD)V")]
    async fn test_native_set_scale() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_scale(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.metal.MTLLayer.validate(JLsun/java2d/metal/MTLSurfaceData;)V"
    )]
    async fn test_validate() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = validate(thread, Parameters::default()).await;
    }
}
