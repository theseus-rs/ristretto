use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.tracing.dtrace.JVM`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/tracing/dtrace/JVM";
    registry.register(
        class_name,
        "activate0",
        "(Ljava/lang/String;[Lsun/tracing/dtrace/DTraceProvider;)J",
        activate_0,
    );
    registry.register(
        class_name,
        "defineClass0",
        "(Ljava/lang/ClassLoader;Ljava/lang/String;[BII)Ljava/lang/Class;",
        define_class_0,
    );
    registry.register(class_name, "dispose0", "(J)V", dispose_0);
    registry.register(
        class_name,
        "isEnabled0",
        "(Ljava/lang/reflect/Method;)Z",
        is_enabled_0,
    );
    registry.register(class_name, "isSupported0", "()Z", is_supported_0);
}

#[async_recursion(?Send)]
async fn activate_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!(
        "sun.tracing.dtrace.JVM.activate0(Ljava/lang/String;[Lsun/tracing/dtrace/DTraceProvider;)J"
    )
}

#[async_recursion(?Send)]
async fn define_class_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.tracing.dtrace.JVM.defineClass0(Ljava/lang/ClassLoader;Ljava/lang/String;[BII)Ljava/lang/Class;")
}

#[async_recursion(?Send)]
async fn dispose_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.tracing.dtrace.JVM.dispose0(J)")
}

#[async_recursion(?Send)]
async fn is_enabled_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.tracing.dtrace.JVM.isEnabled0(Ljava/lang/reflect/Method;)Z")
}

#[async_recursion(?Send)]
async fn is_supported_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.tracing.dtrace.JVM.isSupported0()Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/tracing/dtrace/JVM";
        assert!(registry
            .method(
                class_name,
                "activate0",
                "(Ljava/lang/String;[Lsun/tracing/dtrace/DTraceProvider;)J"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "defineClass0",
                "(Ljava/lang/ClassLoader;Ljava/lang/String;[BII)Ljava/lang/Class;"
            )
            .is_some());
        assert!(registry.method(class_name, "dispose0", "(J)V").is_some());
        assert!(registry
            .method(class_name, "isEnabled0", "(Ljava/lang/reflect/Method;)Z")
            .is_some());
        assert!(registry.method(class_name, "isSupported0", "()Z").is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.tracing.dtrace.JVM.activate0(Ljava/lang/String;[Lsun/tracing/dtrace/DTraceProvider;)J"
    )]
    async fn test_activate_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = activate_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.tracing.dtrace.JVM.defineClass0(Ljava/lang/ClassLoader;Ljava/lang/String;[BII)Ljava/lang/Class;"
    )]
    async fn test_define_class_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = define_class_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.tracing.dtrace.JVM.dispose0(J)")]
    async fn test_dispose_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = dispose_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.tracing.dtrace.JVM.isEnabled0(Ljava/lang/reflect/Method;)Z")]
    async fn test_is_enabled_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_enabled_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.tracing.dtrace.JVM.isSupported0()Z")]
    async fn test_is_supported_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_supported_0(thread, Arguments::default()).await;
    }
}
