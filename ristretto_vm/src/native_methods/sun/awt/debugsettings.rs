use crate::Result;
use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/awt/DebugSettings";

/// Register all native methods for `sun.awt.DebugSettings`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "setCTracingOn", "(Z)V", set_c_tracing_on_1);
    registry.register(
        CLASS_NAME,
        "setCTracingOn",
        "(ZLjava/lang/String;)V",
        set_c_tracing_on_2,
    );
    registry.register(
        CLASS_NAME,
        "setCTracingOn",
        "(ZLjava/lang/String;I)V",
        set_c_tracing_on_3,
    );
}

#[async_recursion(?Send)]
async fn set_c_tracing_on_1(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.DebugSettings.setCTracingOn(Z)V")
}

#[async_recursion(?Send)]
async fn set_c_tracing_on_2(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.DebugSettings.setCTracingOn(ZLjava/lang/String;)V")
}

#[async_recursion(?Send)]
async fn set_c_tracing_on_3(
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
