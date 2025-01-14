use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `com.sun.demo.jvmti.hprof.Tracker`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "com/sun/demo/jvmti/hprof/Tracker";
    registry.register(
        class_name,
        "nativeCallSite",
        "(Ljava/lang/Object;II)V",
        native_call_site,
    );
    registry.register(
        class_name,
        "nativeNewArray",
        "(Ljava/lang/Object;Ljava/lang/Object;)V",
        native_new_array,
    );
    registry.register(
        class_name,
        "nativeObjectInit",
        "(Ljava/lang/Object;Ljava/lang/Object;)V",
        native_object_init,
    );
    registry.register(
        class_name,
        "nativeReturnSite",
        "(Ljava/lang/Object;II)V",
        native_return_site,
    );
}

#[async_recursion(?Send)]
async fn native_call_site(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.demo.jvmti.hprof.Tracker.nativeCallSite(Ljava/lang/Object;II)V")
}

#[async_recursion(?Send)]
async fn native_new_array(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.demo.jvmti.hprof.Tracker.nativeNewArray(Ljava/lang/Object;Ljava/lang/Object;)V")
}

#[async_recursion(?Send)]
async fn native_object_init(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!(
        "com.sun.demo.jvmti.hprof.Tracker.nativeObjectInit(Ljava/lang/Object;Ljava/lang/Object;)V"
    )
}

#[async_recursion(?Send)]
async fn native_return_site(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.demo.jvmti.hprof.Tracker.nativeReturnSite(Ljava/lang/Object;II)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "com/sun/demo/jvmti/hprof/Tracker";
        assert!(registry
            .method(class_name, "nativeCallSite", "(Ljava/lang/Object;II)V")
            .is_some());
        assert!(registry
            .method(
                class_name,
                "nativeNewArray",
                "(Ljava/lang/Object;Ljava/lang/Object;)V"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "nativeObjectInit",
                "(Ljava/lang/Object;Ljava/lang/Object;)V"
            )
            .is_some());
        assert!(registry
            .method(class_name, "nativeReturnSite", "(Ljava/lang/Object;II)V")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.demo.jvmti.hprof.Tracker.nativeCallSite(Ljava/lang/Object;II)V"
    )]
    async fn test_native_call_site() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_call_site(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.demo.jvmti.hprof.Tracker.nativeNewArray(Ljava/lang/Object;Ljava/lang/Object;)V"
    )]
    async fn test_native_new_array() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_new_array(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.demo.jvmti.hprof.Tracker.nativeObjectInit(Ljava/lang/Object;Ljava/lang/Object;)V"
    )]
    async fn test_native_object_init() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_object_init(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.demo.jvmti.hprof.Tracker.nativeReturnSite(Ljava/lang/Object;II)V"
    )]
    async fn test_native_return_site() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_return_site(thread, Arguments::default()).await;
    }
}
