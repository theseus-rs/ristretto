use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "apple/applescript/AppleScriptEngine.createContextFrom(Ljava/lang/Object;)J",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn create_context_from(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("apple.applescript.AppleScriptEngine.createContextFrom(Ljava/lang/Object;)J")
}

#[intrinsic_method(
    "apple/applescript/AppleScriptEngine.createObjectFrom(J)Ljava/lang/Object;",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn create_object_from(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("apple.applescript.AppleScriptEngine.createObjectFrom(J)Ljava/lang/Object;")
}

#[intrinsic_method(
    "apple/applescript/AppleScriptEngine.disposeContext(J)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn dispose_context(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("apple.applescript.AppleScriptEngine.disposeContext(J)V")
}

#[intrinsic_method(
    "apple/applescript/AppleScriptEngine.evalScript(Ljava/lang/String;J)J",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn eval_script(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("apple.applescript.AppleScriptEngine.evalScript(Ljava/lang/String;J)J")
}

#[intrinsic_method(
    "apple/applescript/AppleScriptEngine.evalScriptFromURL(Ljava/lang/String;J)J",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn eval_script_from_url(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("apple.applescript.AppleScriptEngine.evalScriptFromURL(Ljava/lang/String;J)J")
}

#[intrinsic_method(
    "apple/applescript/AppleScriptEngine.initNative()V",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn init_native(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: apple.applescript.AppleScriptEngine.createContextFrom(Ljava/lang/Object;)J"
    )]
    async fn test_create_context_from() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = create_context_from(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: apple.applescript.AppleScriptEngine.createObjectFrom(J)Ljava/lang/Object;"
    )]
    async fn test_create_object_from() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = create_object_from(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: apple.applescript.AppleScriptEngine.disposeContext(J)V"
    )]
    async fn test_dispose_context() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = dispose_context(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: apple.applescript.AppleScriptEngine.evalScript(Ljava/lang/String;J)J"
    )]
    async fn test_eval_script() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = eval_script(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: apple.applescript.AppleScriptEngine.evalScriptFromURL(Ljava/lang/String;J)J"
    )]
    async fn test_eval_script_from_url() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = eval_script_from_url(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_init_native() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_native(thread, Parameters::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }
}
