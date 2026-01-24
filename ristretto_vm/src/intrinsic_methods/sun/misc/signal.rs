use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "sun/misc/Signal.findSignal(Ljava/lang/String;)I",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn find_signal(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.misc.Signal.findSignal(Ljava/lang/String;)I")
}

#[intrinsic_method("sun/misc/Signal.handle0(IJ)J", LessThanOrEqual(JAVA_8))]
#[async_method]
pub(crate) async fn handle_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.misc.Signal.handle0(IJ)J")
}

#[intrinsic_method("sun/misc/Signal.raise0(I)V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub(crate) async fn raise_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.misc.Signal.raise0(I)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.misc.Signal.findSignal(Ljava/lang/String;)I"
    )]
    async fn test_find_signal() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = find_signal(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.misc.Signal.handle0(IJ)J")]
    async fn test_handle_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = handle_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.misc.Signal.raise0(I)V")]
    async fn test_raise_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = raise_0(thread, Parameters::default()).await;
    }
}
