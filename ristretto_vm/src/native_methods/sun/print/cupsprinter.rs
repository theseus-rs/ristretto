use crate::Result;
use crate::native_methods::registry::{JAVA_11, JAVA_17, JAVA_21, JAVA_24, MethodRegistry};
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/print/CUPSPrinter";

/// Register all native methods for `sun.print.CUPSPrinter`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() >= JAVA_11 {
        if registry.java_major_version() <= JAVA_17 {
            registry.register(
                CLASS_NAME,
                "getCupsDefaultPrinters",
                "()[Ljava/lang/String;",
                get_cups_default_printers,
            );
        }

        registry.register(
            CLASS_NAME,
            "getResolutions",
            "(Ljava/lang/String;Ljava/util/ArrayList;)V",
            get_resolutions,
        );
    }

    if registry.java_major_version() >= JAVA_21 {
        registry.register(
            CLASS_NAME,
            "getCupsDefaultPrinters",
            "()[Ljava/lang/String;",
            get_cups_default_printers,
        );
    }

    if registry.java_major_version() >= JAVA_24 {
        registry.register(
            CLASS_NAME,
            "getOutputBins",
            "(Ljava/lang/String;)[Ljava/lang/String;",
            get_output_bins,
        );
    }

    registry.register(
        CLASS_NAME,
        "canConnect",
        "(Ljava/lang/String;I)Z",
        can_connect,
    );
    registry.register(
        CLASS_NAME,
        "getCupsDefaultPrinter",
        "()Ljava/lang/String;",
        get_cups_default_printer,
    );
    registry.register(CLASS_NAME, "getCupsPort", "()I", get_cups_port);
    registry.register(
        CLASS_NAME,
        "getCupsServer",
        "()Ljava/lang/String;",
        get_cups_server,
    );
    registry.register(
        CLASS_NAME,
        "getMedia",
        "(Ljava/lang/String;)[Ljava/lang/String;",
        get_media,
    );
    registry.register(
        CLASS_NAME,
        "getPageSizes",
        "(Ljava/lang/String;)[F",
        get_page_sizes,
    );
    registry.register(CLASS_NAME, "initIDs", "()Z", init_ids);
}

#[async_recursion(?Send)]
async fn can_connect(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.print.CUPSPrinter.canConnect(Ljava/lang/String;I)Z")
}

#[async_recursion(?Send)]
async fn get_cups_default_printer(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.print.CUPSPrinter.getCupsDefaultPrinter()Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn get_cups_default_printers(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.print.CUPSPrinter.getCupsDefaultPrinters()[Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn get_cups_port(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.print.CUPSPrinter.getCupsPort()I")
}

#[async_recursion(?Send)]
async fn get_cups_server(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.print.CUPSPrinter.getCupsServer()Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn get_media(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.print.CUPSPrinter.getMedia(Ljava/lang/String;)[Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn get_output_bins(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.print.CUPSPrinter.getOutputBins(Ljava/lang/String;)[Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn get_page_sizes(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.print.CUPSPrinter.getPageSizes(Ljava/lang/String;)[F")
}

#[async_recursion(?Send)]
async fn get_resolutions(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.print.CUPSPrinter.getResolutions(Ljava/lang/String;Ljava/util/ArrayList;)V")
}

#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.print.CUPSPrinter.canConnect(Ljava/lang/String;I)Z"
    )]
    async fn test_can_connect() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = can_connect(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.print.CUPSPrinter.getCupsDefaultPrinter()Ljava/lang/String;"
    )]
    async fn test_get_cups_default_printer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_cups_default_printer(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.print.CUPSPrinter.getCupsDefaultPrinters()[Ljava/lang/String;"
    )]
    async fn test_get_cups_default_printers() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_cups_default_printers(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.print.CUPSPrinter.getCupsPort()I")]
    async fn test_get_cups_port() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_cups_port(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.print.CUPSPrinter.getCupsServer()Ljava/lang/String;"
    )]
    async fn test_get_cups_server() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_cups_server(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.print.CUPSPrinter.getMedia(Ljava/lang/String;)[Ljava/lang/String;"
    )]
    async fn test_get_media() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_media(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.print.CUPSPrinter.getOutputBins(Ljava/lang/String;)[Ljava/lang/String;"
    )]
    async fn test_get_output_bins() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_output_bins(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.print.CUPSPrinter.getPageSizes(Ljava/lang/String;)[F"
    )]
    async fn test_get_page_sizes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_page_sizes(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.print.CUPSPrinter.getResolutions(Ljava/lang/String;Ljava/util/ArrayList;)V"
    )]
    async fn test_get_resolutions() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_resolutions(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }
}
