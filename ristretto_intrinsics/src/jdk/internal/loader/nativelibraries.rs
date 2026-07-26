use ristretto_classfile::VersionSpecification::{Equal, GreaterThanOrEqual};
use ristretto_classfile::{JAVA_17, JAVA_21};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaObject;
use ristretto_types::Thread;
use ristretto_types::VM;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "jdk/internal/loader/NativeLibraries.findEntry0(Ljdk/internal/loader/NativeLibraries$NativeLibraryImpl;Ljava/lang/String;)J",
    Equal(JAVA_17)
)]
#[async_method]
pub async fn find_entry_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let name = parameters.pop()?.as_string()?;
    let native_library = parameters.pop()?;
    let handle = native_library.as_object_ref()?.value("handle")?.as_i64()?;
    let vm = thread.vm()?;
    let address = crate::native_library::libraries(vm.as_ref())?
        .find(handle, &name)
        .unwrap_or_default();
    Ok(Some(Value::Long(address)))
}

#[intrinsic_method(
    "jdk/internal/loader/NativeLibraries.findBuiltinLib(Ljava/lang/String;)Ljava/lang/String;",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn find_builtin_lib<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let library_file_name = parameters.pop()?.as_string()?;
    if !crate::native_library::is_intrinsic_native_library(&library_file_name) {
        return Ok(Some(Value::Object(None)));
    }
    let vm = thread.vm()?;
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
#[async_method]
pub async fn load_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _throw_exception_if_fail = parameters.pop_bool()?;
    let _is_jni = parameters.pop_bool()?;
    let is_builtin = parameters.pop_bool()?;
    let name = parameters.pop()?.as_string()?;
    let native_library_value = parameters.pop()?;
    let intrinsic = is_builtin && crate::native_library::is_intrinsic_native_library(name.as_str());
    let vm = thread.vm()?;
    let (handle, jni_version) =
        crate::native_library::libraries(vm.as_ref())?.load(&name, intrinsic)?;
    let mut native_library = native_library_value.as_object_mut()?;
    native_library.set_value("handle", Value::Long(handle))?;
    native_library.set_value("jniVersion", Value::Int(i32::from(jni_version)))?;
    Ok(Some(Value::Int(1)))
}

#[intrinsic_method(
    "jdk/internal/loader/NativeLibraries.load(Ljdk/internal/loader/NativeLibraries$NativeLibraryImpl;Ljava/lang/String;ZZ)Z",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn load_1<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _throw_exception_if_fail = parameters.pop_bool()?;
    let is_builtin = parameters.pop_bool()?;
    let name = parameters.pop()?.as_string()?;
    let native_library_value = parameters.pop()?;
    let intrinsic = is_builtin && crate::native_library::is_intrinsic_native_library(name.as_str());
    let vm = thread.vm()?;
    let (handle, jni_version) =
        crate::native_library::libraries(vm.as_ref())?.load(&name, intrinsic)?;
    let mut native_library = native_library_value.as_object_mut()?;
    native_library.set_value("handle", Value::Long(handle))?;
    native_library.set_value("jniVersion", Value::Int(i32::from(jni_version)))?;
    Ok(Some(Value::Int(1)))
}

#[intrinsic_method(
    "jdk/internal/loader/NativeLibraries.unload(Ljava/lang/String;ZZJ)V",
    Equal(JAVA_17)
)]
#[async_method]
pub async fn unload_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let handle = parameters.pop_long()?;
    let _is_builtin = parameters.pop_bool()?;
    let _is_jni = parameters.pop_bool()?;
    let _name = parameters.pop()?.as_string()?;
    let vm = thread.vm()?;
    crate::native_library::libraries(vm.as_ref())?.unload(handle);
    Ok(None)
}

#[intrinsic_method(
    "jdk/internal/loader/NativeLibraries.unload(Ljava/lang/String;ZJ)V",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn unload_1<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let handle = parameters.pop_long()?;
    let _is_builtin = parameters.pop_bool()?;
    let _name = parameters.pop()?.as_string()?;
    let vm = thread.vm()?;
    crate::native_library::libraries(vm.as_ref())?.unload(handle);
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::native_library::JniVersion;
    use ristretto_classloader::Reference;

    #[tokio::test]
    async fn test_find_entry_0() -> Result<()> {
        let (vm, thread) = crate::test::java17_thread().await?;
        let class = thread
            .class("jdk.internal.loader.NativeLibraries$NativeLibraryImpl")
            .await?;
        let native_library = ristretto_classloader::Object::new(class)?;
        let native_library =
            Value::new_object(vm.garbage_collector(), Reference::Object(native_library));
        let name = "missing".to_object(&thread).await?;
        let result = find_entry_0(thread, Parameters::new(vec![native_library, name])).await?;
        assert_eq!(result, Some(Value::Long(0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_load_0() -> Result<()> {
        let (vm, thread) = crate::test::java17_thread().await?;
        let class = thread
            .class("jdk.internal.loader.NativeLibraries$NativeLibraryImpl")
            .await?;
        let native_library = ristretto_classloader::Object::new(class)?;
        let native_library =
            Value::new_object(vm.garbage_collector(), Reference::Object(native_library));
        let name = "sctp".to_object(&thread).await?;
        let parameters = Parameters::new(vec![
            native_library.clone(),
            name,
            Value::Int(1),
            Value::Int(1),
            Value::Int(1),
        ]);
        let result = load_0(thread, parameters).await?;
        assert_eq!(result, Some(Value::Int(1)));
        let native_library = native_library.as_object_ref()?;
        assert_eq!(
            native_library.value("jniVersion")?,
            Value::Int(i32::from(JniVersion::V1_6))
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_load_1() -> Result<()> {
        let (vm, thread) = crate::test::thread().await?;
        let class = thread
            .class("jdk.internal.loader.NativeLibraries$NativeLibraryImpl")
            .await?;
        let native_library = ristretto_classloader::Object::new(class)?;
        let native_library =
            Value::new_object(vm.garbage_collector(), Reference::Object(native_library));
        let name = "sctp".to_object(&thread).await?;
        let parameters = Parameters::new(vec![
            native_library.clone(),
            name,
            Value::Int(1),
            Value::Int(1),
        ]);
        let result = load_1(thread, parameters).await?;
        assert_eq!(result, Some(Value::Int(1)));
        let native_library = native_library.as_object_ref()?;
        assert_eq!(
            native_library.value("jniVersion")?,
            Value::Int(i32::from(JniVersion::V1_6))
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_unload_0() -> Result<()> {
        let (_vm, thread) = crate::test::java17_thread().await?;
        let name = "foo".to_object(&thread).await?;
        let parameters = Parameters::new(vec![name, Value::Int(1), Value::Int(1), Value::Long(2)]);
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
