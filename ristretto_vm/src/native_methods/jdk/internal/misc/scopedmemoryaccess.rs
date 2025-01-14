use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_18: Version = Version::Java18 { minor: 0 };
const JAVA_21: Version = Version::Java21 { minor: 0 };

/// Register all native methods for `jdk.internal.misc.ScopedMemoryAccess`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "jdk/internal/misc/ScopedMemoryAccess";
    let java_version = registry.java_version().clone();

    if java_version <= JAVA_18 {
        registry.register(class_name, "closeScope0", "(Ljdk/internal/misc/ScopedMemoryAccess$Scope;Ljdk/internal/misc/ScopedMemoryAccess$Scope$ScopedAccessError;)Z", close_scope_0);
    } else if java_version <= JAVA_21 {
        registry.register(
            class_name,
            "closeScope0",
            "(Ljdk/internal/foreign/MemorySessionImpl;)Z",
            close_scope_0,
        );
    } else {
        registry.register(
            class_name,
            "closeScope0",
            "(Ljdk/internal/foreign/MemorySessionImpl;Ljdk/internal/misc/ScopedMemoryAccess$ScopedAccessError;)V",
            close_scope_0,
        );
    }

    registry.register(class_name, "registerNatives", "()V", register_natives);
}

#[async_recursion(?Send)]
async fn close_scope_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.ScopedMemoryAccess.closeScope0(Ljdk/internal/misc/ScopedMemoryAccess$Scope;Ljdk/internal/misc/ScopedMemoryAccess$Scope$ScopedAccessError;)Z")
}

#[async_recursion(?Send)]
async fn register_natives(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::new(&Version::Java21 { minor: 0 }, true);
        register(&mut registry);
        let class_name = "jdk/internal/misc/ScopedMemoryAccess";
        assert!(registry
            .method(
                class_name,
                "closeScope0",
                "(Ljdk/internal/foreign/MemorySessionImpl;)Z"
            )
            .is_some());
        assert!(registry
            .method(class_name, "registerNatives", "()V")
            .is_some());
    }

    #[test]
    fn test_register_java_18() {
        let mut registry = MethodRegistry::new(&Version::Java18 { minor: 0 }, true);
        register(&mut registry);
        let class_name = "jdk/internal/misc/ScopedMemoryAccess";
        assert!(registry
            .method(class_name, "closeScope0", "(Ljdk/internal/misc/ScopedMemoryAccess$Scope;Ljdk/internal/misc/ScopedMemoryAccess$Scope$ScopedAccessError;)Z")
            .is_some());
    }

    #[test]
    fn test_register_java_22() {
        let mut registry = MethodRegistry::new(&Version::Java22 { minor: 0 }, true);
        register(&mut registry);
        let class_name = "jdk/internal/misc/ScopedMemoryAccess";
        assert!(registry
            .method(class_name, "closeScope0", "(Ljdk/internal/foreign/MemorySessionImpl;Ljdk/internal/misc/ScopedMemoryAccess$ScopedAccessError;)V")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "jdk.internal.misc.ScopedMemoryAccess.closeScope0(Ljdk/internal/misc/ScopedMemoryAccess$Scope;Ljdk/internal/misc/ScopedMemoryAccess$Scope$ScopedAccessError;)Z"
    )]
    async fn test_close_scope_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = close_scope_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    async fn test_register_natives() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let value = register_natives(thread, Arguments::default()).await?;
        assert_eq!(value, None);
        Ok(())
    }
}
