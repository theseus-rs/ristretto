use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/java2d/loops/ScaledBlit.Scale(Lsun/java2d/SurfaceData;Lsun/java2d/SurfaceData;Ljava/awt/Composite;Lsun/java2d/pipe/Region;IIIIDDDD)V",
    Any
)]
#[async_method]
pub async fn scale<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.java2d.loops.ScaledBlit.Scale(Lsun/java2d/SurfaceData;Lsun/java2d/SurfaceData;Ljava/awt/Composite;Lsun/java2d/pipe/Region;IIIIDDDD)V".to_string()).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_scale() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = scale(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
