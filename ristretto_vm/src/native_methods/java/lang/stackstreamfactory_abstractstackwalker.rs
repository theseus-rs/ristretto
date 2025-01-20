use crate::native_methods::registry::{MethodRegistry, JAVA_18, JAVA_21};
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "java/lang/StackStreamFactory$AbstractStackWalker";

/// Register all native methods for `java.lang.StackStreamFactory$AbstractStackWalker`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() <= JAVA_18 {
        registry.register(
            CLASS_NAME,
            "callStackWalk",
            "(JIII[Ljava/lang/Object;)Ljava/lang/Object;",
            call_stack_walk,
        );
    } else {
        if registry.java_major_version() <= JAVA_21 {
            registry.register(CLASS_NAME, "callStackWalk", "(JILjdk/internal/vm/ContinuationScope;Ljdk/internal/vm/Continuation;II[Ljava/lang/Object;)Ljava/lang/Object;", call_stack_walk);
        }

        registry.register(
            CLASS_NAME,
            "setContinuation",
            "(J[Ljava/lang/Object;Ljdk/internal/vm/Continuation;)V",
            set_continuation,
        );
    }

    if registry.java_major_version() <= JAVA_21 {
        registry.register(
            CLASS_NAME,
            "fetchStackFrames",
            "(JJII[Ljava/lang/Object;)I",
            fetch_stack_frames,
        );
    } else {
        registry.register(CLASS_NAME, "callStackWalk", "(IILjdk/internal/vm/ContinuationScope;Ljdk/internal/vm/Continuation;II[Ljava/lang/Object;)Ljava/lang/Object;", call_stack_walk);
        registry.register(
            CLASS_NAME,
            "fetchStackFrames",
            "(IJIII[Ljava/lang/Object;)I",
            fetch_stack_frames,
        );
    }
}

#[async_recursion(?Send)]
async fn call_stack_walk(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.StackStreamFactory$AbstractStackWalker.callStackWalk()Ljava/lang/Object;")
}

#[async_recursion(?Send)]
async fn fetch_stack_frames(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.StackStreamFactory$AbstractStackWalker.fetchStackFrames()I")
}

#[async_recursion(?Send)]
async fn set_continuation(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.StackStreamFactory$AbstractStackWalker.setContinuation()V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.StackStreamFactory$AbstractStackWalker.callStackWalk()Ljava/lang/Object;"
    )]
    async fn test_call_stack_walk() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = call_stack_walk(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.StackStreamFactory$AbstractStackWalker.fetchStackFrames()I"
    )]
    async fn test_fetch_stack_frames() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = fetch_stack_frames(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.StackStreamFactory$AbstractStackWalker.setContinuation()V"
    )]
    async fn test_set_continuation() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_continuation(thread, Parameters::default()).await;
    }
}
