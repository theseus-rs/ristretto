use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/nio/ch/WindowsSelectorImpl$SubSelector.poll0(JI[I[I[IJJ)I", Any)]
#[async_method]
pub async fn poll0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _fds_buffer = parameters.pop_long()?;
    let _timeout = parameters.pop_long()?;
    let _return_except_fds = parameters.pop_reference()?;
    let _return_write_fds = parameters.pop_reference()?;
    let _return_read_fds = parameters.pop_reference()?;
    let _numfds = parameters.pop_int()?;
    let _poll_address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/ch/WindowsSelectorImpl$SubSelector.poll0(JI[I[I[IJJ)I".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_poll0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = poll0(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Int(0),
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
                Value::Long(0),
                Value::Long(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/nio/ch/WindowsSelectorImpl$SubSelector.poll0(JI[I[I[IJJ)I",
            result.unwrap_err().to_string()
        );
    }
}
