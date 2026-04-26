use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/tracing/dtrace/JVM.activate0(Ljava/lang/String;[Lsun/tracing/dtrace/DTraceProvider;)J",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn activate_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg1 = parameters.pop_reference()?;
    let _arg0 = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.tracing.dtrace.JVM.activate0(Ljava/lang/String;[Lsun/tracing/dtrace/DTraceProvider;)J"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/tracing/dtrace/JVM.defineClass0(Ljava/lang/ClassLoader;Ljava/lang/String;[BII)Ljava/lang/Class;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn define_class_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg4 = parameters.pop_int()?;
    let _arg3 = parameters.pop_int()?;
    let _arg2 = parameters.pop_reference()?;
    let _arg1 = parameters.pop_reference()?;
    let _arg0 = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun.tracing.dtrace.JVM.defineClass0(Ljava/lang/ClassLoader;Ljava/lang/String;[BII)Ljava/lang/Class;".to_string()).into())
}

#[intrinsic_method("sun/tracing/dtrace/JVM.dispose0(J)V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn dispose_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _activation_handle = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun.tracing.dtrace.JVM.dispose0(J)".to_string()).into())
}

#[intrinsic_method(
    "sun/tracing/dtrace/JVM.isEnabled0(Ljava/lang/reflect/Method;)Z",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn is_enabled_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _m = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.tracing.dtrace.JVM.isEnabled0(Ljava/lang/reflect/Method;)Z".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/tracing/dtrace/JVM.isSupported0()Z", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn is_supported_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun.tracing.dtrace.JVM.isSupported0()Z".to_string())
            .into(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_activate_0() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = activate_0(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun.tracing.dtrace.JVM.activate0(Ljava/lang/String;[Lsun/tracing/dtrace/DTraceProvider;)J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_define_class_0() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = define_class_0(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.tracing.dtrace.JVM.defineClass0(Ljava/lang/ClassLoader;Ljava/lang/String;[BII)Ljava/lang/Class;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_dispose_0() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = dispose_0(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.tracing.dtrace.JVM.dispose0(J)",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_is_enabled_0() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = is_enabled_0(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun.tracing.dtrace.JVM.isEnabled0(Ljava/lang/reflect/Method;)Z",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_is_supported_0() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = is_supported_0(thread, Parameters::default()).await;
        assert_eq!(
            "sun.tracing.dtrace.JVM.isSupported0()Z",
            result.unwrap_err().to_string()
        );
    }
}
