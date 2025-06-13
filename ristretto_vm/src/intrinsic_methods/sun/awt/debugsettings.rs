use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("sun/awt/DebugSettings.setCTracingOn(Z)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn set_c_tracing_on_1(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.DebugSettings.setCTracingOn(Z)V")
}

#[intrinsic_method("sun/awt/DebugSettings.setCTracingOn(ZLjava/lang/String;)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn set_c_tracing_on_2(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.DebugSettings.setCTracingOn(ZLjava/lang/String;)V")
}

#[intrinsic_method("sun/awt/DebugSettings.setCTracingOn(ZLjava/lang/String;I)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn set_c_tracing_on_3(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.DebugSettings.setCTracingOn(ZLjava/lang/String;I)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.awt.DebugSettings.setCTracingOn(Z)V")]
    async fn test_set_c_tracing_on_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_c_tracing_on_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.awt.DebugSettings.setCTracingOn(ZLjava/lang/String;)V"
    )]
    async fn test_set_c_tracing_on_2() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_c_tracing_on_2(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.awt.DebugSettings.setCTracingOn(ZLjava/lang/String;I)V"
    )]
    async fn test_set_c_tracing_on_3() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_c_tracing_on_3(thread, Parameters::default()).await;
    }
}
