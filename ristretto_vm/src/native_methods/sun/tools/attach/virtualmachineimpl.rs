use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.tools.attach.VirtualMachineImpl`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/tools/attach/VirtualMachineImpl";
    registry.register(
        class_name,
        "checkPermissions",
        "(Ljava/lang/String;)V",
        check_permissions,
    );
    registry.register(class_name, "close", "(I)V", close);
    registry.register(class_name, "connect", "(ILjava/lang/String;)V", connect);
    registry.register(
        class_name,
        "createAttachFile0",
        "(Ljava/lang/String;)V",
        create_attach_file_0,
    );
    registry.register(
        class_name,
        "getTempDir",
        "()Ljava/lang/String;",
        get_temp_dir,
    );
    registry.register(class_name, "read", "(I[BII)I", read);
    registry.register(class_name, "sendQuitTo", "(I)V", send_quit_to);
    registry.register(class_name, "socket", "()I", socket);
    registry.register(class_name, "write", "(I[BII)V", write);
}

#[async_recursion(?Send)]
async fn check_permissions(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.tools.attach.VirtualMachineImpl.checkPermissions(Ljava/lang/String;)V")
}

#[async_recursion(?Send)]
async fn close(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.tools.attach.VirtualMachineImpl.close(I)V")
}

#[async_recursion(?Send)]
async fn connect(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.tools.attach.VirtualMachineImpl.connect(ILjava/lang/String;)V")
}

#[async_recursion(?Send)]
async fn create_attach_file_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.tools.attach.VirtualMachineImpl.createAttachFile0(Ljava/lang/String;)V")
}

#[async_recursion(?Send)]
async fn get_temp_dir(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.tools.attach.VirtualMachineImpl.getTempDir()Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn read(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.tools.attach.VirtualMachineImpl.read(I[BII)I")
}

#[async_recursion(?Send)]
async fn send_quit_to(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.tools.attach.VirtualMachineImpl.sendQuitTo(I)V")
}

#[async_recursion(?Send)]
async fn socket(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.tools.attach.VirtualMachineImpl.socket()I")
}

#[async_recursion(?Send)]
async fn write(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.tools.attach.VirtualMachineImpl.write(I[BII)V")
}
