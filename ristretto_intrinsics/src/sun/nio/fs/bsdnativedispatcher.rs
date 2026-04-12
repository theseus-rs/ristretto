use ristretto_classfile::JAVA_21;
use ristretto_classfile::VersionSpecification::{Any, GreaterThanOrEqual};
use ristretto_classloader::{Reference, Value};
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Error::InternalError;
use ristretto_types::Thread;
use ristretto_types::VM;
use ristretto_types::{Parameters, Result};
#[cfg(target_os = "macos")]
use std::ffi::CString;
use std::sync::Arc;

#[cfg(target_os = "macos")]
fn last_errno() -> i32 {
    std::io::Error::last_os_error().raw_os_error().unwrap_or(5)
}

use super::common::throw_unix_exception;

#[cfg(target_os = "macos")]
fn to_cstring(path: &str) -> Result<CString> {
    CString::new(path.as_bytes())
        .map_err(|e| InternalError(format!("Invalid path (contains null byte): {e}")))
}

fn read_native_path<V: VM>(vm: &V, address: i64) -> Result<String> {
    let path_bytes = vm.native_memory().read_cstring(address);
    String::from_utf8(path_bytes)
        .map_err(|error| InternalError(format!("Invalid path encoding: {error}")))
}

/// State for iterating mount entries from `getfsstat`/`fsstat_entry`/`endfsstat`.
#[cfg(target_os = "macos")]
struct FsstatIterator {
    entries: Vec<libc::statfs>,
    index: usize,
}

#[cfg(target_os = "macos")]
#[expect(unsafe_code)]
unsafe extern "C" {
    fn clonefile(
        src: *const libc::c_char,
        dst: *const libc::c_char,
        flags: libc::c_int,
    ) -> libc::c_int;
}

/// Attribute list structure for setattrlist/fsetattrlist on macOS.
#[cfg(target_os = "macos")]
#[repr(C)]
struct AttrList {
    bitmapcount: u16,
    reserved: u16,
    commonattr: u32,
    volattr: u32,
    dirattr: u32,
    fileattr: u32,
    forkattr: u32,
}

#[cfg(target_os = "macos")]
const ATTR_BIT_MAP_COUNT: u16 = 5;
#[cfg(target_os = "macos")]
const ATTR_CMN_MODTIME: u32 = 0x0000_0400;
#[cfg(target_os = "macos")]
const ATTR_CMN_ACCTIME: u32 = 0x0000_1000;
#[cfg(target_os = "macos")]
const ATTR_CMN_CRTIME: u32 = 0x0000_0100;

#[cfg(target_os = "macos")]
#[expect(unsafe_code)]
unsafe extern "C" {
    fn setattrlist(
        path: *const libc::c_char,
        attr_list: *mut libc::c_void,
        attr_buf: *mut libc::c_void,
        attr_buf_size: libc::size_t,
        options: libc::c_ulong,
    ) -> libc::c_int;
    fn fsetattrlist(
        fd: libc::c_int,
        attr_list: *mut libc::c_void,
        attr_buf: *mut libc::c_void,
        attr_buf_size: libc::size_t,
        options: libc::c_ulong,
    ) -> libc::c_int;
}

#[intrinsic_method(
    "sun/nio/fs/BsdNativeDispatcher.clonefile0(JJI)I",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn clonefile_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let flags = parameters.pop_int()?;
    let dst_address = parameters.pop_long()?;
    let src_address = parameters.pop_long()?;

    let vm = thread.vm()?;
    let src_path = read_native_path(&*vm, src_address)?;
    let dst_path = read_native_path(&*vm, dst_address)?;

    #[cfg(target_os = "macos")]
    {
        let c_src = to_cstring(&src_path)?;
        let c_dst = to_cstring(&dst_path)?;
        #[expect(unsafe_code)]
        let result = unsafe { clonefile(c_src.as_ptr(), c_dst.as_ptr(), flags) };
        if result < 0 {
            return Err(throw_unix_exception(&thread, last_errno()).await);
        }
        Ok(Some(Value::Int(0)))
    }
    #[cfg(not(target_os = "macos"))]
    {
        // Fallback: use std::fs::copy as an approximation of clonefile
        let _ = flags;
        if let Err(e) = std::fs::copy(&src_path, &dst_path) {
            let errno = e.raw_os_error().unwrap_or(5);
            return Err(throw_unix_exception(&thread, errno).await);
        }
        Ok(Some(Value::Int(0)))
    }
}

#[intrinsic_method("sun/nio/fs/BsdNativeDispatcher.endfsstat(J)V", Any)]
#[async_method]
pub async fn endfsstat<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let handle = parameters.pop_long()?;

    #[cfg(target_os = "macos")]
    {
        if handle != 0 {
            // SAFETY: handle was created by getfsstat via Box::into_raw
            #[expect(unsafe_code)]
            let _ = unsafe { Box::from_raw(handle as *mut FsstatIterator) };
        }
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = handle;
    }

    Ok(None)
}

#[intrinsic_method(
    "sun/nio/fs/BsdNativeDispatcher.fsetattrlist0(IIJJJJ)V",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
#[cfg_attr(not(target_os = "macos"), expect(clippy::needless_pass_by_value))]
pub async fn fsetattrlist_0<T: Thread + 'static>(
    #[cfg_attr(not(target_os = "macos"), expect(unused_variables))] thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let btime = parameters.pop_long()?;
    let crtime = parameters.pop_long()?;
    let acctime = parameters.pop_long()?;
    let modtime = parameters.pop_long()?;
    let commonattr = parameters.pop_int()?;
    let fd = parameters.pop_int()?;

    #[cfg(target_os = "macos")]
    {
        if let Err(errno) = set_attrlist_by_fd(fd, commonattr, modtime, acctime, crtime, btime) {
            return Err(throw_unix_exception(&thread, errno).await);
        }
        Ok(None)
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = (fd, commonattr, modtime, acctime, crtime, btime);
        Err(InternalError(
            "fsetattrlist0 is not supported on this platform".to_string(),
        ))
    }
}

#[intrinsic_method(
    "sun/nio/fs/BsdNativeDispatcher.fsstatEntry(JLsun/nio/fs/UnixMountEntry;)I",
    Any
)]
#[async_method]
pub async fn fsstat_entry<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let mount_entry = parameters.pop()?;
    let handle = parameters.pop_long()?;

    #[cfg(target_os = "macos")]
    {
        if handle == 0 {
            return Ok(Some(Value::Int(-1)));
        }

        // SAFETY: handle was created by getfsstat via Box::into_raw
        #[expect(unsafe_code)]
        let state = unsafe { &mut *(handle as *mut FsstatIterator) };
        if state.index >= state.entries.len() {
            return Ok(Some(Value::Int(-1)));
        }

        let entry = &state.entries[state.index];
        state.index += 1;

        // Extract mount point name, device name, and fs type from statfs
        #[expect(unsafe_code)]
        let mntonname = unsafe {
            std::ffi::CStr::from_ptr(entry.f_mntonname.as_ptr())
                .to_bytes()
                .to_vec()
        };
        #[expect(unsafe_code)]
        let mntfromname = unsafe {
            std::ffi::CStr::from_ptr(entry.f_mntfromname.as_ptr())
                .to_bytes()
                .to_vec()
        };
        #[expect(unsafe_code)]
        let fstypename = unsafe {
            std::ffi::CStr::from_ptr(entry.f_fstypename.as_ptr())
                .to_bytes()
                .to_vec()
        };

        if !mount_entry.is_null() {
            let vm = thread.vm()?;
            let gc = vm.garbage_collector();

            let mut guard = mount_entry.as_reference_mut()?;
            let Reference::Object(object) = &mut *guard else {
                return Err(InternalError(
                    "fsstatEntry: mount entry is not an object".to_string(),
                ));
            };

            object.set_value("name", Value::new_object(gc, Reference::from(mntfromname)))?;
            object.set_value("dir", Value::new_object(gc, Reference::from(mntonname)))?;
            object.set_value("fstype", Value::new_object(gc, Reference::from(fstypename)))?;
            object.set_value(
                "opts",
                Value::new_object(gc, Reference::from(Vec::<u8>::new())),
            )?;
        }

        Ok(Some(Value::Int(0)))
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = (handle, mount_entry, thread);
        // Return -1 to indicate no more entries
        Ok(Some(Value::Int(-1)))
    }
}

#[intrinsic_method("sun/nio/fs/BsdNativeDispatcher.getfsstat()J", Any)]
#[async_method]
#[cfg_attr(not(target_os = "macos"), expect(clippy::needless_pass_by_value))]
pub async fn getfsstat<T: Thread + 'static>(
    #[cfg_attr(not(target_os = "macos"), expect(unused_variables))] thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    #[cfg(target_os = "macos")]
    {
        let result: std::result::Result<Vec<libc::statfs>, i32> = {
            let mut mntbuf: *mut libc::statfs = std::ptr::null_mut();
            // SAFETY: getmntinfo fills mntbuf with a pointer to a system-managed buffer
            #[expect(unsafe_code)]
            let count =
                unsafe { libc::getmntinfo(std::ptr::addr_of_mut!(mntbuf), libc::MNT_NOWAIT) };
            if count <= 0 {
                Err(last_errno())
            } else {
                // Copy the entries so we own them (the system buffer may be reused)
                match usize::try_from(count) {
                    Ok(count_usize) => {
                        #[expect(unsafe_code)]
                        let entries =
                            unsafe { std::slice::from_raw_parts(mntbuf, count_usize).to_vec() };
                        Ok(entries)
                    }
                    Err(_) => Err(last_errno()),
                }
            }
        };
        match result {
            Ok(entries) => {
                let state = Box::new(FsstatIterator { entries, index: 0 });
                let handle = Box::into_raw(state) as i64;
                Ok(Some(Value::Long(handle)))
            }
            Err(errno) => Err(throw_unix_exception(&thread, errno).await),
        }
    }
    #[cfg(not(target_os = "macos"))]
    {
        // Return 0 as a null handle; fsstat_entry will return -1 immediately
        Ok(Some(Value::Long(0)))
    }
}

#[intrinsic_method("sun/nio/fs/BsdNativeDispatcher.getmntonname0(J)[B", Any)]
#[async_method]
pub async fn getmntonname_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let path_address = parameters.pop_long()?;

    let vm = thread.vm()?;
    let path_str = read_native_path(&*vm, path_address)?;

    #[cfg(target_os = "macos")]
    {
        let c_path = to_cstring(&path_str)?;
        #[expect(unsafe_code)]
        let mut stat_buf: libc::statfs = unsafe { std::mem::zeroed() };
        #[expect(unsafe_code)]
        let result = unsafe { libc::statfs(c_path.as_ptr(), std::ptr::addr_of_mut!(stat_buf)) };
        if result < 0 {
            return Err(throw_unix_exception(&thread, last_errno()).await);
        }
        #[expect(unsafe_code)]
        let mount_name = unsafe {
            std::ffi::CStr::from_ptr(stat_buf.f_mntonname.as_ptr())
                .to_bytes()
                .to_vec()
        };
        let gc = vm.garbage_collector();
        Ok(Some(Value::new_object(gc, Reference::from(mount_name))))
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = path_str;
        // Return "/" as default mount point
        let mount_point: Vec<u8> = b"/".to_vec();
        let gc = vm.garbage_collector();
        Ok(Some(Value::new_object(gc, Reference::from(mount_point))))
    }
}

#[intrinsic_method("sun/nio/fs/BsdNativeDispatcher.initIDs()V", Any)]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "sun/nio/fs/BsdNativeDispatcher.setattrlist0(JIJJJJ)V",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn setattrlist_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let btime = parameters.pop_long()?;
    let crtime = parameters.pop_long()?;
    let acctime = parameters.pop_long()?;
    let modtime = parameters.pop_long()?;
    let commonattr = parameters.pop_int()?;
    let path_address = parameters.pop_long()?;

    let vm = thread.vm()?;
    let path_str = read_native_path(&*vm, path_address)?;

    #[cfg(target_os = "macos")]
    {
        if let Err(errno) =
            set_attrlist_by_path(&path_str, commonattr, modtime, acctime, crtime, btime)
        {
            return Err(throw_unix_exception(&thread, errno).await);
        }
        Ok(None)
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = (path_str, commonattr, modtime, acctime, crtime, btime);
        Err(InternalError(
            "setattrlist0 is not supported on this platform".to_string(),
        ))
    }
}

/// Build the attribute buffer for setattrlist/fsetattrlist from the timestamp parameters.
/// Each timestamp parameter is nanoseconds since epoch. The commonattr bitmask specifies
/// which attributes to include in the buffer.
#[cfg(target_os = "macos")]
fn build_attrlist_buf(
    commonattr: i32,
    modtime: i64,
    acctime: i64,
    crtime: i64,
    _btime: i64,
) -> (AttrList, Vec<libc::timespec>) {
    #[expect(clippy::cast_sign_loss)]
    let mask = commonattr as u32;
    let attrlist = AttrList {
        bitmapcount: ATTR_BIT_MAP_COUNT,
        reserved: 0,
        // Only include the bits we can handle
        commonattr: mask & (ATTR_CMN_CRTIME | ATTR_CMN_MODTIME | ATTR_CMN_ACCTIME),
        volattr: 0,
        dirattr: 0,
        fileattr: 0,
        forkattr: 0,
    };

    // Attributes must be written to the buffer in the order defined by the bitmask
    // (lowest bit first): CRTIME (0x100) < MODTIME (0x400) < ACCTIME (0x1000)
    let mut buf = Vec::new();
    let nanos_to_timespec = |nanos: i64| libc::timespec {
        tv_sec: nanos / 1_000_000_000,
        tv_nsec: nanos % 1_000_000_000,
    };

    if mask & ATTR_CMN_CRTIME != 0 {
        buf.push(nanos_to_timespec(crtime));
    }
    if mask & ATTR_CMN_MODTIME != 0 {
        buf.push(nanos_to_timespec(modtime));
    }
    if mask & ATTR_CMN_ACCTIME != 0 {
        buf.push(nanos_to_timespec(acctime));
    }

    (attrlist, buf)
}

#[cfg(target_os = "macos")]
fn set_attrlist_by_path(
    path: &str,
    commonattr: i32,
    modtime: i64,
    acctime: i64,
    crtime: i64,
    btime: i64,
) -> std::result::Result<(), i32> {
    let c_path = CString::new(path.as_bytes()).map_err(|_| 22)?; // EINVAL
    let (mut attr, buf) = build_attrlist_buf(commonattr, modtime, acctime, crtime, btime);

    if buf.is_empty() {
        return Ok(());
    }

    let buf_size = buf.len() * std::mem::size_of::<libc::timespec>();
    #[expect(unsafe_code)]
    let result = unsafe {
        setattrlist(
            c_path.as_ptr(),
            std::ptr::from_mut(&mut attr).cast::<libc::c_void>(),
            buf.as_ptr() as *mut libc::c_void,
            buf_size,
            0,
        )
    };
    if result < 0 {
        return Err(last_errno());
    }
    Ok(())
}

#[cfg(target_os = "macos")]
fn set_attrlist_by_fd(
    fd: i32,
    commonattr: i32,
    modtime: i64,
    acctime: i64,
    crtime: i64,
    btime: i64,
) -> std::result::Result<(), i32> {
    let (mut attr, buf) = build_attrlist_buf(commonattr, modtime, acctime, crtime, btime);

    if buf.is_empty() {
        return Ok(());
    }

    let buf_size = buf.len() * std::mem::size_of::<libc::timespec>();
    #[expect(unsafe_code)]
    let result = unsafe {
        fsetattrlist(
            fd,
            std::ptr::from_mut(&mut attr).cast::<libc::c_void>(),
            buf.as_ptr() as *mut libc::c_void,
            buf_size,
            0,
        )
    };
    if result < 0 {
        return Err(last_errno());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Write a null-terminated C string into native memory and return its address.
    fn write_cstring_to_native<V: VM>(vm: &V, s: &str) -> i64 {
        let mut bytes = s.as_bytes().to_vec();
        bytes.push(0);
        let addr = vm.native_memory().allocate(bytes.len());
        vm.native_memory().write_bytes(addr, &bytes);
        addr
    }

    #[tokio::test]
    async fn test_clonefile_0_empty_path() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut params = Parameters::default();
        params.push_long(0); // src address
        params.push_long(0); // dst address
        params.push_int(0); // flags
        let result = clonefile_0(thread, params).await;
        assert!(matches!(result, Err(ristretto_types::Error::Throwable(_))));
    }

    #[tokio::test]
    async fn test_clonefile_0_default_params() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = clonefile_0(thread, Parameters::default()).await;
        assert!(matches!(
            result,
            Err(ristretto_types::Error::ParametersUnderflow)
        ));
    }

    #[tokio::test]
    #[cfg(target_family = "unix")]
    async fn test_clonefile_0_nonexistent_path() {
        let (vm, thread) = crate::test::thread().await.expect("thread");
        let src_addr = write_cstring_to_native(&*vm, "/nonexistent_clonefile_src");
        let dst_addr = write_cstring_to_native(&*vm, "/nonexistent_clonefile_dst");
        let mut params = Parameters::default();
        params.push_long(src_addr);
        params.push_long(dst_addr);
        params.push_int(0);
        let result = clonefile_0(thread, params).await;
        assert!(matches!(result, Err(ristretto_types::Error::Throwable(_))));
    }

    #[tokio::test]
    async fn test_endfsstat_null_handle() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut params = Parameters::default();
        params.push_long(0); // null handle
        let result = endfsstat(thread, params).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_endfsstat_default_params() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = endfsstat(thread, Parameters::default()).await;
        assert!(matches!(
            result,
            Err(ristretto_types::Error::ParametersUnderflow)
        ));
    }

    #[tokio::test]
    async fn test_fsetattrlist_0_empty_attrs() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut params = Parameters::default();
        params.push_int(0); // fd
        params.push_int(0); // commonattr = 0 (no attrs to set)
        params.push_long(0); // modtime
        params.push_long(0); // acctime
        params.push_long(0); // crtime
        params.push_long(0); // btime
        let result = fsetattrlist_0(thread, params).await;
        #[cfg(target_os = "macos")]
        assert!(result.is_ok());
        #[cfg(not(target_os = "macos"))]
        assert!(matches!(
            result,
            Err(ristretto_types::Error::InternalError(_))
        ));
    }

    #[tokio::test]
    async fn test_fsetattrlist_0_default_params() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = fsetattrlist_0(thread, Parameters::default()).await;
        assert!(matches!(
            result,
            Err(ristretto_types::Error::ParametersUnderflow)
        ));
    }

    #[tokio::test]
    async fn test_fsstat_entry_null_handle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut params = Parameters::default();
        params.push_long(0); // null handle
        params.push(Value::Object(None)); // null mount entry
        let result = fsstat_entry(thread, params).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(Value::Int(-1)));
    }

    #[tokio::test]
    async fn test_fsstat_entry_default_params() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = fsstat_entry(thread, Parameters::default()).await;
        assert!(matches!(
            result,
            Err(ristretto_types::Error::ParametersUnderflow)
        ));
    }

    #[tokio::test]
    #[cfg(target_os = "macos")]
    async fn test_getfsstat_and_endfsstat() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = getfsstat(thread.clone(), Parameters::default()).await;
        assert!(result.is_ok());
        let value = result.unwrap().expect("should return a value");
        match value {
            Value::Long(handle) => {
                assert_ne!(handle, 0, "should return a valid handle");
                // Clean up
                let mut params = Parameters::default();
                params.push_long(handle);
                let result = endfsstat(thread, params).await;
                assert!(result.is_ok());
            }
            _ => panic!("Expected Long value"),
        }
    }

    #[tokio::test]
    #[cfg(not(target_os = "macos"))]
    async fn test_getfsstat_non_macos() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = getfsstat(thread, Parameters::default()).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(Value::Long(0)));
    }

    #[tokio::test]
    async fn test_getmntonname_0_empty_path() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut params = Parameters::default();
        params.push_long(0); // path address (reads empty string)
        let result = getmntonname_0(thread, params).await;
        #[cfg(not(target_os = "macos"))]
        {
            // Non-macOS returns "/" as default mount point
            assert!(result.is_ok());
        }
        #[cfg(target_os = "macos")]
        {
            // Empty path causes statfs to fail with UnixException
            assert!(matches!(result, Err(ristretto_types::Error::Throwable(_))));
        }
    }

    #[tokio::test]
    async fn test_getmntonname_0_default_params() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = getmntonname_0(thread, Parameters::default()).await;
        assert!(matches!(
            result,
            Err(ristretto_types::Error::ParametersUnderflow)
        ));
    }

    #[tokio::test]
    async fn test_getmntonname_0_success() {
        let (vm, thread) = crate::test::thread().await.expect("thread");
        let path_addr = write_cstring_to_native(&*vm, "/");
        let mut params = Parameters::default();
        params.push_long(path_addr);
        let result = getmntonname_0(thread, params).await;
        assert!(result.is_ok());
        let value = result.unwrap();
        assert!(value.is_some());
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_setattrlist_0_empty_attrs() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut params = Parameters::default();
        params.push_long(0); // path address
        params.push_int(0); // commonattr = 0 (no attrs to set)
        params.push_long(0); // modtime
        params.push_long(0); // acctime
        params.push_long(0); // crtime
        params.push_long(0); // btime
        let result = setattrlist_0(thread, params).await;
        #[cfg(target_os = "macos")]
        assert!(result.is_ok());
        #[cfg(not(target_os = "macos"))]
        assert!(matches!(
            result,
            Err(ristretto_types::Error::InternalError(_))
        ));
    }

    #[tokio::test]
    async fn test_setattrlist_0_default_params() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = setattrlist_0(thread, Parameters::default()).await;
        assert!(matches!(
            result,
            Err(ristretto_types::Error::ParametersUnderflow)
        ));
    }
}
