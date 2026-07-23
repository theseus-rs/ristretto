use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/nio/ch/UnixAsynchronousSocketChannelImpl.checkConnect(I)V", Any)]
#[async_method]
pub async fn check_connect<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let fd = parameters.pop_int()?;
    let vm = thread.vm()?;
    let _ = super::net::check_connect(&*vm, fd, false).await?;
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_check_connect() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = check_connect(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert!(result.is_err());
    }
}
