use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::Equal;
use ristretto_classloader::{Reference, Value};
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result, VM};
use std::ffi::{CStr, CString, c_char, c_void};
use std::sync::{Arc, Mutex};

#[derive(Debug)]
struct GioLibrary(Mutex<Option<libloading::Library>>);

#[derive(Debug)]
struct GnomeVfsLibrary(Mutex<Option<libloading::Library>>);

type GioNewFile = unsafe extern "C" fn(*const c_char) -> *mut c_void;
type GioQueryInfo = unsafe extern "C" fn(
    *mut c_void,
    *const c_char,
    i32,
    *mut c_void,
    *mut *mut c_void,
) -> *mut c_void;
type GioGetContentType = unsafe extern "C" fn(*mut c_void) -> *const c_char;
type GObjectUnref = unsafe extern "C" fn(*mut c_void);
type GnomeVfsProbe = unsafe extern "C" fn(*const c_char) -> *const c_char;

fn load_library(candidates: &[&str], required_symbols: &[&[u8]]) -> Option<libloading::Library> {
    for candidate in candidates {
        #[expect(unsafe_code)]
        let Ok(library) = (unsafe { libloading::Library::new(*candidate) }) else {
            continue;
        };
        #[expect(unsafe_code)]
        let valid = required_symbols
            .iter()
            .all(|symbol| unsafe { library.get::<*const c_void>(*symbol).is_ok() });
        if valid {
            return Some(library);
        }
    }
    None
}

fn byte_array<T: Thread>(thread: &Arc<T>, bytes: Vec<u8>) -> Result<Value> {
    Ok(Value::new_object(
        thread.vm()?.garbage_collector(),
        Reference::from(bytes),
    ))
}

#[intrinsic_method("sun/nio/fs/GnomeFileTypeDetector.initializeGio()Z", Equal(JAVA_8))]
#[async_method]
pub async fn initialize_gio<T: Thread + 'static>(
    thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    let vm = thread.vm()?;
    let state = vm
        .resource_manager()
        .get_or_init(|| GioLibrary(Mutex::new(None)))?;
    let mut guard = state
        .0
        .lock()
        .map_err(|_| ristretto_types::Error::InternalError("GIO lock poisoned".into()))?;
    if guard.is_none() {
        *guard = load_library(
            &["libgio-2.0.so", "libgio-2.0.so.0"],
            &[
                b"g_object_unref\0",
                b"g_file_new_for_path\0",
                b"g_file_query_info\0",
                b"g_file_info_get_content_type\0",
            ],
        );
    }
    Ok(Some(Value::from(guard.is_some())))
}
#[intrinsic_method(
    "sun/nio/fs/GnomeFileTypeDetector.initializeGnomeVfs()Z",
    Equal(JAVA_8)
)]
#[async_method]
pub async fn initialize_gnome_vfs<T: Thread + 'static>(
    thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    let vm = thread.vm()?;
    let state = vm
        .resource_manager()
        .get_or_init(|| GnomeVfsLibrary(Mutex::new(None)))?;
    let mut guard = state
        .0
        .lock()
        .map_err(|_| ristretto_types::Error::InternalError("GNOME VFS lock poisoned".into()))?;
    if guard.is_none() {
        *guard = load_library(
            &["libgnomevfs-2.so", "libgnomevfs-2.so.0"],
            &[b"gnome_vfs_init\0", b"gnome_vfs_mime_type_from_name\0"],
        );
        if let Some(library) = guard.as_ref() {
            type Init = unsafe extern "C" fn() -> i32;
            #[expect(unsafe_code)]
            let initialized = unsafe {
                library
                    .get::<Init>(b"gnome_vfs_init\0")
                    .is_ok_and(|initialize| initialize() != 0)
            };
            if !initialized {
                *guard = None;
            }
        }
    }
    Ok(Some(Value::from(guard.is_some())))
}
#[intrinsic_method("sun/nio/fs/GnomeFileTypeDetector.probeUsingGio(J)[B", Equal(JAVA_8))]
#[async_method]
pub async fn probe_using_gio<T: Thread + 'static>(
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
        .get_or_init(|| GioLibrary(Mutex::new(None)))?;
    let mime = {
        let guard = state
            .0
            .lock()
            .map_err(|_| ristretto_types::Error::InternalError("GIO lock poisoned".into()))?;
        let Some(library) = guard.as_ref() else {
            return Ok(Some(Value::Object(None)));
        };
        #[expect(unsafe_code)]
        unsafe {
            let Ok(new_file) = library.get::<GioNewFile>(b"g_file_new_for_path\0") else {
                return Ok(Some(Value::Object(None)));
            };
            let Ok(query_info) = library.get::<GioQueryInfo>(b"g_file_query_info\0") else {
                return Ok(Some(Value::Object(None)));
            };
            let Ok(get_content_type) =
                library.get::<GioGetContentType>(b"g_file_info_get_content_type\0")
            else {
                return Ok(Some(Value::Object(None)));
            };
            let Ok(unref) = library.get::<GObjectUnref>(b"g_object_unref\0") else {
                return Ok(Some(Value::Object(None)));
            };
            let file = new_file(path.as_ptr());
            if file.is_null() {
                None
            } else {
                let attributes = c"standard::content-type";
                let info = query_info(
                    file,
                    attributes.as_ptr(),
                    0,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                );
                let mime = if info.is_null() {
                    None
                } else {
                    let pointer = get_content_type(info);
                    let mime =
                        (!pointer.is_null()).then(|| CStr::from_ptr(pointer).to_bytes().to_vec());
                    unref(info);
                    mime
                };
                unref(file);
                mime
            }
        }
    };
    match mime {
        Some(mime) => Ok(Some(byte_array(&thread, mime)?)),
        None => Ok(Some(Value::Object(None))),
    }
}
#[intrinsic_method(
    "sun/nio/fs/GnomeFileTypeDetector.probeUsingGnomeVfs(J)[B",
    Equal(JAVA_8)
)]
#[async_method]
pub async fn probe_using_gnome_vfs<T: Thread + 'static>(
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
        .get_or_init(|| GnomeVfsLibrary(Mutex::new(None)))?;
    let mime = {
        let guard = state
            .0
            .lock()
            .map_err(|_| ristretto_types::Error::InternalError("GNOME VFS lock poisoned".into()))?;
        let Some(library) = guard.as_ref() else {
            return Ok(Some(Value::Object(None)));
        };
        #[expect(unsafe_code)]
        unsafe {
            let Ok(probe) = library.get::<GnomeVfsProbe>(b"gnome_vfs_mime_type_from_name\0") else {
                return Ok(Some(Value::Object(None)));
            };
            let pointer = probe(path.as_ptr());
            (!pointer.is_null()).then(|| CStr::from_ptr(pointer).to_bytes().to_vec())
        }
    };
    match mime {
        Some(mime) => Ok(Some(byte_array(&thread, mime)?)),
        None => Ok(Some(Value::Object(None))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_initialize_gio() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = initialize_gio(thread, Parameters::default())
            .await
            .expect("initialize");
        assert!(matches!(result, Some(Value::Int(0 | 1))));
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_initialize_gnome_vfs() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = initialize_gnome_vfs(thread, Parameters::default())
            .await
            .expect("initialize");
        assert!(matches!(result, Some(Value::Int(0 | 1))));
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_probe_using_gio() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = probe_using_gio(thread, Parameters::new(vec![Value::Long(0)]))
            .await
            .expect("probe");
        assert_eq!(result, Some(Value::Object(None)));
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_probe_using_gnome_vfs() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = probe_using_gnome_vfs(thread, Parameters::new(vec![Value::Long(0)]))
            .await
            .expect("probe");
        assert_eq!(result, Some(Value::Object(None)));
    }
}
