use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

/// Stub implementation of `ZipFile` methods for Java 8 and earlier
#[intrinsic_method("java/util/zip/ZipFile.close(J)V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn close<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _handle = parameters.pop_long()?;
    Ok(None)
}

#[intrinsic_method("java/util/zip/ZipFile.freeEntry(JJ)V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn free_entry<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _entry_handle = parameters.pop_long()?;
    let _zip_handle = parameters.pop_long()?;
    Ok(None)
}

#[intrinsic_method("java/util/zip/ZipFile.getCommentBytes(J)[B", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn get_comment_bytes<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _handle = parameters.pop_long()?;
    Ok(Some(Value::Object(None)))
}

#[intrinsic_method("java/util/zip/ZipFile.getEntry(J[BZ)J", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn get_entry<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _add_slash = parameters.pop_int()?;
    let _name_ref = parameters.pop_reference()?;
    let _handle = parameters.pop_long()?;
    Ok(Some(Value::Long(0)))
}

#[intrinsic_method("java/util/zip/ZipFile.getEntryBytes(JI)[B", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn get_entry_bytes<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _entry_type = parameters.pop_int()?;
    let _entry_handle = parameters.pop_long()?;
    Ok(Some(Value::Object(None)))
}

#[intrinsic_method("java/util/zip/ZipFile.getEntryCSize(J)J", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn get_entry_c_size<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _entry_handle = parameters.pop_long()?;
    Ok(Some(Value::Long(0)))
}

#[intrinsic_method("java/util/zip/ZipFile.getEntryCrc(J)J", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn get_entry_crc<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _entry_handle = parameters.pop_long()?;
    Ok(Some(Value::Long(0)))
}

#[intrinsic_method("java/util/zip/ZipFile.getEntryFlag(J)I", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn get_entry_flag<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _entry_handle = parameters.pop_long()?;
    Ok(Some(Value::Int(0)))
}

#[intrinsic_method("java/util/zip/ZipFile.getEntryMethod(J)I", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn get_entry_method<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _entry_handle = parameters.pop_long()?;
    Ok(Some(Value::Int(0)))
}

#[intrinsic_method("java/util/zip/ZipFile.getEntrySize(J)J", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn get_entry_size<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _entry_handle = parameters.pop_long()?;
    Ok(Some(Value::Long(0)))
}

#[intrinsic_method("java/util/zip/ZipFile.getEntryTime(J)J", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn get_entry_time<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _entry_handle = parameters.pop_long()?;
    Ok(Some(Value::Long(0)))
}

#[intrinsic_method("java/util/zip/ZipFile.getManifestNum(J)I", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn get_manifest_num<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _handle = parameters.pop_long()?;
    Ok(Some(Value::Int(0)))
}

#[intrinsic_method("java/util/zip/ZipFile.getNextEntry(JI)J", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn get_next_entry<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _index = parameters.pop_int()?;
    let _handle = parameters.pop_long()?;
    Ok(Some(Value::Long(0)))
}

#[intrinsic_method("java/util/zip/ZipFile.getTotal(J)I", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn get_total<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _handle = parameters.pop_long()?;
    Ok(Some(Value::Int(0)))
}

#[intrinsic_method(
    "java/util/zip/ZipFile.getZipMessage(J)Ljava/lang/String;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_zip_message<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _handle = parameters.pop_long()?;
    Ok(Some(Value::Object(None)))
}

#[intrinsic_method("java/util/zip/ZipFile.initIDs()V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn init_ids<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "java/util/zip/ZipFile.open(Ljava/lang/String;IJZ)J",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn open<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _use_mmap = parameters.pop_int()?;
    let _last_modified = parameters.pop_long()?;
    let _mode = parameters.pop_int()?;
    let name_ref = parameters.pop_reference()?;

    let Some(_name_ref) = name_ref else {
        return Err(ristretto_types::JavaError::NullPointerException(Some(
            "file name is null".to_string(),
        ))
        .into());
    };

    // Return a dummy handle; ZipFile functionality not fully implemented. Eventually the
    // implementation should open the file and parse the zip structure
    Ok(Some(Value::Long(1)))
}

#[intrinsic_method("java/util/zip/ZipFile.read(JJJ[BII)I", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn read<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _len = parameters.pop_int()?;
    let _off = parameters.pop_int()?;
    let _output_ref = parameters.pop_reference()?;
    let _pos = parameters.pop_long()?;
    let _entry_handle = parameters.pop_long()?;
    let _handle = parameters.pop_long()?;

    // Return -1 for EOF
    Ok(Some(Value::Int(-1)))
}

#[intrinsic_method("java/util/zip/ZipFile.startsWithLOC(J)Z", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn starts_with_loc<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _handle = parameters.pop_long()?;
    // Assume valid zip file starts with LOC
    Ok(Some(Value::from(true)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }

    #[tokio::test]
    async fn test_close() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;

        let mut parameters = Parameters::default();
        parameters.push_long(1); // handle

        let result = close(thread, parameters).await?;
        assert!(result.is_none());
        Ok(())
    }

    #[tokio::test]
    async fn test_free_entry() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;

        let mut parameters = Parameters::default();
        parameters.push_long(1); // zip handle
        parameters.push_long(2); // entry handle

        let result = free_entry(thread, parameters).await?;
        assert!(result.is_none());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_comment_bytes() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;

        let mut parameters = Parameters::default();
        parameters.push_long(1); // handle

        let result = get_comment_bytes(thread, parameters).await?;
        assert_eq!(Some(Value::Object(None)), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_entry() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;

        let mut parameters = Parameters::default();
        parameters.push_long(1); // handle
        parameters.push_reference(None); // name
        parameters.push_int(0); // add_slash

        let result = get_entry(thread, parameters).await?;
        assert_eq!(Some(Value::Long(0)), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_entry_bytes() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;

        let mut parameters = Parameters::default();
        parameters.push_long(1); // entry handle
        parameters.push_int(0); // type

        let result = get_entry_bytes(thread, parameters).await?;
        assert_eq!(Some(Value::Object(None)), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_entry_c_size() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;

        let mut parameters = Parameters::default();
        parameters.push_long(1); // entry handle

        let result = get_entry_c_size(thread, parameters).await?;
        assert_eq!(Some(Value::Long(0)), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_entry_crc() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;

        let mut parameters = Parameters::default();
        parameters.push_long(1); // entry handle

        let result = get_entry_crc(thread, parameters).await?;
        assert_eq!(Some(Value::Long(0)), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_entry_flag() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;

        let mut parameters = Parameters::default();
        parameters.push_long(1); // entry handle

        let result = get_entry_flag(thread, parameters).await?;
        assert_eq!(Some(Value::Int(0)), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_entry_method() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;

        let mut parameters = Parameters::default();
        parameters.push_long(1); // entry handle

        let result = get_entry_method(thread, parameters).await?;
        assert_eq!(Some(Value::Int(0)), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_entry_size() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;

        let mut parameters = Parameters::default();
        parameters.push_long(1); // entry handle

        let result = get_entry_size(thread, parameters).await?;
        assert_eq!(Some(Value::Long(0)), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_entry_time() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;

        let mut parameters = Parameters::default();
        parameters.push_long(1); // entry handle

        let result = get_entry_time(thread, parameters).await?;
        assert_eq!(Some(Value::Long(0)), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_manifest_num() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;

        let mut parameters = Parameters::default();
        parameters.push_long(1); // handle

        let result = get_manifest_num(thread, parameters).await?;
        assert_eq!(Some(Value::Int(0)), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_next_entry() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;

        let mut parameters = Parameters::default();
        parameters.push_long(1); // handle
        parameters.push_int(0); // index

        let result = get_next_entry(thread, parameters).await?;
        assert_eq!(Some(Value::Long(0)), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_total() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;

        let mut parameters = Parameters::default();
        parameters.push_long(1); // handle

        let result = get_total(thread, parameters).await?;
        assert_eq!(Some(Value::Int(0)), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_zip_message() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;

        let mut parameters = Parameters::default();
        parameters.push_long(1); // handle

        let result = get_zip_message(thread, parameters).await?;
        assert_eq!(Some(Value::Object(None)), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_open() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;

        // Create minimal parameters
        let mut parameters = Parameters::default();
        parameters.push_reference(None); // name (null triggers error)
        parameters.push_int(1); // mode
        parameters.push_long(0); // last_modified
        parameters.push_int(0); // use_mmap

        let result = open(thread, parameters).await;
        // Should return an error since name is null
        assert!(result.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_read() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;

        let mut parameters = Parameters::default();
        parameters.push_long(1); // handle
        parameters.push_long(2); // entry_handle
        parameters.push_long(0); // pos
        parameters.push_reference(None); // output
        parameters.push_int(0); // off
        parameters.push_int(10); // len

        let result = read(thread, parameters).await?;
        assert_eq!(Some(Value::Int(-1)), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_starts_with_loc() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;

        let mut parameters = Parameters::default();
        parameters.push_long(1); // handle

        let result = starts_with_loc(thread, parameters).await?;
        assert_eq!(Some(Value::from(true)), result);
        Ok(())
    }
}
