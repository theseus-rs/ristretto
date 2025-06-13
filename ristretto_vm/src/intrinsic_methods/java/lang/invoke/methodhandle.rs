use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_17;
use ristretto_classfile::VersionSpecification::{Any, GreaterThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "java/lang/invoke/MethodHandle.invoke([Ljava/lang/Object;)Ljava/lang/Object;",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn invoke(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.invoke.MethodHandle.invoke([Ljava/lang/Object;)Ljava/lang/Object;")
}

#[intrinsic_method(
    "java/lang/invoke/MethodHandle.invokeBasic([Ljava/lang/Object;)Ljava/lang/Object;",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn invoke_basic(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.MethodHandle.invokeBasic([Ljava/lang/Object;)Ljava/lang/Object;")
}

#[intrinsic_method(
    "java/lang/invoke/MethodHandle.invokeExact([Ljava/lang/Object;)Ljava/lang/Object;",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn invoke_exact(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.MethodHandle.invokeExact([Ljava/lang/Object;)Ljava/lang/Object;")
}

#[intrinsic_method(
    "java/lang/invoke/MethodHandle.linkToInterface([Ljava/lang/Object;)Ljava/lang/Object;",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn link_to_interface(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.MethodHandle.linkToInterface([Ljava/lang/Object;)Ljava/lang/Object;")
}

#[intrinsic_method(
    "java/lang/invoke/MethodHandle.linkToNative([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn link_to_native(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.MethodHandle.linkToNative([Ljava/lang/Object;)Ljava/lang/Object;")
}

#[intrinsic_method(
    "java/lang/invoke/MethodHandle.linkToSpecial([Ljava/lang/Object;)Ljava/lang/Object;",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn link_to_special(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.MethodHandle.linkToSpecial([Ljava/lang/Object;)Ljava/lang/Object;")
}

#[intrinsic_method(
    "java/lang/invoke/MethodHandle.linkToStatic([Ljava/lang/Object;)Ljava/lang/Object;",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn link_to_static(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.MethodHandle.linkToStatic([Ljava/lang/Object;)Ljava/lang/Object;")
}

#[intrinsic_method(
    "java/lang/invoke/MethodHandle.linkToVirtual([Ljava/lang/Object;)Ljava/lang/Object;",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn link_to_virtual(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.invoke.MethodHandle.linkToVirtual([Ljava/lang/Object;)Ljava/lang/Object;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.MethodHandle.invoke([Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_invoke() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.MethodHandle.invokeBasic([Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_invoke_basic() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_basic(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.MethodHandle.invokeExact([Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_invoke_exact() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_exact(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.MethodHandle.linkToInterface([Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_link_to_interface() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = link_to_interface(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.MethodHandle.linkToNative([Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_link_to_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = link_to_native(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.MethodHandle.linkToSpecial([Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_link_to_special() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = link_to_special(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.MethodHandle.linkToStatic([Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_link_to_static() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = link_to_static(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.MethodHandle.linkToVirtual([Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_link_to_virtual() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = link_to_virtual(thread, Parameters::default()).await;
    }
}
