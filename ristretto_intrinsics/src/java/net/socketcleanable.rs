use ristretto_classfile::VersionSpecification::Between;
use ristretto_classfile::{JAVA_11, JAVA_17};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "java/net/SocketCleanable.cleanupClose0(I)V",
    Between(JAVA_11, JAVA_17)
)]
#[async_method]
pub async fn cleanup_close_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let fd = parameters.pop_int()?;
    #[cfg(not(target_family = "wasm"))]
    if fd >= 0 {
        crate::net_helpers::close_socket(thread.vm()?.as_ref(), fd).await;
    }
    #[cfg(target_family = "wasm")]
    let _ = (thread, fd);
    Ok(None)
}

#[cfg(all(test, not(target_family = "wasm")))]
mod tests {
    use super::*;
    use ristretto_types::VM;

    #[tokio::test]
    async fn test_cleanup_close_0() {
        let (vm, thread) = crate::test::java17_thread().await.expect("thread");
        let fd = super::super::socket_ops::create(vm.as_ref(), true, false)
            .await
            .expect("socket");
        let mut parameters = Parameters::default();
        parameters.push_int(fd);
        let result = cleanup_close_0(thread, parameters)
            .await
            .expect("cleanup result");
        assert_eq!(None, result);
        assert!(vm.socket_handles().get(&fd).await.is_none());
    }
}
