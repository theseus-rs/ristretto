use ristretto_classfile::VersionSpecification::{Between, Equal, GreaterThan};
use ristretto_classfile::{JAVA_11, JAVA_17, JAVA_21};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "java/lang/StackStreamFactory$AbstractStackWalker.callStackWalk(JIII[Ljava/lang/Object;)Ljava/lang/Object;",
    Between(JAVA_11, JAVA_17)
)]
#[async_method]
pub async fn call_stack_walk_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
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
#[async_method]
pub async fn call_stack_walk_1<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
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
#[async_method]
pub async fn call_stack_walk_2<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
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
#[async_method]
pub async fn fetch_stack_frames_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
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
#[async_method]
pub async fn fetch_stack_frames_1<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
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
#[async_method]
pub async fn set_continuation<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
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
