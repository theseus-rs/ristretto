use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "sun/awt/DefaultMouseInfoPeer.fillPointWithCoords(Ljava/awt/Point;)I",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn fill_point_with_coords(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.DefaultMouseInfoPeer.fillPointWithCoords(Ljava/awt/Point;)I")
}

#[intrinsic_method(
    "sun/awt/DefaultMouseInfoPeer.isWindowUnderMouse(Ljava/awt/Window;)Z",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn is_window_under_mouse(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.DefaultMouseInfoPeer.isWindowUnderMouse(Ljava/awt/Window;)Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.awt.DefaultMouseInfoPeer.fillPointWithCoords(Ljava/awt/Point;)I"
    )]
    async fn test_fill_point_with_coords() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = fill_point_with_coords(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.awt.DefaultMouseInfoPeer.isWindowUnderMouse(Ljava/awt/Window;)Z"
    )]
    async fn test_is_window_under_mouse() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_window_under_mouse(thread, Parameters::default()).await;
    }
}
