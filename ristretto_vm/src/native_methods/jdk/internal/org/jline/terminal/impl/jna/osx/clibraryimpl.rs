use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `jdk.internal.org.jline.terminal.impl.jna.osx.CLibraryImpl`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "jdk/internal/org/jline/terminal/impl/jna/osx/CLibraryImpl";
    registry.register(class_name, "initIDs", "()V", init_ids);
    registry.register(
        class_name,
        "ioctl0",
        "(IJLjdk/internal/org/jline/terminal/impl/jna/osx/CLibrary$winsize;)V",
        ioctl_0,
    );
    registry.register(class_name, "isatty", "(I)I", isatty);
    registry.register(
        class_name,
        "tcgetattr",
        "(ILjdk/internal/org/jline/terminal/impl/jna/osx/CLibrary$termios;)V",
        tcgetattr,
    );
    registry.register(
        class_name,
        "tcsetattr",
        "(IILjdk/internal/org/jline/terminal/impl/jna/osx/CLibrary$termios;)V",
        tcsetattr,
    );
    registry.register(class_name, "ttyname_r", "(I[BI)V", ttyname_r);
}

#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn ioctl_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.internal.org.jline.terminal.impl.jna.osx.CLibraryImpl.ioctl0(IJLjdk.internal.org.jline.terminal.impl.jna.osx.CLibrary$winsize;)V")
}

#[async_recursion(?Send)]
async fn isatty(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.internal.org.jline.terminal.impl.jna.osx.CLibraryImpl.isatty(I)I")
}

#[async_recursion(?Send)]
async fn tcgetattr(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.internal.org.jline.terminal.impl.jna.osx.CLibraryImpl.tcgetattr(ILjdk.internal.org.jline.terminal.impl.jna.osx.CLibrary$termios;)V")
}

#[async_recursion(?Send)]
async fn tcsetattr(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.internal.org.jline.terminal.impl.jna.osx.CLibraryImpl.tcsetattr(IILjdk.internal.org.jline.terminal.impl.jna.osx.CLibrary$termios;)V")
}

#[async_recursion(?Send)]
async fn ttyname_r(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.internal.org.jline.terminal.impl.jna.osx.CLibraryImpl.ttyname_r(I[BI)V")
}
