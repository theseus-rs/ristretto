use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::JAVA_25;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "jdk/vm/ci/services/Services.readSystemPropertiesInfo([I)J",
    GreaterThanOrEqual(JAVA_25)
)]
#[async_method]
pub(crate) async fn read_system_properties_info(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.vm.ci.services.Services.readSystemPropertiesInfo([I)J")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.vm.ci.services.Services.readSystemPropertiesInfo([I)J"
    )]
    async fn test_read_system_properties_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = read_system_properties_info(thread, Parameters::default()).await;
    }
}
