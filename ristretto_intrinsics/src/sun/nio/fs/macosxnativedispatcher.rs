use core_foundation_sys::base::{CFRange, CFRelease};
use core_foundation_sys::string::{
    CFStringAppendCharacters, CFStringCreateMutable, CFStringGetCharacters, CFStringGetLength,
    CFStringNormalize,
};
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::{Reference, Value};
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use ristretto_types::{Thread, VM};
use std::sync::Arc;

#[intrinsic_method("sun/nio/fs/MacOSXNativeDispatcher.normalizepath([CI)[C", Any)]
#[async_method]
pub async fn normalizepath<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let form = parameters.pop_int()?;
    let path = parameters.pop()?;
    let chars = {
        let reference = path.as_reference()?;
        let Reference::CharArray(chars) = &*reference else {
            return Err(ristretto_types::Error::InternalError(
                "normalizepath: path is not a char array".to_string(),
            ));
        };
        chars.to_vec()
    };
    let char_count = isize::try_from(chars.len()).map_err(|_| {
        ristretto_types::Error::InternalError("normalizepath: path is too long".to_string())
    })?;
    let normalization_form = isize::try_from(form).map_err(|_| {
        ristretto_types::Error::InternalError(
            "normalizepath: invalid normalization form".to_string(),
        )
    })?;

    #[expect(unsafe_code)]
    let normalized = unsafe {
        let string = CFStringCreateMutable(std::ptr::null(), 0);
        if string.is_null() {
            return Err(ristretto_types::Error::InternalError(
                "normalizepath: CoreFoundation allocation failed".to_string(),
            ));
        }
        CFStringAppendCharacters(string, chars.as_ptr(), char_count);
        CFStringNormalize(string, normalization_form);
        let cf_length = CFStringGetLength(string);
        let Ok(length) = usize::try_from(cf_length) else {
            CFRelease(string.cast());
            return Err(ristretto_types::Error::InternalError(
                "normalizepath: invalid normalized path length".to_string(),
            ));
        };
        let mut result = vec![0u16; length];
        if length != 0 {
            CFStringGetCharacters(string, CFRange::init(0, cf_length), result.as_mut_ptr());
        }
        CFRelease(string.cast());
        result
    };

    let vm = thread.vm()?;
    Ok(Some(Value::new_object(
        vm.garbage_collector(),
        Reference::CharArray(normalized.into_boxed_slice()),
    )))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ristretto_classloader::Reference;
    use ristretto_types::VM;

    #[tokio::test]
    async fn test_normalizepath_default_params() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = normalizepath(thread, Parameters::default()).await;
        assert!(matches!(
            result,
            Err(ristretto_types::Error::ParametersUnderflow)
        ));
    }

    #[tokio::test]
    async fn test_normalizepath_ascii() {
        let (vm, thread) = crate::test::thread().await.expect("thread");
        let chars: Vec<u16> = vec!['/' as u16, 't' as u16, 'm' as u16, 'p' as u16];
        let char_array = Value::new_object(
            vm.garbage_collector(),
            Reference::CharArray(chars.clone().into()),
        );

        let mut params = Parameters::default();
        params.push(char_array.clone());
        params.push_int(0); // NFC form

        let result = normalizepath(thread, params).await;
        assert!(result.is_ok());
        let value = result.unwrap();
        assert!(value.is_some());
    }

    #[tokio::test]
    async fn test_normalizepath_unicode() {
        let (vm, thread) = crate::test::thread().await.expect("thread");
        // Unicode path with accented character (é = U+00E9)
        let chars: Vec<u16> = vec!['/' as u16, 'c' as u16, 'a' as u16, 'f' as u16, 0x00E9];
        let char_array =
            Value::new_object(vm.garbage_collector(), Reference::CharArray(chars.into()));

        let mut params = Parameters::default();
        params.push(char_array);
        params.push_int(0); // NFC form

        let result = normalizepath(thread, params).await;
        assert!(result.is_ok());
        let value = result.unwrap();
        assert!(value.is_some());
    }

    #[tokio::test]
    async fn test_normalizepath_empty() {
        let (vm, thread) = crate::test::thread().await.expect("thread");
        let chars: Vec<u16> = vec![];
        let char_array =
            Value::new_object(vm.garbage_collector(), Reference::CharArray(chars.into()));

        let mut params = Parameters::default();
        params.push(char_array);
        params.push_int(0); // NFC form

        let result = normalizepath(thread, params).await;
        assert!(result.is_ok());
        let value = result.unwrap();
        assert!(value.is_some());
    }
}
