use ristretto_classfile::JAVA_21;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "jdk/internal/loader/NativeLibrary.findEntry0(JLjava/lang/String;)J",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn find_entry_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let name = parameters.pop()?.as_string()?;
    let handle = parameters.pop_long()?;
    let vm = thread.vm()?;
    let address = crate::native_library::libraries(vm.as_ref())?
        .find(handle, &name)
        .unwrap_or_default();
    Ok(Some(Value::Long(address)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::native_library::JniVersion;
    use ristretto_types::JavaObject;

    #[tokio::test]
    async fn test_find_entry_0() -> Result<()> {
        let (vm, thread) = crate::test::thread().await?;
        let native_libraries = crate::native_library::libraries(vm.as_ref())?;
        let (handle, jni_version) = native_libraries.load("sctp", true)?;
        assert_eq!(jni_version, JniVersion::V1_6);
        let name = "missing".to_object(&thread).await?;
        let result = find_entry_0(thread, Parameters::new(vec![Value::Long(handle), name])).await?;
        assert_eq!(result, Some(Value::Long(0)));
        Ok(())
    }
}
