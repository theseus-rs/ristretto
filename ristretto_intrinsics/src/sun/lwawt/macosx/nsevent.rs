use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::{Any, GreaterThan, LessThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/lwawt/macosx/NSEvent.nsKeyModifiersToJavaKeyInfo([I[I)V", Any)]
#[async_method]
pub async fn ns_key_modifiers_to_java_key_info<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.NSEvent.nsKeyModifiersToJavaKeyInfo([I[I)V")
}

#[intrinsic_method("sun/lwawt/macosx/NSEvent.nsToJavaChar(CI)C", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn ns_to_java_char_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.NSEvent.nsToJavaChar(CI)C")
}

#[intrinsic_method("sun/lwawt/macosx/NSEvent.nsToJavaChar(CIZ)C", GreaterThan(JAVA_8))]
#[async_method]
pub async fn ns_to_java_char_1<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.NSEvent.nsToJavaChar(CIZ)C")
}

#[intrinsic_method("sun/lwawt/macosx/NSEvent.nsToJavaKeyInfo([I[I)Z", Any)]
#[async_method]
pub async fn ns_to_java_key_info<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.NSEvent.nsToJavaKeyInfo([I[I)Z")
}

#[intrinsic_method(
    "sun/lwawt/macosx/NSEvent.nsToJavaKeyModifiers(I)I",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn ns_to_java_key_modifiers<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.NSEvent.nsToJavaKeyModifiers(I)I")
}

#[intrinsic_method("sun/lwawt/macosx/NSEvent.nsToJavaModifiers(I)I", GreaterThan(JAVA_8))]
#[async_method]
pub async fn ns_to_java_modifiers<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.NSEvent.nsToJavaModifiers(I)I")
}

#[intrinsic_method(
    "sun/lwawt/macosx/NSEvent.nsToJavaMouseModifiers(II)I",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn ns_to_java_mouse_modifiers<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.NSEvent.nsToJavaMouseModifiers(II)I")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.NSEvent.nsKeyModifiersToJavaKeyInfo([I[I)V"
    )]
    async fn test_ns_key_modifiers_to_java_key_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ns_key_modifiers_to_java_key_info(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.lwawt.macosx.NSEvent.nsToJavaChar(CI)C")]
    async fn test_ns_to_java_char_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ns_to_java_char_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.lwawt.macosx.NSEvent.nsToJavaChar(CIZ)C")]
    async fn test_ns_to_java_char_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ns_to_java_char_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.NSEvent.nsToJavaKeyInfo([I[I)Z"
    )]
    async fn test_ns_to_java_key_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ns_to_java_key_info(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.NSEvent.nsToJavaKeyModifiers(I)I"
    )]
    async fn test_ns_to_java_key_modifiers() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ns_to_java_key_modifiers(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.NSEvent.nsToJavaModifiers(I)I"
    )]
    async fn test_ns_to_java_modifiers() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ns_to_java_modifiers(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.NSEvent.nsToJavaMouseModifiers(II)I"
    )]
    async fn test_ns_to_java_mouse_modifiers() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ns_to_java_mouse_modifiers(thread, Parameters::default()).await;
    }
}
