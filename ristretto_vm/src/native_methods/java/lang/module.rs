use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_11: Version = Version::Java11 { minor: 0 };

/// Register all native methods for `java.lang.Module`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/Module";
    let java_version = registry.java_version();

    if java_version <= &JAVA_11 {
        registry.register(
            class_name,
            "defineModule0",
            "(Ljava/lang/Module;ZLjava/lang/String;Ljava/lang/String;[Ljava/lang/String;)V",
            define_module_0,
        );
    } else {
        registry.register(
            class_name,
            "defineModule0",
            "(Ljava/lang/Module;ZLjava/lang/String;Ljava/lang/String;[Ljava/lang/Object;)V",
            define_module_0,
        );
    }

    registry.register(
        class_name,
        "addExports0",
        "(Ljava/lang/Module;Ljava/lang/String;Ljava/lang/Module;)V",
        add_exports_0,
    );
    registry.register(
        class_name,
        "addExportsToAll0",
        "(Ljava/lang/Module;Ljava/lang/String;)V",
        add_exports_to_all_0,
    );
    registry.register(
        class_name,
        "addExportsToAllUnnamed0",
        "(Ljava/lang/Module;Ljava/lang/String;)V",
        add_exports_to_all_unnamed_0,
    );
    registry.register(
        class_name,
        "addReads0",
        "(Ljava/lang/Module;Ljava/lang/Module;)V",
        add_reads_0,
    );
}

#[async_recursion(?Send)]
async fn add_exports_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.Module.addExports0(Ljava/lang/Module;Ljava/lang/String;Ljava/lang/Module;)V")
}

#[async_recursion(?Send)]
async fn add_exports_to_all_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.lang.Module.addExportsToAll0(Ljava/lang/Module;Ljava/lang/String;)V")
}

#[async_recursion(?Send)]
async fn add_exports_to_all_unnamed_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.lang.Module.addExportsToAllUnnamed0(Ljava/lang/Module;Ljava/lang/String;)V")
}

#[async_recursion(?Send)]
async fn add_reads_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.Module.addReads0(Ljava/lang/Module;Ljava/lang/Module;)V")
}

#[async_recursion(?Send)]
async fn define_module_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.Module.defineModule0(Ljava/lang/Module;ZLjava/lang/String;Ljava/lang/String;[Ljava/lang/String;)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::new(&Version::Java12 { minor: 0 }, true);
        register(&mut registry);
        let class_name = "java/lang/Module";
        assert!(registry
            .method(
                class_name,
                "defineModule0",
                "(Ljava/lang/Module;ZLjava/lang/String;Ljava/lang/String;[Ljava/lang/Object;)V"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "addExports0",
                "(Ljava/lang/Module;Ljava/lang/String;Ljava/lang/Module;)V"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "addExportsToAll0",
                "(Ljava/lang/Module;Ljava/lang/String;)V"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "addExportsToAllUnnamed0",
                "(Ljava/lang/Module;Ljava/lang/String;)V"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "addReads0",
                "(Ljava/lang/Module;Ljava/lang/Module;)V"
            )
            .is_some());
    }

    #[test]
    fn test_register_java_11() {
        let mut registry = MethodRegistry::new(&Version::Java11 { minor: 0 }, true);
        register(&mut registry);
        let class_name = "java/lang/Module";
        assert!(registry
            .method(
                class_name,
                "defineModule0",
                "(Ljava/lang/Module;ZLjava/lang/String;Ljava/lang/String;[Ljava/lang/String;)V"
            )
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.Module.addExports0(Ljava/lang/Module;Ljava/lang/String;Ljava/lang/Module;)V"
    )]
    async fn test_add_exports_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = add_exports_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.Module.addExportsToAll0(Ljava/lang/Module;Ljava/lang/String;)V"
    )]
    async fn test_add_exports_to_all_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = add_exports_to_all_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.Module.addExportsToAllUnnamed0(Ljava/lang/Module;Ljava/lang/String;)V"
    )]
    async fn test_add_exports_to_all_unnamed_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = add_exports_to_all_unnamed_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.Module.addReads0(Ljava/lang/Module;Ljava/lang/Module;)V"
    )]
    async fn test_add_reads_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = add_reads_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.Module.defineModule0(Ljava/lang/Module;ZLjava/lang/String;Ljava/lang/String;[Ljava/lang/String;)V"
    )]
    async fn test_define_module_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = define_module_0(thread, Arguments::default()).await;
    }
}
