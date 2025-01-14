use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.font.CStrike`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/font/CStrike";
    registry.register(
        class_name,
        "createNativeStrikePtr",
        "(J[D[DII)J",
        create_native_strike_ptr,
    );
    registry.register(
        class_name,
        "disposeNativeStrikePtr",
        "(J)V",
        dispose_native_strike_ptr,
    );
    registry.register(
        class_name,
        "getFontMetrics",
        "(J)Lsun/font/StrikeMetrics;",
        get_font_metrics,
    );
    registry.register(
        class_name,
        "getGlyphImagePtrsNative",
        "(J[J[II)V",
        get_glyph_image_ptrs_native,
    );
    registry.register(
        class_name,
        "getNativeGlyphAdvance",
        "(JI)F",
        get_native_glyph_advance,
    );
    registry.register(
        class_name,
        "getNativeGlyphImageBounds",
        "(JILjava/awt/geom/Rectangle2D$Float;DD)V",
        get_native_glyph_image_bounds,
    );
    registry.register(
        class_name,
        "getNativeGlyphOutline",
        "(JIDD)Ljava/awt/geom/GeneralPath;",
        get_native_glyph_outline,
    );
}

#[async_recursion(?Send)]
async fn create_native_strike_ptr(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.font.CStrike.createNativeStrikePtr(J[D[DII)J")
}

#[async_recursion(?Send)]
async fn dispose_native_strike_ptr(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.font.CStrike.disposeNativeStrikePtr(J)V")
}

#[async_recursion(?Send)]
async fn get_font_metrics(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.font.CStrike.getFontMetrics(J)Lsun/font/StrikeMetrics;")
}

#[async_recursion(?Send)]
async fn get_glyph_image_ptrs_native(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.font.CStrike.getGlyphImagePtrsNative(J[J[II)V")
}

#[async_recursion(?Send)]
async fn get_native_glyph_advance(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.font.CStrike.getNativeGlyphAdvance(JI)F")
}

#[async_recursion(?Send)]
async fn get_native_glyph_image_bounds(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.font.CStrike.getNativeGlyphImageBounds(JILjava/awt/geom/Rectangle2D$Float;DD)V")
}

#[async_recursion(?Send)]
async fn get_native_glyph_outline(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.font.CStrike.getNativeGlyphOutline(JIDD)Ljava/awt/geom/GeneralPath;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/font/CStrike";
        assert!(registry
            .method(class_name, "createNativeStrikePtr", "(J[D[DII)J")
            .is_some());
        assert!(registry
            .method(class_name, "disposeNativeStrikePtr", "(J)V")
            .is_some());
        assert!(registry
            .method(class_name, "getFontMetrics", "(J)Lsun/font/StrikeMetrics;")
            .is_some());
        assert!(registry
            .method(class_name, "getGlyphImagePtrsNative", "(J[J[II)V")
            .is_some());
        assert!(registry
            .method(class_name, "getNativeGlyphAdvance", "(JI)F")
            .is_some());
        assert!(registry
            .method(
                class_name,
                "getNativeGlyphImageBounds",
                "(JILjava/awt/geom/Rectangle2D$Float;DD)V"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "getNativeGlyphOutline",
                "(JIDD)Ljava/awt/geom/GeneralPath;"
            )
            .is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.font.CStrike.createNativeStrikePtr(J[D[DII)J")]
    async fn test_create_native_strike_ptr() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = create_native_strike_ptr(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.font.CStrike.disposeNativeStrikePtr(J)V")]
    async fn test_dispose_native_strike_ptr() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = dispose_native_strike_ptr(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.font.CStrike.getFontMetrics(J)Lsun/font/StrikeMetrics;")]
    async fn test_get_font_metrics() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_font_metrics(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.font.CStrike.getGlyphImagePtrsNative(J[J[II)V")]
    async fn test_get_glyph_image_ptrs_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_glyph_image_ptrs_native(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.font.CStrike.getNativeGlyphAdvance(JI)F")]
    async fn test_get_native_glyph_advance() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_native_glyph_advance(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.font.CStrike.getNativeGlyphImageBounds(JILjava/awt/geom/Rectangle2D$Float;DD)V"
    )]
    async fn test_get_native_glyph_image_bounds() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_native_glyph_image_bounds(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.font.CStrike.getNativeGlyphOutline(JIDD)Ljava/awt/geom/GeneralPath;"
    )]
    async fn test_get_native_glyph_outline() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_native_glyph_outline(thread, Arguments::default()).await;
    }
}
