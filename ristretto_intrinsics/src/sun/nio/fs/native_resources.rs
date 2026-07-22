//! Per-VM ownership for libc resources represented as opaque Java `long` values.

use ristretto_types::{Result, VM};
use std::collections::HashMap;
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::{Arc, Mutex};

const FIRST_HANDLE: i64 = 0x6000_0000_0000_0000;

#[derive(Debug)]
struct NativeResources {
    next_handle: AtomicI64,
    files: Mutex<HashMap<i64, usize>>,
    directories: Mutex<HashMap<i64, usize>>,
}

impl NativeResources {
    fn new() -> Self {
        Self {
            next_handle: AtomicI64::new(FIRST_HANDLE),
            files: Mutex::new(HashMap::new()),
            directories: Mutex::new(HashMap::new()),
        }
    }

    fn next_handle(&self) -> i64 {
        self.next_handle.fetch_add(1, Ordering::Relaxed)
    }
}

impl Drop for NativeResources {
    #[expect(unsafe_code)]
    fn drop(&mut self) {
        if let Ok(files) = self.files.get_mut() {
            for pointer in files.drain().map(|(_, pointer)| pointer) {
                unsafe {
                    libc::fclose(pointer as *mut libc::FILE);
                }
            }
        }
        if let Ok(directories) = self.directories.get_mut() {
            for pointer in directories.drain().map(|(_, pointer)| pointer) {
                unsafe {
                    libc::closedir(pointer as *mut libc::DIR);
                }
            }
        }
    }
}

fn resources<V: VM + ?Sized>(vm: &V) -> Result<Arc<NativeResources>> {
    vm.resource_manager().get_or_init(NativeResources::new)
}

pub(crate) fn insert_file<V: VM + ?Sized>(vm: &V, pointer: *mut libc::FILE) -> Result<i64> {
    let resources = match resources(vm) {
        Ok(resources) => resources,
        Err(error) => {
            #[expect(unsafe_code)]
            unsafe {
                libc::fclose(pointer);
            }
            return Err(error);
        }
    };
    let handle = resources.next_handle();
    let Ok(mut files) = resources.files.lock() else {
        #[expect(unsafe_code)]
        unsafe {
            libc::fclose(pointer);
        }
        return Err(ristretto_types::Error::InternalError(
            "native file lock poisoned".into(),
        ));
    };
    files.insert(handle, pointer as usize);
    Ok(handle)
}

pub(crate) fn with_file<V, F, R>(vm: &V, handle: i64, operation: F) -> Result<Option<R>>
where
    V: VM + ?Sized,
    F: FnOnce(*mut libc::FILE) -> R,
{
    let resources = resources(vm)?;
    let files = resources
        .files
        .lock()
        .map_err(|_| ristretto_types::Error::InternalError("native file lock poisoned".into()))?;
    Ok(files
        .get(&handle)
        .map(|pointer| operation(*pointer as *mut libc::FILE)))
}

pub(crate) fn take_file<V: VM + ?Sized>(vm: &V, handle: i64) -> Result<Option<usize>> {
    let resources = resources(vm)?;
    Ok(resources
        .files
        .lock()
        .map_err(|_| ristretto_types::Error::InternalError("native file lock poisoned".into()))?
        .remove(&handle))
}

pub(crate) fn insert_directory<V: VM + ?Sized>(vm: &V, pointer: *mut libc::DIR) -> Result<i64> {
    let resources = match resources(vm) {
        Ok(resources) => resources,
        Err(error) => {
            #[expect(unsafe_code)]
            unsafe {
                libc::closedir(pointer);
            }
            return Err(error);
        }
    };
    let handle = resources.next_handle();
    let Ok(mut directories) = resources.directories.lock() else {
        #[expect(unsafe_code)]
        unsafe {
            libc::closedir(pointer);
        }
        return Err(ristretto_types::Error::InternalError(
            "native directory lock poisoned".into(),
        ));
    };
    directories.insert(handle, pointer as usize);
    Ok(handle)
}

pub(crate) fn with_directory<V, F, R>(vm: &V, handle: i64, operation: F) -> Result<Option<R>>
where
    V: VM + ?Sized,
    F: FnOnce(*mut libc::DIR) -> R,
{
    let resources = resources(vm)?;
    let directories = resources.directories.lock().map_err(|_| {
        ristretto_types::Error::InternalError("native directory lock poisoned".into())
    })?;
    Ok(directories
        .get(&handle)
        .map(|pointer| operation(*pointer as *mut libc::DIR)))
}

pub(crate) fn take_directory<V: VM + ?Sized>(vm: &V, handle: i64) -> Result<Option<usize>> {
    let resources = resources(vm)?;
    Ok(resources
        .directories
        .lock()
        .map_err(|_| {
            ristretto_types::Error::InternalError("native directory lock poisoned".into())
        })?
        .remove(&handle))
}
