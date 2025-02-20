use crate::Result;
use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "java/util/zip/ZipFile";

/// Register all native methods for `java.util.zip.ZipFile`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "close", "(J)V", close);
    registry.register(CLASS_NAME, "freeEntry", "(JJ)V", free_entry);
    registry.register(CLASS_NAME, "getCommentBytes", "(J)[B", get_comment_bytes);
    registry.register(CLASS_NAME, "getEntry", "(J[BZ)J", get_entry);
    registry.register(CLASS_NAME, "getEntryBytes", "(JI)[B", get_entry_bytes);
    registry.register(CLASS_NAME, "getEntryCSize", "(J)J", get_entry_c_size);
    registry.register(CLASS_NAME, "getEntryCrc", "(J)J", get_entry_crc);
    registry.register(CLASS_NAME, "getEntryFlag", "(J)I", get_entry_flag);
    registry.register(CLASS_NAME, "getEntryMethod", "(J)I", get_entry_method);
    registry.register(CLASS_NAME, "getEntrySize", "(J)J", get_entry_size);
    registry.register(CLASS_NAME, "getEntryTime", "(J)J", get_entry_time);
    registry.register(CLASS_NAME, "getManifestNum", "(J)I", get_manifest_num);
    registry.register(CLASS_NAME, "getNextEntry", "(JI)J", get_next_entry);
    registry.register(CLASS_NAME, "getTotal", "(J)I", get_total);
    registry.register(
        CLASS_NAME,
        "getZipMessage",
        "(J)Ljava/lang/String;",
        get_zip_message,
    );
    registry.register(CLASS_NAME, "initIDs", "()V", init_ids);
    registry.register(CLASS_NAME, "open", "(Ljava/lang/String;IJZ)J", open);
    registry.register(CLASS_NAME, "read", "(JJJ[BII)I", read);
    registry.register(CLASS_NAME, "startsWithLOC", "(J)Z", starts_with_loc);
}

#[async_recursion(?Send)]
async fn close(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.util.zip.ZipFile.close(J)V")
}

#[async_recursion(?Send)]
async fn free_entry(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.util.zip.ZipFile.freeEntry(JJ)V")
}

#[async_recursion(?Send)]
async fn get_comment_bytes(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.util.zip.ZipFile.getCommentBytes(J)[B")
}

#[async_recursion(?Send)]
async fn get_entry(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.util.zip.ZipFile.getEntry(J[BZ)J")
}

#[async_recursion(?Send)]
async fn get_entry_bytes(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.util.zip.ZipFile.getEntryBytes(JI)[B")
}

#[async_recursion(?Send)]
async fn get_entry_c_size(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.util.zip.ZipFile.getEntryCSize(J)J")
}

#[async_recursion(?Send)]
async fn get_entry_crc(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.util.zip.ZipFile.getEntryCrc(J)J")
}

#[async_recursion(?Send)]
async fn get_entry_flag(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.util.zip.ZipFile.getEntryFlag(J)I")
}

#[async_recursion(?Send)]
async fn get_entry_method(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.util.zip.ZipFile.getEntryMethod(J)I")
}

#[async_recursion(?Send)]
async fn get_entry_size(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.util.zip.ZipFile.getEntrySize(J)J")
}

#[async_recursion(?Send)]
async fn get_entry_time(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.util.zip.ZipFile.getEntryTime(J)J")
}

#[async_recursion(?Send)]
async fn get_manifest_num(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.util.zip.ZipFile.getManifestNum(J)I")
}

#[async_recursion(?Send)]
async fn get_next_entry(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.util.zip.ZipFile.getNextEntry(JI)J")
}

#[async_recursion(?Send)]
async fn get_total(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.util.zip.ZipFile.getTotal(J)I")
}

#[async_recursion(?Send)]
async fn get_zip_message(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.util.zip.ZipFile.getZipMessage(J)Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn open(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.util.zip.ZipFile.open(Ljava/lang/String;IJZ)J")
}

#[async_recursion(?Send)]
async fn read(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.util.zip.ZipFile.read(JJJ[BII)I")
}

#[async_recursion(?Send)]
async fn starts_with_loc(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.util.zip.ZipFile.startsWithLOC(J)Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.util.zip.ZipFile.close(J)V")]
    async fn test_close() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = close(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.util.zip.ZipFile.freeEntry(JJ)V")]
    async fn test_free_entry() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = free_entry(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.util.zip.ZipFile.getCommentBytes(J)[B")]
    async fn test_get_comment_bytes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_comment_bytes(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.util.zip.ZipFile.getEntry(J[BZ)J")]
    async fn test_get_entry() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_entry(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.util.zip.ZipFile.getEntryBytes(JI)[B")]
    async fn test_get_entry_bytes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_entry_bytes(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.util.zip.ZipFile.getEntryCSize(J)J")]
    async fn test_get_entry_c_size() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_entry_c_size(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.util.zip.ZipFile.getEntryCrc(J)J")]
    async fn test_get_entry_crc() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_entry_crc(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.util.zip.ZipFile.getEntryFlag(J)I")]
    async fn test_get_entry_flag() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_entry_flag(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.util.zip.ZipFile.getEntryMethod(J)I")]
    async fn test_get_entry_method() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_entry_method(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.util.zip.ZipFile.getEntrySize(J)J")]
    async fn test_get_entry_size() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_entry_size(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.util.zip.ZipFile.getEntryTime(J)J")]
    async fn test_get_entry_time() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_entry_time(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.util.zip.ZipFile.getManifestNum(J)I")]
    async fn test_get_manifest_num() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_manifest_num(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.util.zip.ZipFile.getNextEntry(JI)J")]
    async fn test_get_next_entry() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_next_entry(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.util.zip.ZipFile.getTotal(J)I")]
    async fn test_get_total() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_total(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.util.zip.ZipFile.getZipMessage(J)Ljava/lang/String;"
    )]
    async fn test_get_zip_message() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_zip_message(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.util.zip.ZipFile.open(Ljava/lang/String;IJZ)J"
    )]
    async fn test_open() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = open(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.util.zip.ZipFile.read(JJJ[BII)I")]
    async fn test_read() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = read(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.util.zip.ZipFile.startsWithLOC(J)Z")]
    async fn test_starts_with_loc() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = starts_with_loc(thread, Parameters::default()).await;
    }
}
