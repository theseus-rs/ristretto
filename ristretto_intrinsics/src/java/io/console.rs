#[cfg(not(target_family = "wasm"))]
use console::Term;
use ristretto_classfile::VersionSpecification::{GreaterThanOrEqual, LessThanOrEqual};
use ristretto_classfile::{JAVA_17, JAVA_21};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("java/io/Console.echo(Z)Z", LessThanOrEqual(JAVA_17))]
#[async_method]
pub async fn echo<T: Thread + 'static>(
    _thread: Arc<T>,
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
#[async_method]
pub async fn encoding<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    // TODO: Implement actual encoding functionality
    Ok(Some(Value::Object(None)))
}

#[intrinsic_method("java/io/Console.istty()Z", LessThanOrEqual(JAVA_17))]
#[async_method]
pub async fn istty<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    #[cfg(not(target_family = "wasm"))]
    {
        let terminal = Term::stdout();
        let is_terminal = terminal.is_term();
        Ok(Some(Value::from(is_terminal)))
    }
    #[cfg(target_family = "wasm")]
    {
        Ok(Some(Value::from(false)))
    }
}

#[intrinsic_method("java/io/Console.ttyStatus()I", GreaterThanOrEqual(JAVA_21))]
#[async_method]
pub async fn tty_status<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::from(0i32)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_echo() -> Result<()> {
        let (_vm, thread) = crate::test::java17_thread().await.expect("thread");
        let mut parameters = Parameters::default();
        parameters.push_bool(true);
        let enabled = echo(thread, parameters)
            .await?
            .expect("enabled")
            .as_bool()?;
        assert!(!enabled);
        Ok(())
    }

    #[tokio::test]
    async fn test_encoding() -> Result<()> {
        let (_vm, thread) = crate::test::java21_thread().await.expect("thread");
        let value = encoding(thread, Parameters::default())
            .await?
            .expect("encoding");
        assert!(value.is_null());
        Ok(())
    }

    #[tokio::test]
    async fn test_tty_status() -> Result<()> {
        let (_vm, thread) = crate::test::java21_thread().await.expect("thread");
        let value = tty_status(thread, Parameters::default())
            .await?
            .expect("tty_status");
        assert_eq!(0, value.as_i32()?);
        Ok(())
    }

    #[tokio::test]
    async fn test_istty() -> Result<()> {
        // This test is mainly for coverage as the test is using the same state to verify the result
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = istty(thread, Parameters::default()).await?.expect("istty");
        let is_tty = result.as_bool()?;
        let terminal = Term::stdout();
        let expected_is_terminal = terminal.is_term();
        assert_eq!(expected_is_terminal, is_tty);
        Ok(())
    }
}
