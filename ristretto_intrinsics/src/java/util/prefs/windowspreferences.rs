use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::{Reference, Value};
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result, VM};
use std::sync::Arc;

const ERROR_SUCCESS: i32 = 0;

// Stub handle for in-memory preferences (no real registry access)
const STUB_HANDLE: i64 = 0x8000_0001;

/// `WindowsRegOpenKey(long hKey, byte[] subKey, int securityMask) -> long[]`
/// Returns `[handle, errorCode]`
#[intrinsic_method("java/util/prefs/WindowsPreferences.WindowsRegOpenKey(J[BI)[J", Any)]
#[async_method]
pub async fn windows_reg_open_key<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _security_mask = parameters.pop_int()?;
    let _sub_key = parameters.pop_reference()?;
    let _h_key = parameters.pop_long()?;
    let vm = thread.vm()?;
    let collector = vm.garbage_collector();
    let result = Value::new_object(
        collector,
        Reference::from(vec![STUB_HANDLE, i64::from(ERROR_SUCCESS)]),
    );
    Ok(Some(result))
}

/// `WindowsRegCloseKey(long hKey) -> int`
#[intrinsic_method("java/util/prefs/WindowsPreferences.WindowsRegCloseKey(J)I", Any)]
#[async_method]
pub async fn windows_reg_close_key<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h_key = parameters.pop_long()?;
    Ok(Some(Value::Int(ERROR_SUCCESS)))
}

/// `WindowsRegCreateKeyEx(long hKey, byte[] subKey) -> long[]`
/// Returns `[handle, isNewKey, errorCode]`
#[intrinsic_method("java/util/prefs/WindowsPreferences.WindowsRegCreateKeyEx(J[B)[J", Any)]
#[async_method]
pub async fn windows_reg_create_key_ex<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _sub_key = parameters.pop_reference()?;
    let _h_key = parameters.pop_long()?;
    let vm = thread.vm()?;
    let collector = vm.garbage_collector();
    let result = Value::new_object(
        collector,
        Reference::from(vec![STUB_HANDLE, 1_i64, i64::from(ERROR_SUCCESS)]),
    );
    Ok(Some(result))
}

/// `WindowsRegDeleteKey(long hKey, byte[] subKey) -> int`
#[intrinsic_method("java/util/prefs/WindowsPreferences.WindowsRegDeleteKey(J[B)I", Any)]
#[async_method]
pub async fn windows_reg_delete_key<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _sub_key = parameters.pop_reference()?;
    let _h_key = parameters.pop_long()?;
    Ok(Some(Value::Int(ERROR_SUCCESS)))
}

/// `WindowsRegFlushKey(long hKey) -> int`
#[intrinsic_method("java/util/prefs/WindowsPreferences.WindowsRegFlushKey(J)I", Any)]
#[async_method]
pub async fn windows_reg_flush_key<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h_key = parameters.pop_long()?;
    Ok(Some(Value::Int(ERROR_SUCCESS)))
}

/// `WindowsRegQueryValueEx(long hKey, byte[] valueName) -> byte[]`
#[intrinsic_method(
    "java/util/prefs/WindowsPreferences.WindowsRegQueryValueEx(J[B)[B",
    Any
)]
#[async_method]
pub async fn windows_reg_query_value_ex<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _value_name = parameters.pop_reference()?;
    let _h_key = parameters.pop_long()?;
    Ok(Some(Value::Object(None)))
}

/// `WindowsRegSetValueEx(long hKey, byte[] valueName, byte[] data) -> int`
#[intrinsic_method("java/util/prefs/WindowsPreferences.WindowsRegSetValueEx(J[B[B)I", Any)]
#[async_method]
pub async fn windows_reg_set_value_ex<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _data = parameters.pop_reference()?;
    let _value_name = parameters.pop_reference()?;
    let _h_key = parameters.pop_long()?;
    Ok(Some(Value::Int(ERROR_SUCCESS)))
}

/// `WindowsRegDeleteValue(long hKey, byte[] valueName) -> int`
#[intrinsic_method("java/util/prefs/WindowsPreferences.WindowsRegDeleteValue(J[B)I", Any)]
#[async_method]
pub async fn windows_reg_delete_value<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _value_name = parameters.pop_reference()?;
    let _h_key = parameters.pop_long()?;
    Ok(Some(Value::Int(ERROR_SUCCESS)))
}

/// `WindowsRegQueryInfoKey(long hKey) -> long[]`
#[intrinsic_method("java/util/prefs/WindowsPreferences.WindowsRegQueryInfoKey(J)[J", Any)]
#[async_method]
pub async fn windows_reg_query_info_key<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h_key = parameters.pop_long()?;
    let vm = thread.vm()?;
    let collector = vm.garbage_collector();
    let result = Value::new_object(
        collector,
        Reference::from(vec![0i64, i64::from(ERROR_SUCCESS), 0, 0, 0, 0, 0, 0, 0]),
    );
    Ok(Some(result))
}

/// `WindowsRegEnumKeyEx(long hKey, int subKeyIndex, int maxKeyLength) -> byte[]`
#[intrinsic_method("java/util/prefs/WindowsPreferences.WindowsRegEnumKeyEx(JII)[B", Any)]
#[async_method]
pub async fn windows_reg_enum_key_ex<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _max_key_length = parameters.pop_int()?;
    let _sub_key_index = parameters.pop_int()?;
    let _h_key = parameters.pop_long()?;
    // Return null — no keys
    Ok(Some(Value::Object(None)))
}

/// `WindowsRegEnumValue(long hKey, int valueIndex, int maxValueNameLength) -> byte[]`
#[intrinsic_method("java/util/prefs/WindowsPreferences.WindowsRegEnumValue(JII)[B", Any)]
#[async_method]
pub async fn windows_reg_enum_value<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _max_value_name_length = parameters.pop_int()?;
    let _value_index = parameters.pop_int()?;
    let _h_key = parameters.pop_long()?;
    // Return null — no values
    Ok(Some(Value::Object(None)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_windows_reg_close_key() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut params = Parameters::default();
        params.push(Value::Long(0));
        let result = windows_reg_close_key(thread, params).await?;
        assert_eq!(result, Some(Value::Int(ERROR_SUCCESS)));
        Ok(())
    }

    #[tokio::test]
    async fn test_windows_reg_flush_key() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut params = Parameters::default();
        params.push(Value::Long(0));
        let result = windows_reg_flush_key(thread, params).await?;
        assert_eq!(result, Some(Value::Int(ERROR_SUCCESS)));
        Ok(())
    }

    #[tokio::test]
    async fn test_windows_reg_delete_value() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut params = Parameters::default();
        params.push(Value::Long(0));
        params.push(Value::Object(None));
        let result = windows_reg_delete_value(thread, params).await?;
        assert_eq!(result, Some(Value::Int(ERROR_SUCCESS)));
        Ok(())
    }

    #[tokio::test]
    async fn test_windows_reg_query_info_key() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut params = Parameters::default();
        params.push(Value::Long(0));
        let result = windows_reg_query_info_key(thread, params).await?;
        assert!(result.is_some());
        Ok(())
    }
}
