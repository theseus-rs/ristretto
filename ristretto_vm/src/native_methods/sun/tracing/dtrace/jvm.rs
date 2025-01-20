use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/tracing/dtrace/JVM";

/// Register all native methods for `sun.tracing.dtrace.JVM`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "activate0",
        "(Ljava/lang/String;[Lsun/tracing/dtrace/DTraceProvider;)J",
        activate_0,
    );
    registry.register(
        CLASS_NAME,
        "defineClass0",
        "(Ljava/lang/ClassLoader;Ljava/lang/String;[BII)Ljava/lang/Class;",
        define_class_0,
    );
    registry.register(CLASS_NAME, "dispose0", "(J)V", dispose_0);
    registry.register(
        CLASS_NAME,
        "isEnabled0",
        "(Ljava/lang/reflect/Method;)Z",
        is_enabled_0,
    );
    registry.register(CLASS_NAME, "isSupported0", "()Z", is_supported_0);
}

#[async_recursion(?Send)]
async fn activate_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "sun.tracing.dtrace.JVM.activate0(Ljava/lang/String;[Lsun/tracing/dtrace/DTraceProvider;)J"
    )
}

#[async_recursion(?Send)]
async fn define_class_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.tracing.dtrace.JVM.defineClass0(Ljava/lang/ClassLoader;Ljava/lang/String;[BII)Ljava/lang/Class;")
}

#[async_recursion(?Send)]
async fn dispose_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.tracing.dtrace.JVM.dispose0(J)")
}

#[async_recursion(?Send)]
async fn is_enabled_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.tracing.dtrace.JVM.isEnabled0(Ljava/lang/reflect/Method;)Z")
}

#[async_recursion(?Send)]
async fn is_supported_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.tracing.dtrace.JVM.isSupported0()Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.tracing.dtrace.JVM.activate0(Ljava/lang/String;[Lsun/tracing/dtrace/DTraceProvider;)J"
    )]
    async fn test_activate_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = activate_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.tracing.dtrace.JVM.defineClass0(Ljava/lang/ClassLoader;Ljava/lang/String;[BII)Ljava/lang/Class;"
    )]
    async fn test_define_class_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = define_class_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.tracing.dtrace.JVM.dispose0(J)")]
    async fn test_dispose_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = dispose_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.tracing.dtrace.JVM.isEnabled0(Ljava/lang/reflect/Method;)Z"
    )]
    async fn test_is_enabled_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_enabled_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.tracing.dtrace.JVM.isSupported0()Z")]
    async fn test_is_supported_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_supported_0(thread, Parameters::default()).await;
    }
}
