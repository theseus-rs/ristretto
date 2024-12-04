use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `com.sun.java.util.jar.pack.NativeUnpack`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "com/sun/java/util/jar/pack/NativeUnpack";
    registry.register(class_name, "finish", "()J", finish);
    registry.register(
        class_name,
        "getNextFile",
        "([Ljava/lang/Object;)Z",
        get_next_file,
    );
    registry.register(
        class_name,
        "getOption",
        "(Ljava/lang/String;)Ljava/lang/String;",
        get_option,
    );
    registry.register(
        class_name,
        "getUnusedInput",
        "()Ljava/nio/ByteBuffer;",
        get_unused_input,
    );
    registry.register(class_name, "initIDs", "()V", init_ids);
    registry.register(
        class_name,
        "setOption",
        "(Ljava/lang/String;Ljava/lang/String;)Z",
        set_option,
    );
    registry.register(class_name, "start", "(Ljava/nio/ByteBuffer;J)J", start);
}

#[async_recursion(?Send)]
async fn finish(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_next_file(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_option(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_unused_input(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn set_option(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn start(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
