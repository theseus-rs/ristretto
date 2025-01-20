use crate::native_methods::registry::{MethodRegistry, JAVA_11};
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "java/lang/Module";

/// Register all native methods for `java.lang.Module`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() <= JAVA_11 {
        registry.register(
            CLASS_NAME,
            "defineModule0",
            "(Ljava/lang/Module;ZLjava/lang/String;Ljava/lang/String;[Ljava/lang/String;)V",
            define_module_0,
        );
    } else {
        registry.register(
            CLASS_NAME,
            "defineModule0",
            "(Ljava/lang/Module;ZLjava/lang/String;Ljava/lang/String;[Ljava/lang/Object;)V",
            define_module_0,
        );
    }

    registry.register(
        CLASS_NAME,
        "addExports0",
        "(Ljava/lang/Module;Ljava/lang/String;Ljava/lang/Module;)V",
        add_exports_0,
    );
    registry.register(
        CLASS_NAME,
        "addExportsToAll0",
        "(Ljava/lang/Module;Ljava/lang/String;)V",
        add_exports_to_all_0,
    );
    registry.register(
        CLASS_NAME,
        "addExportsToAllUnnamed0",
        "(Ljava/lang/Module;Ljava/lang/String;)V",
        add_exports_to_all_unnamed_0,
    );
    registry.register(
        CLASS_NAME,
        "addReads0",
        "(Ljava/lang/Module;Ljava/lang/Module;)V",
        add_reads_0,
    );
}

#[async_recursion(?Send)]
async fn add_exports_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.Module.addExports0(Ljava/lang/Module;Ljava/lang/String;Ljava/lang/Module;)V")
}

#[async_recursion(?Send)]
async fn add_exports_to_all_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.Module.addExportsToAll0(Ljava/lang/Module;Ljava/lang/String;)V")
}

#[async_recursion(?Send)]
async fn add_exports_to_all_unnamed_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.Module.addExportsToAllUnnamed0(Ljava/lang/Module;Ljava/lang/String;)V")
}

#[async_recursion(?Send)]
async fn add_reads_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.Module.addReads0(Ljava/lang/Module;Ljava/lang/Module;)V")
}

#[async_recursion(?Send)]
async fn define_module_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.Module.defineModule0(Ljava/lang/Module;ZLjava/lang/String;Ljava/lang/String;[Ljava/lang/String;)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.Module.addExports0(Ljava/lang/Module;Ljava/lang/String;Ljava/lang/Module;)V"
    )]
    async fn test_add_exports_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = add_exports_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.Module.addExportsToAll0(Ljava/lang/Module;Ljava/lang/String;)V"
    )]
    async fn test_add_exports_to_all_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = add_exports_to_all_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.Module.addExportsToAllUnnamed0(Ljava/lang/Module;Ljava/lang/String;)V"
    )]
    async fn test_add_exports_to_all_unnamed_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = add_exports_to_all_unnamed_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.Module.addReads0(Ljava/lang/Module;Ljava/lang/Module;)V"
    )]
    async fn test_add_reads_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = add_reads_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.Module.defineModule0(Ljava/lang/Module;ZLjava/lang/String;Ljava/lang/String;[Ljava/lang/String;)V"
    )]
    async fn test_define_module_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = define_module_0(thread, Parameters::default()).await;
    }
}
