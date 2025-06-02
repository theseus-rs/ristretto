use crate::Result;
use crate::intrinsic_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "apple/laf/JRSUIConstants";

/// Register all intrinsic methods for `apple.laf.JRSUIConstants`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "getPtrForConstant",
        "(I)J",
        get_ptr_for_constant,
    );
}

#[async_recursion(?Send)]
async fn get_ptr_for_constant(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("apple.laf.JRSUIConstants.getPtrForConstant(I)J")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: apple.laf.JRSUIConstants.getPtrForConstant(I)J"
    )]
    async fn test_get_ptr_for_constant() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_ptr_for_constant(thread, Parameters::default()).await;
    }
}
