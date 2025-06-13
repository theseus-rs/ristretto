use crate::Error::InternalError;
use crate::JavaError::NullPointerException;
use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use bitflags::bitflags;
use ristretto_classfile::VersionSpecification::{
    Any, Between, Equal, GreaterThanOrEqual, LessThan, LessThanOrEqual,
};
use ristretto_classfile::{JAVA_8, JAVA_11, JAVA_17, JAVA_21, JAVA_24};
use ristretto_classloader::{Reference, Value};
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

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

#[intrinsic_method("sun/nio/fs/UnixNativeDispatcher.access0(JI)V", LessThan(JAVA_21))]
#[async_recursion(?Send)]
pub(crate) async fn access_0_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.access0(JI)V");
}

#[intrinsic_method(
    "sun/nio/fs/UnixNativeDispatcher.access0(JI)I",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn access_0_1(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.access0(JI)I");
}

#[intrinsic_method("sun/nio/fs/UnixNativeDispatcher.chmod0(JI)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn chmod_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.chmod0(JI)V");
}

#[intrinsic_method("sun/nio/fs/UnixNativeDispatcher.chown0(JII)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn chown_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.chown0(JII)V");
}

#[intrinsic_method("sun/nio/fs/UnixNativeDispatcher.close(I)V", LessThanOrEqual(JAVA_8))]
#[async_recursion(?Send)]
pub(crate) async fn close(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    close_0(thread, parameters).await
}

#[intrinsic_method(
    "sun/nio/fs/UnixNativeDispatcher.close0(I)V",
    GreaterThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn close_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.close0(I)V");
}

#[intrinsic_method("sun/nio/fs/UnixNativeDispatcher.closedir(J)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn closedir(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.closedir(J)V");
}

#[intrinsic_method("sun/nio/fs/UnixNativeDispatcher.dup(I)I", Any)]
#[async_recursion(?Send)]
pub(crate) async fn dup(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.dup(I)I");
}

#[intrinsic_method(
    "sun/nio/fs/UnixNativeDispatcher.exists0(J)Z",
    Between(JAVA_11, JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn exists_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.exists0(J)Z");
}

#[intrinsic_method("sun/nio/fs/UnixNativeDispatcher.fclose(I)V", LessThanOrEqual(JAVA_8))]
#[async_recursion(?Send)]
pub(crate) async fn fclose_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.fclose(I)V");
}

#[intrinsic_method("sun/nio/fs/UnixNativeDispatcher.fclose(J)V", LessThanOrEqual(JAVA_11))]
#[async_recursion(?Send)]
pub(crate) async fn fclose_1(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.fclose(J)V");
}

#[intrinsic_method(
    "sun/nio/fs/UnixNativeDispatcher.fchmod(II)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn fchmod(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    fchmod_0(thread, parameters).await
}

#[intrinsic_method(
    "sun/nio/fs/UnixNativeDispatcher.fchmod0(II)V",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn fchmod_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.fchmod0(II)V");
}

#[intrinsic_method(
    "sun/nio/fs/UnixNativeDispatcher.fchown(III)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn fchown(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    fchown_0(thread, parameters).await
}

#[intrinsic_method(
    "sun/nio/fs/UnixNativeDispatcher.fchown0(III)V",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn fchown_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.fchown0(III)V");
}

#[intrinsic_method("sun/nio/fs/UnixNativeDispatcher.fdopendir(I)J", Any)]
#[async_recursion(?Send)]
pub(crate) async fn fdopendir(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.fdopendir(I)J");
}

#[intrinsic_method(
    "sun/nio/fs/UnixNativeDispatcher.fgetxattr0(IJJI)I",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn fgetxattr_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.fgetxattr0(IJJI)I");
}

#[intrinsic_method(
    "sun/nio/fs/UnixNativeDispatcher.flistxattr(IJI)I",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn flistxattr(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.flistxattr(IJI)I");
}

#[intrinsic_method(
    "sun/nio/fs/UnixNativeDispatcher.fopen0(JJ)J",
    LessThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn fopen_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.fopen0(JJ)J");
}

#[intrinsic_method(
    "sun/nio/fs/UnixNativeDispatcher.fpathconf(II)J",
    LessThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn fpathconf(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.fpathconf(II)J");
}

#[intrinsic_method(
    "sun/nio/fs/UnixNativeDispatcher.fremovexattr0(IJ)V",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn fremovexattr_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.fremovexattr0(IJ)V");
}

#[intrinsic_method(
    "sun/nio/fs/UnixNativeDispatcher.fsetxattr0(IJJI)V",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn fsetxattr_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.fsetxattr0(IJJI)V");
}

#[intrinsic_method(
    "sun/nio/fs/UnixNativeDispatcher.fstat(ILsun/nio/fs/UnixFileAttributes;)V",
    LessThanOrEqual(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn fstat(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    fstat_0(thread, parameters).await
}

#[intrinsic_method(
    "sun/nio/fs/UnixNativeDispatcher.fstat0(ILsun/nio/fs/UnixFileAttributes;)V",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn fstat_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.fstat0(ILsun/nio/fs/UnixFileAttributes;)V");
}

#[intrinsic_method(
    "sun/nio/fs/UnixNativeDispatcher.fstatat0(IJILsun/nio/fs/UnixFileAttributes;)V",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn fstatat_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.fstatat0(IJILsun/nio/fs/UnixFileAttributes;)V");
}

#[intrinsic_method("sun/nio/fs/UnixNativeDispatcher.futimens(IJJ)V", Equal(JAVA_17))]
#[async_recursion(?Send)]
pub(crate) async fn futimens(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    futimens_0(thread, parameters).await
}

#[intrinsic_method(
    "sun/nio/fs/UnixNativeDispatcher.futimens0(IJJ)V",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn futimens_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.futimens0(IJJ)V");
}

#[intrinsic_method(
    "sun/nio/fs/UnixNativeDispatcher.futimes(IJJ)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn futimes(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    futimes_0(thread, parameters).await
}

#[intrinsic_method("sun/nio/fs/UnixNativeDispatcher.futimes0(IJJ)V", Equal(JAVA_21))]
#[async_recursion(?Send)]
pub(crate) async fn futimes_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.futimes0(IJJ)V");
}

#[intrinsic_method("sun/nio/fs/UnixNativeDispatcher.getcwd()[B", Any)]
#[expect(clippy::cast_possible_wrap)]
#[async_recursion(?Send)]
pub(crate) async fn getcwd(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    let current_dir_path =
        std::env::current_dir().map_err(|error| InternalError(format!("getcwd: {error}")))?;
    let current_dir_str = current_dir_path.to_string_lossy();
    let current_dir = current_dir_str
        .as_bytes()
        .to_vec()
        .iter()
        .map(|&b| b as i8)
        .collect::<Vec<i8>>();
    let current_dir_bytes = Reference::from(current_dir);
    Ok(Some(Value::Object(Some(current_dir_bytes))))
}

#[intrinsic_method("sun/nio/fs/UnixNativeDispatcher.getgrgid(I)[B", Any)]
#[async_recursion(?Send)]
pub(crate) async fn getgrgid(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.getgrgid(I)[B");
}

#[intrinsic_method("sun/nio/fs/UnixNativeDispatcher.getgrnam0(J)I", Any)]
#[async_recursion(?Send)]
pub(crate) async fn getgrnam_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.getgrnam0(J)I");
}

#[intrinsic_method(
    "sun/nio/fs/UnixNativeDispatcher.getlinelen(J)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn getlinelen(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.getlinelen(J)I");
}

#[intrinsic_method("sun/nio/fs/UnixNativeDispatcher.getpwnam0(J)I", Any)]
#[async_recursion(?Send)]
pub(crate) async fn getpwnam_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.getpwnam0(J)I");
}

#[intrinsic_method("sun/nio/fs/UnixNativeDispatcher.getpwuid(I)[B", Any)]
#[async_recursion(?Send)]
pub(crate) async fn getpwuid(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.getpwuid(I)[B");
}

#[intrinsic_method("sun/nio/fs/UnixNativeDispatcher.init()I", Any)]
#[async_recursion(?Send)]
pub(crate) async fn init(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    let capabilities = SupportsFlags::empty();
    // TODO: Implement the capabilities check
    let capabilities = capabilities.bits();
    Ok(Some(Value::Int(capabilities)))
}

#[intrinsic_method("sun/nio/fs/UnixNativeDispatcher.lchown0(JII)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn lchown_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.lchown0(JII)V");
}

#[intrinsic_method("sun/nio/fs/UnixNativeDispatcher.link0(JJ)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn link_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.link0(JJ)V");
}

#[intrinsic_method(
    "sun/nio/fs/UnixNativeDispatcher.lstat0(JLsun/nio/fs/UnixFileAttributes;)V",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn lstat_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.lstat0(JLsun/nio/fs/UnixFileAttributes;)V");
}

#[intrinsic_method(
    "sun/nio/fs/UnixNativeDispatcher.lutimes0(JJJ)V",
    Between(JAVA_17, JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn lutimes_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.lutimes0(JJJ)V");
}

#[intrinsic_method("sun/nio/fs/UnixNativeDispatcher.mkdir0(JI)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn mkdir_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.mkdir0(JI)V");
}

#[intrinsic_method("sun/nio/fs/UnixNativeDispatcher.mknod0(JIJ)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn mknod_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.mknod0(JIJ)V");
}

#[intrinsic_method("sun/nio/fs/UnixNativeDispatcher.open0(JII)I", Any)]
#[async_recursion(?Send)]
pub(crate) async fn open_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.open0(JII)I");
}

#[intrinsic_method("sun/nio/fs/UnixNativeDispatcher.openat0(IJII)I", Any)]
#[async_recursion(?Send)]
pub(crate) async fn openat_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.openat0(IJII)I");
}

#[intrinsic_method("sun/nio/fs/UnixNativeDispatcher.opendir0(J)J", Any)]
#[async_recursion(?Send)]
pub(crate) async fn opendir_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.opendir0(J)J");
}

#[intrinsic_method(
    "sun/nio/fs/UnixNativeDispatcher.pathconf0(JI)J",
    LessThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn pathconf_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.pathconf0(JI)J");
}

#[intrinsic_method("sun/nio/fs/UnixNativeDispatcher.read(IJI)I", LessThanOrEqual(JAVA_17))]
#[async_recursion(?Send)]
pub(crate) async fn read(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    read_0(thread, parameters).await
}

#[intrinsic_method(
    "sun/nio/fs/UnixNativeDispatcher.read0(IJI)I",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn read_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.read0(IJI)I");
}

#[intrinsic_method(
    "sun/nio/fs/UnixNativeDispatcher.readdir(J)[B",
    LessThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn readdir(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    readdir_0(thread, parameters).await
}

#[intrinsic_method(
    "sun/nio/fs/UnixNativeDispatcher.readdir0(J)[B",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn readdir_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.readdir0(J)[B");
}

#[intrinsic_method("sun/nio/fs/UnixNativeDispatcher.readlink0(J)[B", Any)]
#[async_recursion(?Send)]
pub(crate) async fn readlink_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.readlink0(J)[B");
}

#[intrinsic_method("sun/nio/fs/UnixNativeDispatcher.realpath0(J)[B", Any)]
#[async_recursion(?Send)]
pub(crate) async fn realpath_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.realpath0(J)[B");
}

#[intrinsic_method("sun/nio/fs/UnixNativeDispatcher.rename0(JJ)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn rename_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.rename0(JJ)V");
}

#[intrinsic_method("sun/nio/fs/UnixNativeDispatcher.renameat0(IJIJ)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn renameat_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.renameat0(IJIJ)V");
}

#[intrinsic_method("sun/nio/fs/UnixNativeDispatcher.rewind(J)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn rewind(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.rewind(J)V");
}

#[intrinsic_method("sun/nio/fs/UnixNativeDispatcher.rmdir0(J)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn rmdir_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.rmdir0(J)V");
}

#[intrinsic_method(
    "sun/nio/fs/UnixNativeDispatcher.stat0(JLsun/nio/fs/UnixFileAttributes;)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn stat_0_0(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    let _ = stat_0_1(thread, parameters).await?;
    Ok(None)
}

#[intrinsic_method(
    "sun/nio/fs/UnixNativeDispatcher.stat0(JLsun/nio/fs/UnixFileAttributes;)I",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn stat_0_1(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let Ok(_attributes) = parameters.pop_object() else {
        return Err(NullPointerException("attributes is null".to_string()).into());
    };
    let _path = parameters.pop_long()?;
    // TODO: Implement the stat0 method

    Ok(Some(Value::Int(0)))
}

#[intrinsic_method("sun/nio/fs/UnixNativeDispatcher.stat1(J)I", Between(JAVA_11, JAVA_17))]
#[async_recursion(?Send)]
pub(crate) async fn stat_1(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.stat1(J)I");
}

#[intrinsic_method(
    "sun/nio/fs/UnixNativeDispatcher.statvfs0(JLsun/nio/fs/UnixFileStoreAttributes;)V",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn statvfs_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.statvfs0(JLsun/nio/fs/UnixFileStoreAttributes;)V");
}

#[intrinsic_method("sun/nio/fs/UnixNativeDispatcher.strerror(I)[B", Any)]
#[async_recursion(?Send)]
pub(crate) async fn strerror(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.strerror(I)[B");
}

#[intrinsic_method("sun/nio/fs/UnixNativeDispatcher.symlink0(JJ)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn symlink_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.symlink0(JJ)V");
}

#[intrinsic_method("sun/nio/fs/UnixNativeDispatcher.unlink0(J)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn unlink_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.unlink0(J)V");
}

#[intrinsic_method("sun/nio/fs/UnixNativeDispatcher.unlinkat0(IJI)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn unlinkat_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.unlinkat0(IJI)V");
}

#[intrinsic_method(
    "sun/nio/fs/UnixNativeDispatcher.utimes0(JJJ)V",
    LessThanOrEqual(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn utimes_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.utimes0(JJJ)V");
}

#[intrinsic_method(
    "sun/nio/fs/UnixNativeDispatcher.utimensat0(IJJJI)V",
    GreaterThanOrEqual(JAVA_24)
)]
#[async_recursion(?Send)]
pub(crate) async fn utimensat_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.utimensat0(IJJJI)V");
}

#[intrinsic_method(
    "sun/nio/fs/UnixNativeDispatcher.write(IJI)I",
    LessThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn write(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    write_0(thread, parameters).await
}

#[intrinsic_method(
    "sun/nio/fs/UnixNativeDispatcher.write0(IJI)I",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn write_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixNativeDispatcher.write0(IJI)I");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.fs.UnixNativeDispatcher.access0(JI)V")]
    async fn test_access_0_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = access_0_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.fs.UnixNativeDispatcher.access0(JI)I")]
    async fn test_access_0_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = access_0_1(thread, Parameters::default()).await;
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
    #[should_panic(expected = "not yet implemented: sun.nio.fs.UnixNativeDispatcher.fclose(I)V")]
    async fn test_fclose_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = fclose_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.fs.UnixNativeDispatcher.fclose(J)V")]
    async fn test_fclose_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = fclose_1(thread, Parameters::default()).await;
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
    async fn test_stat_0_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let unix_file_attributes = thread
            .object("sun.nio.fs.UnixFileAttributes", "", Vec::<Value>::new())
            .await?;
        let parameters = Parameters::new(vec![Value::Long(0), unix_file_attributes]);
        let result = stat_0_0(thread, parameters).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_stat_0_1() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let unix_file_attributes = thread
            .object("sun.nio.fs.UnixFileAttributes", "", Vec::<Value>::new())
            .await?;
        let parameters = Parameters::new(vec![Value::Long(0), unix_file_attributes]);
        let result = stat_0_1(thread, parameters).await?;
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
    #[should_panic(
        expected = "not yet implemented: sun.nio.fs.UnixNativeDispatcher.utimensat0(IJJJI)V"
    )]
    async fn test_utimensat_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = utimensat_0(thread, Parameters::default()).await;
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
