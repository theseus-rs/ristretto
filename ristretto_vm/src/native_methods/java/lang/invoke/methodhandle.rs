use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_17: Version = Version::Java17 { minor: 0 };

/// Register all native methods for `java.lang.invoke.MethodHandle`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/invoke/MethodHandle";
    let java_version = registry.java_version();

    if java_version >= &JAVA_17 {
        registry.register(
            class_name,
            "linkToNative",
            "([Ljava/lang/Object;)Ljava/lang/Object;",
            link_to_native,
        );
    }

    registry.register(
        class_name,
        "invoke",
        "([Ljava/lang/Object;)Ljava/lang/Object;",
        invoke,
    );
    registry.register(
        class_name,
        "invokeBasic",
        "([Ljava/lang/Object;)Ljava/lang/Object;",
        invoke_basic,
    );
    registry.register(
        class_name,
        "invokeExact",
        "([Ljava/lang/Object;)Ljava/lang/Object;",
        invoke_exact,
    );
    registry.register(
        class_name,
        "linkToInterface",
        "([Ljava/lang/Object;)Ljava/lang/Object;",
        link_to_interface,
    );
    registry.register(
        class_name,
        "linkToSpecial",
        "([Ljava/lang/Object;)Ljava/lang/Object;",
        link_to_special,
    );
    registry.register(
        class_name,
        "linkToStatic",
        "([Ljava/lang/Object;)Ljava/lang/Object;",
        link_to_static,
    );
    registry.register(
        class_name,
        "linkToVirtual",
        "([Ljava/lang/Object;)Ljava/lang/Object;",
        link_to_virtual,
    );
}

#[async_recursion(?Send)]
async fn invoke(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.invoke.MethodHandle.invoke([Ljava/lang/Object;)Ljava/lang/Object;")
}

#[async_recursion(?Send)]
async fn invoke_basic(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.invoke.MethodHandle.invokeBasic([Ljava/lang/Object;)Ljava/lang/Object;")
}

#[async_recursion(?Send)]
async fn invoke_exact(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.invoke.MethodHandle.invokeExact([Ljava/lang/Object;)Ljava/lang/Object;")
}

#[async_recursion(?Send)]
async fn link_to_interface(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.invoke.MethodHandle.linkToInterface([Ljava/lang/Object;)Ljava/lang/Object;")
}

#[async_recursion(?Send)]
async fn link_to_native(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.invoke.MethodHandle.linkToNative([Ljava/lang/Object;)Ljava/lang/Object;")
}

#[async_recursion(?Send)]
async fn link_to_special(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.invoke.MethodHandle.linkToSpecial([Ljava/lang/Object;)Ljava/lang/Object;")
}

#[async_recursion(?Send)]
async fn link_to_static(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.invoke.MethodHandle.linkToStatic([Ljava/lang/Object;)Ljava/lang/Object;")
}

#[async_recursion(?Send)]
async fn link_to_virtual(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.invoke.MethodHandle.linkToVirtual([Ljava/lang/Object;)Ljava/lang/Object;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::new(&Version::Java17 { minor: 0 }, true);
        register(&mut registry);
        let class_name = "java/lang/invoke/MethodHandle";
        assert!(registry
            .method(
                class_name,
                "linkToNative",
                "([Ljava/lang/Object;)Ljava/lang/Object;"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "invoke",
                "([Ljava/lang/Object;)Ljava/lang/Object;"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "invokeBasic",
                "([Ljava/lang/Object;)Ljava/lang/Object;"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "invokeExact",
                "([Ljava/lang/Object;)Ljava/lang/Object;"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "linkToInterface",
                "([Ljava/lang/Object;)Ljava/lang/Object;"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "linkToSpecial",
                "([Ljava/lang/Object;)Ljava/lang/Object;"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "linkToStatic",
                "([Ljava/lang/Object;)Ljava/lang/Object;"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "linkToVirtual",
                "([Ljava/lang/Object;)Ljava/lang/Object;"
            )
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.MethodHandle.invoke([Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_invoke() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.MethodHandle.invokeBasic([Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_invoke_basic() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_basic(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.MethodHandle.invokeExact([Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_invoke_exact() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_exact(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.MethodHandle.linkToInterface([Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_link_to_interface() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = link_to_interface(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.MethodHandle.linkToNative([Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_link_to_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = link_to_native(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.MethodHandle.linkToSpecial([Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_link_to_special() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = link_to_special(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.MethodHandle.linkToStatic([Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_link_to_static() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = link_to_static(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.invoke.MethodHandle.linkToVirtual([Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_link_to_virtual() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = link_to_virtual(thread, Arguments::default()).await;
    }
}
