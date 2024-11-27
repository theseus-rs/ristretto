use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `com.sun.demo.jvmti.hprof.Tracker`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "com/sun/demo/jvmti/hprof/Tracker";
    registry.register(
        class_name,
        "nativeCallSite",
        "(Ljava/lang/Object;II)V",
        native_call_site,
    );
    registry.register(
        class_name,
        "nativeNewArray",
        "(Ljava/lang/Object;Ljava/lang/Object;)V",
        native_new_array,
    );
    registry.register(
        class_name,
        "nativeObjectInit",
        "(Ljava/lang/Object;Ljava/lang/Object;)V",
        native_object_init,
    );
    registry.register(
        class_name,
        "nativeReturnSite",
        "(Ljava/lang/Object;II)V",
        native_return_site,
    );
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn native_call_site(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn native_new_array(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn native_object_init(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn native_return_site(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
