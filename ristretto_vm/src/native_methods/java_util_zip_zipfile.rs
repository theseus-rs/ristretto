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

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn close(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn free_entry(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_comment_bytes(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_entry(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_entry_bytes(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_entry_c_size(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_entry_crc(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_entry_flag(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_entry_method(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_entry_size(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_entry_time(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_manifest_num(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_next_entry(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_total(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_zip_message(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn open(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn read(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn starts_with_loc(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
