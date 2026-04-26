use ristretto_classfile::JAVA_8;
use ristretto_classfile::JAVA_11;
#[cfg(not(target_os = "windows"))]
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classfile::VersionSpecification::Equal;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::{Reference, Value};
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
#[cfg(target_os = "windows")]
use ristretto_types::JavaError;
use ristretto_types::{Parameters, Result, Thread, VM};
use std::sync::Arc;
use windows_sys::Win32::System::Registry::{
    HKEY, KEY_READ, REG_OPTION_NON_VOLATILE, REG_SZ, RegCloseKey, RegCreateKeyExA, RegDeleteKeyA,
    RegDeleteValueA, RegEnumKeyExA, RegEnumValueA, RegFlushKey, RegOpenKeyExA, RegQueryInfoKeyA,
    RegQueryValueExA, RegSetValueExA,
};

/// Extracts a byte array from a `Value`, converting from Java's signed `i8` to `u8`.
/// Returns `None` if the value is null.
fn extract_bytes(value: &Value) -> Option<Vec<u8>> {
    let guard = value.as_byte_vec_ref().ok()?;
    #[expect(clippy::cast_sign_loss)]
    Some(guard.iter().map(|&b| b as u8).collect())
}

/// `WindowsRegOpenKey(long hKey, byte[] subKey, int securityMask) -> long[]`
///
/// Opens a Windows registry key. Returns a `long[2]`:
/// - `[0]`: the opened key handle (or 0 on failure)
/// - `[1]`: the Win32 error code (0 = `ERROR_SUCCESS`)
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "java/util/prefs/WindowsPreferences.WindowsRegOpenKey(J[BI)[J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn windows_reg_open_key<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let security_mask = parameters.pop_int()?;
    let sub_key_val = parameters.pop()?;
    let h_key = parameters.pop_long()?;

    let sub_key = extract_bytes(&sub_key_val).unwrap_or_default();
    let result = reg_open_key(h_key, &sub_key, security_mask);

    let long_array: Vec<i64> = result.to_vec();
    let reference = Reference::from(long_array);
    let vm = thread.vm()?;
    let value = Value::new_object(vm.garbage_collector(), reference);
    Ok(Some(value))
}

/// `WindowsRegCloseKey(long hKey) -> int`
///
/// Closes a previously opened registry key handle.
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "java/util/prefs/WindowsPreferences.WindowsRegCloseKey(J)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn windows_reg_close_key<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let h_key = parameters.pop_long()?;
    let result = reg_close_key(h_key);
    Ok(Some(Value::Int(result)))
}

/// `WindowsRegCreateKeyEx(long hKey, byte[] subKey) -> long[]`
///
/// Creates or opens a registry key. Returns a `long[3]`:
/// - `[0]`: the key handle
/// - `[1]`: the Win32 error code
/// - `[2]`: disposition (`REG_CREATED_NEW_KEY` = 1 or `REG_OPENED_EXISTING_KEY` = 2)
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "java/util/prefs/WindowsPreferences.WindowsRegCreateKeyEx(J[B)[J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn windows_reg_create_key_ex<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let sub_key_val = parameters.pop()?;
    let h_key = parameters.pop_long()?;

    let sub_key = extract_bytes(&sub_key_val).unwrap_or_default();
    let result = reg_create_key_ex(h_key, &sub_key);

    let long_array: Vec<i64> = result.to_vec();
    let reference = Reference::from(long_array);
    let vm = thread.vm()?;
    let value = Value::new_object(vm.garbage_collector(), reference);
    Ok(Some(value))
}

/// `WindowsRegDeleteKey(long hKey, byte[] subKey) -> int`
///
/// Deletes a registry subkey.
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "java/util/prefs/WindowsPreferences.WindowsRegDeleteKey(J[B)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn windows_reg_delete_key<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let sub_key_val = parameters.pop()?;
    let h_key = parameters.pop_long()?;

    let sub_key = extract_bytes(&sub_key_val).unwrap_or_default();
    let result = reg_delete_key(h_key, &sub_key);
    Ok(Some(Value::Int(result)))
}

/// `WindowsRegFlushKey(long hKey) -> int`
///
/// Writes all attributes of the specified key to the registry.
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "java/util/prefs/WindowsPreferences.WindowsRegFlushKey(J)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn windows_reg_flush_key<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let h_key = parameters.pop_long()?;
    let result = reg_flush_key(h_key);
    Ok(Some(Value::Int(result)))
}

/// `WindowsRegQueryValueEx(long hKey, byte[] valueName) -> byte[]`
///
/// Retrieves the data for a named value under a registry key.
/// Returns `null` if the value does not exist or is not of type `REG_SZ`.
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "java/util/prefs/WindowsPreferences.WindowsRegQueryValueEx(J[B)[B",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn windows_reg_query_value_ex<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let value_name_val = parameters.pop()?;
    let h_key = parameters.pop_long()?;

    let value_name = extract_bytes(&value_name_val).unwrap_or_default();
    match reg_query_value_ex(h_key, &value_name) {
        Some(data) => {
            #[expect(clippy::cast_possible_wrap)]
            let byte_array: Vec<i8> = data.into_iter().map(|b| b as i8).collect();
            let reference = Reference::from(byte_array);
            let vm = thread.vm()?;
            let value = Value::new_object(vm.garbage_collector(), reference);
            Ok(Some(value))
        }
        None => Ok(Some(Value::Object(None))),
    }
}

/// `WindowsRegSetValueEx(long hKey, byte[] valueName, byte[] data) -> int`
///
/// Sets the data for a named value under a registry key as `REG_SZ`.
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "java/util/prefs/WindowsPreferences.WindowsRegSetValueEx(J[B[B)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn windows_reg_set_value_ex<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let data_val = parameters.pop()?;
    let value_name_val = parameters.pop()?;
    let h_key = parameters.pop_long()?;

    let Some(value_name) = extract_bytes(&value_name_val) else {
        return Ok(Some(Value::Int(-1)));
    };
    let Some(data) = extract_bytes(&data_val) else {
        return Ok(Some(Value::Int(-1)));
    };

    let result = reg_set_value_ex(h_key, &value_name, &data);
    Ok(Some(Value::Int(result)))
}

/// `WindowsRegDeleteValue(long hKey, byte[] valueName) -> int`
///
/// Removes a named value from the specified registry key.
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "java/util/prefs/WindowsPreferences.WindowsRegDeleteValue(J[B)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn windows_reg_delete_value<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let value_name_val = parameters.pop()?;
    let h_key = parameters.pop_long()?;

    let Some(value_name) = extract_bytes(&value_name_val) else {
        return Ok(Some(Value::Int(-1)));
    };

    let result = reg_delete_value(h_key, &value_name);
    Ok(Some(Value::Int(result)))
}

/// `WindowsRegQueryInfoKey(long hKey) -> long[]`
///
/// Retrieves information about a registry key. Returns a `long[5]`:
/// - `[0]`: number of subkeys
/// - `[1]`: error code
/// - `[2]`: number of values
/// - `[3]`: max subkey name length
/// - `[4]`: max value name length
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "java/util/prefs/WindowsPreferences.WindowsRegQueryInfoKey(J)[J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn windows_reg_query_info_key<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let h_key = parameters.pop_long()?;
    let result = reg_query_info_key(h_key);

    let long_array: Vec<i64> = result.to_vec();
    let reference = Reference::from(long_array);
    let vm = thread.vm()?;
    let value = Value::new_object(vm.garbage_collector(), reference);
    Ok(Some(value))
}

/// `WindowsRegEnumKeyEx(long hKey, int subKeyIndex, int maxKeyLength) -> byte[]`
///
/// Enumerates subkeys of a registry key by index.
/// Returns `null` if the index is out of range.
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "java/util/prefs/WindowsPreferences.WindowsRegEnumKeyEx(JII)[B",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn windows_reg_enum_key_ex<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let max_key_length = parameters.pop_int()?;
    let sub_key_index = parameters.pop_int()?;
    let h_key = parameters.pop_long()?;

    match reg_enum_key_ex(h_key, sub_key_index, max_key_length) {
        Some(data) => {
            #[expect(clippy::cast_possible_wrap)]
            let byte_array: Vec<i8> = data.into_iter().map(|b| b as i8).collect();
            let reference = Reference::from(byte_array);
            let vm = thread.vm()?;
            let value = Value::new_object(vm.garbage_collector(), reference);
            Ok(Some(value))
        }
        None => Ok(Some(Value::Object(None))),
    }
}

/// `WindowsRegEnumValue(long hKey, int valueIndex, int maxValueNameLength) -> byte[]`
///
/// Enumerates value names of a registry key by index.
/// Returns `null` if the index is out of range.
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "java/util/prefs/WindowsPreferences.WindowsRegEnumValue(JII)[B",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn windows_reg_enum_value<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let max_value_name_length = parameters.pop_int()?;
    let value_index = parameters.pop_int()?;
    let h_key = parameters.pop_long()?;

    match reg_enum_value(h_key, value_index, max_value_name_length) {
        Some(data) => {
            #[expect(clippy::cast_possible_wrap)]
            let byte_array: Vec<i8> = data.into_iter().map(|b| b as i8).collect();
            let reference = Reference::from(byte_array);
            let vm = thread.vm()?;
            let value = Value::new_object(vm.garbage_collector(), reference);
            Ok(Some(value))
        }
        None => Ok(Some(Value::Object(None))),
    }
}

#[must_use]
fn reg_open_key(h_key: i64, sub_key: &[u8], security_mask: i32) -> [i64; 2] {
    let mut handle: HKEY = std::ptr::null_mut();
    #[expect(unsafe_code)]
    // SAFETY: sub_key is a null-terminated byte array from Java's
    // WindowsPreferences.stringToByteArray(). handle is a valid out pointer.
    let error_code = unsafe {
        RegOpenKeyExA(
            h_key as HKEY,
            sub_key.as_ptr(),
            0,
            security_mask.cast_unsigned(),
            &raw mut handle,
        )
    };
    [handle as i64, i64::from(error_code)]
}

#[must_use]
fn reg_close_key(h_key: i64) -> i32 {
    #[expect(unsafe_code)]
    // SAFETY: h_key is expected to be a valid registry handle from a prior open/create call.
    let result = unsafe { RegCloseKey(h_key as HKEY) };
    result.cast_signed()
}

#[must_use]
fn reg_create_key_ex(h_key: i64, sub_key: &[u8]) -> [i64; 3] {
    let mut handle: HKEY = std::ptr::null_mut();
    let mut disposition: u32 = 0;
    #[expect(unsafe_code)]
    // SAFETY: sub_key is a null-terminated byte array. handle and disposition are valid out
    // pointers. NULL is passed for security attributes and class name per OpenJDK convention.
    let error_code = unsafe {
        RegCreateKeyExA(
            h_key as HKEY,
            sub_key.as_ptr(),
            0,
            std::ptr::null(),
            REG_OPTION_NON_VOLATILE,
            KEY_READ,
            std::ptr::null(),
            &raw mut handle,
            &raw mut disposition,
        )
    };
    [handle as i64, i64::from(error_code), i64::from(disposition)]
}

#[must_use]
fn reg_delete_key(h_key: i64, sub_key: &[u8]) -> i32 {
    #[expect(unsafe_code)]
    // SAFETY: h_key is a valid handle; sub_key is a null-terminated byte array.
    let result = unsafe { RegDeleteKeyA(h_key as HKEY, sub_key.as_ptr()) };
    result.cast_signed()
}

#[must_use]
fn reg_flush_key(h_key: i64) -> i32 {
    #[expect(unsafe_code)]
    // SAFETY: h_key is expected to be a valid registry handle.
    let result = unsafe { RegFlushKey(h_key as HKEY) };
    result.cast_signed()
}

#[must_use]
fn reg_query_value_ex(h_key: i64, value_name: &[u8]) -> Option<Vec<u8>> {
    let mut value_type: u32 = 0;
    let mut value_size: u32 = 0;

    #[expect(unsafe_code)]
    // SAFETY: First call queries the size. h_key is valid; value_name is null-terminated.
    let result = unsafe {
        RegQueryValueExA(
            h_key as HKEY,
            value_name.as_ptr(),
            std::ptr::null(),
            &raw mut value_type,
            std::ptr::null_mut(),
            &raw mut value_size,
        )
    };
    if result != 0 {
        return None;
    }

    let mut buffer = vec![0u8; value_size as usize];
    #[expect(unsafe_code)]
    // SAFETY: Second call reads the data into a properly sized buffer.
    let result = unsafe {
        RegQueryValueExA(
            h_key as HKEY,
            value_name.as_ptr(),
            std::ptr::null(),
            &raw mut value_type,
            buffer.as_mut_ptr(),
            &raw mut value_size,
        )
    };
    if result != 0 {
        return None;
    }

    // Only return REG_SZ values, matching OpenJDK behavior
    if value_type == REG_SZ {
        buffer.truncate(value_size as usize);
        Some(buffer)
    } else {
        None
    }
}

#[expect(clippy::cast_possible_truncation)]
#[must_use]
fn reg_set_value_ex(h_key: i64, value_name: &[u8], data: &[u8]) -> i32 {
    #[expect(unsafe_code)]
    // SAFETY: h_key is valid; value_name and data are valid byte slices.
    // data.len() fits in u32 for registry values.
    let result = unsafe {
        RegSetValueExA(
            h_key as HKEY,
            value_name.as_ptr(),
            0,
            REG_SZ,
            data.as_ptr(),
            data.len() as u32,
        )
    };
    result.cast_signed()
}

#[must_use]
fn reg_delete_value(h_key: i64, value_name: &[u8]) -> i32 {
    #[expect(unsafe_code)]
    // SAFETY: h_key is valid; value_name is a null-terminated byte array.
    let result = unsafe { RegDeleteValueA(h_key as HKEY, value_name.as_ptr()) };
    result.cast_signed()
}

#[must_use]
fn reg_query_info_key(h_key: i64) -> [i64; 5] {
    let mut sub_keys_number: u32 = 0;
    let mut max_sub_key_length: u32 = 0;
    let mut values_number: u32 = 0;
    let mut max_value_name_length: u32 = 0;

    #[expect(unsafe_code)]
    // SAFETY: h_key is valid; all out pointers are properly initialized.
    // NULL is passed for unused parameters per OpenJDK convention.
    let error_code = unsafe {
        RegQueryInfoKeyA(
            h_key as HKEY,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null(),
            &raw mut sub_keys_number,
            &raw mut max_sub_key_length,
            std::ptr::null_mut(),
            &raw mut values_number,
            &raw mut max_value_name_length,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        )
    };
    [
        i64::from(sub_keys_number),
        i64::from(error_code),
        i64::from(values_number),
        i64::from(max_sub_key_length),
        i64::from(max_value_name_length),
    ]
}

#[expect(clippy::cast_sign_loss)]
#[must_use]
fn reg_enum_key_ex(h_key: i64, sub_key_index: i32, max_key_length: i32) -> Option<Vec<u8>> {
    let mut size = max_key_length.cast_unsigned();
    let mut buffer = vec![0u8; max_key_length as usize];

    #[expect(unsafe_code)]
    // SAFETY: h_key is valid; buffer is properly sized; size is an in/out parameter.
    // NULL is passed for unused parameters per OpenJDK convention.
    let result = unsafe {
        RegEnumKeyExA(
            h_key as HKEY,
            sub_key_index.cast_unsigned(),
            buffer.as_mut_ptr(),
            &raw mut size,
            std::ptr::null(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        )
    };
    if result != 0 {
        return None;
    }

    // Return size+1 bytes to include the null terminator, matching OpenJDK behavior
    buffer.truncate((size + 1) as usize);
    Some(buffer)
}

#[expect(clippy::cast_sign_loss)]
#[must_use]
fn reg_enum_value(h_key: i64, value_index: i32, max_value_name_length: i32) -> Option<Vec<u8>> {
    let mut size = max_value_name_length.cast_unsigned();
    let mut buffer = vec![0u8; max_value_name_length as usize];

    #[expect(unsafe_code)]
    // SAFETY: h_key is valid; buffer is properly sized; size is an in/out parameter.
    // NULL is passed for unused parameters per OpenJDK convention.
    let result = unsafe {
        RegEnumValueA(
            h_key as HKEY,
            value_index.cast_unsigned(),
            buffer.as_mut_ptr(),
            &raw mut size,
            std::ptr::null(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        )
    };
    if result != 0 {
        return None;
    }

    // Return size+1 bytes to include the null terminator, matching OpenJDK behavior
    buffer.truncate((size + 1) as usize);
    Some(buffer)
}

#[cfg(target_os = "windows")]
#[intrinsic_method(
    "java/util/prefs/WindowsPreferences.WindowsRegCloseKey(I)I",
    Equal(JAVA_8)
)]
#[async_method]
pub async fn windows_reg_close_key_windows_v8_v1<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h_key = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/util/prefs/WindowsPreferences.WindowsRegCloseKey(I)I".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "java/util/prefs/WindowsPreferences.WindowsRegCreateKeyEx(I[B)[I",
    Equal(JAVA_8)
)]
#[async_method]
pub async fn windows_reg_create_key_ex_windows_v8_v1<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _lp_sub_key = parameters.pop_reference()?;
    let _h_key = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/util/prefs/WindowsPreferences.WindowsRegCreateKeyEx(I[B)[I".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "java/util/prefs/WindowsPreferences.WindowsRegDeleteKey(I[B)I",
    Equal(JAVA_8)
)]
#[async_method]
pub async fn windows_reg_delete_key_windows_v8_v1<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _lp_sub_key = parameters.pop_reference()?;
    let _h_key = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/util/prefs/WindowsPreferences.WindowsRegDeleteKey(I[B)I".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "java/util/prefs/WindowsPreferences.WindowsRegDeleteValue(I[B)I",
    Equal(JAVA_8)
)]
#[async_method]
pub async fn windows_reg_delete_value_windows_v8_v1<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _value_name = parameters.pop_reference()?;
    let _h_key = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/util/prefs/WindowsPreferences.WindowsRegDeleteValue(I[B)I".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "java/util/prefs/WindowsPreferences.WindowsRegEnumKeyEx(III)[B",
    Equal(JAVA_8)
)]
#[async_method]
pub async fn windows_reg_enum_key_ex_windows_v8_v1<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _max_key_length = parameters.pop_int()?;
    let _sub_key_index = parameters.pop_int()?;
    let _h_key = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/util/prefs/WindowsPreferences.WindowsRegEnumKeyEx(III)[B".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "java/util/prefs/WindowsPreferences.WindowsRegEnumValue(III)[B",
    Equal(JAVA_8)
)]
#[async_method]
pub async fn windows_reg_enum_value_windows_v8_v1<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _max_value_name_length = parameters.pop_int()?;
    let _value_index = parameters.pop_int()?;
    let _h_key = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/util/prefs/WindowsPreferences.WindowsRegEnumValue(III)[B".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "java/util/prefs/WindowsPreferences.WindowsRegFlushKey(I)I",
    Equal(JAVA_8)
)]
#[async_method]
pub async fn windows_reg_flush_key_windows_v8_v1<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h_key = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/util/prefs/WindowsPreferences.WindowsRegFlushKey(I)I".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "java/util/prefs/WindowsPreferences.WindowsRegOpenKey(I[BI)[I",
    Equal(JAVA_8)
)]
#[async_method]
pub async fn windows_reg_open_key_windows_v8_v1<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _security_mask = parameters.pop_int()?;
    let _lp_sub_key = parameters.pop_reference()?;
    let _h_key = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/util/prefs/WindowsPreferences.WindowsRegOpenKey(I[BI)[I".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "java/util/prefs/WindowsPreferences.WindowsRegQueryInfoKey(I)[I",
    Equal(JAVA_8)
)]
#[async_method]
pub async fn windows_reg_query_info_key_windows_v8_v1<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h_key = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/util/prefs/WindowsPreferences.WindowsRegQueryInfoKey(I)[I".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "java/util/prefs/WindowsPreferences.WindowsRegQueryValueEx(I[B)[B",
    Equal(JAVA_8)
)]
#[async_method]
pub async fn windows_reg_query_value_ex_windows_v8_v1<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _value_name = parameters.pop_reference()?;
    let _h_key = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/util/prefs/WindowsPreferences.WindowsRegQueryValueEx(I[B)[B".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "java/util/prefs/WindowsPreferences.WindowsRegSetValueEx(I[B[B)I",
    Equal(JAVA_8)
)]
#[async_method]
pub async fn windows_reg_set_value_ex_windows_v8_v1<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _data = parameters.pop_reference()?;
    let _value_name = parameters.pop_reference()?;
    let _h_key = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/util/prefs/WindowsPreferences.WindowsRegSetValueEx(I[B[B)I".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "java/util/prefs/WindowsPreferences.WindowsRegCloseKey(I)I",
    Equal(JAVA_8)
)]
#[async_method]
pub async fn windows_reg_close_key_windows_v8_v2<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h_key = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/util/prefs/WindowsPreferences.WindowsRegCloseKey(I)I".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "java/util/prefs/WindowsPreferences.WindowsRegCreateKeyEx(I[B)[I",
    Equal(JAVA_8)
)]
#[async_method]
pub async fn windows_reg_create_key_ex_windows_v8_v2<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _lp_sub_key = parameters.pop_reference()?;
    let _h_key = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/util/prefs/WindowsPreferences.WindowsRegCreateKeyEx(I[B)[I".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "java/util/prefs/WindowsPreferences.WindowsRegDeleteKey(I[B)I",
    Equal(JAVA_8)
)]
#[async_method]
pub async fn windows_reg_delete_key_windows_v8_v2<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _lp_sub_key = parameters.pop_reference()?;
    let _h_key = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/util/prefs/WindowsPreferences.WindowsRegDeleteKey(I[B)I".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "java/util/prefs/WindowsPreferences.WindowsRegDeleteValue(I[B)I",
    Equal(JAVA_8)
)]
#[async_method]
pub async fn windows_reg_delete_value_windows_v8_v2<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _value_name = parameters.pop_reference()?;
    let _h_key = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/util/prefs/WindowsPreferences.WindowsRegDeleteValue(I[B)I".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "java/util/prefs/WindowsPreferences.WindowsRegEnumKeyEx(III)[B",
    Equal(JAVA_8)
)]
#[async_method]
pub async fn windows_reg_enum_key_ex_windows_v8_v2<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _max_key_length = parameters.pop_int()?;
    let _sub_key_index = parameters.pop_int()?;
    let _h_key = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/util/prefs/WindowsPreferences.WindowsRegEnumKeyEx(III)[B".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "java/util/prefs/WindowsPreferences.WindowsRegEnumValue(III)[B",
    Equal(JAVA_8)
)]
#[async_method]
pub async fn windows_reg_enum_value_windows_v8_v2<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _max_value_name_length = parameters.pop_int()?;
    let _value_index = parameters.pop_int()?;
    let _h_key = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/util/prefs/WindowsPreferences.WindowsRegEnumValue(III)[B".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "java/util/prefs/WindowsPreferences.WindowsRegFlushKey(I)I",
    Equal(JAVA_8)
)]
#[async_method]
pub async fn windows_reg_flush_key_windows_v8_v2<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h_key = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/util/prefs/WindowsPreferences.WindowsRegFlushKey(I)I".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "java/util/prefs/WindowsPreferences.WindowsRegOpenKey(I[BI)[I",
    Equal(JAVA_8)
)]
#[async_method]
pub async fn windows_reg_open_key_windows_v8_v2<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _security_mask = parameters.pop_int()?;
    let _lp_sub_key = parameters.pop_reference()?;
    let _h_key = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/util/prefs/WindowsPreferences.WindowsRegOpenKey(I[BI)[I".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "java/util/prefs/WindowsPreferences.WindowsRegQueryInfoKey(I)[I",
    Equal(JAVA_8)
)]
#[async_method]
pub async fn windows_reg_query_info_key_windows_v8_v2<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h_key = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/util/prefs/WindowsPreferences.WindowsRegQueryInfoKey(I)[I".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "java/util/prefs/WindowsPreferences.WindowsRegQueryValueEx(I[B)[B",
    Equal(JAVA_8)
)]
#[async_method]
pub async fn windows_reg_query_value_ex_windows_v8_v2<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _value_name = parameters.pop_reference()?;
    let _h_key = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/util/prefs/WindowsPreferences.WindowsRegQueryValueEx(I[B)[B".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "java/util/prefs/WindowsPreferences.WindowsRegSetValueEx(I[B[B)I",
    Equal(JAVA_8)
)]
#[async_method]
pub async fn windows_reg_set_value_ex_windows_v8_v2<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _data = parameters.pop_reference()?;
    let _value_name = parameters.pop_reference()?;
    let _h_key = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java/util/prefs/WindowsPreferences.WindowsRegSetValueEx(I[B[B)I".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_windows_reg_open_key() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let sub_key: Vec<i8> = b"Software\0".iter().map(|&b| b.cast_signed()).collect();
        let sub_key_ref = Reference::from(sub_key);
        let sub_key_val = Value::new_object(thread.vm()?.garbage_collector(), sub_key_ref);

        let mut parameters = Parameters::default();
        parameters.push_long(0x8000_0001); // HKEY_CURRENT_USER
        parameters.push(sub_key_val);
        parameters.push_int(0x2_0019); // KEY_READ

        let result = windows_reg_open_key(thread.clone(), parameters)
            .await?
            .expect("value");
        let handle_val = {
            let result_ref = result.as_reference()?;
            let Reference::LongArray(arr) = &*result_ref else {
                panic!("Expected LongArray result");
            };
            assert_eq!(0, arr[1], "Expected ERROR_SUCCESS");
            assert_ne!(0, arr[0], "Expected valid handle");
            arr[0]
        };
        // Close the key we opened
        let mut close_params = Parameters::default();
        close_params.push_long(handle_val);
        let close_result = windows_reg_close_key(thread, close_params)
            .await?
            .expect("value");
        assert_eq!(0, close_result.as_i32()?);
        Ok(())
    }

    #[tokio::test]
    async fn test_windows_reg_create_and_delete_key() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;

        // Create a test key under HKCU
        let sub_key: Vec<i8> = b"Software\\JavaSoft\\Prefs\\ristretto_test_key\0"
            .iter()
            .map(|&b| b.cast_signed())
            .collect();
        let sub_key_ref = Reference::from(sub_key);
        let sub_key_val = Value::new_object(thread.vm()?.garbage_collector(), sub_key_ref);

        let mut parameters = Parameters::default();
        parameters.push_long(0x8000_0001); // HKEY_CURRENT_USER
        parameters.push(sub_key_val);

        let result = windows_reg_create_key_ex(thread.clone(), parameters)
            .await?
            .expect("value");
        let handle = {
            let result_ref = result.as_reference()?;
            let Reference::LongArray(arr) = &*result_ref else {
                panic!("Expected LongArray result");
            };
            assert_eq!(0, arr[1], "Expected ERROR_SUCCESS");
            assert_ne!(0, arr[0], "Expected valid handle");
            arr[0]
        };

        // Close the created key
        let mut close_params = Parameters::default();
        close_params.push_long(handle);
        let _ = windows_reg_close_key(thread.clone(), close_params).await?;

        // Delete the test key
        let del_key: Vec<i8> = b"Software\\JavaSoft\\Prefs\\ristretto_test_key\0"
            .iter()
            .map(|&b| b.cast_signed())
            .collect();
        let del_key_ref = Reference::from(del_key);
        let del_key_val = Value::new_object(thread.vm()?.garbage_collector(), del_key_ref);

        let mut del_params = Parameters::default();
        del_params.push_long(0x8000_0001); // HKEY_CURRENT_USER
        del_params.push(del_key_val);
        let del_result = windows_reg_delete_key(thread, del_params)
            .await?
            .expect("value");
        assert_eq!(0, del_result.as_i32()?);
        Ok(())
    }

    #[tokio::test]
    async fn test_windows_reg_query_info_key() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;

        let mut parameters = Parameters::default();
        parameters.push_long(0x8000_0001); // HKEY_CURRENT_USER

        let result = windows_reg_query_info_key(thread, parameters)
            .await?
            .expect("value");
        let result_ref = result.as_reference()?;
        let Reference::LongArray(arr) = &*result_ref else {
            panic!("Expected LongArray result");
        };
        assert_eq!(5, arr.len());
        assert_eq!(0, arr[1], "Expected ERROR_SUCCESS");
        Ok(())
    }

    #[tokio::test]
    async fn test_windows_reg_set_and_query_value() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;

        // Create a test key
        let sub_key: Vec<i8> = b"Software\\JavaSoft\\Prefs\\ristretto_val_test\0"
            .iter()
            .map(|&b| b.cast_signed())
            .collect();
        let sub_key_ref = Reference::from(sub_key);
        let sub_key_val = Value::new_object(thread.vm()?.garbage_collector(), sub_key_ref);

        let mut create_params = Parameters::default();
        create_params.push_long(0x8000_0001);
        create_params.push(sub_key_val);
        let create_result = windows_reg_create_key_ex(thread.clone(), create_params)
            .await?
            .expect("value");
        let handle = {
            let create_ref = create_result.as_reference()?;
            if let Reference::LongArray(arr) = &*create_ref {
                assert_eq!(0, arr[1]);
                arr[0]
            } else {
                panic!("Expected LongArray");
            }
        };

        // We need a handle with write access for RegSetValueEx
        let mut close_params = Parameters::default();
        close_params.push_long(handle);
        let _ = windows_reg_close_key(thread.clone(), close_params).await?;

        // Re-open with write access
        let sub_key2: Vec<i8> = b"Software\\JavaSoft\\Prefs\\ristretto_val_test\0"
            .iter()
            .map(|&b| b.cast_signed())
            .collect();
        let sub_key_ref2 = Reference::from(sub_key2);
        let sub_key_val2 = Value::new_object(thread.vm()?.garbage_collector(), sub_key_ref2);

        let mut open_params = Parameters::default();
        open_params.push_long(0x8000_0001);
        open_params.push(sub_key_val2);
        open_params.push_int(0xf_003f); // KEY_ALL_ACCESS
        let open_result = windows_reg_open_key(thread.clone(), open_params)
            .await?
            .expect("value");
        let handle = {
            let open_ref = open_result.as_reference()?;
            if let Reference::LongArray(arr) = &*open_ref {
                assert_eq!(0, arr[1]);
                arr[0]
            } else {
                panic!("Expected LongArray");
            }
        };

        // Set a value
        let val_name: Vec<i8> = b"testVal\0".iter().map(|&b| b.cast_signed()).collect();
        let val_name_ref = Reference::from(val_name);
        let val_name_val = Value::new_object(thread.vm()?.garbage_collector(), val_name_ref);

        let val_data: Vec<i8> = b"hello\0".iter().map(|&b| b.cast_signed()).collect();
        let val_data_ref = Reference::from(val_data);
        let val_data_val = Value::new_object(thread.vm()?.garbage_collector(), val_data_ref);

        let mut set_params = Parameters::default();
        set_params.push_long(handle);
        set_params.push(val_name_val);
        set_params.push(val_data_val);
        let set_result = windows_reg_set_value_ex(thread.clone(), set_params)
            .await?
            .expect("value");
        assert_eq!(0, set_result.as_i32()?);

        // Query the value back
        let qval_name: Vec<i8> = b"testVal\0".iter().map(|&b| b.cast_signed()).collect();
        let qval_name_ref = Reference::from(qval_name);
        let qval_name_val = Value::new_object(thread.vm()?.garbage_collector(), qval_name_ref);

        let mut query_params = Parameters::default();
        query_params.push_long(handle);
        query_params.push(qval_name_val);
        let query_result = windows_reg_query_value_ex(thread.clone(), query_params)
            .await?
            .expect("value");
        assert_ne!(
            Value::Object(None),
            query_result,
            "Expected non-null result"
        );

        // Delete the value
        let dval_name: Vec<i8> = b"testVal\0".iter().map(|&b| b.cast_signed()).collect();
        let dval_name_ref = Reference::from(dval_name);
        let dval_name_val = Value::new_object(thread.vm()?.garbage_collector(), dval_name_ref);

        let mut del_val_params = Parameters::default();
        del_val_params.push_long(handle);
        del_val_params.push(dval_name_val);
        let del_result = windows_reg_delete_value(thread.clone(), del_val_params)
            .await?
            .expect("value");
        assert_eq!(0, del_result.as_i32()?);

        // Close handle
        let mut close_params = Parameters::default();
        close_params.push_long(handle);
        let _ = windows_reg_close_key(thread.clone(), close_params).await?;

        // Delete the test key
        let del_key: Vec<i8> = b"Software\\JavaSoft\\Prefs\\ristretto_val_test\0"
            .iter()
            .map(|&b| b.cast_signed())
            .collect();
        let del_key_ref = Reference::from(del_key);
        let del_key_val = Value::new_object(thread.vm()?.garbage_collector(), del_key_ref);

        let mut del_params = Parameters::default();
        del_params.push_long(0x8000_0001);
        del_params.push(del_key_val);
        let _ = windows_reg_delete_key(thread, del_params).await?;
        Ok(())
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_windows_reg_close_key_windows_v8_v1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            windows_reg_close_key_windows_v8_v1(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "java/util/prefs/WindowsPreferences.WindowsRegCloseKey(I)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_windows_reg_create_key_ex_windows_v8_v1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = windows_reg_create_key_ex_windows_v8_v1(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "java/util/prefs/WindowsPreferences.WindowsRegCreateKeyEx(I[B)[I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_windows_reg_delete_key_windows_v8_v1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = windows_reg_delete_key_windows_v8_v1(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "java/util/prefs/WindowsPreferences.WindowsRegDeleteKey(I[B)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_windows_reg_delete_value_windows_v8_v1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = windows_reg_delete_value_windows_v8_v1(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "java/util/prefs/WindowsPreferences.WindowsRegDeleteValue(I[B)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_windows_reg_enum_key_ex_windows_v8_v1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = windows_reg_enum_key_ex_windows_v8_v1(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Int(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "java/util/prefs/WindowsPreferences.WindowsRegEnumKeyEx(III)[B",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_windows_reg_enum_value_windows_v8_v1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = windows_reg_enum_value_windows_v8_v1(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Int(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "java/util/prefs/WindowsPreferences.WindowsRegEnumValue(III)[B",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_windows_reg_flush_key_windows_v8_v1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            windows_reg_flush_key_windows_v8_v1(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "java/util/prefs/WindowsPreferences.WindowsRegFlushKey(I)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_windows_reg_open_key_windows_v8_v1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = windows_reg_open_key_windows_v8_v1(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Object(None), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "java/util/prefs/WindowsPreferences.WindowsRegOpenKey(I[BI)[I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_windows_reg_query_info_key_windows_v8_v1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            windows_reg_query_info_key_windows_v8_v1(thread, Parameters::new(vec![Value::Int(0)]))
                .await;
        assert_eq!(
            "java/util/prefs/WindowsPreferences.WindowsRegQueryInfoKey(I)[I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_windows_reg_query_value_ex_windows_v8_v1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = windows_reg_query_value_ex_windows_v8_v1(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "java/util/prefs/WindowsPreferences.WindowsRegQueryValueEx(I[B)[B",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_windows_reg_set_value_ex_windows_v8_v1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = windows_reg_set_value_ex_windows_v8_v1(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Object(None),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "java/util/prefs/WindowsPreferences.WindowsRegSetValueEx(I[B[B)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_windows_reg_close_key_windows_v8_v2() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            windows_reg_close_key_windows_v8_v2(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "java/util/prefs/WindowsPreferences.WindowsRegCloseKey(I)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_windows_reg_create_key_ex_windows_v8_v2() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = windows_reg_create_key_ex_windows_v8_v2(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "java/util/prefs/WindowsPreferences.WindowsRegCreateKeyEx(I[B)[I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_windows_reg_delete_key_windows_v8_v2() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = windows_reg_delete_key_windows_v8_v2(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "java/util/prefs/WindowsPreferences.WindowsRegDeleteKey(I[B)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_windows_reg_delete_value_windows_v8_v2() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = windows_reg_delete_value_windows_v8_v2(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "java/util/prefs/WindowsPreferences.WindowsRegDeleteValue(I[B)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_windows_reg_enum_key_ex_windows_v8_v2() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = windows_reg_enum_key_ex_windows_v8_v2(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Int(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "java/util/prefs/WindowsPreferences.WindowsRegEnumKeyEx(III)[B",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_windows_reg_enum_value_windows_v8_v2() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = windows_reg_enum_value_windows_v8_v2(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Int(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "java/util/prefs/WindowsPreferences.WindowsRegEnumValue(III)[B",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_windows_reg_flush_key_windows_v8_v2() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            windows_reg_flush_key_windows_v8_v2(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "java/util/prefs/WindowsPreferences.WindowsRegFlushKey(I)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_windows_reg_open_key_windows_v8_v2() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = windows_reg_open_key_windows_v8_v2(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Object(None), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "java/util/prefs/WindowsPreferences.WindowsRegOpenKey(I[BI)[I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_windows_reg_query_info_key_windows_v8_v2() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            windows_reg_query_info_key_windows_v8_v2(thread, Parameters::new(vec![Value::Int(0)]))
                .await;
        assert_eq!(
            "java/util/prefs/WindowsPreferences.WindowsRegQueryInfoKey(I)[I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_windows_reg_query_value_ex_windows_v8_v2() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = windows_reg_query_value_ex_windows_v8_v2(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "java/util/prefs/WindowsPreferences.WindowsRegQueryValueEx(I[B)[B",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_windows_reg_set_value_ex_windows_v8_v2() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = windows_reg_set_value_ex_windows_v8_v2(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Object(None),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "java/util/prefs/WindowsPreferences.WindowsRegSetValueEx(I[B[B)I",
            result.unwrap_err().to_string()
        );
    }
}
