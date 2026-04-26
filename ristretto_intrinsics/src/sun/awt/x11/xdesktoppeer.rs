use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/awt/X11/XDesktopPeer.gnome_url_show([B)Z", Any)]
#[async_method]
pub async fn gnome_url_show<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _url = parameters.pop_reference()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XDesktopPeer.gnome_url_show([B)Z".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XDesktopPeer.init(IZ)Z", Any)]
#[async_method]
pub async fn init<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _verbose = parameters.pop_bool()?;
    let _gtk_version = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError("sun/awt/X11/XDesktopPeer.init(IZ)Z".to_string()).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_gnome_url_show() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = gnome_url_show(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/awt/X11/XDesktopPeer.gnome_url_show([B)Z",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_init() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init(
            thread,
            Parameters::new(vec![Value::Int(0), Value::from(false)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XDesktopPeer.init(IZ)Z",
            result.unwrap_err().to_string()
        );
    }
}
