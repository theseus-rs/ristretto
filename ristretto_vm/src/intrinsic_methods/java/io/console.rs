use crate::Result;
use crate::intrinsic_methods::registry::{JAVA_17, JAVA_21, MethodRegistry};
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use console::Term;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "java/io/Console";

/// Register all intrinsic methods for `java.io.Console`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() <= JAVA_17 {
        registry.register(CLASS_NAME, "echo", "(Z)Z", echo);
    }

    if registry.java_major_version() <= JAVA_21 {
        registry.register(CLASS_NAME, "encoding", "()Ljava/lang/String;", encoding);
    }

    registry.register(CLASS_NAME, "istty", "()Z", istty);
}

#[async_recursion(?Send)]
async fn echo(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.io.Console.echo(Z)Z")
}

#[async_recursion(?Send)]
async fn encoding(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(Some(Value::Object(None)))
}

#[async_recursion(?Send)]
async fn istty(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    let terminal = Term::stdout();
    let is_terminal = terminal.is_term();
    Ok(Some(Value::from(is_terminal)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.io.Console.echo(Z)Z")]
    async fn test_echo() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = echo(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_encoding() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let value = encoding(thread, Parameters::default())
            .await?
            .expect("encoding")
            .to_reference()?;
        assert!(value.is_none());
        Ok(())
    }

    #[tokio::test]
    async fn test_istty() -> Result<()> {
        // This test is mainly for coverage as the test is using the same state to verify the result
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = istty(thread, Parameters::default()).await?.expect("istty");
        let is_tty: bool = result.try_into()?;
        let terminal = Term::stdout();
        let expected_is_terminal = terminal.is_term();
        assert_eq!(expected_is_terminal, is_tty);
        Ok(())
    }
}
