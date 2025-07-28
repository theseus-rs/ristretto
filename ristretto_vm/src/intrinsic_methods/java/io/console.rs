use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use console::Term;
use ristretto_classfile::VersionSpecification::{Any, LessThanOrEqual};
use ristretto_classfile::{JAVA_17, JAVA_21};
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("java/io/Console.echo(Z)Z", LessThanOrEqual(JAVA_17))]
#[async_recursion(?Send)]
pub(crate) async fn echo(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _echo: bool = parameters.pop_bool()?;
    // TODO: Implement actual echo functionality
    Ok(Some(Value::from(false)))
}

#[intrinsic_method(
    "java/io/Console.encoding()Ljava/lang/String;",
    LessThanOrEqual(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn encoding(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    // TODO: Implement actual encoding functionality
    Ok(Some(Value::Object(None)))
}

#[intrinsic_method("java/io/Console.istty()Z", Any)]
#[async_recursion(?Send)]
pub(crate) async fn istty(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    let terminal = Term::stdout();
    let is_terminal = terminal.is_term();
    Ok(Some(Value::from(is_terminal)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_echo() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut parameters = Parameters::default();
        parameters.push_bool(true);
        let enabled: bool = echo(thread, parameters)
            .await?
            .expect("enabled")
            .try_into()?;
        assert!(!enabled);
        Ok(())
    }

    #[tokio::test]
    async fn test_encoding() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let value = encoding(thread, Parameters::default())
            .await?
            .expect("encoding");
        assert!(value.is_null());
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
