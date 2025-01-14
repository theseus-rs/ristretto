use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.awt.DefaultMouseInfoPeer`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/awt/DefaultMouseInfoPeer";
    registry.register(
        class_name,
        "fillPointWithCoords",
        "(Ljava/awt/Point;)I",
        fill_point_with_coords,
    );
    registry.register(
        class_name,
        "isWindowUnderMouse",
        "(Ljava/awt/Window;)Z",
        is_window_under_mouse,
    );
}

#[async_recursion(?Send)]
async fn fill_point_with_coords(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.awt.DefaultMouseInfoPeer.fillPointWithCoords(Ljava/awt/Point;)I")
}

#[async_recursion(?Send)]
async fn is_window_under_mouse(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.awt.DefaultMouseInfoPeer.isWindowUnderMouse(Ljava/awt/Window;)Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/awt/DefaultMouseInfoPeer";
        assert!(registry
            .method(class_name, "fillPointWithCoords", "(Ljava/awt/Point;)I")
            .is_some());
        assert!(registry
            .method(class_name, "isWindowUnderMouse", "(Ljava/awt/Window;)Z")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.awt.DefaultMouseInfoPeer.fillPointWithCoords(Ljava/awt/Point;)I"
    )]
    async fn test_fill_point_with_coords() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = fill_point_with_coords(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.awt.DefaultMouseInfoPeer.isWindowUnderMouse(Ljava/awt/Window;)Z"
    )]
    async fn test_is_window_under_mouse() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_window_under_mouse(thread, Arguments::default()).await;
    }
}
