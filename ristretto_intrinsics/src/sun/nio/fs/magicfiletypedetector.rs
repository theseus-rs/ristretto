use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::Equal;
use ristretto_classloader::{Reference, Value};
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result, VM};
use std::ffi::{CStr, CString, c_char, c_int, c_void};
use std::sync::{Arc, Mutex};

const MAGIC_MIME_TYPE: c_int = 0x10;

#[derive(Debug)]
struct MagicLibrary(Mutex<Option<libloading::Library>>);

type MagicOpen = unsafe extern "C" fn(c_int) -> *mut c_void;
type MagicLoad = unsafe extern "C" fn(*mut c_void, *const c_char) -> c_int;
type MagicFile = unsafe extern "C" fn(*mut c_void, *const c_char) -> *const c_char;
type MagicClose = unsafe extern "C" fn(*mut c_void);

fn load_magic() -> Option<libloading::Library> {
    for candidate in ["libmagic.so", "libmagic.so.1"] {
        #[expect(unsafe_code)]
        let Ok(library) = (unsafe { libloading::Library::new(candidate) }) else {
            continue;
        };
        #[expect(unsafe_code)]
        let valid = [
            b"magic_open\0".as_slice(),
            b"magic_load\0".as_slice(),
            b"magic_file\0".as_slice(),
            b"magic_close\0".as_slice(),
        ]
        .iter()
        .all(|symbol| unsafe { library.get::<*const c_void>(*symbol).is_ok() });
        if valid {
            return Some(library);
        }
    }
    None
}

#[intrinsic_method("sun/nio/fs/MagicFileTypeDetector.initialize0()Z", Equal(JAVA_8))]
#[async_method]
pub async fn initialize0<T: Thread + 'static>(
    thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    let vm = thread.vm()?;
    let state = vm
        .resource_manager()
        .get_or_init(|| MagicLibrary(Mutex::new(None)))?;
    let mut guard = state
        .0
        .lock()
        .map_err(|_| ristretto_types::Error::InternalError("libmagic lock poisoned".into()))?;
    if guard.is_none() {
        *guard = load_magic();
    }
    Ok(Some(Value::from(guard.is_some())))
}
#[intrinsic_method("sun/nio/fs/MagicFileTypeDetector.probe0(J)[B", Equal(JAVA_8))]
#[async_method]
pub async fn probe0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let path_address = parameters.pop_long()?;
    let vm = thread.vm()?;
    let Some(path) = vm.native_memory().try_read_cstring(path_address) else {
        return Ok(Some(Value::Object(None)));
    };
    let Ok(path) = CString::new(path) else {
        return Ok(Some(Value::Object(None)));
    };
    let state = vm
        .resource_manager()
        .get_or_init(|| MagicLibrary(Mutex::new(None)))?;
    let mime = {
        let guard = state
            .0
            .lock()
            .map_err(|_| ristretto_types::Error::InternalError("libmagic lock poisoned".into()))?;
        let Some(library) = guard.as_ref() else {
            return Ok(Some(Value::Object(None)));
        };
        #[expect(unsafe_code)]
        unsafe {
            let Ok(open) = library.get::<MagicOpen>(b"magic_open\0") else {
                return Ok(Some(Value::Object(None)));
            };
            let Ok(load) = library.get::<MagicLoad>(b"magic_load\0") else {
                return Ok(Some(Value::Object(None)));
            };
            let Ok(file) = library.get::<MagicFile>(b"magic_file\0") else {
                return Ok(Some(Value::Object(None)));
            };
            let Ok(close) = library.get::<MagicClose>(b"magic_close\0") else {
                return Ok(Some(Value::Object(None)));
            };
            let cookie = open(MAGIC_MIME_TYPE);
            if cookie.is_null() {
                None
            } else {
                let mime = if load(cookie, std::ptr::null()) == -1 {
                    None
                } else {
                    let pointer = file(cookie, path.as_ptr());
                    (!pointer.is_null()).then(|| CStr::from_ptr(pointer).to_bytes().to_vec())
                };
                close(cookie);
                mime
            }
        }
    };
    let Some(mime) = mime else {
        return Ok(Some(Value::Object(None)));
    };
    Ok(Some(Value::new_object(
        vm.garbage_collector(),
        Reference::from(mime),
    )))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_initialize0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = initialize0(thread, Parameters::default())
            .await
            .expect("initialize");
        assert!(matches!(result, Some(Value::Int(0 | 1))));
    }

    #[tokio::test]
    async fn test_probe0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = probe0(thread, Parameters::new(vec![Value::Long(0)]))
            .await
            .expect("probe");
        assert_eq!(result, Some(Value::Object(None)));
    }
}
