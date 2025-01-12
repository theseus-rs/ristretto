use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_8: Version = Version::Java8 { minor: 0 };

/// Register all native methods for `sun.font.SunLayoutEngine`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/font/SunLayoutEngine";
    let java_version = registry.java_version();

    if java_version <= &JAVA_8 {
        registry.register(class_name, "initGVIDs", "()V", init_gv_ids);
        registry.register(class_name, "nativeLayout", "(Lsun/font/Font2D;Lsun/font/FontStrike;[FII[CIIIIIIILjava/awt/geom/Point2D$Float;Lsun/font/GlyphLayout$GVData;JJ)V", native_layout);
    } else {
        registry.register(
            class_name,
            "createFace",
            "(Lsun/font/Font2D;J)J",
            create_face,
        );
        registry.register(class_name, "disposeFace", "(J)V", dispose_face);
        registry.register(class_name, "shape", "(Lsun/font/Font2D;Lsun/font/FontStrike;F[FJ[CLsun/font/GlyphLayout$GVData;IIIILjava/awt/geom/Point2D$Float;II)Z", shape);
    }
}

#[async_recursion(?Send)]
async fn create_face(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.font.SunLayoutEngine.createFace(Lsun/font/Font2D;J)J")
}

#[async_recursion(?Send)]
async fn dispose_face(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.font.SunLayoutEngine.disposeFace(J)V")
}

#[async_recursion(?Send)]
async fn init_gv_ids(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.font.SunLayoutEngine.initGVIDs()V")
}

#[async_recursion(?Send)]
async fn native_layout(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.font.SunLayoutEngine.nativeLayout(Lsun/font/Font2D;Lsun/font/FontStrike;[FII[CIIIIIIILjava/awt/geom/Point2D$Float;Lsun/font/GlyphLayout$GVData;JJ)V")
}

#[async_recursion(?Send)]
async fn shape(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.font.SunLayoutEngine.shape(Lsun/font/Font2D;Lsun/font/FontStrike;F[FJ[CLsun/font/GlyphLayout$GVData;IIIILjava/awt/geom/Point2D$Float;II)Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/font/SunLayoutEngine";
        let java_version = registry.java_version();

        if java_version <= &JAVA_8 {
            assert!(registry.method(class_name, "initGVIDs", "()V").is_some());
            assert!(registry.method(class_name, "nativeLayout", "(Lsun/font/Font2D;Lsun/font/FontStrike;[FII[CIIIIIIILjava/awt/geom/Point2D$Float;Lsun/font/GlyphLayout$GVData;JJ)V").is_some());
        } else {
            assert!(registry
                .method(class_name, "createFace", "(Lsun/font/Font2D;J)J")
                .is_some());
            assert!(registry.method(class_name, "disposeFace", "(J)V").is_some());
            assert!(registry.method(class_name, "shape", "(Lsun/font/Font2D;Lsun/font/FontStrike;F[FJ[CLsun/font/GlyphLayout$GVData;IIIILjava/awt/geom/Point2D$Float;II)Z").is_some());
        }
    }

    #[tokio::test]
    #[should_panic(expected = "sun.font.SunLayoutEngine.createFace(Lsun/font/Font2D;J)J")]
    async fn test_create_face() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = create_face(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.font.SunLayoutEngine.disposeFace(J)V")]
    async fn test_dispose_face() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = dispose_face(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.font.SunLayoutEngine.initGVIDs()V")]
    async fn test_init_gv_ids() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_gv_ids(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.font.SunLayoutEngine.nativeLayout(Lsun/font/Font2D;Lsun/font/FontStrike;[FII[CIIIIIIILjava/awt/geom/Point2D$Float;Lsun/font/GlyphLayout$GVData;JJ)V"
    )]
    async fn test_native_layout() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_layout(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.font.SunLayoutEngine.shape(Lsun/font/Font2D;Lsun/font/FontStrike;F[FJ[CLsun/font/GlyphLayout$GVData;IIIILjava/awt/geom/Point2D$Float;II)Z"
    )]
    async fn test_shape() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = shape(thread, Arguments::default()).await;
    }
}
