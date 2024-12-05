use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.misc.Perf`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/misc/Perf";
    registry.register(
        class_name,
        "attach",
        "(Ljava/lang/String;II)Ljava/nio/ByteBuffer;",
        attach,
    );
    registry.register(
        class_name,
        "createByteArray",
        "(Ljava/lang/String;II[BI)Ljava/nio/ByteBuffer;",
        create_byte_array,
    );
    registry.register(
        class_name,
        "createLong",
        "(Ljava/lang/String;IIJ)Ljava/nio/ByteBuffer;",
        create_long,
    );
    registry.register(class_name, "detach", "(Ljava/nio/ByteBuffer;)V", detach);
    registry.register(class_name, "highResCounter", "()J", high_res_counter);
    registry.register(class_name, "highResFrequency", "()J", high_res_frequency);
    registry.register(class_name, "registerNatives", "()V", register_natives);
}

#[async_recursion(?Send)]
async fn attach(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.misc.Perf.attach(Ljava/lang/String;II)Ljava/nio/ByteBuffer;")
}

#[async_recursion(?Send)]
async fn create_byte_array(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.misc.Perf.createByteArray(Ljava/lang/String;II[BI)Ljava/nio/ByteBuffer;")
}

#[async_recursion(?Send)]
async fn create_long(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.misc.Perf.createLong(Ljava/lang/String;IIJ)Ljava/nio/ByteBuffer;")
}

#[async_recursion(?Send)]
async fn detach(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.misc.Perf.detach(Ljava/nio/ByteBuffer;)V")
}

#[async_recursion(?Send)]
async fn high_res_counter(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.misc.Perf.highResCounter()J")
}

#[async_recursion(?Send)]
async fn high_res_frequency(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.misc.Perf.highResFrequency()J")
}

#[async_recursion(?Send)]
async fn register_natives(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}
