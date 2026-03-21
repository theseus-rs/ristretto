use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/java2d/x11/X11PMBlitBgLoops.nativeBlitBg(JJJIIIIIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_blit_bg<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.x11.X11PMBlitBgLoops.nativeBlitBg(JJJIIIIIII)V".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_native_blit_bg() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = native_blit_bg(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
