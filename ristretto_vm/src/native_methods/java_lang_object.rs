use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Error::InternalError;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::{Reference, Value};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::sync::Arc;

/// Register all native methods for java.lang.Object.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/Object";
    registry.register(class_name, "<init>", "()V", init);
    registry.register(class_name, "clone", "()Ljava/lang/Object;", clone);
    registry.register(class_name, "getClass", "()Ljava/lang/Class;", get_class);
    registry.register(class_name, "hashCode", "()I", hash_code);
    registry.register(class_name, "notifyAll", "()V", notify_all);
    registry.register(class_name, "registerNatives", "()V", register_natives);
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn init(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    // This is a no-op method to optimize Object initialization since it is called frequently.
    // This prevents the need to create a new frame and allocate memory unnecessarily for the call
    // to the constructor for every object.
    Ok(None)
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn clone(_thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let object = arguments.pop_object()?;
    let cloned_object = object.clone();
    Ok(Some(Value::Object(cloned_object)))
}

#[async_recursion(?Send)]
async fn get_class(thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let Some(object) = arguments.pop_object()? else {
        return Err(InternalError("no object reference defined".to_string()));
    };

    let class_name = object.class_name();
    let vm = thread.vm()?;
    let class = vm.to_class_value(&thread, class_name.as_str()).await?;
    Ok(Some(class))
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn hash_code(_thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let Some(object) = arguments.pop_object()? else {
        return Err(InternalError("no object reference defined".to_string()));
    };
    let hash_code = object_hash_code(&object);
    Ok(Some(Value::Int(hash_code)))
}

pub(crate) fn object_hash_code(object: &Reference) -> i32 {
    let value = format!("{object}");
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    let hash_code = hasher.finish();
    #[expect(clippy::cast_possible_truncation)]
    let hash_code = hash_code as i32;
    hash_code
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn notify_all(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn register_natives(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}
