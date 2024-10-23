use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Error::InternalError;
use crate::Result;
use ristretto_classloader::{Reference, Value};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// Register all native methods for jdk.internal.loader.NativeLibraries.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "jdk/internal/loader/NativeLibraries";
    registry.register(
        class_name,
        "findBuiltinLib",
        "(Ljava/lang/String;)Ljava/lang/String;",
        find_builtin_lib,
    );
    registry.register(
        class_name,
        "load",
        "(Ljdk/internal/loader/NativeLibraries$NativeLibraryImpl;Ljava/lang/String;ZZ)Z",
        load,
    );
}

fn find_builtin_lib(
    thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Pin<Box<dyn Future<Output = Result<Option<Value>>>>> {
    Box::pin(async move {
        let Some(Reference::Object(object)) = arguments.pop_object()? else {
            return Err(InternalError("argument must be an object".to_string()));
        };
        let vm = thread.vm()?;
        let library_file_name = object.as_string()?;
        let library_path = vm
            .java_home()
            .join("lib")
            .join(library_file_name)
            .to_string_lossy()
            .to_string();
        let vm = thread.vm()?;
        let library_name = vm.string(library_path).await?;
        Ok(Some(library_name))
    })
}

#[expect(clippy::needless_pass_by_value)]
fn load(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Pin<Box<dyn Future<Output = Result<Option<Value>>>>> {
    Box::pin(async move { Ok(Some(Value::Int(1))) })
}
