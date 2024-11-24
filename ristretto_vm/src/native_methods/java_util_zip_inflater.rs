use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.util.zip.Inflater`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/util/zip/Inflater";
    registry.register(class_name, "end", "(J)V", end);
    registry.register(class_name, "getAdler", "(J)I", get_adler);
    registry.register(class_name, "inflateBytes", "(J[BII)I", inflate_bytes);
    registry.register(class_name, "init", "(Z)J", init);
    registry.register(class_name, "initIDs", "()V", init_ids);
    registry.register(class_name, "reset", "(J)V", reset);
    registry.register(class_name, "setDictionary", "(J[BII)V", set_dictionary);
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn end(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_adler(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn inflate_bytes(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn init(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn reset(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn set_dictionary(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
