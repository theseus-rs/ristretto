use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("apple/laf/JRSUIControl.disposeCFDictionary(J)V", Any)]
#[async_method]
pub async fn dispose_cf_dictionary<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _cf_dictionary_ptr = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "apple.laf.JRSUIControl.disposeCFDictionary(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method("apple/laf/JRSUIControl.getCFDictionary(Z)J", Any)]
#[async_method]
pub async fn get_cf_dictionary<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _flipped = parameters.pop_bool()?;
    Err(
        JavaError::UnsatisfiedLinkError("apple.laf.JRSUIControl.getCFDictionary(Z)J".to_string())
            .into(),
    )
}

#[intrinsic_method("apple/laf/JRSUIControl.getNativeHitPart(JJJDDDDDD)I", Any)]
#[async_method]
pub async fn get_native_hit_part<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _hit_y = parameters.pop_double()?;
    let _hit_x = parameters.pop_double()?;
    let _h = parameters.pop_double()?;
    let _w = parameters.pop_double()?;
    let _y = parameters.pop_double()?;
    let _x = parameters.pop_double()?;
    let _new_properties = parameters.pop_long()?;
    let _old_properties = parameters.pop_long()?;
    let _cf_dictionary_ptr = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "apple.laf.JRSUIControl.getNativeHitPart(JJJDDDDDD)I".to_string(),
    )
    .into())
}

#[intrinsic_method("apple/laf/JRSUIControl.getNativePartBounds([DJJJDDDDI)V", Any)]
#[async_method]
pub async fn get_native_part_bounds<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _part = parameters.pop_int()?;
    let _h = parameters.pop_double()?;
    let _w = parameters.pop_double()?;
    let _y = parameters.pop_double()?;
    let _x = parameters.pop_double()?;
    let _new_properties = parameters.pop_long()?;
    let _old_properties = parameters.pop_long()?;
    let _cf_dictionary_ptr = parameters.pop_long()?;
    let _rect = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "apple.laf.JRSUIControl.getNativePartBounds([DJJJDDDDI)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "apple/laf/JRSUIControl.getNativeScrollBarOffsetChange(JJJDDDDIII)D",
    Any
)]
#[async_method]
pub async fn get_native_scroll_bar_offset_change<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _extent = parameters.pop_int()?;
    let _visible_amount = parameters.pop_int()?;
    let _offset = parameters.pop_int()?;
    let _h = parameters.pop_double()?;
    let _w = parameters.pop_double()?;
    let _y = parameters.pop_double()?;
    let _x = parameters.pop_double()?;
    let _new_properties = parameters.pop_long()?;
    let _old_properties = parameters.pop_long()?;
    let _cf_dictionary_ptr = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "apple.laf.JRSUIControl.getNativeScrollBarOffsetChange(JJJDDDDIII)D".to_string(),
    )
    .into())
}

#[intrinsic_method("apple/laf/JRSUIControl.getPtrOfBuffer(Ljava/nio/ByteBuffer;)J", Any)]
#[async_method]
pub async fn get_ptr_of_buffer<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _byte_buffer = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "apple.laf.JRSUIControl.getPtrOfBuffer(Ljava/nio/ByteBuffer;)J".to_string(),
    )
    .into())
}

#[intrinsic_method("apple/laf/JRSUIControl.initNativeJRSUI()I", Any)]
#[async_method]
pub async fn init_native_jrsui<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("apple.laf.JRSUIControl.initNativeJRSUI()I".to_string())
            .into(),
    )
}

#[intrinsic_method("apple/laf/JRSUIControl.paintChangesImage([IIIJJJDDDDJ)I", Any)]
#[async_method]
pub async fn paint_changes_image<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _byte_buffer_ptr = parameters.pop_long()?;
    let _h = parameters.pop_double()?;
    let _w = parameters.pop_double()?;
    let _y = parameters.pop_double()?;
    let _x = parameters.pop_double()?;
    let _new_properties = parameters.pop_long()?;
    let _old_properties = parameters.pop_long()?;
    let _cf_dictionary_ptr = parameters.pop_long()?;
    let _img_h = parameters.pop_int()?;
    let _img_w = parameters.pop_int()?;
    let _data = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "apple.laf.JRSUIControl.paintChangesImage([IIIJJJDDDDJ)I".to_string(),
    )
    .into())
}

#[intrinsic_method("apple/laf/JRSUIControl.paintChangesToCGContext(JJJJDDDDJ)I", Any)]
#[async_method]
pub async fn paint_changes_to_cg_context<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _byte_buffer_ptr = parameters.pop_long()?;
    let _h = parameters.pop_double()?;
    let _w = parameters.pop_double()?;
    let _y = parameters.pop_double()?;
    let _x = parameters.pop_double()?;
    let _new_properties = parameters.pop_long()?;
    let _old_properties = parameters.pop_long()?;
    let _cf_dictionary_ptr = parameters.pop_long()?;
    let _cg_context = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "apple.laf.JRSUIControl.paintChangesToCGContext(JJJJDDDDJ)I".to_string(),
    )
    .into())
}

#[intrinsic_method("apple/laf/JRSUIControl.paintImage([IIIJJJDDDD)I", Any)]
#[async_method]
pub async fn paint_image<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h = parameters.pop_double()?;
    let _w = parameters.pop_double()?;
    let _y = parameters.pop_double()?;
    let _x = parameters.pop_double()?;
    let _new_properties = parameters.pop_long()?;
    let _old_properties = parameters.pop_long()?;
    let _cf_dictionary_ptr = parameters.pop_long()?;
    let _img_h = parameters.pop_int()?;
    let _img_w = parameters.pop_int()?;
    let _data = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "apple.laf.JRSUIControl.paintImage([IIIJJJDDDD)I".to_string(),
    )
    .into())
}

#[intrinsic_method("apple/laf/JRSUIControl.paintToCGContext(JJJJDDDD)I", Any)]
#[async_method]
pub async fn paint_to_cg_context<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h = parameters.pop_double()?;
    let _w = parameters.pop_double()?;
    let _y = parameters.pop_double()?;
    let _x = parameters.pop_double()?;
    let _new_properties = parameters.pop_long()?;
    let _old_properties = parameters.pop_long()?;
    let _cf_dictionary_ptr = parameters.pop_long()?;
    let _cg_context = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "apple.laf.JRSUIControl.paintToCGContext(JJJJDDDD)I".to_string(),
    )
    .into())
}

#[intrinsic_method("apple/laf/JRSUIControl.syncChanges(JJ)I", Any)]
#[async_method]
pub async fn sync_changes<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _byte_buffer_ptr = parameters.pop_long()?;
    let _cf_dictionary_ptr = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("apple.laf.JRSUIControl.syncChanges(JJ)I".to_string())
            .into(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_dispose_cf_dictionary() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = dispose_cf_dictionary(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "apple.laf.JRSUIControl.disposeCFDictionary(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_cf_dictionary() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_cf_dictionary(thread, Parameters::new(vec![Value::from(false)])).await;
        assert_eq!(
            "apple.laf.JRSUIControl.getCFDictionary(Z)J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_native_hit_part() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_native_hit_part(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Double(0.0),
                Value::Double(0.0),
                Value::Double(0.0),
                Value::Double(0.0),
                Value::Double(0.0),
                Value::Double(0.0),
            ]),
        )
        .await;
        assert_eq!(
            "apple.laf.JRSUIControl.getNativeHitPart(JJJDDDDDD)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_native_part_bounds() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_native_part_bounds(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Double(0.0),
                Value::Double(0.0),
                Value::Double(0.0),
                Value::Double(0.0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "apple.laf.JRSUIControl.getNativePartBounds([DJJJDDDDI)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_native_scroll_bar_offset_change() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_native_scroll_bar_offset_change(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Double(0.0),
                Value::Double(0.0),
                Value::Double(0.0),
                Value::Double(0.0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "apple.laf.JRSUIControl.getNativeScrollBarOffsetChange(JJJDDDDIII)D",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_ptr_of_buffer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_ptr_of_buffer(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "apple.laf.JRSUIControl.getPtrOfBuffer(Ljava/nio/ByteBuffer;)J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_init_native_jrsui() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_native_jrsui(thread, Parameters::default()).await;
        assert_eq!(
            "apple.laf.JRSUIControl.initNativeJRSUI()I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_paint_changes_image() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = paint_changes_image(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Double(0.0),
                Value::Double(0.0),
                Value::Double(0.0),
                Value::Double(0.0),
                Value::Long(0),
            ]),
        )
        .await;
        assert_eq!(
            "apple.laf.JRSUIControl.paintChangesImage([IIIJJJDDDDJ)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_paint_changes_to_cg_context() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = paint_changes_to_cg_context(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Double(0.0),
                Value::Double(0.0),
                Value::Double(0.0),
                Value::Double(0.0),
                Value::Long(0),
            ]),
        )
        .await;
        assert_eq!(
            "apple.laf.JRSUIControl.paintChangesToCGContext(JJJJDDDDJ)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_paint_image() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = paint_image(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Double(0.0),
                Value::Double(0.0),
                Value::Double(0.0),
                Value::Double(0.0),
            ]),
        )
        .await;
        assert_eq!(
            "apple.laf.JRSUIControl.paintImage([IIIJJJDDDD)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_paint_to_cg_context() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = paint_to_cg_context(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Long(0),
                Value::Double(0.0),
                Value::Double(0.0),
                Value::Double(0.0),
                Value::Double(0.0),
            ]),
        )
        .await;
        assert_eq!(
            "apple.laf.JRSUIControl.paintToCGContext(JJJJDDDD)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_sync_changes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = sync_changes(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "apple.laf.JRSUIControl.syncChanges(JJ)I",
            result.unwrap_err().to_string()
        );
    }
}
