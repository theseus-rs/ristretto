use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "apple/applescript/AppleScriptEngine";

/// Register all native methods for `apple.applescript.AppleScriptEngine`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "createContextFrom",
        "(Ljava/lang/Object;)J",
        create_context_from,
    );
    registry.register(
        CLASS_NAME,
        "createObjectFrom",
        "(J)Ljava/lang/Object;",
        create_object_from,
    );
    registry.register(CLASS_NAME, "disposeContext", "(J)V", dispose_context);
    registry.register(
        CLASS_NAME,
        "evalScript",
        "(Ljava/lang/String;J)J",
        eval_script,
    );
    registry.register(
        CLASS_NAME,
        "evalScriptFromURL",
        "(Ljava/lang/String;J)J",
        eval_script_from_url,
    );
    registry.register(CLASS_NAME, "initNative", "()V", init_native);
}

#[async_recursion(?Send)]
async fn create_context_from(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("apple.applescript.AppleScriptEngine.createContextFrom(Ljava/lang/Object;)J")
}

#[async_recursion(?Send)]
async fn create_object_from(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("apple.applescript.AppleScriptEngine.createObjectFrom(J)Ljava/lang/Object;")
}

#[async_recursion(?Send)]
async fn dispose_context(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("apple.applescript.AppleScriptEngine.disposeContext(J)V")
}

#[async_recursion(?Send)]
async fn eval_script(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("apple.applescript.AppleScriptEngine.evalScript(Ljava/lang/String;J)J")
}

#[async_recursion(?Send)]
async fn eval_script_from_url(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("apple.applescript.AppleScriptEngine.evalScriptFromURL(Ljava/lang/String;J)J")
}

#[async_recursion(?Send)]
async fn init_native(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
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
        let _ = create_context_from(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: apple.applescript.AppleScriptEngine.createObjectFrom(J)Ljava/lang/Object;"
    )]
    async fn test_create_object_from() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = create_object_from(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: apple.applescript.AppleScriptEngine.disposeContext(J)V"
    )]
    async fn test_dispose_context() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = dispose_context(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: apple.applescript.AppleScriptEngine.evalScript(Ljava/lang/String;J)J"
    )]
    async fn test_eval_script() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = eval_script(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: apple.applescript.AppleScriptEngine.evalScriptFromURL(Ljava/lang/String;J)J"
    )]
    async fn test_eval_script_from_url() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = eval_script_from_url(thread, Arguments::default()).await;
    }

    #[tokio::test]
    async fn test_init_native() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_native(thread, Arguments::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }
}
