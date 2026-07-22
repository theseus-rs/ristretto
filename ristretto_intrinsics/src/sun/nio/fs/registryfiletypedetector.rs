#![expect(
    clippy::items_after_statements,
    reason = "Win32 registry imports stay beside their platform-only calls"
)]

use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::{async_method, intrinsic_method};
use ristretto_types::Error::InternalError;
use ristretto_types::{Parameters, Result, Thread, VM};
use std::sync::Arc;

fn read_wide_string<V: VM + ?Sized>(vm: &V, address: i64) -> Result<Vec<u16>> {
    let mut result = Vec::new();
    let mut current = address;
    loop {
        let value = vm
            .native_memory()
            .read_i16(current)
            .ok_or_else(|| InternalError(format!("invalid native UTF-16 address: {current}")))?
            .cast_unsigned();
        result.push(value);
        if value == 0 {
            return Ok(result);
        }
        current = current
            .checked_add(2)
            .ok_or_else(|| InternalError("native UTF-16 address overflow".to_string()))?;
    }
}

#[intrinsic_method(
    "sun/nio/fs/RegistryFileTypeDetector.queryStringValue(JJ)Ljava/lang/String;",
    Any
)]
#[async_method]
#[expect(unsafe_code)]
pub async fn query_string_value<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let name_address = parameters.pop_long()?;
    let key_address = parameters.pop_long()?;
    let vm = thread.vm()?;
    let key = read_wide_string(&*vm, key_address)?;
    let name = read_wide_string(&*vm, name_address)?;

    use windows_sys::Win32::Foundation::ERROR_SUCCESS;
    use windows_sys::Win32::System::Registry::{
        HKEY, HKEY_CLASSES_ROOT, KEY_READ, REG_SZ, RegCloseKey, RegOpenKeyExW, RegQueryValueExW,
    };

    let value = (|| -> Result<Option<String>> {
        let mut registry_key: HKEY = std::ptr::null_mut();
        if unsafe {
            RegOpenKeyExW(
                HKEY_CLASSES_ROOT,
                key.as_ptr(),
                0,
                KEY_READ,
                &raw mut registry_key,
            )
        } != ERROR_SUCCESS
        {
            return Ok(None);
        }

        let mut value_type = 0u32;
        let mut size = 0u32;
        let status = unsafe {
            RegQueryValueExW(
                registry_key,
                name.as_ptr(),
                std::ptr::null(),
                &raw mut value_type,
                std::ptr::null_mut(),
                &raw mut size,
            )
        };
        if status != ERROR_SUCCESS || value_type != REG_SZ || size < 2 {
            unsafe { RegCloseKey(registry_key) };
            return Ok(None);
        }

        let mut bytes = vec![0u8; usize::try_from(size).unwrap_or(0)];
        let status = unsafe {
            RegQueryValueExW(
                registry_key,
                name.as_ptr(),
                std::ptr::null(),
                &raw mut value_type,
                bytes.as_mut_ptr(),
                &raw mut size,
            )
        };
        unsafe { RegCloseKey(registry_key) };
        if status != ERROR_SUCCESS || value_type != REG_SZ {
            return Ok(None);
        }

        let chars: Vec<u16> = bytes
            .get(..usize::try_from(size).unwrap_or(0))
            .unwrap_or_default()
            .chunks_exact(2)
            .filter_map(|chunk| <[u8; 2]>::try_from(chunk).ok().map(u16::from_le_bytes))
            .take_while(|character| *character != 0)
            .collect();
        String::from_utf16(&chars)
            .map(Some)
            .map_err(|error| InternalError(error.to_string()))
    })()?;
    let Some(value) = value else {
        return Ok(Some(Value::Object(None)));
    };
    Ok(Some(thread.intern_string(&value).await?))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_query_string_value_invalid_address() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = query_string_value(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert!(result.is_err());
    }
}
