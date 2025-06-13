use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::{Any, GreaterThanOrEqual};
use ristretto_classfile::{JAVA_11, JAVA_21};
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "sun/instrument/InstrumentationImpl.appendToClassLoaderSearch0(JLjava/lang/String;Z)V",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn append_to_class_loader_search_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.instrument.InstrumentationImpl.appendToClassLoaderSearch0(JLjava/lang/String;Z)V")
}

#[intrinsic_method(
    "sun/instrument/InstrumentationImpl.getAllLoadedClasses0(J)[Ljava/lang/Class;",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn get_all_loaded_classes_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.instrument.InstrumentationImpl.getAllLoadedClasses0(J)[Ljava/lang/Class;")
}

#[intrinsic_method(
    "sun/instrument/InstrumentationImpl.getInitiatedClasses0(JLjava/lang/ClassLoader;)[Ljava/lang/Class;",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn get_initiated_classes_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.instrument.InstrumentationImpl.getInitiatedClasses0(JLjava/lang/ClassLoader;)[Ljava/lang/Class;"
    )
}

#[intrinsic_method(
    "sun/instrument/InstrumentationImpl.getObjectSize0(JLjava/lang/Object;)J",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn get_object_size_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.instrument.InstrumentationImpl.getObjectSize0(JLjava/lang/Object;)J")
}

#[intrinsic_method(
    "sun/instrument/InstrumentationImpl.isModifiableClass0(JLjava/lang/Class;)Z",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn is_modifiable_class_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.instrument.InstrumentationImpl.isModifiableClass0(JLjava/lang/Class;)Z")
}

#[intrinsic_method(
    "sun/instrument/InstrumentationImpl.isRetransformClassesSupported0(J)Z",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn is_retransform_classes_supported_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.instrument.InstrumentationImpl.isRetransformClassesSupported0(J)Z")
}

#[intrinsic_method(
    "sun/instrument/InstrumentationImpl.jarFile(J)Ljava/lang/String;",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn jar_file(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.instrument.InstrumentationImpl.jarFile(J)Ljava/lang/String;")
}

#[intrinsic_method(
    "sun/instrument/InstrumentationImpl.loadAgent0(Ljava/lang/String;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn load_agent_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.instrument.InstrumentationImpl.loadAgent0(Ljava/lang/String;)V")
}

#[intrinsic_method(
    "sun/instrument/InstrumentationImpl.redefineClasses0(J[Ljava/lang/instrument/ClassDefinition;)V",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn redefine_classes_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.instrument.InstrumentationImpl.redefineClasses0(J[Ljava/lang/instrument/ClassDefinition;)V"
    )
}

#[intrinsic_method(
    "sun/instrument/InstrumentationImpl.retransformClasses0(J[Ljava/lang/Class;)V",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn retransform_classes_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.instrument.InstrumentationImpl.retransformClasses0(J[Ljava/lang/Class;)V")
}

#[intrinsic_method(
    "sun/instrument/InstrumentationImpl.setHasRetransformableTransformers(JZ)V",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn set_has_retransformable_transformers(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.instrument.InstrumentationImpl.setHasRetransformableTransformers(JZ)V")
}

#[intrinsic_method(
    "sun/instrument/InstrumentationImpl.setHasTransformers(JZ)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn set_has_transformers(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.instrument.InstrumentationImpl.setHasTransformers(JZ)V")
}

#[intrinsic_method(
    "sun/instrument/InstrumentationImpl.setNativeMethodPrefixes(J[Ljava/lang/String;Z)V",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn set_native_method_prefixes(
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
