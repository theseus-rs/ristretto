use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("com/apple/eio/FileManager._findFolder(SIZ)Ljava/lang/String;", Any)]
#[async_method]
pub async fn find_folder<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _create_if_needed = parameters.pop_bool()?;
    let _folder_type = parameters.pop_int()?;
    let _domain = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.apple.eio.FileManager._findFolder(SIZ)Ljava/lang/String;".to_string(),
    )
    .into())
}

#[intrinsic_method("com/apple/eio/FileManager._getFileCreator(Ljava/lang/String;)I", Any)]
#[async_method]
pub async fn get_file_creator<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg0 = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.apple.eio.FileManager._getFileCreator(Ljava/lang/String;)I".to_string(),
    )
    .into())
}

#[intrinsic_method("com/apple/eio/FileManager._getFileType(Ljava/lang/String;)I", Any)]
#[async_method]
pub async fn get_file_type<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg0 = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.apple.eio.FileManager._getFileType(Ljava/lang/String;)I".to_string(),
    )
    .into())
}

#[intrinsic_method("com/apple/eio/FileManager._moveToTrash(Ljava/lang/String;)Z", Any)]
#[async_method]
pub async fn move_to_trash<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg0 = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.apple.eio.FileManager._moveToTrash(Ljava/lang/String;)Z".to_string(),
    )
    .into())
}

#[intrinsic_method("com/apple/eio/FileManager._openURL(Ljava/lang/String;)V", Any)]
#[async_method]
pub async fn open_url<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg0 = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.apple.eio.FileManager._openURL(Ljava/lang/String;)V".to_string(),
    )
    .into())
}

#[intrinsic_method("com/apple/eio/FileManager._revealInFinder(Ljava/lang/String;)Z", Any)]
#[async_method]
pub async fn reveal_in_finder<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg0 = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.apple.eio.FileManager._revealInFinder(Ljava/lang/String;)Z".to_string(),
    )
    .into())
}

#[intrinsic_method("com/apple/eio/FileManager._setFileCreator(Ljava/lang/String;I)V", Any)]
#[async_method]
pub async fn set_file_creator<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg1 = parameters.pop_int()?;
    let _arg0 = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.apple.eio.FileManager._setFileCreator(Ljava/lang/String;I)V".to_string(),
    )
    .into())
}

#[intrinsic_method("com/apple/eio/FileManager._setFileType(Ljava/lang/String;I)V", Any)]
#[async_method]
pub async fn set_file_type<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg1 = parameters.pop_int()?;
    let _arg0 = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.apple.eio.FileManager._setFileType(Ljava/lang/String;I)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/apple/eio/FileManager._setFileTypeAndCreator(Ljava/lang/String;II)V",
    Any
)]
#[async_method]
pub async fn set_file_type_and_creator<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg2 = parameters.pop_int()?;
    let _arg1 = parameters.pop_int()?;
    let _arg0 = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.apple.eio.FileManager._setFileTypeAndCreator(Ljava/lang/String;II)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/apple/eio/FileManager.getNativePathToApplicationBundle()Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_native_path_to_application_bundle<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.apple.eio.FileManager.getNativePathToApplicationBundle()Ljava/lang/String;"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/apple/eio/FileManager.getNativeResourceFromBundle(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_native_resource_from_bundle<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _java_type_name = parameters.pop_reference()?;
    let _java_sub_dir_name = parameters.pop_reference()?;
    let _java_resource_name = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("com.apple.eio.FileManager.getNativeResourceFromBundle(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;".to_string()).into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_find_folder() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = find_folder(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Int(0), Value::from(false)]),
        )
        .await;
        assert_eq!(
            "com.apple.eio.FileManager._findFolder(SIZ)Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_file_creator() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_file_creator(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "com.apple.eio.FileManager._getFileCreator(Ljava/lang/String;)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_file_type() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_file_type(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "com.apple.eio.FileManager._getFileType(Ljava/lang/String;)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_move_to_trash() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = move_to_trash(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "com.apple.eio.FileManager._moveToTrash(Ljava/lang/String;)Z",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_open_url() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = open_url(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "com.apple.eio.FileManager._openURL(Ljava/lang/String;)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_reveal_in_finder() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = reveal_in_finder(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "com.apple.eio.FileManager._revealInFinder(Ljava/lang/String;)Z",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_set_file_creator() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_file_creator(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "com.apple.eio.FileManager._setFileCreator(Ljava/lang/String;I)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_set_file_type() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_file_type(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "com.apple.eio.FileManager._setFileType(Ljava/lang/String;I)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_set_file_type_and_creator() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_file_type_and_creator(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Int(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "com.apple.eio.FileManager._setFileTypeAndCreator(Ljava/lang/String;II)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_native_path_to_application_bundle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_native_path_to_application_bundle(thread, Parameters::default()).await;
        assert_eq!(
            "com.apple.eio.FileManager.getNativePathToApplicationBundle()Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_native_resource_from_bundle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_native_resource_from_bundle(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "com.apple.eio.FileManager.getNativeResourceFromBundle(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }
}
