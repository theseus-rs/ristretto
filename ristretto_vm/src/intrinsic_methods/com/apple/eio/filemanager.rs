use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("com/apple/eio/FileManager._findFolder(SIZ)Ljava/lang/String;", Any)]
#[async_recursion(?Send)]
pub(crate) async fn find_folder(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.apple.eio.FileManager._findFolder(SIZ)Ljava/lang/String;")
}

#[intrinsic_method("com/apple/eio/FileManager._getFileCreator(Ljava/lang/String;)I", Any)]
#[async_recursion(?Send)]
pub(crate) async fn get_file_creator(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.apple.eio.FileManager._getFileCreator(Ljava/lang/String;)I")
}

#[intrinsic_method("com/apple/eio/FileManager._getFileType(Ljava/lang/String;)I", Any)]
#[async_recursion(?Send)]
pub(crate) async fn get_file_type(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.apple.eio.FileManager._getFileType(Ljava/lang/String;)I")
}

#[intrinsic_method("com/apple/eio/FileManager._moveToTrash(Ljava/lang/String;)Z", Any)]
#[async_recursion(?Send)]
pub(crate) async fn move_to_trash(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.apple.eio.FileManager._moveToTrash(Ljava/lang/String;)Z")
}

#[intrinsic_method("com/apple/eio/FileManager._openURL(Ljava/lang/String;)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn open_url(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.apple.eio.FileManager._openURL(Ljava/lang/String;)V")
}

#[intrinsic_method("com/apple/eio/FileManager._revealInFinder(Ljava/lang/String;)Z", Any)]
#[async_recursion(?Send)]
pub(crate) async fn reveal_in_finder(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.apple.eio.FileManager._revealInFinder(Ljava/lang/String;)Z")
}

#[intrinsic_method("com/apple/eio/FileManager._setFileCreator(Ljava/lang/String;I)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn set_file_creator(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.apple.eio.FileManager._setFileCreator(Ljava/lang/String;I)V")
}

#[intrinsic_method("com/apple/eio/FileManager._setFileType(Ljava/lang/String;I)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn set_file_type(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.apple.eio.FileManager._setFileType(Ljava/lang/String;I)V")
}

#[intrinsic_method(
    "com/apple/eio/FileManager._setFileTypeAndCreator(Ljava/lang/String;II)V",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn set_file_type_and_creator(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.apple.eio.FileManager._setFileTypeAndCreator(Ljava/lang/String;II)V")
}

#[intrinsic_method(
    "com/apple/eio/FileManager.getNativePathToApplicationBundle()Ljava/lang/String;",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn get_native_path_to_application_bundle(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.apple.eio.FileManager.getNativePathToApplicationBundle()Ljava/lang/String;")
}

#[intrinsic_method(
    "com/apple/eio/FileManager.getNativeResourceFromBundle(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn get_native_resource_from_bundle(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "com.apple.eio.FileManager.getNativeResourceFromBundle(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;"
    )
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
