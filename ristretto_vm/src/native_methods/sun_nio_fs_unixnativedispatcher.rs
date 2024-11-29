use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Error::InternalError;
use crate::Result;
use async_recursion::async_recursion;
use bitflags::bitflags;
use ristretto_classfile::Version;
use ristretto_classloader::{ConcurrentVec, Reference, Value};
use std::sync::Arc;

const JAVA_22: Version = Version::Java22 { minor: 0 };

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
    let class_name = "sun/nio/fs/UnixNativeDispatcher";
    let java_version = registry.java_version();

    if java_version <= &JAVA_22 {
        registry.register(class_name, "access0", "(JI)V", access_0);
        registry.register(class_name, "exists0", "(J)Z", exists_0);
    } else {
        registry.register(class_name, "access0", "(JI)I", access_0);
    }

    registry.register(class_name, "access0", "(JI)I", access_0);
    registry.register(class_name, "chmod0", "(JI)V", chmod_0);
    registry.register(class_name, "chown0", "(JII)V", chown_0);
    registry.register(class_name, "close0", "(I)V", close_0);
    registry.register(class_name, "closedir", "(J)V", closedir);
    registry.register(class_name, "dup", "(I)I", dup);
    registry.register(class_name, "fchmod0", "(II)V", fchmod_0);
    registry.register(class_name, "fchown0", "(III)V", fchown_0);
    registry.register(class_name, "fdopendir", "(I)J", fdopendir);
    registry.register(class_name, "fgetxattr0", "(IJJI)I", fgetxattr_0);
    registry.register(class_name, "flistxattr", "(IJI)I", flistxattr);
    registry.register(class_name, "fremovexattr0", "(IJ)V", fremovexattr_0);
    registry.register(class_name, "fsetxattr0", "(IJJI)V", fsetxattr_0);
    registry.register(
        class_name,
        "fstat0",
        "(ILsun/nio/fs/UnixFileAttributes;)V",
        fstat_0,
    );
    registry.register(
        class_name,
        "fstatat0",
        "(IJILsun/nio/fs/UnixFileAttributes;)V",
        fstatat_0,
    );
    registry.register(class_name, "futimens0", "(IJJ)V", futimens_0);
    registry.register(class_name, "futimes0", "(IJJ)V", futimes_0);
    registry.register(class_name, "getcwd", "()[B", getcwd);
    registry.register(class_name, "getgrgid", "(I)[B", getgrgid);
    registry.register(class_name, "getgrnam0", "(J)I", getgrnam_0);
    registry.register(class_name, "getlinelen", "(J)I", getlinelen);
    registry.register(class_name, "getpwnam0", "(J)I", getpwnam_0);
    registry.register(class_name, "getpwuid", "(I)[B", getpwuid);
    registry.register(class_name, "init", "()I", init);
    registry.register(class_name, "lchown0", "(JII)V", lchown_0);
    registry.register(class_name, "link0", "(JJ)V", link_0);
    registry.register(
        class_name,
        "lstat0",
        "(JLsun/nio/fs/UnixFileAttributes;)V",
        lstat_0,
    );
    registry.register(class_name, "lutimes0", "(JJJ)V", lutimes_0);
    registry.register(class_name, "mkdir0", "(JI)V", mkdir_0);
    registry.register(class_name, "mknod0", "(JIJ)V", mknod_0);
    registry.register(class_name, "open0", "(JII)I", open_0);
    registry.register(class_name, "openat0", "(IJII)I", openat_0);
    registry.register(class_name, "opendir0", "(J)J", opendir_0);
    registry.register(class_name, "read0", "(IJI)I", read_0);
    registry.register(class_name, "readdir0", "(J)[B", readdir_0);
    registry.register(class_name, "readlink0", "(J)[B", readlink_0);
    registry.register(class_name, "realpath0", "(J)[B", realpath_0);
    registry.register(class_name, "rename0", "(JJ)V", rename_0);
    registry.register(class_name, "renameat0", "(IJIJ)V", renameat_0);
    registry.register(class_name, "rewind", "(J)V", rewind);
    registry.register(class_name, "rmdir0", "(J)V", rmdir_0);
    registry.register(
        class_name,
        "stat0",
        "(JLsun/nio/fs/UnixFileAttributes;)I",
        stat_0,
    );
    registry.register(
        class_name,
        "statvfs0",
        "(JLsun/nio/fs/UnixFileStoreAttributes;)V",
        statvfs_0,
    );
    registry.register(class_name, "strerror", "(I)[B", strerror);
    registry.register(class_name, "symlink0", "(JJ)V", symlink_0);
    registry.register(class_name, "unlink0", "(J)V", unlink_0);
    registry.register(class_name, "unlinkat0", "(IJI)V", unlinkat_0);
    registry.register(class_name, "utimes0", "(JJJ)V", utimes_0);
    registry.register(class_name, "write0", "(IJI)I", write_0);
}

#[async_recursion(?Send)]
async fn access_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn chmod_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn chown_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn close_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn closedir(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn dup(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn exists_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn fchmod_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn fchown_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn fdopendir(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn fgetxattr_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn flistxattr(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn fremovexattr_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn fsetxattr_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn fstat_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn fstatat_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn futimens_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn futimes_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::cast_possible_wrap)]
#[async_recursion(?Send)]
async fn getcwd(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
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
async fn getgrgid(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn getgrnam_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn getlinelen(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn getpwnam_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn getpwuid(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn init(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    let capabilities = SupportsFlags::empty();
    // TODO: Implement the capabilities check
    let capabilities = capabilities.bits();
    Ok(Some(Value::Int(capabilities)))
}

#[async_recursion(?Send)]
async fn lchown_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn link_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn lstat_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn lutimes_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn mkdir_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn mknod_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn open_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn openat_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn opendir_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn read_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn readdir_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn readlink_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn realpath_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn rename_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn renameat_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn rewind(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn rmdir_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn stat_0(_thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let Some(_attributes) = arguments.pop_reference()? else {
        return Err(InternalError("attributes is null".to_string()));
    };
    let _path = arguments.pop_long()?;
    // TODO: Implement the stat0 method

    Ok(Some(Value::Int(0)))
}

#[async_recursion(?Send)]
async fn statvfs_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn strerror(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn symlink_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn unlink_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn unlinkat_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn utimes_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn write_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
