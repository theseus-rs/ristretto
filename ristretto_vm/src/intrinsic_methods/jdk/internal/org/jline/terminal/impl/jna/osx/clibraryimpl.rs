use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::JAVA_21;
use ristretto_classfile::VersionSpecification::Equal;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "jdk/internal/org/jline/terminal/impl/jna/osx/CLibraryImpl.initIDs()V",
    Equal(JAVA_21)
)]
#[async_method]
pub(crate) async fn init_ids(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "jdk/internal/org/jline/terminal/impl/jna/osx/CLibraryImpl.ioctl0(IJLjdk/internal/org/jline/terminal/impl/jna/osx/CLibrary$winsize;)V",
    Equal(JAVA_21)
)]
#[async_method]
pub(crate) async fn ioctl_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "jdk.internal.org.jline.terminal.impl.jna.osx.CLibraryImpl.ioctl0(IJLjdk.internal.org.jline.terminal.impl.jna.osx.CLibrary$winsize;)V"
    )
}

#[intrinsic_method(
    "jdk/internal/org/jline/terminal/impl/jna/osx/CLibraryImpl.isatty(I)I",
    Equal(JAVA_21)
)]
#[async_method]
pub(crate) async fn isatty(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("jdk.internal.org.jline.terminal.impl.jna.osx.CLibraryImpl.isatty(I)I")
}

#[intrinsic_method(
    "jdk/internal/org/jline/terminal/impl/jna/osx/CLibraryImpl.tcgetattr(ILjdk/internal/org/jline/terminal/impl/jna/osx/CLibrary$termios;)V",
    Equal(JAVA_21)
)]
#[async_method]
pub(crate) async fn tcgetattr(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "jdk.internal.org.jline.terminal.impl.jna.osx.CLibraryImpl.tcgetattr(ILjdk.internal.org.jline.terminal.impl.jna.osx.CLibrary$termios;)V"
    )
}

#[intrinsic_method(
    "jdk/internal/org/jline/terminal/impl/jna/osx/CLibraryImpl.tcsetattr(IILjdk/internal/org/jline/terminal/impl/jna/osx/CLibrary$termios;)V",
    Equal(JAVA_21)
)]
#[async_method]
pub(crate) async fn tcsetattr(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "jdk.internal.org.jline.terminal.impl.jna.osx.CLibraryImpl.tcsetattr(IILjdk.internal.org.jline.terminal.impl.jna.osx.CLibrary$termios;)V"
    )
}

#[intrinsic_method(
    "jdk/internal/org/jline/terminal/impl/jna/osx/CLibraryImpl.ttyname_r(I[BI)V",
    Equal(JAVA_21)
)]
#[async_method]
pub(crate) async fn ttyname_r(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.org.jline.terminal.impl.jna.osx.CLibraryImpl.ttyname_r(I[BI)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.org.jline.terminal.impl.jna.osx.CLibraryImpl.ioctl0(IJLjdk.internal.org.jline.terminal.impl.jna.osx.CLibrary$winsize;)V"
    )]
    async fn test_ioctl_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ioctl_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.org.jline.terminal.impl.jna.osx.CLibraryImpl.isatty(I)I"
    )]
    async fn test_isatty() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = isatty(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.org.jline.terminal.impl.jna.osx.CLibraryImpl.tcgetattr(ILjdk.internal.org.jline.terminal.impl.jna.osx.CLibrary$termios;)V"
    )]
    async fn test_tcgetattr() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = tcgetattr(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.org.jline.terminal.impl.jna.osx.CLibraryImpl.tcsetattr(IILjdk.internal.org.jline.terminal.impl.jna.osx.CLibrary$termios;)V"
    )]
    async fn test_tcsetattr() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = tcsetattr(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.org.jline.terminal.impl.jna.osx.CLibraryImpl.ttyname_r(I[BI)V"
    )]
    async fn test_ttyname_r() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ttyname_r(thread, Parameters::default()).await;
    }
}
