use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("sun/net/PortConfig.getLower0()I", Any)]
#[async_recursion(?Send)]
pub(crate) async fn get_lower_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.net.PortConfig.getLower0()I")
}

#[intrinsic_method("sun/net/PortConfig.getUpper0()I", Any)]
#[async_recursion(?Send)]
pub(crate) async fn get_upper_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.net.PortConfig.getUpper0()I")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.net.PortConfig.getLower0()I")]
    async fn test_get_lower_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_lower_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.net.PortConfig.getUpper0()I")]
    async fn test_get_upper_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_upper_0(thread, Parameters::default()).await;
    }
}
