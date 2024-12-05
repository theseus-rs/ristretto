use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `com.apple.eio.FileManager`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "com/apple/eio/FileManager";
    registry.register(
        class_name,
        "_findFolder",
        "(SIZ)Ljava/lang/String;",
        find_folder,
    );
    registry.register(
        class_name,
        "_getFileCreator",
        "(Ljava/lang/String;)I",
        get_file_creator,
    );
    registry.register(
        class_name,
        "_getFileType",
        "(Ljava/lang/String;)I",
        get_file_type,
    );
    registry.register(
        class_name,
        "_moveToTrash",
        "(Ljava/lang/String;)Z",
        move_to_trash,
    );
    registry.register(class_name, "_openURL", "(Ljava/lang/String;)V", open_url);
    registry.register(
        class_name,
        "_revealInFinder",
        "(Ljava/lang/String;)Z",
        reveal_in_finder,
    );
    registry.register(
        class_name,
        "_setFileCreator",
        "(Ljava/lang/String;I)V",
        set_file_creator,
    );
    registry.register(
        class_name,
        "_setFileType",
        "(Ljava/lang/String;I)V",
        set_file_type,
    );
    registry.register(
        class_name,
        "_setFileTypeAndCreator",
        "(Ljava/lang/String;II)V",
        set_file_type_and_creator,
    );
    registry.register(
        class_name,
        "getNativePathToApplicationBundle",
        "()Ljava/lang/String;",
        get_native_path_to_application_bundle,
    );
    registry.register(
        class_name,
        "getNativeResourceFromBundle",
        "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
        get_native_resource_from_bundle,
    );
}

#[async_recursion(?Send)]
async fn find_folder(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.apple.eio.FileManager._findFolder(SIZ)Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn get_file_creator(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.apple.eio.FileManager._getFileCreator(Ljava/lang/String;)I")
}

#[async_recursion(?Send)]
async fn get_file_type(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.apple.eio.FileManager._getFileType(Ljava/lang/String;)I")
}

#[async_recursion(?Send)]
async fn move_to_trash(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.apple.eio.FileManager._moveToTrash(Ljava/lang/String;)Z")
}

#[async_recursion(?Send)]
async fn open_url(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.apple.eio.FileManager._openURL(Ljava/lang/String;)V")
}

#[async_recursion(?Send)]
async fn reveal_in_finder(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.apple.eio.FileManager._revealInFinder(Ljava/lang/String;)Z")
}

#[async_recursion(?Send)]
async fn set_file_creator(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.apple.eio.FileManager._setFileCreator(Ljava/lang/String;I)V")
}

#[async_recursion(?Send)]
async fn set_file_type(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.apple.eio.FileManager._setFileType(Ljava/lang/String;I)V")
}

#[async_recursion(?Send)]
async fn set_file_type_and_creator(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("com.apple.eio.FileManager._setFileTypeAndCreator(Ljava/lang/String;II)V")
}

#[async_recursion(?Send)]
async fn get_native_path_to_application_bundle(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("com.apple.eio.FileManager.getNativePathToApplicationBundle()Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn get_native_resource_from_bundle(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("com.apple.eio.FileManager.getNativeResourceFromBundle(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;")
}
