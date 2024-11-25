use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_11: Version = Version::Java11 { minor: 0 };

/// Register all native methods for `sun.print.CUPSPrinter`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/print/CUPSPrinter";
    let java_version = registry.java_version();

    if java_version >= &JAVA_11 {
        registry.register(
            class_name,
            "getCupsDefaultPrinters",
            "()[Ljava/lang/String;",
            get_cups_default_printers,
        );
        registry.register(
            class_name,
            "getResolutions",
            "(Ljava/lang/String;Ljava/util/ArrayList;)V",
            get_resolutions,
        );
    }

    registry.register(
        class_name,
        "canConnect",
        "(Ljava/lang/String;I)Z",
        can_connect,
    );
    registry.register(
        class_name,
        "getCupsDefaultPrinter",
        "()Ljava/lang/String;",
        get_cups_default_printer,
    );
    registry.register(class_name, "getCupsPort", "()I", get_cups_port);
    registry.register(
        class_name,
        "getCupsServer",
        "()Ljava/lang/String;",
        get_cups_server,
    );
    registry.register(
        class_name,
        "getMedia",
        "(Ljava/lang/String;)[Ljava/lang/String;",
        get_media,
    );
    registry.register(
        class_name,
        "getPageSizes",
        "(Ljava/lang/String;)[F",
        get_page_sizes,
    );
    registry.register(class_name, "initIDs", "()Z", init_ids);
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn can_connect(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_cups_default_printer(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_cups_default_printers(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_cups_port(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_cups_server(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_media(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_page_sizes(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_resolutions(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}
