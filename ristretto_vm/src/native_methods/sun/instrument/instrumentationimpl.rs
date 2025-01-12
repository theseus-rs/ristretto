use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_11: Version = Version::Java11 { minor: 0 };
const JAVA_21: Version = Version::Java21 { minor: 0 };

/// Register all native methods for `sun.instrument.InstrumentationImpl`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/instrument/InstrumentationImpl";
    let java_version = registry.java_version().clone();

    if java_version >= JAVA_11 {
        registry.register(
            class_name,
            "loadAgent0",
            "(Ljava/lang/String;)V",
            load_agent_0,
        );
        registry.register(
            class_name,
            "setHasTransformers",
            "(JZ)V",
            set_has_transformers,
        );
    }

    if java_version >= JAVA_21 {
        registry.register(class_name, "jarFile", "(J)Ljava/lang/String;", jar_file);
    }

    registry.register(
        class_name,
        "appendToClassLoaderSearch0",
        "(JLjava/lang/String;Z)V",
        append_to_class_loader_search_0,
    );
    registry.register(
        class_name,
        "getAllLoadedClasses0",
        "(J)[Ljava/lang/Class;",
        get_all_loaded_classes_0,
    );
    registry.register(
        class_name,
        "getInitiatedClasses0",
        "(JLjava/lang/ClassLoader;)[Ljava/lang/Class;",
        get_initiated_classes_0,
    );
    registry.register(
        class_name,
        "getObjectSize0",
        "(JLjava/lang/Object;)J",
        get_object_size_0,
    );
    registry.register(
        class_name,
        "isModifiableClass0",
        "(JLjava/lang/Class;)Z",
        is_modifiable_class_0,
    );
    registry.register(
        class_name,
        "isRetransformClassesSupported0",
        "(J)Z",
        is_retransform_classes_supported_0,
    );
    registry.register(
        class_name,
        "redefineClasses0",
        "(J[Ljava/lang/instrument/ClassDefinition;)V",
        redefine_classes_0,
    );
    registry.register(
        class_name,
        "retransformClasses0",
        "(J[Ljava/lang/Class;)V",
        retransform_classes_0,
    );
    registry.register(
        class_name,
        "setHasRetransformableTransformers",
        "(JZ)V",
        set_has_retransformable_transformers,
    );
    registry.register(
        class_name,
        "setNativeMethodPrefixes",
        "(J[Ljava/lang/String;Z)V",
        set_native_method_prefixes,
    );
}

#[async_recursion(?Send)]
async fn append_to_class_loader_search_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.instrument.InstrumentationImpl.appendToClassLoaderSearch0(JLjava/lang/String;Z)V")
}

#[async_recursion(?Send)]
async fn get_all_loaded_classes_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.instrument.InstrumentationImpl.getAllLoadedClasses0(J)[Ljava/lang/Class;")
}

#[async_recursion(?Send)]
async fn get_initiated_classes_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.instrument.InstrumentationImpl.getInitiatedClasses0(JLjava/lang/ClassLoader;)[Ljava/lang/Class;")
}

#[async_recursion(?Send)]
async fn get_object_size_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.instrument.InstrumentationImpl.getObjectSize0(JLjava/lang/Object;)J")
}

#[async_recursion(?Send)]
async fn is_modifiable_class_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.instrument.InstrumentationImpl.isModifiableClass0(JLjava/lang/Class;)Z")
}

#[async_recursion(?Send)]
async fn is_retransform_classes_supported_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.instrument.InstrumentationImpl.isRetransformClassesSupported0(J)Z")
}

#[async_recursion(?Send)]
async fn jar_file(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.instrument.InstrumentationImpl.jarFile(J)Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn load_agent_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.instrument.InstrumentationImpl.loadAgent0(Ljava/lang/String;)V")
}

#[async_recursion(?Send)]
async fn redefine_classes_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.instrument.InstrumentationImpl.redefineClasses0(J[Ljava/lang/instrument/ClassDefinition;)V")
}

#[async_recursion(?Send)]
async fn retransform_classes_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.instrument.InstrumentationImpl.retransformClasses0(J[Ljava/lang/Class;)V")
}

#[async_recursion(?Send)]
async fn set_has_retransformable_transformers(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.instrument.InstrumentationImpl.setHasRetransformableTransformers(JZ)V")
}

#[async_recursion(?Send)]
async fn set_has_transformers(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.instrument.InstrumentationImpl.setHasTransformers(JZ)V")
}

#[async_recursion(?Send)]
async fn set_native_method_prefixes(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.instrument.InstrumentationImpl.setNativeMethodPrefixes(J[Ljava/lang/String;Z)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::new(&Version::Java11 { minor: 0 }, true);
        register(&mut registry);
        let class_name = "sun/instrument/InstrumentationImpl";
        assert!(registry
            .method(class_name, "loadAgent0", "(Ljava/lang/String;)V")
            .is_some());
        assert!(registry
            .method(class_name, "setHasTransformers", "(JZ)V")
            .is_some());
        assert!(registry
            .method(class_name, "jarFile", "(J)Ljava/lang/String;")
            .is_none());
        assert!(registry
            .method(
                class_name,
                "appendToClassLoaderSearch0",
                "(JLjava/lang/String;Z)V"
            )
            .is_some());
        assert!(registry
            .method(class_name, "getAllLoadedClasses0", "(J)[Ljava/lang/Class;")
            .is_some());
        assert!(registry
            .method(
                class_name,
                "getInitiatedClasses0",
                "(JLjava/lang/ClassLoader;)[Ljava/lang/Class;"
            )
            .is_some());
        assert!(registry
            .method(class_name, "getObjectSize0", "(JLjava/lang/Object;)J")
            .is_some());
        assert!(registry
            .method(class_name, "isModifiableClass0", "(JLjava/lang/Class;)Z")
            .is_some());
        assert!(registry
            .method(class_name, "isRetransformClassesSupported0", "(J)Z")
            .is_some());
        assert!(registry
            .method(
                class_name,
                "redefineClasses0",
                "(J[Ljava/lang/instrument/ClassDefinition;)V"
            )
            .is_some());
        assert!(registry
            .method(class_name, "retransformClasses0", "(J[Ljava/lang/Class;)V")
            .is_some());
        assert!(registry
            .method(class_name, "setHasRetransformableTransformers", "(JZ)V")
            .is_some());
        assert!(registry
            .method(
                class_name,
                "setNativeMethodPrefixes",
                "(J[Ljava/lang/String;Z)V"
            )
            .is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.instrument.InstrumentationImpl.loadAgent0(Ljava/lang/String;)V")]
    async fn test_load_agent_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = load_agent_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.instrument.InstrumentationImpl.setHasTransformers(JZ)V")]
    async fn test_set_has_transformers() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_has_transformers(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.instrument.InstrumentationImpl.jarFile(J)Ljava/lang/String;")]
    async fn test_jar_file() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = jar_file(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.instrument.InstrumentationImpl.appendToClassLoaderSearch0(JLjava/lang/String;Z)V"
    )]
    async fn test_append_to_class_loader_search_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = append_to_class_loader_search_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.instrument.InstrumentationImpl.getAllLoadedClasses0(J)[Ljava/lang/Class;"
    )]
    async fn test_get_all_loaded_classes_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_all_loaded_classes_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.instrument.InstrumentationImpl.getInitiatedClasses0(JLjava/lang/ClassLoader;)[Ljava/lang/Class;"
    )]
    async fn test_get_initiated_classes_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_initiated_classes_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.instrument.InstrumentationImpl.getObjectSize0(JLjava/lang/Object;)J"
    )]
    async fn test_get_object_size_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_object_size_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.instrument.InstrumentationImpl.isModifiableClass0(JLjava/lang/Class;)Z"
    )]
    async fn test_is_modifiable_class_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_modifiable_class_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.instrument.InstrumentationImpl.isRetransformClassesSupported0(J)Z"
    )]
    async fn test_is_retransform_classes_supported_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_retransform_classes_supported_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.instrument.InstrumentationImpl.redefineClasses0(J[Ljava/lang/instrument/ClassDefinition;)V"
    )]
    async fn test_redefine_classes_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = redefine_classes_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.instrument.InstrumentationImpl.retransformClasses0(J[Ljava/lang/Class;)V"
    )]
    async fn test_retransform_classes_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = retransform_classes_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.instrument.InstrumentationImpl.setHasRetransformableTransformers(JZ)V"
    )]
    async fn test_set_has_retransformable_transformers() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_has_retransformable_transformers(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.instrument.InstrumentationImpl.setNativeMethodPrefixes(J[Ljava/lang/String;Z)V"
    )]
    async fn test_set_native_method_prefixes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_native_method_prefixes(thread, Arguments::default()).await;
    }
}
