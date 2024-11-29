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
    todo!()
}

#[async_recursion(?Send)]
async fn add_exports_to_all_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn add_exports_to_all_unnamed_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn add_reads_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn define_module_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
