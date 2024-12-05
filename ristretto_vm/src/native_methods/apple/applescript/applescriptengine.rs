use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `apple.applescript.AppleScriptEngine`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "apple/applescript/AppleScriptEngine";
    registry.register(
        class_name,
        "createContextFrom",
        "(Ljava/lang/Object;)J",
        create_context_from,
    );
    registry.register(
        class_name,
        "createObjectFrom",
        "(J)Ljava/lang/Object;",
        create_object_from,
    );
    registry.register(class_name, "disposeContext", "(J)V", dispose_context);
    registry.register(
        class_name,
        "evalScript",
        "(Ljava/lang/String;J)J",
        eval_script,
    );
    registry.register(
        class_name,
        "evalScriptFromURL",
        "(Ljava/lang/String;J)J",
        eval_script_from_url,
    );
    registry.register(class_name, "initNative", "()V", init_native);
}

#[async_recursion(?Send)]
async fn create_context_from(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("apple.applescript.AppleScriptEngine.createContextFrom(Ljava/lang/Object;)J")
}

#[async_recursion(?Send)]
async fn create_object_from(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("apple.applescript.AppleScriptEngine.createObjectFrom(J)Ljava/lang/Object;")
}

#[async_recursion(?Send)]
async fn dispose_context(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("apple.applescript.AppleScriptEngine.disposeContext(J)V")
}

#[async_recursion(?Send)]
async fn eval_script(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("apple.applescript.AppleScriptEngine.evalScript(Ljava/lang/String;J)J")
}

#[async_recursion(?Send)]
async fn eval_script_from_url(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("apple.applescript.AppleScriptEngine.evalScriptFromURL(Ljava/lang/String;J)J")
}

#[async_recursion(?Send)]
async fn init_native(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}
