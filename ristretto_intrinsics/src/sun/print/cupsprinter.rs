use ristretto_classfile::VersionSpecification::{Any, GreaterThanOrEqual};
use ristretto_classfile::{JAVA_11, JAVA_25};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/print/CUPSPrinter.canConnect(Ljava/lang/String;I)Z", Any)]
#[async_method]
pub async fn can_connect<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.print.CUPSPrinter.canConnect(Ljava/lang/String;I)Z")
}

#[intrinsic_method("sun/print/CUPSPrinter.getCupsDefaultPrinter()Ljava/lang/String;", Any)]
#[async_method]
pub async fn get_cups_default_printer<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.print.CUPSPrinter.getCupsDefaultPrinter()Ljava/lang/String;")
}

#[intrinsic_method(
    "sun/print/CUPSPrinter.getCupsDefaultPrinters()[Ljava/lang/String;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_cups_default_printers<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.print.CUPSPrinter.getCupsDefaultPrinters()[Ljava/lang/String;")
}

#[intrinsic_method("sun/print/CUPSPrinter.getCupsPort()I", Any)]
#[async_method]
pub async fn get_cups_port<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.print.CUPSPrinter.getCupsPort()I")
}

#[intrinsic_method("sun/print/CUPSPrinter.getCupsServer()Ljava/lang/String;", Any)]
#[async_method]
pub async fn get_cups_server<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.print.CUPSPrinter.getCupsServer()Ljava/lang/String;")
}

#[intrinsic_method(
    "sun/print/CUPSPrinter.getMedia(Ljava/lang/String;)[Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_media<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.print.CUPSPrinter.getMedia(Ljava/lang/String;)[Ljava/lang/String;")
}

#[intrinsic_method(
    "sun/print/CUPSPrinter.getOutputBins(Ljava/lang/String;)[Ljava/lang/String;",
    GreaterThanOrEqual(JAVA_25)
)]
#[async_method]
pub async fn get_output_bins<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.print.CUPSPrinter.getOutputBins(Ljava/lang/String;)[Ljava/lang/String;")
}

#[intrinsic_method("sun/print/CUPSPrinter.getPageSizes(Ljava/lang/String;)[F", Any)]
#[async_method]
pub async fn get_page_sizes<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.print.CUPSPrinter.getPageSizes(Ljava/lang/String;)[F")
}

#[intrinsic_method(
    "sun/print/CUPSPrinter.getResolutions(Ljava/lang/String;Ljava/util/ArrayList;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_resolutions<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.print.CUPSPrinter.getResolutions(Ljava/lang/String;Ljava/util/ArrayList;)V")
}

#[intrinsic_method("sun/print/CUPSPrinter.initIDs()Z", Any)]
#[async_method]
pub async fn init_ids<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
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
