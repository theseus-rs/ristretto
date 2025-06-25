use crate::Result;
use crate::java_object::JavaObject;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::{Equal, GreaterThanOrEqual};
use ristretto_classfile::{JAVA_17, JAVA_21};
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "jdk/internal/loader/NativeLibraries.findEntry0(Ljdk/internal/loader/NativeLibraries$NativeLibraryImpl;Ljava/lang/String;)J",
    Equal(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn find_entry_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "jdk.internal.loader.NativeLibraries.findEntry0(Ljdk/internal/loader/NativeLibraries$NativeLibraryImpl;Ljava/lang/String;)J"
    )
}

#[intrinsic_method(
    "jdk/internal/loader/NativeLibraries.findBuiltinLib(Ljava/lang/String;)Ljava/lang/String;",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn find_builtin_lib(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let object = parameters.pop_object()?;
    let vm = thread.vm()?;
    let library_file_name: String = object.try_into()?;
    let library_path = vm
        .java_home()
        .join("lib")
        .join(library_file_name)
        .to_string_lossy()
        .to_string();
    let library_name = library_path.to_object(&thread).await?;
    Ok(Some(library_name))
}

#[intrinsic_method(
    "jdk/internal/loader/NativeLibraries.load(Ljdk/internal/loader/NativeLibraries$NativeLibraryImpl;Ljava/lang/String;ZZZ)Z",
    Equal(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn load_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(Some(Value::Int(1)))
}

#[intrinsic_method(
    "jdk/internal/loader/NativeLibraries.load(Ljdk/internal/loader/NativeLibraries$NativeLibraryImpl;Ljava/lang/String;ZZ)Z",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn load_1(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(Some(Value::Int(1)))
}

#[intrinsic_method(
    "jdk/internal/loader/NativeLibraries.unload(Ljava/lang/String;ZZJ)V",
    Equal(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn unload_0(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _handle = parameters.pop_long()?;
    let _is_builtin = parameters.pop_bool()?;
    let _name: String = parameters.pop_object()?.try_into()?;
    Ok(None)
}

#[intrinsic_method(
    "jdk/internal/loader/NativeLibraries.unload(Ljava/lang/String;ZJ)V",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn unload_1(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _handle = parameters.pop_long()?;
    let _is_builtin = parameters.pop_bool()?;
    let _name: String = parameters.pop_object()?.try_into()?;
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.loader.NativeLibraries.findEntry0(Ljdk/internal/loader/NativeLibraries$NativeLibraryImpl;Ljava/lang/String;)J"
    )]
    async fn test_find_entry_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = find_entry_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_load_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = load_0(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::Int(1)));
        Ok(())
    }

    #[tokio::test]
    async fn test_load_1() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = load_1(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::Int(1)));
        Ok(())
    }

    #[tokio::test]
    async fn test_unload_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let name = "foo".to_object(&thread).await?;
        let parameters = Parameters::new(vec![name, Value::Int(1), Value::Long(2)]);
        let result = unload_0(thread, parameters).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_unload_1() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let name = "foo".to_object(&thread).await?;
        let parameters = Parameters::new(vec![name, Value::Int(1), Value::Long(2)]);
        let result = unload_1(thread, parameters).await?;
        assert_eq!(result, None);
        Ok(())
    }
}
