use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/lwawt/macosx/CFileDialog.nativeRunFileDialog(Ljava/lang/String;IZZZZLjava/lang/String;Ljava/lang/String;)[Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn native_run_file_dialog<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _file = parameters.pop_reference()?;
    let _directory = parameters.pop_reference()?;
    let _has_filter = parameters.pop_bool()?;
    let _choose_directories = parameters.pop_bool()?;
    let _navigate_apps = parameters.pop_bool()?;
    let _multiple_mode = parameters.pop_bool()?;
    let _mode = parameters.pop_int()?;
    let _title = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun.lwawt.macosx.CFileDialog.nativeRunFileDialog(Ljava/lang/String;IZZZZLjava/lang/String;Ljava/lang/String;)[Ljava/lang/String;".to_string()).into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_native_run_file_dialog() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_run_file_dialog(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Int(0),
                Value::from(false),
                Value::from(false),
                Value::from(false),
                Value::from(false),
                Value::Object(None),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "sun.lwawt.macosx.CFileDialog.nativeRunFileDialog(Ljava/lang/String;IZZZZLjava/lang/String;Ljava/lang/String;)[Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }
}
