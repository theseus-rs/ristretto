use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/nio/fs/MacOSXNativeDispatcher.normalizepath([CI)[C", Any)]
#[async_method]
pub async fn normalizepath<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _form = parameters.pop_int()?;
    let path = parameters.pop()?;
    // Return the path as-is; JVM char arrays are already in valid Unicode form
    Ok(Some(path))
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
