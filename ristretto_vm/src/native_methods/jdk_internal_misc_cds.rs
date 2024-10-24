use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use ristretto_classloader::Value;
use std::future::Future;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::pin::Pin;
use std::sync::Arc;

/// Register all native methods for jdk.internal.misc.CDS.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "jdk/internal/misc/CDS";
    registry.register(
        class_name,
        "getRandomSeedForDumping",
        "()J",
        get_random_seed_for_dumping,
    );
    registry.register(
        class_name,
        "initializeFromArchive",
        "(Ljava/lang/Class;)V",
        initialize_from_archive,
    );
    registry.register(class_name, "isDumpingArchive0", "()Z", is_dumping_archive_0);
    registry.register(
        class_name,
        "isDumpingClassList0",
        "()Z",
        is_dumping_class_list_0,
    );
    registry.register(class_name, "isSharingEnabled0", "()Z", is_sharing_enabled_0);
}

#[expect(clippy::cast_possible_wrap)]
#[expect(clippy::needless_pass_by_value)]
fn get_random_seed_for_dumping(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Pin<Box<dyn Future<Output = Result<Option<Value>>>>> {
    Box::pin(async move {
        let version = env!("CARGO_PKG_VERSION");
        let mut hasher = DefaultHasher::new();
        version.hash(&mut hasher);
        let hash = hasher.finish() as i64;
        Ok(Some(Value::Long(hash)))
    })
}

#[expect(clippy::needless_pass_by_value)]
fn initialize_from_archive(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Pin<Box<dyn Future<Output = Result<Option<Value>>>>> {
    Box::pin(async move {
        let _class = arguments.pop_object()?;
        Ok(None)
    })
}

#[expect(clippy::needless_pass_by_value)]
fn is_dumping_archive_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Pin<Box<dyn Future<Output = Result<Option<Value>>>>> {
    Box::pin(async move { Ok(Some(Value::Int(0))) })
}

#[expect(clippy::needless_pass_by_value)]
fn is_dumping_class_list_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Pin<Box<dyn Future<Output = Result<Option<Value>>>>> {
    Box::pin(async move { Ok(Some(Value::Int(0))) })
}

#[expect(clippy::needless_pass_by_value)]
fn is_sharing_enabled_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Pin<Box<dyn Future<Output = Result<Option<Value>>>>> {
    Box::pin(async move { Ok(Some(Value::Int(0))) })
}
