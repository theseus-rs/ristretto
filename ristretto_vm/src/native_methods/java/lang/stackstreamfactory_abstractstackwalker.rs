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

/// Register all native methods for `java.lang.StackStreamFactory$AbstractStackWalker`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/StackStreamFactory$AbstractStackWalker";
    let java_version = registry.java_version().clone();

    if java_version <= JAVA_18 {
        registry.register(
            class_name,
            "callStackWalk",
            "(JIII[Ljava/lang/Object;)Ljava/lang/Object;",
            call_stack_walk,
        );
    } else {
        if java_version <= JAVA_21 {
            registry.register(class_name, "callStackWalk", "(JILjdk/internal/vm/ContinuationScope;Ljdk/internal/vm/Continuation;II[Ljava/lang/Object;)Ljava/lang/Object;", call_stack_walk);
        }

        registry.register(
            class_name,
            "setContinuation",
            "(J[Ljava/lang/Object;Ljdk/internal/vm/Continuation;)V",
            set_continuation,
        );
    }

    if java_version <= JAVA_21 {
        registry.register(
            class_name,
            "fetchStackFrames",
            "(JJII[Ljava/lang/Object;)I",
            fetch_stack_frames,
        );
    } else {
        registry.register(class_name, "callStackWalk", "(IILjdk/internal/vm/ContinuationScope;Ljdk/internal/vm/Continuation;II[Ljava/lang/Object;)Ljava/lang/Object;", call_stack_walk);
        registry.register(
            class_name,
            "fetchStackFrames",
            "(IJIII[Ljava/lang/Object;)I",
            fetch_stack_frames,
        );
    }
}

#[async_recursion(?Send)]
async fn call_stack_walk(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.StackStreamFactory$AbstractStackWalker.callStackWalk()Ljava/lang/Object;")
}

#[async_recursion(?Send)]
async fn fetch_stack_frames(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.StackStreamFactory$AbstractStackWalker.fetchStackFrames()I")
}

#[async_recursion(?Send)]
async fn set_continuation(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.StackStreamFactory$AbstractStackWalker.setContinuation()V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::new(&Version::Java21 { minor: 0 }, true);
        register(&mut registry);
        let class_name = "java/lang/StackStreamFactory$AbstractStackWalker";
        assert!(registry
            .method(
                class_name,
                "callStackWalk",
                "(JILjdk/internal/vm/ContinuationScope;Ljdk/internal/vm/Continuation;II[Ljava/lang/Object;)Ljava/lang/Object;"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "setContinuation",
                "(J[Ljava/lang/Object;Ljdk/internal/vm/Continuation;)V"
            )
            .is_some());
        assert!(registry
            .method(class_name, "fetchStackFrames", "(JJII[Ljava/lang/Object;)I")
            .is_some());
    }

    #[test]
    fn test_register_java_18() {
        let mut registry = MethodRegistry::new(&Version::Java18 { minor: 0 }, true);
        register(&mut registry);
        let class_name = "java/lang/StackStreamFactory$AbstractStackWalker";
        assert!(registry
            .method(
                class_name,
                "callStackWalk",
                "(JIII[Ljava/lang/Object;)Ljava/lang/Object;"
            )
            .is_some());
    }

    #[test]
    fn test_register_java_22() {
        let mut registry = MethodRegistry::new(&Version::Java22 { minor: 0 }, true);
        register(&mut registry);
        let class_name = "java/lang/StackStreamFactory$AbstractStackWalker";
        assert!(registry
            .method(
                class_name,
                "fetchStackFrames",
                "(IJIII[Ljava/lang/Object;)I"
            )
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.StackStreamFactory$AbstractStackWalker.callStackWalk()Ljava/lang/Object;"
    )]
    async fn test_call_stack_walk() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = call_stack_walk(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.StackStreamFactory$AbstractStackWalker.fetchStackFrames()I"
    )]
    async fn test_fetch_stack_frames() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = fetch_stack_frames(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.StackStreamFactory$AbstractStackWalker.setContinuation()V"
    )]
    async fn test_set_continuation() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_continuation(thread, Arguments::default()).await;
    }
}
