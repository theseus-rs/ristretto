use ristretto_classfile::JAVA_21;
use ristretto_classfile::VersionSpecification::Equal;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "jdk/internal/org/jline/terminal/impl/jna/osx/CLibraryImpl.initIDs()V",
    Equal(JAVA_21)
)]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "jdk/internal/org/jline/terminal/impl/jna/osx/CLibraryImpl.ioctl0(IJLjdk/internal/org/jline/terminal/impl/jna/osx/CLibrary$winsize;)V",
    Equal(JAVA_21)
)]
#[async_method]
pub async fn ioctl_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg2 = parameters.pop_reference()?;
    let _arg1 = parameters.pop_long()?;
    let _arg0 = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError("jdk.internal.org.jline.terminal.impl.jna.osx.CLibraryImpl.ioctl0(IJLjdk.internal.org.jline.terminal.impl.jna.osx.CLibrary$winsize;)V".to_string()).into())
}

#[intrinsic_method(
    "jdk/internal/org/jline/terminal/impl/jna/osx/CLibraryImpl.isatty(I)I",
    Equal(JAVA_21)
)]
#[async_method]
pub async fn isatty<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _fd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.internal.org.jline.terminal.impl.jna.osx.CLibraryImpl.isatty(I)I".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "jdk/internal/org/jline/terminal/impl/jna/osx/CLibraryImpl.tcgetattr(ILjdk/internal/org/jline/terminal/impl/jna/osx/CLibrary$termios;)V",
    Equal(JAVA_21)
)]
#[async_method]
pub async fn tcgetattr<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg1 = parameters.pop_reference()?;
    let _arg0 = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError("jdk.internal.org.jline.terminal.impl.jna.osx.CLibraryImpl.tcgetattr(ILjdk.internal.org.jline.terminal.impl.jna.osx.CLibrary$termios;)V".to_string()).into())
}

#[intrinsic_method(
    "jdk/internal/org/jline/terminal/impl/jna/osx/CLibraryImpl.tcsetattr(IILjdk/internal/org/jline/terminal/impl/jna/osx/CLibrary$termios;)V",
    Equal(JAVA_21)
)]
#[async_method]
pub async fn tcsetattr<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg2 = parameters.pop_reference()?;
    let _arg1 = parameters.pop_int()?;
    let _arg0 = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError("jdk.internal.org.jline.terminal.impl.jna.osx.CLibraryImpl.tcsetattr(IILjdk.internal.org.jline.terminal.impl.jna.osx.CLibrary$termios;)V".to_string()).into())
}

#[intrinsic_method(
    "jdk/internal/org/jline/terminal/impl/jna/osx/CLibraryImpl.ttyname_r(I[BI)V",
    Equal(JAVA_21)
)]
#[async_method]
pub async fn ttyname_r<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _len = parameters.pop_int()?;
    let _buf = parameters.pop_reference()?;
    let _fd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.internal.org.jline.terminal.impl.jna.osx.CLibraryImpl.ttyname_r(I[BI)V".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ioctl_0() {
        let (_vm, thread) = crate::test::java21_thread().await.expect("thread");
        let result = ioctl_0(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Long(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "jdk.internal.org.jline.terminal.impl.jna.osx.CLibraryImpl.ioctl0(IJLjdk.internal.org.jline.terminal.impl.jna.osx.CLibrary$winsize;)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_isatty() {
        let (_vm, thread) = crate::test::java21_thread().await.expect("thread");
        let result = isatty(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "jdk.internal.org.jline.terminal.impl.jna.osx.CLibraryImpl.isatty(I)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_tcgetattr() {
        let (_vm, thread) = crate::test::java21_thread().await.expect("thread");
        let result = tcgetattr(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "jdk.internal.org.jline.terminal.impl.jna.osx.CLibraryImpl.tcgetattr(ILjdk.internal.org.jline.terminal.impl.jna.osx.CLibrary$termios;)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_tcsetattr() {
        let (_vm, thread) = crate::test::java21_thread().await.expect("thread");
        let result = tcsetattr(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Int(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "jdk.internal.org.jline.terminal.impl.jna.osx.CLibraryImpl.tcsetattr(IILjdk.internal.org.jline.terminal.impl.jna.osx.CLibrary$termios;)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_ttyname_r() {
        let (_vm, thread) = crate::test::java21_thread().await.expect("thread");
        let result = ttyname_r(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Object(None), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "jdk.internal.org.jline.terminal.impl.jna.osx.CLibraryImpl.ttyname_r(I[BI)V",
            result.unwrap_err().to_string()
        );
    }
}
