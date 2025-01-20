use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "com/apple/eio/FileManager";

/// Register all native methods for `com.apple.eio.FileManager`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "_findFolder",
        "(SIZ)Ljava/lang/String;",
        find_folder,
    );
    registry.register(
        CLASS_NAME,
        "_getFileCreator",
        "(Ljava/lang/String;)I",
        get_file_creator,
    );
    registry.register(
        CLASS_NAME,
        "_getFileType",
        "(Ljava/lang/String;)I",
        get_file_type,
    );
    registry.register(
        CLASS_NAME,
        "_moveToTrash",
        "(Ljava/lang/String;)Z",
        move_to_trash,
    );
    registry.register(CLASS_NAME, "_openURL", "(Ljava/lang/String;)V", open_url);
    registry.register(
        CLASS_NAME,
        "_revealInFinder",
        "(Ljava/lang/String;)Z",
        reveal_in_finder,
    );
    registry.register(
        CLASS_NAME,
        "_setFileCreator",
        "(Ljava/lang/String;I)V",
        set_file_creator,
    );
    registry.register(
        CLASS_NAME,
        "_setFileType",
        "(Ljava/lang/String;I)V",
        set_file_type,
    );
    registry.register(
        CLASS_NAME,
        "_setFileTypeAndCreator",
        "(Ljava/lang/String;II)V",
        set_file_type_and_creator,
    );
    registry.register(
        CLASS_NAME,
        "getNativePathToApplicationBundle",
        "()Ljava/lang/String;",
        get_native_path_to_application_bundle,
    );
    registry.register(
        CLASS_NAME,
        "getNativeResourceFromBundle",
        "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
        get_native_resource_from_bundle,
    );
}

#[async_recursion(?Send)]
async fn find_folder(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("com.apple.eio.FileManager._findFolder(SIZ)Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn get_file_creator(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("com.apple.eio.FileManager._getFileCreator(Ljava/lang/String;)I")
}

#[async_recursion(?Send)]
async fn get_file_type(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("com.apple.eio.FileManager._getFileType(Ljava/lang/String;)I")
}

#[async_recursion(?Send)]
async fn move_to_trash(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("com.apple.eio.FileManager._moveToTrash(Ljava/lang/String;)Z")
}

#[async_recursion(?Send)]
async fn open_url(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("com.apple.eio.FileManager._openURL(Ljava/lang/String;)V")
}

#[async_recursion(?Send)]
async fn reveal_in_finder(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("com.apple.eio.FileManager._revealInFinder(Ljava/lang/String;)Z")
}

#[async_recursion(?Send)]
async fn set_file_creator(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("com.apple.eio.FileManager._setFileCreator(Ljava/lang/String;I)V")
}

#[async_recursion(?Send)]
async fn set_file_type(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("com.apple.eio.FileManager._setFileType(Ljava/lang/String;I)V")
}

#[async_recursion(?Send)]
async fn set_file_type_and_creator(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.apple.eio.FileManager._setFileTypeAndCreator(Ljava/lang/String;II)V")
}

#[async_recursion(?Send)]
async fn get_native_path_to_application_bundle(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.apple.eio.FileManager.getNativePathToApplicationBundle()Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn get_native_resource_from_bundle(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.apple.eio.FileManager.getNativeResourceFromBundle(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.apple.eio.FileManager._findFolder(SIZ)Ljava/lang/String;"
    )]
    async fn test_find_folder() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = find_folder(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.apple.eio.FileManager._getFileCreator(Ljava/lang/String;)I"
    )]
    async fn test_get_file_creator() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_file_creator(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.apple.eio.FileManager._getFileType(Ljava/lang/String;)I"
    )]
    async fn test_get_file_type() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_file_type(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.apple.eio.FileManager._moveToTrash(Ljava/lang/String;)Z"
    )]
    async fn test_move_to_trash() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = move_to_trash(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.apple.eio.FileManager._openURL(Ljava/lang/String;)V"
    )]
    async fn test_open_url() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = open_url(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.apple.eio.FileManager._revealInFinder(Ljava/lang/String;)Z"
    )]
    async fn test_reveal_in_finder() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = reveal_in_finder(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.apple.eio.FileManager._setFileCreator(Ljava/lang/String;I)V"
    )]
    async fn test_set_file_creator() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_file_creator(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.apple.eio.FileManager._setFileType(Ljava/lang/String;I)V"
    )]
    async fn test_set_file_type() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_file_type(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.apple.eio.FileManager._setFileTypeAndCreator(Ljava/lang/String;II)V"
    )]
    async fn test_set_file_type_and_creator() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_file_type_and_creator(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.apple.eio.FileManager.getNativePathToApplicationBundle()Ljava/lang/String;"
    )]
    async fn test_get_native_path_to_application_bundle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_native_path_to_application_bundle(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.apple.eio.FileManager.getNativeResourceFromBundle(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;"
    )]
    async fn test_get_native_resource_from_bundle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_native_resource_from_bundle(thread, Parameters::default()).await;
    }
}
