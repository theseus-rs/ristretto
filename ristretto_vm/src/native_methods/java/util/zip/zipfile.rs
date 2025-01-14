use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.util.zip.ZipFile`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/util/zip/ZipFile";
    registry.register(class_name, "close", "(J)V", close);
    registry.register(class_name, "freeEntry", "(JJ)V", free_entry);
    registry.register(class_name, "getCommentBytes", "(J)[B", get_comment_bytes);
    registry.register(class_name, "getEntry", "(J[BZ)J", get_entry);
    registry.register(class_name, "getEntryBytes", "(JI)[B", get_entry_bytes);
    registry.register(class_name, "getEntryCSize", "(J)J", get_entry_c_size);
    registry.register(class_name, "getEntryCrc", "(J)J", get_entry_crc);
    registry.register(class_name, "getEntryFlag", "(J)I", get_entry_flag);
    registry.register(class_name, "getEntryMethod", "(J)I", get_entry_method);
    registry.register(class_name, "getEntrySize", "(J)J", get_entry_size);
    registry.register(class_name, "getEntryTime", "(J)J", get_entry_time);
    registry.register(class_name, "getManifestNum", "(J)I", get_manifest_num);
    registry.register(class_name, "getNextEntry", "(JI)J", get_next_entry);
    registry.register(class_name, "getTotal", "(J)I", get_total);
    registry.register(
        class_name,
        "getZipMessage",
        "(J)Ljava/lang/String;",
        get_zip_message,
    );
    registry.register(class_name, "initIDs", "()V", init_ids);
    registry.register(class_name, "open", "(Ljava/lang/String;IJZ)J", open);
    registry.register(class_name, "read", "(JJJ[BII)I", read);
    registry.register(class_name, "startsWithLOC", "(J)Z", starts_with_loc);
}

#[async_recursion(?Send)]
async fn close(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.util.zip.ZipFile.close(J)V")
}

#[async_recursion(?Send)]
async fn free_entry(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.util.zip.ZipFile.freeEntry(JJ)V")
}

#[async_recursion(?Send)]
async fn get_comment_bytes(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.util.zip.ZipFile.getCommentBytes(J)[B")
}

#[async_recursion(?Send)]
async fn get_entry(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.util.zip.ZipFile.getEntry(J[BZ)J")
}

#[async_recursion(?Send)]
async fn get_entry_bytes(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.util.zip.ZipFile.getEntryBytes(JI)[B")
}

#[async_recursion(?Send)]
async fn get_entry_c_size(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.util.zip.ZipFile.getEntryCSize(J)J")
}

#[async_recursion(?Send)]
async fn get_entry_crc(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.util.zip.ZipFile.getEntryCrc(J)J")
}

#[async_recursion(?Send)]
async fn get_entry_flag(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.util.zip.ZipFile.getEntryFlag(J)I")
}

#[async_recursion(?Send)]
async fn get_entry_method(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.util.zip.ZipFile.getEntryMethod(J)I")
}

#[async_recursion(?Send)]
async fn get_entry_size(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.util.zip.ZipFile.getEntrySize(J)J")
}

#[async_recursion(?Send)]
async fn get_entry_time(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.util.zip.ZipFile.getEntryTime(J)J")
}

#[async_recursion(?Send)]
async fn get_manifest_num(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.util.zip.ZipFile.getManifestNum(J)I")
}

#[async_recursion(?Send)]
async fn get_next_entry(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.util.zip.ZipFile.getNextEntry(JI)J")
}

#[async_recursion(?Send)]
async fn get_total(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.util.zip.ZipFile.getTotal(J)I")
}

#[async_recursion(?Send)]
async fn get_zip_message(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.util.zip.ZipFile.getZipMessage(J)Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn open(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.util.zip.ZipFile.open(Ljava/lang/String;IJZ)J")
}

#[async_recursion(?Send)]
async fn read(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.util.zip.ZipFile.read(JJJ[BII)I")
}

#[async_recursion(?Send)]
async fn starts_with_loc(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.util.zip.ZipFile.startsWithLOC(J)Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "java/util/zip/ZipFile";
        assert!(registry.method(class_name, "close", "(J)V").is_some());
        assert!(registry.method(class_name, "freeEntry", "(JJ)V").is_some());
        assert!(registry
            .method(class_name, "getCommentBytes", "(J)[B")
            .is_some());
        assert!(registry.method(class_name, "getEntry", "(J[BZ)J").is_some());
        assert!(registry
            .method(class_name, "getEntryBytes", "(JI)[B")
            .is_some());
        assert!(registry
            .method(class_name, "getEntryCSize", "(J)J")
            .is_some());
        assert!(registry.method(class_name, "getEntryCrc", "(J)J").is_some());
        assert!(registry
            .method(class_name, "getEntryFlag", "(J)I")
            .is_some());
        assert!(registry
            .method(class_name, "getEntryMethod", "(J)I")
            .is_some());
        assert!(registry
            .method(class_name, "getEntrySize", "(J)J")
            .is_some());
        assert!(registry
            .method(class_name, "getEntryTime", "(J)J")
            .is_some());
        assert!(registry
            .method(class_name, "getManifestNum", "(J)I")
            .is_some());
        assert!(registry
            .method(class_name, "getNextEntry", "(JI)J")
            .is_some());
        assert!(registry.method(class_name, "getTotal", "(J)I").is_some());
        assert!(registry
            .method(class_name, "getZipMessage", "(J)Ljava/lang/String;")
            .is_some());
        assert!(registry.method(class_name, "initIDs", "()V").is_some());
        assert!(registry
            .method(class_name, "open", "(Ljava/lang/String;IJZ)J")
            .is_some());
        assert!(registry.method(class_name, "read", "(JJJ[BII)I").is_some());
        assert!(registry
            .method(class_name, "startsWithLOC", "(J)Z")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.util.zip.ZipFile.close(J)V")]
    async fn test_close() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = close(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.util.zip.ZipFile.freeEntry(JJ)V")]
    async fn test_free_entry() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = free_entry(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.util.zip.ZipFile.getCommentBytes(J)[B")]
    async fn test_get_comment_bytes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_comment_bytes(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.util.zip.ZipFile.getEntry(J[BZ)J")]
    async fn test_get_entry() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_entry(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.util.zip.ZipFile.getEntryBytes(JI)[B")]
    async fn test_get_entry_bytes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_entry_bytes(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.util.zip.ZipFile.getEntryCSize(J)J")]
    async fn test_get_entry_c_size() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_entry_c_size(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.util.zip.ZipFile.getEntryCrc(J)J")]
    async fn test_get_entry_crc() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_entry_crc(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.util.zip.ZipFile.getEntryFlag(J)I")]
    async fn test_get_entry_flag() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_entry_flag(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.util.zip.ZipFile.getEntryMethod(J)I")]
    async fn test_get_entry_method() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_entry_method(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.util.zip.ZipFile.getEntrySize(J)J")]
    async fn test_get_entry_size() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_entry_size(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.util.zip.ZipFile.getEntryTime(J)J")]
    async fn test_get_entry_time() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_entry_time(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.util.zip.ZipFile.getManifestNum(J)I")]
    async fn test_get_manifest_num() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_manifest_num(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.util.zip.ZipFile.getNextEntry(JI)J")]
    async fn test_get_next_entry() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_next_entry(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.util.zip.ZipFile.getTotal(J)I")]
    async fn test_get_total() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_total(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.util.zip.ZipFile.getZipMessage(J)Ljava/lang/String;"
    )]
    async fn test_get_zip_message() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_zip_message(thread, Arguments::default()).await;
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_ids(thread, Arguments::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.util.zip.ZipFile.open(Ljava/lang/String;IJZ)J"
    )]
    async fn test_open() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = open(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.util.zip.ZipFile.read(JJJ[BII)I")]
    async fn test_read() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = read(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.util.zip.ZipFile.startsWithLOC(J)Z")]
    async fn test_starts_with_loc() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = starts_with_loc(thread, Arguments::default()).await;
    }
}
