use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/lwawt/macosx/CDesktopPeer._lsOpenFile(Ljava/lang/String;ILjava/lang/String;)I",
    Any
)]
#[async_method]
pub async fn ls_open_file<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg2 = parameters.pop_reference()?;
    let _arg1 = parameters.pop_int()?;
    let _arg0 = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CDesktopPeer._lsOpenFile(Ljava/lang/String;ILjava/lang/String;)I"
            .to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/CDesktopPeer._lsOpenURI(Ljava/lang/String;I)I", Any)]
#[async_method]
pub async fn ls_open_uri<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg1 = parameters.pop_int()?;
    let _arg0 = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CDesktopPeer._lsOpenURI(Ljava/lang/String;I)I".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ls_open_file() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = ls_open_file(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Int(0),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "sun.lwawt.macosx.CDesktopPeer._lsOpenFile(Ljava/lang/String;ILjava/lang/String;)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_ls_open_uri() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = ls_open_uri(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun.lwawt.macosx.CDesktopPeer._lsOpenURI(Ljava/lang/String;I)I",
            result.unwrap_err().to_string()
        );
    }
}
