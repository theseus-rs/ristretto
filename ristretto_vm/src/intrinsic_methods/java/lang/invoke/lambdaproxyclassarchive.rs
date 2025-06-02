use crate::Result;
use crate::intrinsic_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "java/lang/invoke/LambdaProxyClassArchive";

/// Register all intrinsic methods for `java.lang.invoke.LambdaProxyClassArchive`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "addToArchive", "(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/invoke/MethodType;Ljava/lang/invoke/MethodType;Ljava/lang/invoke/MemberName;Ljava/lang/invoke/MethodType;Ljava/lang/Class;)V", add_to_archive);
    registry.register(CLASS_NAME, "findFromArchive", "(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/invoke/MethodType;Ljava/lang/invoke/MethodType;Ljava/lang/invoke/MemberName;Ljava/lang/invoke/MethodType;)Ljava/lang/Class;", find_from_archive);
}

#[async_recursion(?Send)]
async fn add_to_archive(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.LambdaProxyClassArchive.addToArchive(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/invoke/MethodType;Ljava/lang/invoke/MethodType;Ljava/lang/invoke/MemberName;Ljava/lang/invoke/MethodType;Ljava/lang/Class;)V"
    )
}

#[async_recursion(?Send)]
async fn find_from_archive(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.invoke.LambdaProxyClassArchive.findFromArchive(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/invoke/MethodType;Ljava/lang/invoke/MethodType;Ljava/lang/invoke/MemberName;Ljava/lang/invoke/MethodType;)Ljava/lang/Class;"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.LambdaProxyClassArchive.addToArchive(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/invoke/MethodType;Ljava/lang/invoke/MethodType;Ljava/lang/invoke/MemberName;Ljava/lang/invoke/MethodType;Ljava/lang/Class;)V"
    )]
    async fn test_add_to_archive() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = add_to_archive(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.LambdaProxyClassArchive.findFromArchive(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/invoke/MethodType;Ljava/lang/invoke/MethodType;Ljava/lang/invoke/MemberName;Ljava/lang/invoke/MethodType;)Ljava/lang/Class;"
    )]
    async fn test_find_from_archive() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = find_from_archive(thread, Parameters::default()).await;
    }
}
