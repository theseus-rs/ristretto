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
    "apple/applescript/AppleScriptEngine.createContextFrom(Ljava/lang/Object;)J",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn create_context_from<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg0 = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "apple.applescript.AppleScriptEngine.createContextFrom(Ljava/lang/Object;)J".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "apple/applescript/AppleScriptEngine.createObjectFrom(J)Ljava/lang/Object;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn create_object_from<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg0 = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "apple.applescript.AppleScriptEngine.createObjectFrom(J)Ljava/lang/Object;".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "apple/applescript/AppleScriptEngine.disposeContext(J)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn dispose_context<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg0 = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "apple.applescript.AppleScriptEngine.disposeContext(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "apple/applescript/AppleScriptEngine.evalScript(Ljava/lang/String;J)J",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn eval_script<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg1 = parameters.pop_long()?;
    let _arg0 = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "apple.applescript.AppleScriptEngine.evalScript(Ljava/lang/String;J)J".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "apple/applescript/AppleScriptEngine.evalScriptFromURL(Ljava/lang/String;J)J",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn eval_script_from_url<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg1 = parameters.pop_long()?;
    let _arg0 = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "apple.applescript.AppleScriptEngine.evalScriptFromURL(Ljava/lang/String;J)J".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "apple/applescript/AppleScriptEngine.initNative()V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn init_native<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_context_from() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = create_context_from(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "apple.applescript.AppleScriptEngine.createContextFrom(Ljava/lang/Object;)J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_create_object_from() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = create_object_from(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "apple.applescript.AppleScriptEngine.createObjectFrom(J)Ljava/lang/Object;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_dispose_context() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = dispose_context(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "apple.applescript.AppleScriptEngine.disposeContext(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_eval_script() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = eval_script(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "apple.applescript.AppleScriptEngine.evalScript(Ljava/lang/String;J)J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_eval_script_from_url() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = eval_script_from_url(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "apple.applescript.AppleScriptEngine.evalScriptFromURL(Ljava/lang/String;J)J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_init_native() -> Result<()> {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = init_native(thread, Parameters::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }
}
