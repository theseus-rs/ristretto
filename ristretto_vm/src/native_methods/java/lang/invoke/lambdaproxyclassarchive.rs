use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.lang.invoke.LambdaProxyClassArchive`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/invoke/LambdaProxyClassArchive";
    registry.register(class_name, "addToArchive", "(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/invoke/MethodType;Ljava/lang/invoke/MethodType;Ljava/lang/invoke/MemberName;Ljava/lang/invoke/MethodType;Ljava/lang/Class;)V", add_to_archive);
    registry.register(class_name, "findFromArchive", "(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/invoke/MethodType;Ljava/lang/invoke/MethodType;Ljava/lang/invoke/MemberName;Ljava/lang/invoke/MethodType;)Ljava/lang/Class;", find_from_archive);
}

#[async_recursion(?Send)]
async fn add_to_archive(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.invoke.LambdaProxyClassArchive.addToArchive(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/invoke/MethodType;Ljava/lang/invoke/MethodType;Ljava/lang/invoke/MemberName;Ljava/lang/invoke/MethodType;Ljava/lang/Class;)V")
}

#[async_recursion(?Send)]
async fn find_from_archive(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.invoke.LambdaProxyClassArchive.findFromArchive(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/invoke/MethodType;Ljava/lang/invoke/MethodType;Ljava/lang/invoke/MemberName;Ljava/lang/invoke/MethodType;)Ljava/lang/Class;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "java/lang/invoke/LambdaProxyClassArchive";
        assert!(registry.method(class_name, "addToArchive", "(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/invoke/MethodType;Ljava/lang/invoke/MethodType;Ljava/lang/invoke/MemberName;Ljava/lang/invoke/MethodType;Ljava/lang/Class;)V").is_some());
        assert!(registry.method(class_name, "findFromArchive", "(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/invoke/MethodType;Ljava/lang/invoke/MethodType;Ljava/lang/invoke/MemberName;Ljava/lang/invoke/MethodType;)Ljava/lang/Class;").is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.LambdaProxyClassArchive.addToArchive(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/invoke/MethodType;Ljava/lang/invoke/MethodType;Ljava/lang/invoke/MemberName;Ljava/lang/invoke/MethodType;Ljava/lang/Class;)V"
    )]
    async fn test_add_to_archive() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = add_to_archive(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.LambdaProxyClassArchive.findFromArchive(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/invoke/MethodType;Ljava/lang/invoke/MethodType;Ljava/lang/invoke/MemberName;Ljava/lang/invoke/MethodType;)Ljava/lang/Class;"
    )]
    async fn test_find_from_archive() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = find_from_archive(thread, Arguments::default()).await;
    }
}
