use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/awt/X11/GtkFileDialogPeer.initIDs()V", Any)]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/GtkFileDialogPeer.initIDs()V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/GtkFileDialogPeer.quit()V", Any)]
#[async_method]
pub async fn quit<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun/awt/X11/GtkFileDialogPeer.quit()V".to_string()).into())
}
#[intrinsic_method(
    "sun/awt/X11/GtkFileDialogPeer.run(Ljava/lang/String;ILjava/lang/String;Ljava/lang/String;Ljava/io/FilenameFilter;ZII)V",
    Any
)]
#[async_method]
pub async fn run<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _multiple = parameters.pop_bool()?;
    let _jfilter = parameters.pop_reference()?;
    let _jfile = parameters.pop_reference()?;
    let _jdir = parameters.pop_reference()?;
    let _mode = parameters.pop_int()?;
    let _jtitle = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun/awt/X11/GtkFileDialogPeer.run(Ljava/lang/String;ILjava/lang/String;Ljava/lang/String;Ljava/io/FilenameFilter;ZII)V".to_string()).into())
}
#[intrinsic_method("sun/awt/X11/GtkFileDialogPeer.setBounds(IIIII)V", Any)]
#[async_method]
pub async fn set_bounds<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _op = parameters.pop_int()?;
    let _height = parameters.pop_int()?;
    let _width = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/GtkFileDialogPeer.setBounds(IIIII)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/GtkFileDialogPeer.toFront()V", Any)]
#[async_method]
pub async fn to_front<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/GtkFileDialogPeer.toFront()V".to_string())
            .into(),
    )
}
#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_init_ids() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_ids(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/X11/GtkFileDialogPeer.initIDs()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_quit() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = quit(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/X11/GtkFileDialogPeer.quit()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_run() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = run(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Int(0),
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
                Value::from(false),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/GtkFileDialogPeer.run(Ljava/lang/String;ILjava/lang/String;Ljava/lang/String;Ljava/io/FilenameFilter;ZII)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_set_bounds() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_bounds(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/GtkFileDialogPeer.setBounds(IIIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_to_front() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = to_front(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/X11/GtkFileDialogPeer.toFront()V",
            result.unwrap_err().to_string()
        );
    }
}
