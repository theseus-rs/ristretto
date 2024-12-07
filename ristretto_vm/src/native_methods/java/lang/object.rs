use crate::arguments::Arguments;
use crate::java_object::JavaObject;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Error::InternalError;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::{Reference, Value};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::sync::Arc;

const JAVA_11: Version = Version::Java11 { minor: 0 };
const JAVA_18: Version = Version::Java18 { minor: 0 };

/// Register all native methods for `java.lang.Object`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/Object";
    let java_version = registry.java_version().clone();

    if java_version <= JAVA_18 {
        registry.register(class_name, "wait", "(J)V", wait);
    } else {
        registry.register(class_name, "wait0", "(J)V", wait_0);
    }

    registry.register(class_name, "clone", "()Ljava/lang/Object;", clone);
    registry.register(class_name, "getClass", "()Ljava/lang/Class;", get_class);
    registry.register(class_name, "hashCode", "()I", hash_code);
    registry.register(class_name, "notify", "()V", notify);
    registry.register(class_name, "notifyAll", "()V", notify_all);
    registry.register(class_name, "registerNatives", "()V", register_natives);

    if registry.use_optimizations() {
        registry.register(class_name, "<init>", "()V", init);
    }
}

#[async_recursion(?Send)]
async fn init(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    // This is a no-op method to optimize Object initialization since it is called frequently.
    // This prevents the need to create a new frame and allocate memory unnecessarily for the call
    // to the constructor for every object.
    Ok(None)
}

#[async_recursion(?Send)]
async fn clone(_thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let object = arguments.pop_reference()?;
    let cloned_object = object.clone();
    Ok(Some(Value::Object(cloned_object)))
}

#[async_recursion(?Send)]
async fn get_class(thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let Some(object) = arguments.pop_reference()? else {
        return Err(InternalError("no object reference defined".to_string()));
    };

    let class_name = object.class_name();
    let vm = thread.vm()?;
    let class = thread.class(class_name).await?;
    let class = class.to_object(&vm).await?;
    Ok(Some(class))
}

#[async_recursion(?Send)]
async fn hash_code(_thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let Some(object) = arguments.pop_reference()? else {
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

#[async_recursion(?Send)]
async fn notify(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.Object.notify()V")
}

#[async_recursion(?Send)]
async fn notify_all(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn register_natives(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn wait(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.Object.wait(J)V")
}

#[async_recursion(?Send)]
async fn wait_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.Object.wait0(J)V")
}
