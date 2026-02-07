use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/awt/CGraphicsConfig.nativeGetBounds(I)Ljava/awt/geom/Rectangle2D;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_get_bounds<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.CGraphicsConfig.nativeGetBounds(I)Ljava/awt/geom/Rectangle2D;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.awt.CGraphicsConfig.nativeGetBounds(I)Ljava/awt/geom/Rectangle2D;"
    )]
    async fn test_native_get_bounds() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_get_bounds(thread, Parameters::default()).await;
    }
}
