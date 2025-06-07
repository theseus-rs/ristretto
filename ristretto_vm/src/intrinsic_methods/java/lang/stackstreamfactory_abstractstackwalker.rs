use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::{Between, Equal, GreaterThan};
use ristretto_classfile::{JAVA_11, JAVA_17, JAVA_21};
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "java/lang/StackStreamFactory$AbstractStackWalker.callStackWalk(JIII[Ljava/lang/Object;)Ljava/lang/Object;",
    Between(JAVA_11, JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn call_stack_walk_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.StackStreamFactory$AbstractStackWalker.callStackWalk(JIII[Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[intrinsic_method(
    "java/lang/StackStreamFactory$AbstractStackWalker.callStackWalk(JILjdk/internal/vm/ContinuationScope;Ljdk/internal/vm/Continuation;II[Ljava/lang/Object;)Ljava/lang/Object;",
    Equal(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn call_stack_walk_1(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.StackStreamFactory$AbstractStackWalker.callStackWalk(JILjdk/internal/vm/ContinuationScope;Ljdk/internal/vm/Continuation;II[Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[intrinsic_method(
    "java/lang/StackStreamFactory$AbstractStackWalker.callStackWalk(IILjdk/internal/vm/ContinuationScope;Ljdk/internal/vm/Continuation;II[Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThan(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn call_stack_walk_2(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.StackStreamFactory$AbstractStackWalker.callStackWalk(IILjdk/internal/vm/ContinuationScope;Ljdk/internal/vm/Continuation;II[Ljava/lang/Object;)Ljava/lang/Object;"
    )
}

#[intrinsic_method(
    "java/lang/StackStreamFactory$AbstractStackWalker.fetchStackFrames(JJII[Ljava/lang/Object;)I",
    Between(JAVA_11, JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn fetch_stack_frames_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.StackStreamFactory$AbstractStackWalker.fetchStackFrames(JJII[Ljava/lang/Object;)I"
    )
}

#[intrinsic_method(
    "java/lang/StackStreamFactory$AbstractStackWalker.fetchStackFrames(IJIII[Ljava/lang/Object;)I",
    GreaterThan(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn fetch_stack_frames_1(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.StackStreamFactory$AbstractStackWalker.fetchStackFrames(IJIII[Ljava/lang/Object;)I"
    )
}

#[intrinsic_method(
    "java/lang/StackStreamFactory$AbstractStackWalker.setContinuation(J[Ljava/lang/Object;Ljdk/internal/vm/Continuation;)V",
    GreaterThan(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn set_continuation(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.StackStreamFactory$AbstractStackWalker.setContinuation(J[Ljava/lang/Object;Ljdk/internal/vm/Continuation;)V"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.StackStreamFactory$AbstractStackWalker.callStackWalk(JIII[Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_call_stack_walk_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = call_stack_walk_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.StackStreamFactory$AbstractStackWalker.callStackWalk(JILjdk/internal/vm/ContinuationScope;Ljdk/internal/vm/Continuation;II[Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_call_stack_walk_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = call_stack_walk_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.StackStreamFactory$AbstractStackWalker.callStackWalk(IILjdk/internal/vm/ContinuationScope;Ljdk/internal/vm/Continuation;II[Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_call_stack_walk_2() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = call_stack_walk_2(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.StackStreamFactory$AbstractStackWalker.fetchStackFrames(JJII[Ljava/lang/Object;)I"
    )]
    async fn test_fetch_stack_frames_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = fetch_stack_frames_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.StackStreamFactory$AbstractStackWalker.fetchStackFrames(IJIII[Ljava/lang/Object;)I"
    )]
    async fn test_fetch_stack_frames_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = fetch_stack_frames_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.StackStreamFactory$AbstractStackWalker.setContinuation(J[Ljava/lang/Object;Ljdk/internal/vm/Continuation;)V"
    )]
    async fn test_set_continuation() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_continuation(thread, Parameters::default()).await;
    }
}
