use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_20: Version = Version::Java20 { minor: 0 };

/// Register all native methods for `jdk.internal.vm.Continuation`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "jdk/internal/vm/Continuation";
    let java_version = registry.java_version();

    if java_version >= &JAVA_20 {
        registry.register(class_name, "doYield", "()I", do_yield);
    }

    registry.register(
        class_name,
        "enterSpecial",
        "(Ljdk/internal/vm/Continuation;ZZ)V",
        enter_special,
    );
    registry.register(
        class_name,
        "isPinned0",
        "(Ljdk/internal/vm/ContinuationScope;)I",
        is_pinned_0,
    );
    registry.register(class_name, "pin", "()V", pin);
    registry.register(class_name, "registerNatives", "()V", register_natives);
    registry.register(class_name, "unpin", "()V", unpin);
}

#[async_recursion(?Send)]
async fn do_yield(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.internal.vm.Continuation.doYield()I")
}

#[async_recursion(?Send)]
async fn enter_special(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.internal.vm.Continuation.enterSpecial(Ljdk/internal/vm/Continuation;ZZ)V")
}

#[async_recursion(?Send)]
async fn is_pinned_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.internal.vm.Continuation.isPinned0(Ljdk/internal/vm/ContinuationScope;)I")
}

#[async_recursion(?Send)]
async fn pin(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.internal.vm.Continuation.pin()V")
}

#[async_recursion(?Send)]
async fn register_natives(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn unpin(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.internal.vm.Continuation.unpin()V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::new(&Version::Java21 { minor: 0 }, true);
        register(&mut registry);
        let class_name = "jdk/internal/vm/Continuation";
        assert!(registry
            .method(
                class_name,
                "enterSpecial",
                "(Ljdk/internal/vm/Continuation;ZZ)V"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "isPinned0",
                "(Ljdk/internal/vm/ContinuationScope;)I"
            )
            .is_some());
        assert!(registry.method(class_name, "pin", "()V").is_some());
        assert!(registry
            .method(class_name, "registerNatives", "()V")
            .is_some());
        assert!(registry.method(class_name, "unpin", "()V").is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "jdk.internal.vm.Continuation.doYield()I")]
    async fn test_do_yield() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = do_yield(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "jdk.internal.vm.Continuation.enterSpecial(Ljdk/internal/vm/Continuation;ZZ)V"
    )]
    async fn test_enter_special() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = enter_special(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "jdk.internal.vm.Continuation.isPinned0(Ljdk/internal/vm/ContinuationScope;)I"
    )]
    async fn test_is_pinned_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_pinned_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "jdk.internal.vm.Continuation.pin()V")]
    async fn test_pin() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = pin(thread, Arguments::default()).await;
    }

    #[tokio::test]
    async fn test_register_natives() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = register_natives(thread, Arguments::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(expected = "jdk.internal.vm.Continuation.unpin()V")]
    async fn test_unpin() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = unpin(thread, Arguments::default()).await;
    }
}
