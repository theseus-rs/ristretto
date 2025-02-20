use crate::Result;
use crate::native_methods::registry::{JAVA_21, MethodRegistry};
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "jdk/internal/org/jline/terminal/impl/jna/osx/CLibraryImpl";

/// Register all native methods for `jdk.internal.org.jline.terminal.impl.jna.osx.CLibraryImpl`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() != JAVA_21 {
        return;
    }
    registry.register(CLASS_NAME, "initIDs", "()V", init_ids);
    registry.register(
        CLASS_NAME,
        "ioctl0",
        "(IJLjdk/internal/org/jline/terminal/impl/jna/osx/CLibrary$winsize;)V",
        ioctl_0,
    );
    registry.register(CLASS_NAME, "isatty", "(I)I", isatty);
    registry.register(
        CLASS_NAME,
        "tcgetattr",
        "(ILjdk/internal/org/jline/terminal/impl/jna/osx/CLibrary$termios;)V",
        tcgetattr,
    );
    registry.register(
        CLASS_NAME,
        "tcsetattr",
        "(IILjdk/internal/org/jline/terminal/impl/jna/osx/CLibrary$termios;)V",
        tcsetattr,
    );
    registry.register(CLASS_NAME, "ttyname_r", "(I[BI)V", ttyname_r);
}

#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn ioctl_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "jdk.internal.org.jline.terminal.impl.jna.osx.CLibraryImpl.ioctl0(IJLjdk.internal.org.jline.terminal.impl.jna.osx.CLibrary$winsize;)V"
    )
}

#[async_recursion(?Send)]
async fn isatty(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("jdk.internal.org.jline.terminal.impl.jna.osx.CLibraryImpl.isatty(I)I")
}

#[async_recursion(?Send)]
async fn tcgetattr(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "jdk.internal.org.jline.terminal.impl.jna.osx.CLibraryImpl.tcgetattr(ILjdk.internal.org.jline.terminal.impl.jna.osx.CLibrary$termios;)V"
    )
}

#[async_recursion(?Send)]
async fn tcsetattr(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "jdk.internal.org.jline.terminal.impl.jna.osx.CLibraryImpl.tcsetattr(IILjdk.internal.org.jline.terminal.impl.jna.osx.CLibrary$termios;)V"
    )
}

#[async_recursion(?Send)]
async fn ttyname_r(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
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
