use crate::Result;
use crate::intrinsic_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/font/CStrike";

/// Register all intrinsic methods for `sun.font.CStrike`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "createNativeStrikePtr",
        "(J[D[DII)J",
        create_native_strike_ptr,
    );
    registry.register(
        CLASS_NAME,
        "disposeNativeStrikePtr",
        "(J)V",
        dispose_native_strike_ptr,
    );
    registry.register(
        CLASS_NAME,
        "getFontMetrics",
        "(J)Lsun/font/StrikeMetrics;",
        get_font_metrics,
    );
    registry.register(
        CLASS_NAME,
        "getGlyphImagePtrsNative",
        "(J[J[II)V",
        get_glyph_image_ptrs_native,
    );
    registry.register(
        CLASS_NAME,
        "getNativeGlyphAdvance",
        "(JI)F",
        get_native_glyph_advance,
    );
    registry.register(
        CLASS_NAME,
        "getNativeGlyphImageBounds",
        "(JILjava/awt/geom/Rectangle2D$Float;DD)V",
        get_native_glyph_image_bounds,
    );
    registry.register(
        CLASS_NAME,
        "getNativeGlyphOutline",
        "(JIDD)Ljava/awt/geom/GeneralPath;",
        get_native_glyph_outline,
    );
}

#[async_recursion(?Send)]
async fn create_native_strike_ptr(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.CStrike.createNativeStrikePtr(J[D[DII)J")
}

#[async_recursion(?Send)]
async fn dispose_native_strike_ptr(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.CStrike.disposeNativeStrikePtr(J)V")
}

#[async_recursion(?Send)]
async fn get_font_metrics(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.font.CStrike.getFontMetrics(J)Lsun/font/StrikeMetrics;")
}

#[async_recursion(?Send)]
async fn get_glyph_image_ptrs_native(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.CStrike.getGlyphImagePtrsNative(J[J[II)V")
}

#[async_recursion(?Send)]
async fn get_native_glyph_advance(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.CStrike.getNativeGlyphAdvance(JI)F")
}

#[async_recursion(?Send)]
async fn get_native_glyph_image_bounds(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.CStrike.getNativeGlyphImageBounds(JILjava/awt/geom/Rectangle2D$Float;DD)V")
}

#[async_recursion(?Send)]
async fn get_native_glyph_outline(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.CStrike.getNativeGlyphOutline(JIDD)Ljava/awt/geom/GeneralPath;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.font.CStrike.createNativeStrikePtr(J[D[DII)J"
    )]
    async fn test_create_native_strike_ptr() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = create_native_strike_ptr(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.font.CStrike.disposeNativeStrikePtr(J)V")]
    async fn test_dispose_native_strike_ptr() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = dispose_native_strike_ptr(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.font.CStrike.getFontMetrics(J)Lsun/font/StrikeMetrics;"
    )]
    async fn test_get_font_metrics() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_font_metrics(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.font.CStrike.getGlyphImagePtrsNative(J[J[II)V"
    )]
    async fn test_get_glyph_image_ptrs_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_glyph_image_ptrs_native(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.font.CStrike.getNativeGlyphAdvance(JI)F")]
    async fn test_get_native_glyph_advance() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_native_glyph_advance(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.font.CStrike.getNativeGlyphImageBounds(JILjava/awt/geom/Rectangle2D$Float;DD)V"
    )]
    async fn test_get_native_glyph_image_bounds() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_native_glyph_image_bounds(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.font.CStrike.getNativeGlyphOutline(JIDD)Ljava/awt/geom/GeneralPath;"
    )]
    async fn test_get_native_glyph_outline() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_native_glyph_outline(thread, Parameters::default()).await;
    }
}
