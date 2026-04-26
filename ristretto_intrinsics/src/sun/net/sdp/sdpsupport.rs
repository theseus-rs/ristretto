use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/net/sdp/SdpSupport.convert0(I)V", Any)]
#[async_method]
pub async fn convert_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _fd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError("sun.net.sdp.SdpSupport.convert0(I)V".to_string()).into())
}

#[intrinsic_method("sun/net/sdp/SdpSupport.create0()I", Any)]
#[async_method]
pub async fn create_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.net.sdp.SdpSupport.create0()I".to_string()).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_convert_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = convert_0(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun.net.sdp.SdpSupport.convert0(I)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_create_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create_0(thread, Parameters::default()).await;
        assert_eq!(
            "sun.net.sdp.SdpSupport.create0()I",
            result.unwrap_err().to_string()
        );
    }
}
