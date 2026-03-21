use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::{Any, GreaterThan, LessThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/font/CFont.createNativeFont(Ljava/lang/String;I)J", Any)]
#[async_method]
pub async fn create_native_font<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.font.CFont.createNativeFont(Ljava/lang/String;I)J".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/font/CFont.disposeNativeFont(J)V", Any)]
#[async_method]
pub async fn dispose_native_font<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.font.CFont.disposeNativeFont(J)V".to_string()).into())
}

#[intrinsic_method("sun/font/CFont.getCGFontPtrNative(J)J", GreaterThan(JAVA_8))]
#[async_method]
pub async fn get_cg_font_ptr_native<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.font.CFont.getCGFontPtrNative(J)J".to_string()).into())
}

#[intrinsic_method("sun/font/CFont.getCascadeList(JLjava/util/ArrayList;)V", Any)]
#[async_method]
pub async fn get_cascade_list<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.font.CFont.getCascadeList(JLjava/util/ArrayList;)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/font/CFont.getLayoutTableCacheNative(J)J",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_layout_table_cache_native<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun.font.CFont.getLayoutTableCacheNative(J)J".to_string())
            .into(),
    )
}

#[intrinsic_method("sun/font/CFont.getTableBytesNative(JI)[B", Any)]
#[async_method]
pub async fn get_table_bytes_native<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun.font.CFont.getTableBytesNative(JI)[B".to_string())
            .into(),
    )
}

#[intrinsic_method("sun/font/CFont.getWeightNative(J)F", Any)]
#[async_method]
pub async fn get_weight_native<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.font.CFont.getWeightNative(J)F".to_string()).into())
}

#[intrinsic_method("sun/font/CFont.getWidthNative(J)F", Any)]
#[async_method]
pub async fn get_width_native<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.font.CFont.getWidthNative(J)F".to_string()).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_native_font() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create_native_font(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_dispose_native_font() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = dispose_native_font(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_cg_font_ptr_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_cg_font_ptr_native(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_cascade_list() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_cascade_list(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_layout_table_cache_native() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = get_layout_table_cache_native(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_table_bytes_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_table_bytes_native(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_weight_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_weight_native(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_width_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_width_native(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
