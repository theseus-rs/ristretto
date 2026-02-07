use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("apple/laf/JRSUIFocus.beginNativeFocus(JI)I", Any)]
#[async_method]
pub async fn begin_native_focus<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("apple.laf.JRSUIFocus.beginNativeFocus(JI)I")
}

#[intrinsic_method("apple/laf/JRSUIFocus.endNativeFocus(J)I", Any)]
#[async_method]
pub async fn end_native_focus<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("apple.laf.JRSUIFocus.endNativeFocus(J)I")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: apple.laf.JRSUIFocus.beginNativeFocus(JI)I")]
    async fn test_begin_native_focus() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = begin_native_focus(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: apple.laf.JRSUIFocus.endNativeFocus(J)I")]
    async fn test_end_native_focus() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = end_native_focus(thread, Parameters::default()).await;
    }
}
