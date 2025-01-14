use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_8: Version = Version::Java8 { minor: 0 };

/// Register all native methods for `sun.awt.DebugSettings`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/awt/DebugSettings";
    let java_version = registry.java_version();

    if java_version <= &JAVA_8 {
        registry.register(class_name, "setCTracingOn", "(Z)V", set_c_tracing_on_1);
    }

    registry.register(
        class_name,
        "setCTracingOn",
        "(ZLjava/lang/String;)V",
        set_c_tracing_on_2,
    );
    registry.register(
        class_name,
        "setCTracingOn",
        "(ZLjava/lang/String;I)V",
        set_c_tracing_on_3,
    );
}

#[async_recursion(?Send)]
async fn set_c_tracing_on_1(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.awt.DebugSettings.setCTracingOn(Z)V")
}

#[async_recursion(?Send)]
async fn set_c_tracing_on_2(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.awt.DebugSettings.setCTracingOn(ZLjava/lang/String;)V")
}

#[async_recursion(?Send)]
async fn set_c_tracing_on_3(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.awt.DebugSettings.setCTracingOn(ZLjava/lang/String;I)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::new(&Version::Java8 { minor: 0 }, true);
        register(&mut registry);
        let class_name = "sun/awt/DebugSettings";
        assert!(registry
            .method(class_name, "setCTracingOn", "(Z)V")
            .is_some());
        assert!(registry
            .method(class_name, "setCTracingOn", "(ZLjava/lang/String;)V")
            .is_some());
        assert!(registry
            .method(class_name, "setCTracingOn", "(ZLjava/lang/String;I)V")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.awt.DebugSettings.setCTracingOn(Z)V")]
    async fn test_set_c_tracing_on_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_c_tracing_on_1(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.awt.DebugSettings.setCTracingOn(ZLjava/lang/String;)V")]
    async fn test_set_c_tracing_on_2() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_c_tracing_on_2(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.awt.DebugSettings.setCTracingOn(ZLjava/lang/String;I)V")]
    async fn test_set_c_tracing_on_3() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_c_tracing_on_3(thread, Arguments::default()).await;
    }
}
