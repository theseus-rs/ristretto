use ristretto_classfile::VersionSpecification::{Any, GreaterThanOrEqual};
use ristretto_classfile::{JAVA_11, JAVA_21};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/instrument/InstrumentationImpl.appendToClassLoaderSearch0(JLjava/lang/String;Z)V",
    Any
)]
#[async_method]
pub async fn append_to_class_loader_search_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.instrument.InstrumentationImpl.appendToClassLoaderSearch0(JLjava/lang/String;Z)V"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/instrument/InstrumentationImpl.getAllLoadedClasses0(J)[Ljava/lang/Class;",
    Any
)]
#[async_method]
pub async fn get_all_loaded_classes_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.instrument.InstrumentationImpl.getAllLoadedClasses0(J)[Ljava/lang/Class;".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/instrument/InstrumentationImpl.getInitiatedClasses0(JLjava/lang/ClassLoader;)[Ljava/lang/Class;",
    Any
)]
#[async_method]
pub async fn get_initiated_classes_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.instrument.InstrumentationImpl.getInitiatedClasses0(JLjava/lang/ClassLoader;)[Ljava/lang/Class;".to_string()).into())
}

#[intrinsic_method(
    "sun/instrument/InstrumentationImpl.getObjectSize0(JLjava/lang/Object;)J",
    Any
)]
#[async_method]
pub async fn get_object_size_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.instrument.InstrumentationImpl.getObjectSize0(JLjava/lang/Object;)J".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/instrument/InstrumentationImpl.isModifiableClass0(JLjava/lang/Class;)Z",
    Any
)]
#[async_method]
pub async fn is_modifiable_class_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.instrument.InstrumentationImpl.isModifiableClass0(JLjava/lang/Class;)Z".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/instrument/InstrumentationImpl.isRetransformClassesSupported0(J)Z",
    Any
)]
#[async_method]
pub async fn is_retransform_classes_supported_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.instrument.InstrumentationImpl.isRetransformClassesSupported0(J)Z".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/instrument/InstrumentationImpl.jarFile(J)Ljava/lang/String;",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn jar_file<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.instrument.InstrumentationImpl.jarFile(J)Ljava/lang/String;".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/instrument/InstrumentationImpl.loadAgent0(Ljava/lang/String;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn load_agent_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.instrument.InstrumentationImpl.loadAgent0(Ljava/lang/String;)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/instrument/InstrumentationImpl.redefineClasses0(J[Ljava/lang/instrument/ClassDefinition;)V",
    Any
)]
#[async_method]
pub async fn redefine_classes_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.instrument.InstrumentationImpl.redefineClasses0(J[Ljava/lang/instrument/ClassDefinition;)V".to_string()).into())
}

#[intrinsic_method(
    "sun/instrument/InstrumentationImpl.retransformClasses0(J[Ljava/lang/Class;)V",
    Any
)]
#[async_method]
pub async fn retransform_classes_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.instrument.InstrumentationImpl.retransformClasses0(J[Ljava/lang/Class;)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/instrument/InstrumentationImpl.setHasRetransformableTransformers(JZ)V",
    Any
)]
#[async_method]
pub async fn set_has_retransformable_transformers<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.instrument.InstrumentationImpl.setHasRetransformableTransformers(JZ)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/instrument/InstrumentationImpl.setHasTransformers(JZ)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn set_has_transformers<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.instrument.InstrumentationImpl.setHasTransformers(JZ)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/instrument/InstrumentationImpl.setNativeMethodPrefixes(J[Ljava/lang/String;Z)V",
    Any
)]
#[async_method]
pub async fn set_native_method_prefixes<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.instrument.InstrumentationImpl.setNativeMethodPrefixes(J[Ljava/lang/String;Z)V"
            .to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_load_agent_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = load_agent_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_has_transformers() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_has_transformers(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_jar_file() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = jar_file(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_append_to_class_loader_search_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = append_to_class_loader_search_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_all_loaded_classes_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_all_loaded_classes_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_initiated_classes_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_initiated_classes_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_object_size_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_object_size_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_is_modifiable_class_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = is_modifiable_class_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_is_retransform_classes_supported_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = is_retransform_classes_supported_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_redefine_classes_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = redefine_classes_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_retransform_classes_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = retransform_classes_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_has_retransformable_transformers() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_has_retransformable_transformers(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_native_method_prefixes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_native_method_prefixes(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
