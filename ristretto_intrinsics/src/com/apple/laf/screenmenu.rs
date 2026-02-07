use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "com/apple/laf/ScreenMenu.addMenuListeners(Lcom/apple/laf/ScreenMenu;J)J",
    Any
)]
#[async_method]
pub async fn add_menu_listeners<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.apple.laf.ScreenMenu.addMenuListeners(Lcom/apple/laf/ScreenMenu;J)J")
}

#[intrinsic_method("com/apple/laf/ScreenMenu.removeMenuListeners(J)V", Any)]
#[async_method]
pub async fn remove_menu_listeners<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.apple.laf.ScreenMenu.removeMenuListeners(J)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.apple.laf.ScreenMenu.addMenuListeners(Lcom/apple/laf/ScreenMenu;J)J"
    )]
    async fn test_add_menu_listeners() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = add_menu_listeners(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.apple.laf.ScreenMenu.removeMenuListeners(J)V"
    )]
    async fn test_remove_menu_listeners() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = remove_menu_listeners(thread, Parameters::default()).await;
    }
}
