use crate::native_methods::registry::{MethodRegistry, JAVA_11, JAVA_17, JAVA_21, JAVA_23, JAVA_8};
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Error::InternalError;
use crate::JavaError::NullPointerException;
use crate::Result;
use async_recursion::async_recursion;
use bitflags::bitflags;
use ristretto_classloader::{ConcurrentVec, Reference, Value};
use std::sync::Arc;

const CLASS_NAME: &str = "sun/nio/fs/UnixNativeDispatcher";

bitflags! {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct SupportsFlags: i32 {
        /// Supports openat and other *at calls.
        const OPENAT = 1 << 1;  // syscalls
        /// Supports futimes or futimesat
        const FUTIMES = 1 << 2;
        /// Supports futimens
        const FUTIMENS = 1 << 3;
        /// Supports lutimes
        const LUTIMES = 1 << 4;
        /// Supports extended attributes
        const XATTR = 1 << 5;
        /// Supports file birth (creation) time attribute
        const BIRTHTIME = 1 << 16; // other features
    }
}

/// Register all native methods for `sun.nio.fs.UnixNativeDispatcher`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() <= JAVA_8 {
        registry.register(CLASS_NAME, "close", "(I)V", fclose);
    }

    if registry.java_major_version() <= JAVA_11 {
        registry.register(CLASS_NAME, "fclose", "(J)V", fclose);
        registry.register(CLASS_NAME, "fopen0", "(JJ)J", fopen_0);
        registry.register(CLASS_NAME, "fpathconf", "(II)J", fpathconf);
        registry.register(CLASS_NAME, "pathconf0", "(JI)J", pathconf_0);
    }

    if registry.java_major_version() >= JAVA_11 {
        if registry.java_major_version() <= JAVA_17 {
            registry.register(CLASS_NAME, "exists0", "(J)Z", exists_0);
            registry.register(CLASS_NAME, "stat1", "(J)I", stat_1);
        }

        registry.register(CLASS_NAME, "close0", "(I)V", close_0);
        registry.register(CLASS_NAME, "getlinelen", "(J)I", getlinelen);
    }

    if registry.java_major_version() == JAVA_17 {
        registry.register(CLASS_NAME, "futimens", "(IJJ)V", futimens);
    }
    if registry.java_major_version() >= JAVA_17 {
        registry.register(CLASS_NAME, "fgetxattr0", "(IJJI)I", fgetxattr_0);
        registry.register(CLASS_NAME, "flistxattr", "(IJI)I", flistxattr);
        registry.register(CLASS_NAME, "fremovexattr0", "(IJ)V", fremovexattr_0);
        registry.register(CLASS_NAME, "fsetxattr0", "(IJJI)V", fsetxattr_0);
        registry.register(CLASS_NAME, "lutimes0", "(JJJ)V", lutimes_0);
    }

    if registry.java_major_version() <= JAVA_17 {
        registry.register(CLASS_NAME, "fchmod", "(II)V", fchmod);
        registry.register(CLASS_NAME, "fchown", "(III)V", fchown);
        registry.register(
            CLASS_NAME,
            "fstat",
            "(ILsun/nio/fs/UnixFileAttributes;)V",
            fstat,
        );
        registry.register(CLASS_NAME, "futimes", "(IJJ)V", futimes);
        registry.register(CLASS_NAME, "read", "(IJI)I", read);
        registry.register(CLASS_NAME, "readdir", "(J)[B", readdir);
        registry.register(
            CLASS_NAME,
            "stat0",
            "(JLsun/nio/fs/UnixFileAttributes;)V",
            stat_0,
        );
        registry.register(CLASS_NAME, "write", "(IJI)I", write);
    } else {
        registry.register(
            CLASS_NAME,
            "stat0",
            "(JLsun/nio/fs/UnixFileAttributes;)I",
            stat_0,
        );
    }

    if registry.java_major_version() >= JAVA_21 {
        registry.register(CLASS_NAME, "fchmod0", "(II)V", fchmod_0);
        registry.register(CLASS_NAME, "fchown0", "(III)V", fchown_0);
        registry.register(
            CLASS_NAME,
            "fstat0",
            "(ILsun/nio/fs/UnixFileAttributes;)V",
            fstat_0,
        );
        registry.register(CLASS_NAME, "futimens0", "(IJJ)V", futimens_0);
        registry.register(CLASS_NAME, "futimes0", "(IJJ)V", futimes_0);
        registry.register(CLASS_NAME, "read0", "(IJI)I", read_0);
        registry.register(CLASS_NAME, "readdir0", "(J)[B", readdir_0);
        registry.register(CLASS_NAME, "write0", "(IJI)I", write_0);
    }

    if registry.java_major_version() == JAVA_21 || registry.java_major_version() >= JAVA_23 {
        registry.register(CLASS_NAME, "access0", "(JI)I", access_0);
    } else {
        registry.register(CLASS_NAME, "access0", "(JI)V", access_0);
    }

    registry.register(CLASS_NAME, "chmod0", "(JI)V", chmod_0);
    registry.register(CLASS_NAME, "chown0", "(JII)V", chown_0);
    registry.register(CLASS_NAME, "closedir", "(J)V", closedir);
    registry.register(CLASS_NAME, "dup", "(I)I", dup);
    registry.register(CLASS_NAME, "fdopendir", "(I)J", fdopendir);
    registry.register(
        CLASS_NAME,
        "fstatat0",
        "(IJILsun/nio/fs/UnixFileAttributes;)V",
        fstatat_0,
    );
    registry.register(CLASS_NAME, "getcwd", "()[B", getcwd);
    registry.register(CLASS_NAME, "getgrgid", "(I)[B", getgrgid);
    registry.register(CLASS_NAME, "getgrnam0", "(J)I", getgrnam_0);
    registry.register(CLASS_NAME, "getpwnam0", "(J)I", getpwnam_0);
    registry.register(CLASS_NAME, "getpwuid", "(I)[B", getpwuid);
    registry.register(CLASS_NAME, "init", "()I", init);
    registry.register(CLASS_NAME, "lchown0", "(JII)V", lchown_0);
    registry.register(CLASS_NAME, "link0", "(JJ)V", link_0);
    registry.register(
        CLASS_NAME,
        "lstat0",
        "(JLsun/nio/fs/UnixFileAttributes;)V",
        lstat_0,
    );
    registry.register(CLASS_NAME, "mkdir0", "(JI)V", mkdir_0);
    registry.register(CLASS_NAME, "mknod0", "(JIJ)V", mknod_0);
    registry.register(CLASS_NAME, "open0", "(JII)I", open_0);
    registry.register(CLASS_NAME, "openat0", "(IJII)I", openat_0);
    registry.register(CLASS_NAME, "opendir0", "(J)J", opendir_0);
    registry.register(CLASS_NAME, "readlink0", "(J)[B", readlink_0);
    registry.register(CLASS_NAME, "realpath0", "(J)[B", realpath_0);
    registry.register(CLASS_NAME, "rename0", "(JJ)V", rename_0);
    registry.register(CLASS_NAME, "renameat0", "(IJIJ)V", renameat_0);
    registry.register(CLASS_NAME, "rewind", "(J)V", rewind);
    registry.register(CLASS_NAME, "rmdir0", "(J)V", rmdir_0);

    registry.register(
        CLASS_NAME,
        "statvfs0",
        "(JLsun/nio/fs/UnixFileStoreAttributes;)V",
        statvfs_0,
    );
    registry.register(CLASS_NAME, "strerror", "(I)[B", strerror);
    registry.register(CLASS_NAME, "symlink0", "(JJ)V", symlink_0);
    registry.register(CLASS_NAME, "unlink0", "(J)V", unlink_0);
    registry.register(CLASS_NAME, "unlinkat0", "(IJI)V", unlinkat_0);
    registry.register(CLASS_NAME, "utimes0", "(JJJ)V", utimes_0);
}

#[async_recursion(?Send)]
async fn access_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.access0(JI)I");
}

#[async_recursion(?Send)]
async fn chmod_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.chmod0(JI)V");
}

#[async_recursion(?Send)]
async fn chown_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.chown0(JII)V");
}

#[async_recursion(?Send)]
async fn close(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    close_0(thread, parameters).await
}

#[async_recursion(?Send)]
async fn close_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.close0(I)V");
}

#[async_recursion(?Send)]
async fn closedir(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.closedir(J)V");
}

#[async_recursion(?Send)]
async fn dup(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.dup(I)I");
}

#[async_recursion(?Send)]
async fn exists_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.exists0(J)Z");
}

#[async_recursion(?Send)]
async fn fclose(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.fclose(J)V");
}

#[async_recursion(?Send)]
async fn fchmod(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    fchmod_0(thread, parameters).await
}

#[async_recursion(?Send)]
async fn fchmod_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.fchmod0(II)V");
}

#[async_recursion(?Send)]
async fn fchown(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    fchown_0(thread, parameters).await
}

#[async_recursion(?Send)]
async fn fchown_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.fchown0(III)V");
}

#[async_recursion(?Send)]
async fn fdopendir(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.fdopendir(I)J");
}

#[async_recursion(?Send)]
async fn fgetxattr_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.fgetxattr0(IJJI)I");
}

#[async_recursion(?Send)]
async fn flistxattr(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.flistxattr(IJI)I");
}

#[async_recursion(?Send)]
async fn fopen_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.fopen0(JJ)J");
}

#[async_recursion(?Send)]
async fn fpathconf(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.fpathconf(II)J");
}

#[async_recursion(?Send)]
async fn fremovexattr_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.fremovexattr0(IJ)V");
}

#[async_recursion(?Send)]
async fn fsetxattr_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.fsetxattr0(IJJI)V");
}

#[async_recursion(?Send)]
async fn fstat(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    fstat_0(thread, parameters).await
}

#[async_recursion(?Send)]
async fn fstat_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.fstat0(ILsun/nio/fs/UnixFileAttributes;)V");
}

#[async_recursion(?Send)]
async fn fstatat_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.fstatat0(IJILsun/nio/fs/UnixFileAttributes;)V");
}

#[async_recursion(?Send)]
async fn futimens(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    futimens_0(thread, parameters).await
}

#[async_recursion(?Send)]
async fn futimens_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.futimens0(IJJ)V");
}

#[async_recursion(?Send)]
async fn futimes(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    futimes_0(thread, parameters).await
}

#[async_recursion(?Send)]
async fn futimes_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.futimes0(IJJ)V");
}

#[expect(clippy::cast_possible_wrap)]
#[async_recursion(?Send)]
async fn getcwd(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    let current_dir_path =
        std::env::current_dir().map_err(|error| InternalError(format!("getcwd: {error}")))?;
    let current_dir_str = current_dir_path.to_string_lossy();
    let current_dir = current_dir_str
        .as_bytes()
        .to_vec()
        .iter()
        .map(|&b| b as i8)
        .collect();
    let current_dir_vec = ConcurrentVec::from(current_dir);
    let current_dir_bytes = Reference::ByteArray(current_dir_vec);
    Ok(Some(Value::Object(Some(current_dir_bytes))))
}

#[async_recursion(?Send)]
async fn getgrgid(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.getgrgid(I)[B");
}

#[async_recursion(?Send)]
async fn getgrnam_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.getgrnam0(J)I");
}

#[async_recursion(?Send)]
async fn getlinelen(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.getlinelen(J)I");
}

#[async_recursion(?Send)]
async fn getpwnam_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.getpwnam0(J)I");
}

#[async_recursion(?Send)]
async fn getpwuid(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.getpwuid(I)[B");
}

#[async_recursion(?Send)]
async fn init(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    let capabilities = SupportsFlags::empty();
    // TODO: Implement the capabilities check
    let capabilities = capabilities.bits();
    Ok(Some(Value::Int(capabilities)))
}

#[async_recursion(?Send)]
async fn lchown_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.lchown0(JII)V");
}

#[async_recursion(?Send)]
async fn link_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.link0(JJ)V");
}

#[async_recursion(?Send)]
async fn lstat_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.lstat0(JLsun/nio/fs/UnixFileAttributes;)V");
}

#[async_recursion(?Send)]
async fn lutimes_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.lutimes0(JJJ)V");
}

#[async_recursion(?Send)]
async fn mkdir_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.mkdir0(JI)V");
}

#[async_recursion(?Send)]
async fn mknod_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.mknod0(JIJ)V");
}

#[async_recursion(?Send)]
async fn open_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.open0(JII)I");
}

#[async_recursion(?Send)]
async fn openat_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.openat0(IJII)I");
}

#[async_recursion(?Send)]
async fn opendir_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.opendir0(J)J");
}

#[async_recursion(?Send)]
async fn pathconf_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.pathconf0(JI)J");
}

#[async_recursion(?Send)]
async fn read(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    read_0(thread, parameters).await
}

#[async_recursion(?Send)]
async fn read_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.read0(IJI)I");
}

#[async_recursion(?Send)]
async fn readdir(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    readdir_0(thread, parameters).await
}

#[async_recursion(?Send)]
async fn readdir_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.readdir0(J)[B");
}

#[async_recursion(?Send)]
async fn readlink_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.readlink0(J)[B");
}

#[async_recursion(?Send)]
async fn realpath_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.realpath0(J)[B");
}

#[async_recursion(?Send)]
async fn rename_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.rename0(JJ)V");
}

#[async_recursion(?Send)]
async fn renameat_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.renameat0(IJIJ)V");
}

#[async_recursion(?Send)]
async fn rewind(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.rewind(J)V");
}

#[async_recursion(?Send)]
async fn rmdir_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.rmdir0(J)V");
}

#[async_recursion(?Send)]
async fn stat_0(thread: Arc<Thread>, mut parameters: Parameters) -> Result<Option<Value>> {
    let vm = thread.vm()?;
    let Ok(_attributes) = parameters.pop_object() else {
        return Err(NullPointerException("attributes is null".to_string()).into());
    };
    let _path = parameters.pop_long()?;
    // TODO: Implement the stat0 method

    if vm.java_major_version() <= JAVA_17 {
        Ok(None)
    } else {
        Ok(Some(Value::Int(0)))
    }
}

#[async_recursion(?Send)]
async fn stat_1(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.stat1(J)I");
}

#[async_recursion(?Send)]
async fn statvfs_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.statvfs0(JLsun/nio/fs/UnixFileStoreAttributes;)V");
}

#[async_recursion(?Send)]
async fn strerror(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.strerror(I)[B");
}

#[async_recursion(?Send)]
async fn symlink_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.symlink0(JJ)V");
}

#[async_recursion(?Send)]
async fn unlink_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.unlink0(J)V");
}

#[async_recursion(?Send)]
async fn unlinkat_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.unlinkat0(IJI)V");
}

#[async_recursion(?Send)]
async fn utimes_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.utimes0(JJJ)V");
}

#[async_recursion(?Send)]
async fn write(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    write_0(thread, parameters).await
}

#[async_recursion(?Send)]
async fn write_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.write0(IJI)I");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.fs.UnixNativeDispatcher.access0(JI)I")]
    async fn test_access_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = access_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.fs.UnixNativeDispatcher.chmod0(JI)V")]
    async fn test_chmod_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = chmod_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.fs.UnixNativeDispatcher.chown0(JII)V")]
    async fn test_chown_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = chown_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.fs.UnixNativeDispatcher.close0(I)V")]
    async fn test_close() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = close(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.fs.UnixNativeDispatcher.close0(I)V")]
    async fn test_close_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = close_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.fs.UnixNativeDispatcher.closedir(J)V")]
    async fn test_closedir() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = closedir(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.fs.UnixNativeDispatcher.dup(I)I")]
    async fn test_dup() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = dup(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.fs.UnixNativeDispatcher.exists0(J)Z")]
    async fn test_exists_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = exists_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.fs.UnixNativeDispatcher.fchmod0(II)V")]
    async fn test_fchmod() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = fchmod(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.fs.UnixNativeDispatcher.fchmod0(II)V")]
    async fn test_fchmod_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = fchmod_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.fs.UnixNativeDispatcher.fchown0(III)V")]
    async fn test_fchown() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = fchown(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.fs.UnixNativeDispatcher.fchown0(III)V")]
    async fn test_fchown_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = fchown_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.fs.UnixNativeDispatcher.fclose(J)V")]
    async fn test_fclose() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = fclose(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.fs.UnixNativeDispatcher.fdopendir(I)J")]
    async fn test_fdopendir() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = fdopendir(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.fs.UnixNativeDispatcher.fgetxattr0(IJJI)I"
    )]
    async fn test_fgetxattr_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = fgetxattr_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.fs.UnixNativeDispatcher.flistxattr(IJI)I"
    )]
    async fn test_flistxattr() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = flistxattr(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.fs.UnixNativeDispatcher.fopen0(JJ)J")]
    async fn test_fopen_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = fopen_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.fs.UnixNativeDispatcher.fpathconf(II)J"
    )]
    async fn test_fpathconf() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = fpathconf(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.fs.UnixNativeDispatcher.fremovexattr0(IJ)V"
    )]
    async fn test_fremovexattr_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = fremovexattr_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.fs.UnixNativeDispatcher.fsetxattr0(IJJI)V"
    )]
    async fn test_fsetxattr_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = fsetxattr_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.fs.UnixNativeDispatcher.fstat0(ILsun/nio/fs/UnixFileAttributes;)V"
    )]
    async fn test_fstat() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = fstat(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.fs.UnixNativeDispatcher.fstat0(ILsun/nio/fs/UnixFileAttributes;)V"
    )]
    async fn test_fstat_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = fstat_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.fs.UnixNativeDispatcher.fstatat0(IJILsun/nio/fs/UnixFileAttributes;)V"
    )]
    async fn test_fstatat_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = fstatat_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.fs.UnixNativeDispatcher.futimens0(IJJ)V"
    )]
    async fn test_futimens() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = futimens(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.fs.UnixNativeDispatcher.futimens0(IJJ)V"
    )]
    async fn test_futimens_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = futimens_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.fs.UnixNativeDispatcher.futimes0(IJJ)V"
    )]
    async fn test_futimes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = futimes(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.fs.UnixNativeDispatcher.futimes0(IJJ)V"
    )]
    async fn test_futimes_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = futimes_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_getcwd() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = getcwd(thread, Parameters::default()).await?;
        let bytes: Vec<u8> = result.expect("cwd").try_into()?;
        let cwd = String::from_utf8_lossy(&bytes);
        let current_dir_path =
            std::env::current_dir().map_err(|error| InternalError(format!("getcwd: {error}")))?;
        let expected = current_dir_path.to_string_lossy();
        assert_eq!(cwd, expected);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.fs.UnixNativeDispatcher.getgrgid(I)[B")]
    async fn test_getgrgid() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = getgrgid(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.fs.UnixNativeDispatcher.getgrnam0(J)I")]
    async fn test_getgrnam_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = getgrnam_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.fs.UnixNativeDispatcher.getlinelen(J)I"
    )]
    async fn test_getlinelen() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = getlinelen(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.fs.UnixNativeDispatcher.getpwnam0(J)I")]
    async fn test_getpwnam_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = getpwnam_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.fs.UnixNativeDispatcher.getpwuid(I)[B")]
    async fn test_getpwuid() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = getpwuid(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_init() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::Int(0)));
        Ok(())
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.fs.UnixNativeDispatcher.lchown0(JII)V")]
    async fn test_lchown_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = lchown_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.fs.UnixNativeDispatcher.link0(JJ)V")]
    async fn test_link_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = link_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.fs.UnixNativeDispatcher.lstat0(JLsun/nio/fs/UnixFileAttributes;)V"
    )]
    async fn test_lstat_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = lstat_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.fs.UnixNativeDispatcher.lutimes0(JJJ)V"
    )]
    async fn test_lutimes_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = lutimes_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.fs.UnixNativeDispatcher.mkdir0(JI)V")]
    async fn test_mkdir_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = mkdir_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.fs.UnixNativeDispatcher.mknod0(JIJ)V")]
    async fn test_mknod_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = mknod_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.fs.UnixNativeDispatcher.open0(JII)I")]
    async fn test_open_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = open_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.fs.UnixNativeDispatcher.openat0(IJII)I"
    )]
    async fn test_openat_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = openat_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.fs.UnixNativeDispatcher.opendir0(J)J")]
    async fn test_opendir_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = opendir_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.fs.UnixNativeDispatcher.pathconf0(JI)J"
    )]
    async fn test_pathconf_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = pathconf_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.fs.UnixNativeDispatcher.read0(IJI)I")]
    async fn test_read() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = read(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.fs.UnixNativeDispatcher.read0(IJI)I")]
    async fn test_read_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = read_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.fs.UnixNativeDispatcher.readdir0(J)[B")]
    async fn test_readdir() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = readdir(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.fs.UnixNativeDispatcher.readdir0(J)[B")]
    async fn test_readdir_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = readdir_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.fs.UnixNativeDispatcher.readlink0(J)[B"
    )]
    async fn test_readlink_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = readlink_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.fs.UnixNativeDispatcher.realpath0(J)[B"
    )]
    async fn test_realpath_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = realpath_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.fs.UnixNativeDispatcher.rename0(JJ)V")]
    async fn test_rename_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = rename_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.fs.UnixNativeDispatcher.renameat0(IJIJ)V"
    )]
    async fn test_renameat_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = renameat_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.fs.UnixNativeDispatcher.rewind(J)V")]
    async fn test_rewind() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = rewind(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.fs.UnixNativeDispatcher.rmdir0(J)V")]
    async fn test_rmdir_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = rmdir_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_stat_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let unix_file_attributes = thread
            .object("sun.nio.fs.UnixFileAttributes", "", Vec::<Value>::new())
            .await?;
        let parameters = Parameters::new(vec![Value::Long(0), unix_file_attributes]);
        let result = stat_0(thread, parameters).await?;
        let result: i32 = result.expect("stat").try_into()?;
        assert_eq!(result, 0);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.fs.UnixNativeDispatcher.stat1(J)I")]
    async fn test_stat_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = stat_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.fs.UnixNativeDispatcher.statvfs0(JLsun/nio/fs/UnixFileStoreAttributes;)V"
    )]
    async fn test_statvfs_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = statvfs_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.fs.UnixNativeDispatcher.strerror(I)[B")]
    async fn test_strerror() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = strerror(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.fs.UnixNativeDispatcher.symlink0(JJ)V")]
    async fn test_symlink_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = symlink_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.fs.UnixNativeDispatcher.unlink0(J)V")]
    async fn test_unlink_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = unlink_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.fs.UnixNativeDispatcher.unlinkat0(IJI)V"
    )]
    async fn test_unlinkat_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = unlinkat_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.fs.UnixNativeDispatcher.utimes0(JJJ)V")]
    async fn test_utimes_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = utimes_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.fs.UnixNativeDispatcher.write0(IJI)I")]
    async fn test_write() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = write(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.fs.UnixNativeDispatcher.write0(IJI)I")]
    async fn test_write_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = write_0(thread, Parameters::default()).await;
    }
}
