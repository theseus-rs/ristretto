use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/nio/ch/PollSelectorImpl.poll(JII)I", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn poll<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let timeout = parameters.pop_int()?;
    let num_fds = parameters.pop_int()?;
    let poll_address = parameters.pop_long()?;
    let result = super::posix::poll(&thread, poll_address, num_fds, i64::from(timeout)).await?;
    Ok(Some(Value::Int(result)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_poll() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = poll(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Int(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(Some(Value::Int(0)), result.expect("poll"));
    }
}
