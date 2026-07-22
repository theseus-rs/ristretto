use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaObject;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

use core_foundation_sys::base::{CFRelease, CFTypeRef};
use core_foundation_sys::string::{
    CFStringCreateWithCharacters, CFStringGetCharacters, CFStringGetLength, CFStringRef,
};

#[link(name = "CoreServices", kind = "framework")]
#[expect(unsafe_code)]
unsafe extern "C" {
    static kUTTagClassFilenameExtension: CFStringRef;
    static kUTTagClassMIMEType: CFStringRef;

    fn UTTypeCreatePreferredIdentifierForTag(
        tag_class: CFStringRef,
        tag: CFStringRef,
        conforming_to_uti: CFStringRef,
    ) -> CFStringRef;
    fn UTTypeCopyPreferredTagWithClass(uti: CFStringRef, tag_class: CFStringRef) -> CFStringRef;
}

#[intrinsic_method(
    "sun/nio/fs/UTIFileTypeDetector.probe0(Ljava/lang/String;)Ljava/lang/String;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn probe_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let extension = parameters.pop()?.as_string()?;
    let extension: Vec<u16> = extension.encode_utf16().collect();
    let Ok(extension_length) = isize::try_from(extension.len()) else {
        return Ok(Some(Value::Object(None)));
    };

    #[expect(unsafe_code)]
    let mime_type = unsafe {
        let extension_ref =
            CFStringCreateWithCharacters(std::ptr::null(), extension.as_ptr(), extension_length);
        if extension_ref.is_null() {
            return Ok(Some(Value::Object(None)));
        }
        let uti = UTTypeCreatePreferredIdentifierForTag(
            kUTTagClassFilenameExtension,
            extension_ref,
            std::ptr::null(),
        );
        CFRelease(extension_ref.cast::<std::ffi::c_void>() as CFTypeRef);
        if uti.is_null() {
            return Ok(Some(Value::Object(None)));
        }
        let mime = UTTypeCopyPreferredTagWithClass(uti, kUTTagClassMIMEType);
        CFRelease(uti.cast::<std::ffi::c_void>() as CFTypeRef);
        if mime.is_null() {
            return Ok(Some(Value::Object(None)));
        }
        let cf_length = CFStringGetLength(mime);
        let Ok(length) = usize::try_from(cf_length) else {
            CFRelease(mime.cast::<std::ffi::c_void>() as CFTypeRef);
            return Ok(Some(Value::Object(None)));
        };
        let mut chars = vec![0u16; length];
        if length != 0 {
            CFStringGetCharacters(
                mime,
                core_foundation_sys::base::CFRange::init(0, cf_length),
                chars.as_mut_ptr(),
            );
        }
        CFRelease(mime.cast::<std::ffi::c_void>() as CFTypeRef);
        chars
    };

    let mime_type = String::from_utf16(&mime_type)
        .map_err(|error| ristretto_types::Error::InternalError(error.to_string()))?;
    Ok(Some(mime_type.to_object(&*thread).await?))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ristretto_types::JavaObject;

    #[tokio::test]
    async fn test_probe_0_default_params() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = probe_0(thread, Parameters::default()).await;
        assert!(matches!(
            result,
            Err(ristretto_types::Error::ParametersUnderflow)
        ));
    }

    #[tokio::test]
    async fn test_probe_0_known_extension() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let file_name = "txt".to_object(&*thread).await.expect("to_object");
        let mut params = Parameters::default();
        params.push(file_name);
        let result = probe_0(thread, params).await;
        assert!(result.is_ok());
        let value = result.unwrap();
        assert!(value.is_some());
    }

    #[tokio::test]
    async fn test_probe_0_unknown_extension() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let file_name = "zzzzz".to_object(&*thread).await.expect("to_object");
        let mut params = Parameters::default();
        params.push(file_name);
        let result = probe_0(thread, params).await;
        assert!(result.is_ok());
        let value = result.unwrap();
        // Unknown extension returns null
        assert_eq!(value, Some(Value::Object(None)));
    }
}
