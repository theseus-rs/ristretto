use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_11: Version = Version::Java11 { minor: 0 };
const JAVA_17: Version = Version::Java17 { minor: 0 };
const JAVA_19: Version = Version::Java19 { minor: 0 };
const JAVA_23: Version = Version::Java23 { minor: 0 };

/// Register all native methods for `sun.print.CUPSPrinter`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/print/CUPSPrinter";
    let java_version = registry.java_version().clone();

    if java_version >= JAVA_11 {
        if java_version <= JAVA_17 {
            registry.register(
                class_name,
                "getCupsDefaultPrinters",
                "()[Ljava/lang/String;",
                get_cups_default_printers,
            );
        }

        registry.register(
            class_name,
            "getResolutions",
            "(Ljava/lang/String;Ljava/util/ArrayList;)V",
            get_resolutions,
        );
    }

    if java_version >= JAVA_19 {
        registry.register(
            class_name,
            "getCupsDefaultPrinters",
            "()[Ljava/lang/String;",
            get_cups_default_printers,
        );
    }

    if java_version >= JAVA_23 {
        registry.register(
            class_name,
            "getOutputBins",
            "(Ljava/lang/String;)[Ljava/lang/String;",
            get_output_bins,
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

#[async_recursion(?Send)]
async fn can_connect(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.print.CUPSPrinter.canConnect(Ljava/lang/String;I)Z")
}

#[async_recursion(?Send)]
async fn get_cups_default_printer(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.print.CUPSPrinter.getCupsDefaultPrinter()Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn get_cups_default_printers(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.print.CUPSPrinter.getCupsDefaultPrinters()[Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn get_cups_port(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.print.CUPSPrinter.getCupsPort()I")
}

#[async_recursion(?Send)]
async fn get_cups_server(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.print.CUPSPrinter.getCupsServer()Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn get_media(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.print.CUPSPrinter.getMedia(Ljava/lang/String;)[Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn get_output_bins(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.print.CUPSPrinter.getOutputBins(Ljava/lang/String;)[Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn get_page_sizes(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.print.CUPSPrinter.getPageSizes(Ljava/lang/String;)[F")
}

#[async_recursion(?Send)]
async fn get_resolutions(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.print.CUPSPrinter.getResolutions(Ljava/lang/String;Ljava/util/ArrayList;)V")
}

#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::new(&Version::Java11 { minor: 0 }, true);
        register(&mut registry);
        let class_name = "sun/print/CUPSPrinter";
        assert!(registry
            .method(class_name, "canConnect", "(Ljava/lang/String;I)Z")
            .is_some());
        assert!(registry
            .method(class_name, "getCupsDefaultPrinter", "()Ljava/lang/String;")
            .is_some());
        assert!(registry
            .method(
                class_name,
                "getCupsDefaultPrinters",
                "()[Ljava/lang/String;"
            )
            .is_some());
        assert!(registry.method(class_name, "getCupsPort", "()I").is_some());
        assert!(registry
            .method(class_name, "getCupsServer", "()Ljava/lang/String;")
            .is_some());
        assert!(registry
            .method(
                class_name,
                "getMedia",
                "(Ljava/lang/String;)[Ljava/lang/String;"
            )
            .is_some());
        assert!(registry
            .method(class_name, "getPageSizes", "(Ljava/lang/String;)[F")
            .is_some());
        assert!(registry
            .method(
                class_name,
                "getResolutions",
                "(Ljava/lang/String;Ljava/util/ArrayList;)V"
            )
            .is_some());
        assert!(registry.method(class_name, "initIDs", "()Z").is_some());
    }

    #[test]
    fn test_register_java_23() {
        let mut registry = MethodRegistry::new(&Version::Java23 { minor: 0 }, true);
        register(&mut registry);
        let class_name = "sun/print/CUPSPrinter";
        assert!(registry
            .method(
                class_name,
                "getOutputBins",
                "(Ljava/lang/String;)[Ljava/lang/String;"
            )
            .is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.print.CUPSPrinter.canConnect(Ljava/lang/String;I)Z")]
    async fn test_can_connect() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = can_connect(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.print.CUPSPrinter.getCupsDefaultPrinter()Ljava/lang/String;")]
    async fn test_get_cups_default_printer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_cups_default_printer(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.print.CUPSPrinter.getCupsDefaultPrinters()[Ljava/lang/String;")]
    async fn test_get_cups_default_printers() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_cups_default_printers(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.print.CUPSPrinter.getCupsPort()I")]
    async fn test_get_cups_port() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_cups_port(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.print.CUPSPrinter.getCupsServer()Ljava/lang/String;")]
    async fn test_get_cups_server() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_cups_server(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.print.CUPSPrinter.getMedia(Ljava/lang/String;)[Ljava/lang/String;"
    )]
    async fn test_get_media() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_media(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.print.CUPSPrinter.getOutputBins(Ljava/lang/String;)[Ljava/lang/String;"
    )]
    async fn test_get_output_bins() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_output_bins(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.print.CUPSPrinter.getPageSizes(Ljava/lang/String;)[F")]
    async fn test_get_page_sizes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_page_sizes(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.print.CUPSPrinter.getResolutions(Ljava/lang/String;Ljava/util/ArrayList;)V"
    )]
    async fn test_get_resolutions() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_resolutions(thread, Arguments::default()).await;
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init_ids(thread, Arguments::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }
}
