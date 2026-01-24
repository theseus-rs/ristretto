use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "com/sun/demo/jvmti/hprof/Tracker.nativeCallSite(Ljava/lang/Object;II)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn native_call_site(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.demo.jvmti.hprof.Tracker.nativeCallSite(Ljava/lang/Object;II)V")
}

#[intrinsic_method(
    "com/sun/demo/jvmti/hprof/Tracker.nativeNewArray(Ljava/lang/Object;Ljava/lang/Object;)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn native_new_array(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.demo.jvmti.hprof.Tracker.nativeNewArray(Ljava/lang/Object;Ljava/lang/Object;)V")
}

#[intrinsic_method(
    "com/sun/demo/jvmti/hprof/Tracker.nativeObjectInit(Ljava/lang/Object;Ljava/lang/Object;)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn native_object_init(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "com.sun.demo.jvmti.hprof.Tracker.nativeObjectInit(Ljava/lang/Object;Ljava/lang/Object;)V"
    )
}

#[intrinsic_method(
    "com/sun/demo/jvmti/hprof/Tracker.nativeReturnSite(Ljava/lang/Object;II)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn native_return_site(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.demo.jvmti.hprof.Tracker.nativeReturnSite(Ljava/lang/Object;II)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.demo.jvmti.hprof.Tracker.nativeCallSite(Ljava/lang/Object;II)V"
    )]
    async fn test_native_call_site() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_call_site(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.demo.jvmti.hprof.Tracker.nativeNewArray(Ljava/lang/Object;Ljava/lang/Object;)V"
    )]
    async fn test_native_new_array() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_new_array(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.demo.jvmti.hprof.Tracker.nativeObjectInit(Ljava/lang/Object;Ljava/lang/Object;)V"
    )]
    async fn test_native_object_init() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_object_init(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.demo.jvmti.hprof.Tracker.nativeReturnSite(Ljava/lang/Object;II)V"
    )]
    async fn test_native_return_site() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_return_site(thread, Parameters::default()).await;
    }
}
