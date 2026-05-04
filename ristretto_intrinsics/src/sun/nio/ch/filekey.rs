#[cfg(not(target_family = "wasm"))]
use crate::java::io::filedescriptor::file_descriptor_from_java_object;
#[cfg(target_family = "unix")]
use crate::sun::nio::fs::managed_files;
use ristretto_classfile::JAVA_21;
#[cfg(not(target_family = "wasm"))]
use ristretto_classfile::JAVA_25;
#[cfg(not(target_family = "wasm"))]
use ristretto_classfile::VersionSpecification::Equal;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
#[cfg(not(target_family = "wasm"))]
use ristretto_classloader::Reference;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
#[cfg(not(target_family = "wasm"))]
use ristretto_types::Error::InternalError;
#[cfg(target_family = "wasm")]
use ristretto_types::JavaError;
use ristretto_types::Thread;
#[cfg(not(target_family = "wasm"))]
use ristretto_types::VM;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

/// `sun.nio.ch.FileKey.init(Ljava/io/FileDescriptor;)V` (Java <= 21).
///
/// Instance method whose receiver is a `FileKey` object. Looks up file information for the
/// supplied `FileDescriptor` and stores the platform-specific identity fields on the receiver:
/// * Unix: `st_dev` and `st_ino`.
/// * Windows: `dwVolumeSerialNumber`, `nFileIndexHigh`, and `nFileIndexLow`.
#[intrinsic_method(
    "sun/nio/ch/FileKey.init(Ljava/io/FileDescriptor;)V",
    LessThanOrEqual(JAVA_21)
)]
#[async_method]
#[cfg_attr(target_family = "wasm", expect(clippy::needless_pass_by_value))]
pub async fn init_0<T: Thread + 'static>(
    #[cfg_attr(target_family = "wasm", expect(unused_variables))] thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let fd_value = parameters.pop()?;
    let this = parameters.pop_reference()?;

    #[cfg(target_family = "wasm")]
    {
        let _ = (fd_value, this);
        return Err(JavaError::UnsatisfiedLinkError(
            "sun.nio.ch.FileKey.init(Ljava/io/FileDescriptor;)V".to_string(),
        )
        .into());
    }

    #[cfg(not(target_family = "wasm"))]
    {
        let Some(this) = this else {
            return Err(InternalError("FileKey.init: this is null".to_string()));
        };
        let vm = thread.vm()?;
        let fd = file_descriptor_from_java_object(&vm, &fd_value)?;

        #[cfg(target_family = "unix")]
        let identity = {
            use std::os::unix::fs::MetadataExt;
            let metadata = managed_files::metadata(vm.file_handles(), fd)
                .await
                .map_err(|error| InternalError(format!("FileKey.init: {error}")))?;
            #[expect(clippy::cast_possible_wrap)]
            let dev = metadata.dev() as i64;
            #[expect(clippy::cast_possible_wrap)]
            let ino = metadata.ino() as i64;
            (dev, ino)
        };

        #[cfg(target_os = "windows")]
        let identity = {
            let (serial, _n_links, index) =
                crate::sun::nio::fs::windowsnativedispatcher::file_identity(vm.file_handles(), fd)
                    .await
                    .ok_or_else(|| {
                        InternalError("FileKey.init: file identity unavailable".to_string())
                    })?;
            #[expect(clippy::cast_possible_wrap)]
            let high = (index >> 32) as i64;
            #[expect(clippy::cast_possible_wrap)]
            let low = (index & 0xFFFF_FFFF) as i64;
            (i64::from(serial), high, low)
        };

        let this_value = Value::Object(Some(this));
        let mut object = this_value.as_object_mut()?;

        #[cfg(target_family = "unix")]
        {
            let (dev, ino) = identity;
            object.set_value("st_dev", Value::Long(dev))?;
            object.set_value("st_ino", Value::Long(ino))?;
        }

        #[cfg(target_os = "windows")]
        {
            let (serial, high, low) = identity;
            object.set_value("dwVolumeSerialNumber", Value::Long(serial))?;
            object.set_value("nFileIndexHigh", Value::Long(high))?;
            object.set_value("nFileIndexLow", Value::Long(low))?;
        }

        Ok(None)
    }
}

/// `sun.nio.ch.FileKey.init(Ljava/io/FileDescriptor;[J)V` (Unix, Java 25+).
///
/// Static method that fills the supplied `long[]` with `[st_dev, st_ino]` taken from the metadata
/// of the file referenced by the supplied `FileDescriptor`.
#[cfg(target_family = "unix")]
#[intrinsic_method("sun/nio/ch/FileKey.init(Ljava/io/FileDescriptor;[J)V", Equal(JAVA_25))]
#[async_method]
pub async fn init_1<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    use std::os::unix::fs::MetadataExt;
    let Some(finfo) = parameters.pop_reference()? else {
        return Err(InternalError("FileKey.init: finfo is null".to_string()));
    };
    let fd = parameters.pop()?;
    let vm = thread.vm()?;
    let fd = file_descriptor_from_java_object(&vm, &fd)?;
    let metadata = managed_files::metadata(vm.file_handles(), fd)
        .await
        .map_err(|error| InternalError(format!("FileKey.init: {error}")))?;

    let mut guard = finfo.write();
    let Reference::LongArray(values) = &mut *guard else {
        return Err(InternalError(
            "FileKey.init: finfo is not a long array".to_string(),
        ));
    };
    if values.len() < 2 {
        return Err(InternalError("FileKey.init: finfo length < 2".to_string()));
    }
    #[expect(clippy::cast_possible_wrap)]
    {
        values[0] = metadata.dev() as i64;
        values[1] = metadata.ino() as i64;
    }

    Ok(None)
}

/// `sun.nio.ch.FileKey.init(Ljava/io/FileDescriptor;[I)V` (Windows, Java 25+).
///
/// Static method that fills the supplied `int[]` with the volume serial number and file index
/// (high/low halves) taken from the metadata of the file referenced by the supplied
/// `FileDescriptor`.
#[cfg(target_os = "windows")]
#[intrinsic_method("sun/nio/ch/FileKey.init(Ljava/io/FileDescriptor;[I)V", Equal(JAVA_25))]
#[async_method]
pub async fn init_2<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let Some(finfo) = parameters.pop_reference()? else {
        return Err(InternalError("FileKey.init: finfo is null".to_string()));
    };
    let fd = parameters.pop()?;
    let vm = thread.vm()?;
    let fd = file_descriptor_from_java_object(&vm, &fd)?;
    let (serial, _n_links, index) =
        crate::sun::nio::fs::windowsnativedispatcher::file_identity(vm.file_handles(), fd)
            .await
            .ok_or_else(|| InternalError("FileKey.init: file identity unavailable".to_string()))?;

    let mut guard = finfo.write();
    let Reference::IntArray(values) = &mut *guard else {
        return Err(InternalError(
            "FileKey.init: finfo is not an int array".to_string(),
        ));
    };
    if values.len() < 3 {
        return Err(InternalError("FileKey.init: finfo length < 3".to_string()));
    }
    #[expect(clippy::cast_possible_wrap)]
    {
        values[0] = serial as i32;
        values[1] = (index >> 32) as i32;
        values[2] = (index & 0xFFFF_FFFF) as i32;
    }

    Ok(None)
}

/// `sun.nio.ch.FileKey.initIDs()V`.
///
/// In `OpenJDK` this caches `jfieldID` references; we have nothing to do.
#[intrinsic_method("sun/nio/ch/FileKey.initIDs()V", LessThanOrEqual(JAVA_21))]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[cfg(test)]
#[cfg(not(target_family = "wasm"))]
mod tests {
    use super::*;
    use ristretto_classloader::Object;
    use ristretto_types::handles::{FileHandle, FileModeFlags};
    use std::io::Write;
    use tempfile::NamedTempFile;

    /// Helper: create a `FileDescriptor` Java object whose `fd` field is set.
    async fn create_file_descriptor(thread: &Arc<ristretto_vm::Thread>, fd: i32) -> Result<Value> {
        let vm = thread.vm()?;
        let class = thread.class("java/io/FileDescriptor").await?;
        let mut object = Object::new(class)?;
        object.set_value("fd", Value::Int(fd))?;
        // On Windows the resolver may also read `handle`; default to -1 so `fd` wins.
        let _ = object.set_value("handle", Value::Long(-1));
        Ok(Value::new_object(
            vm.garbage_collector(),
            Reference::Object(object),
        ))
    }

    /// Helper: open a temp file and register it under a managed fd.
    async fn open_test_file() -> (
        Arc<ristretto_vm::VM>,
        Arc<ristretto_vm::Thread>,
        NamedTempFile,
        i64,
    ) {
        let (vm, thread) = crate::test::java25_thread().await.expect("thread");
        let mut tmp = NamedTempFile::new().expect("tmp");
        tmp.write_all(b"filekey-test").expect("write");
        tmp.flush().expect("flush");
        let path = tmp.path().to_path_buf();
        let std_file = std::fs::OpenOptions::new()
            .read(true)
            .open(&path)
            .expect("open");
        let tokio_file = tokio::fs::File::from_std(std_file);
        let handle: FileHandle = (tokio_file, FileModeFlags::READ_ONLY).into();
        let fd = i64::from(vm.next_nio_fd());
        vm.file_handles().insert(fd, handle).await.expect("insert");
        (vm, thread, tmp, fd)
    }

    #[tokio::test]
    async fn test_init_0_null_this() {
        let (_vm, thread) = crate::test::java21_thread().await.expect("thread");
        let result = init_0(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "Internal error: FileKey.init: this is null",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_init_0_invalid_fd() -> Result<()> {
        let (vm, thread) = crate::test::java21_thread().await?;
        let class = thread.class("java/io/FileDescriptor").await?;
        let object = Object::new(class)?;
        let this = Value::new_object(vm.garbage_collector(), Reference::Object(object));
        let fd = create_file_descriptor(&thread, 9_999_999).await?;
        let result = init_0(thread, Parameters::new(vec![this, fd])).await;
        let message = result.unwrap_err().to_string();
        assert!(
            message.contains("FileKey.init"),
            "unexpected error: {message}"
        );
        Ok(())
    }

    #[cfg(target_family = "unix")]
    #[tokio::test]
    async fn test_init_1_null_finfo() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_1(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "Internal error: FileKey.init: finfo is null",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_family = "unix")]
    #[tokio::test]
    async fn test_init_1_wrong_array_type() -> Result<()> {
        let (vm, thread) = crate::test::java25_thread().await?;
        let finfo = Value::new_object(vm.garbage_collector(), Reference::from(vec![0i32; 2]));
        let fd = create_file_descriptor(&thread, -1).await?;
        let result = init_1(thread, Parameters::new(vec![fd, finfo])).await;
        let message = result.unwrap_err().to_string();
        assert!(
            message.contains("FileKey.init"),
            "unexpected error: {message}"
        );
        Ok(())
    }

    #[cfg(target_family = "unix")]
    #[tokio::test]
    async fn test_init_1_short_array() -> Result<()> {
        let (vm, thread, _tmp, fd_num) = open_test_file().await;
        let finfo = Value::new_object(vm.garbage_collector(), Reference::from(vec![0i64; 1]));
        let fd = create_file_descriptor(&thread, i32::try_from(fd_num)?).await?;
        let result = init_1(thread, Parameters::new(vec![fd, finfo])).await;
        assert_eq!(
            "Internal error: FileKey.init: finfo length < 2",
            result.unwrap_err().to_string()
        );
        Ok(())
    }

    #[cfg(target_family = "unix")]
    #[tokio::test]
    async fn test_init_1_success() -> Result<()> {
        let (vm, thread, _tmp, fd_num) = open_test_file().await;
        let finfo = Value::new_object(vm.garbage_collector(), Reference::from(vec![0i64; 2]));
        let fd = create_file_descriptor(&thread, i32::try_from(fd_num)?).await?;
        let result = init_1(thread, Parameters::new(vec![fd, finfo.clone()])).await?;
        assert_eq!(None, result);
        let guard = finfo.as_reference()?;
        let Reference::LongArray(values) = &*guard else {
            panic!("expected long array");
        };
        // st_ino should be non-zero for a real file on a real filesystem.
        assert!(values[1] != 0, "expected non-zero st_ino, got {values:?}");
        Ok(())
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::java21_thread().await?;
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_init_2_null_finfo() {
        let (_vm, thread) = crate::test::java25_thread().await.expect("thread");
        let result = init_2(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "Internal error: FileKey.init: finfo is null",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_init_2_wrong_array_type() -> Result<()> {
        let (vm, thread) = crate::test::java25_thread().await?;
        let finfo = Value::new_object(vm.garbage_collector(), Reference::from(vec![0i64; 3]));
        let fd = create_file_descriptor(&thread, -1).await?;
        let result = init_2(thread, Parameters::new(vec![fd, finfo])).await;
        let message = result.unwrap_err().to_string();
        assert!(
            message.contains("FileKey.init"),
            "unexpected error: {message}"
        );
        Ok(())
    }
}

#[cfg(test)]
#[cfg(target_family = "wasm")]
mod wasm_tests {
    use super::*;

    #[tokio::test]
    async fn test_init_0_unsupported() {
        let (_vm, thread) = crate::test::java21_thread().await.expect("thread");
        let result = init_0(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun.nio.ch.FileKey.init(Ljava/io/FileDescriptor;)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_init_ids() {
        let (_vm, thread) = crate::test::java21_thread().await.expect("thread");
        let result = init_ids(thread, Parameters::default()).await.expect("ok");
        assert_eq!(result, None);
    }
}
