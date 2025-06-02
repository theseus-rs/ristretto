use crate::Result;
use crate::intrinsic_methods::registry::{JAVA_11, JAVA_21, MethodRegistry};
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/instrument/InstrumentationImpl";

/// Register all intrinsic methods for `sun.instrument.InstrumentationImpl`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() >= JAVA_11 {
        registry.register(
            CLASS_NAME,
            "loadAgent0",
            "(Ljava/lang/String;)V",
            load_agent_0,
        );
        registry.register(
            CLASS_NAME,
            "setHasTransformers",
            "(JZ)V",
            set_has_transformers,
        );
    }

    if registry.java_major_version() >= JAVA_21 {
        registry.register(CLASS_NAME, "jarFile", "(J)Ljava/lang/String;", jar_file);
    }

    registry.register(
        CLASS_NAME,
        "appendToClassLoaderSearch0",
        "(JLjava/lang/String;Z)V",
        append_to_class_loader_search_0,
    );
    registry.register(
        CLASS_NAME,
        "getAllLoadedClasses0",
        "(J)[Ljava/lang/Class;",
        get_all_loaded_classes_0,
    );
    registry.register(
        CLASS_NAME,
        "getInitiatedClasses0",
        "(JLjava/lang/ClassLoader;)[Ljava/lang/Class;",
        get_initiated_classes_0,
    );
    registry.register(
        CLASS_NAME,
        "getObjectSize0",
        "(JLjava/lang/Object;)J",
        get_object_size_0,
    );
    registry.register(
        CLASS_NAME,
        "isModifiableClass0",
        "(JLjava/lang/Class;)Z",
        is_modifiable_class_0,
    );
    registry.register(
        CLASS_NAME,
        "isRetransformClassesSupported0",
        "(J)Z",
        is_retransform_classes_supported_0,
    );
    registry.register(
        CLASS_NAME,
        "redefineClasses0",
        "(J[Ljava/lang/instrument/ClassDefinition;)V",
        redefine_classes_0,
    );
    registry.register(
        CLASS_NAME,
        "retransformClasses0",
        "(J[Ljava/lang/Class;)V",
        retransform_classes_0,
    );
    registry.register(
        CLASS_NAME,
        "setHasRetransformableTransformers",
        "(JZ)V",
        set_has_retransformable_transformers,
    );
    registry.register(
        CLASS_NAME,
        "setNativeMethodPrefixes",
        "(J[Ljava/lang/String;Z)V",
        set_native_method_prefixes,
    );
}

#[async_recursion(?Send)]
async fn append_to_class_loader_search_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.instrument.InstrumentationImpl.appendToClassLoaderSearch0(JLjava/lang/String;Z)V")
}

#[async_recursion(?Send)]
async fn get_all_loaded_classes_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.instrument.InstrumentationImpl.getAllLoadedClasses0(J)[Ljava/lang/Class;")
}

#[async_recursion(?Send)]
async fn get_initiated_classes_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.instrument.InstrumentationImpl.getInitiatedClasses0(JLjava/lang/ClassLoader;)[Ljava/lang/Class;"
    )
}

#[async_recursion(?Send)]
async fn get_object_size_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.instrument.InstrumentationImpl.getObjectSize0(JLjava/lang/Object;)J")
}

#[async_recursion(?Send)]
async fn is_modifiable_class_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.instrument.InstrumentationImpl.isModifiableClass0(JLjava/lang/Class;)Z")
}

#[async_recursion(?Send)]
async fn is_retransform_classes_supported_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.instrument.InstrumentationImpl.isRetransformClassesSupported0(J)Z")
}

#[async_recursion(?Send)]
async fn jar_file(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.instrument.InstrumentationImpl.jarFile(J)Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn load_agent_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.instrument.InstrumentationImpl.loadAgent0(Ljava/lang/String;)V")
}

#[async_recursion(?Send)]
async fn redefine_classes_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.instrument.InstrumentationImpl.redefineClasses0(J[Ljava/lang/instrument/ClassDefinition;)V"
    )
}

#[async_recursion(?Send)]
async fn retransform_classes_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.instrument.InstrumentationImpl.retransformClasses0(J[Ljava/lang/Class;)V")
}

#[async_recursion(?Send)]
async fn set_has_retransformable_transformers(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.instrument.InstrumentationImpl.setHasRetransformableTransformers(JZ)V")
}

#[async_recursion(?Send)]
async fn set_has_transformers(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.instrument.InstrumentationImpl.setHasTransformers(JZ)V")
}

#[async_recursion(?Send)]
async fn set_native_method_prefixes(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.instrument.InstrumentationImpl.setNativeMethodPrefixes(J[Ljava/lang/String;Z)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.instrument.InstrumentationImpl.loadAgent0(Ljava/lang/String;)V"
    )]
    async fn test_load_agent_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = load_agent_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.instrument.InstrumentationImpl.setHasTransformers(JZ)V"
    )]
    async fn test_set_has_transformers() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_has_transformers(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.instrument.InstrumentationImpl.jarFile(J)Ljava/lang/String;"
    )]
    async fn test_jar_file() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = jar_file(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.instrument.InstrumentationImpl.appendToClassLoaderSearch0(JLjava/lang/String;Z)V"
    )]
    async fn test_append_to_class_loader_search_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = append_to_class_loader_search_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.instrument.InstrumentationImpl.getAllLoadedClasses0(J)[Ljava/lang/Class;"
    )]
    async fn test_get_all_loaded_classes_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_all_loaded_classes_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.instrument.InstrumentationImpl.getInitiatedClasses0(JLjava/lang/ClassLoader;)[Ljava/lang/Class;"
    )]
    async fn test_get_initiated_classes_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_initiated_classes_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.instrument.InstrumentationImpl.getObjectSize0(JLjava/lang/Object;)J"
    )]
    async fn test_get_object_size_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_object_size_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.instrument.InstrumentationImpl.isModifiableClass0(JLjava/lang/Class;)Z"
    )]
    async fn test_is_modifiable_class_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_modifiable_class_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.instrument.InstrumentationImpl.isRetransformClassesSupported0(J)Z"
    )]
    async fn test_is_retransform_classes_supported_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_retransform_classes_supported_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.instrument.InstrumentationImpl.redefineClasses0(J[Ljava/lang/instrument/ClassDefinition;)V"
    )]
    async fn test_redefine_classes_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = redefine_classes_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.instrument.InstrumentationImpl.retransformClasses0(J[Ljava/lang/Class;)V"
    )]
    async fn test_retransform_classes_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = retransform_classes_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.instrument.InstrumentationImpl.setHasRetransformableTransformers(JZ)V"
    )]
    async fn test_set_has_retransformable_transformers() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_has_retransformable_transformers(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.instrument.InstrumentationImpl.setNativeMethodPrefixes(J[Ljava/lang/String;Z)V"
    )]
    async fn test_set_native_method_prefixes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_native_method_prefixes(thread, Parameters::default()).await;
    }
}
