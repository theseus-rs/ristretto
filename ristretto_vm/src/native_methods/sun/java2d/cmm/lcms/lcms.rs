use crate::arguments::Arguments;
use crate::native_methods::registry::{MethodRegistry, JAVA_11, JAVA_17, JAVA_19, JAVA_20, JAVA_8};
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/java2d/cmm/lcms/LCMS";

/// Register all native methods for `sun.java2d.cmm.lcms.LCMS`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() <= JAVA_8 {
        registry.register(CLASS_NAME, "freeTransform", "(J)V", free_transform);
    }

    if registry.java_major_version() <= JAVA_11 {
        registry.register(CLASS_NAME, "colorConvert", "(Lsun/java2d/cmm/lcms/LCMSTransform;Lsun/java2d/cmm/lcms/LCMSImageLayout;Lsun/java2d/cmm/lcms/LCMSImageLayout;)V", color_convert);
        registry.register(
            CLASS_NAME,
            "getProfileDataNative",
            "(J[B)V",
            get_profile_data_native,
        );
        registry.register(
            CLASS_NAME,
            "getProfileSizeNative",
            "(J)I",
            get_profile_size_native,
        );
    } else {
        registry.register(
            CLASS_NAME,
            "colorConvert",
            "(JLsun/java2d/cmm/lcms/LCMSImageLayout;Lsun/java2d/cmm/lcms/LCMSImageLayout;)V",
            color_convert,
        );
        registry.register(
            CLASS_NAME,
            "getProfileDataNative",
            "(J)[B",
            get_profile_data_native,
        );
    }

    if registry.java_major_version() == JAVA_17 {
        registry.register(
            CLASS_NAME,
            "colorConvert",
            "(JLsun/java2d/cmm/lcms/LCMSImageLayout;Lsun/java2d/cmm/lcms/LCMSImageLayout;)V",
            color_convert,
        );
    }

    if registry.java_major_version() <= JAVA_17 {
        registry.register(
            CLASS_NAME,
            "initLCMS",
            "(Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;)V",
            init_lcms,
        );
    } else if registry.java_major_version() <= JAVA_19 {
        registry.register(
            CLASS_NAME,
            "colorConvert",
            "(JIIIIIIZZLjava/lang/Object;Ljava/lang/Object;II)V",
            color_convert,
        );
    }

    if registry.java_major_version() <= JAVA_19 {
        registry.register(
            CLASS_NAME,
            "getProfileID",
            "(Ljava/awt/color/ICC_Profile;)Lsun/java2d/cmm/lcms/LCMSProfile;",
            get_profile_id,
        );
    } else {
        registry.register(
            CLASS_NAME,
            "colorConvert",
            "(JIIIIIILjava/lang/Object;Ljava/lang/Object;II)V",
            color_convert,
        );
    }

    if registry.java_major_version() <= JAVA_20 {
        registry.register(
            CLASS_NAME,
            "createNativeTransform",
            "([JIIZIZLjava/lang/Object;)J",
            create_native_transform,
        );
    } else {
        registry.register(
            CLASS_NAME,
            "createNativeTransform",
            "([JIIILjava/lang/Object;)J",
            create_native_transform,
        );
    }

    registry.register(CLASS_NAME, "getTagNative", "(JI)[B", get_tag_native);
    registry.register(
        CLASS_NAME,
        "loadProfileNative",
        "([BLjava/lang/Object;)J",
        load_profile_native,
    );
    registry.register(
        CLASS_NAME,
        "setTagDataNative",
        "(JI[B)V",
        set_tag_data_native,
    );
}

#[async_recursion(?Send)]
async fn color_convert(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.cmm.lcms.LCMS.colorConvert(Lsun/java2d/cmm/lcms/LCMSTransform;Lsun/java2d/cmm/lcms/LCMSImageLayout;Lsun/java2d/cmm/lcms/LCMSImageLayout;)V")
}

#[async_recursion(?Send)]
async fn create_native_transform(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.java2d.cmm.lcms.LCMS.createNativeTransform([JIIILjava/lang/Object;)J")
}

#[async_recursion(?Send)]
async fn free_transform(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.cmm.lcms.LCMS.freeTransform(J)V")
}

#[async_recursion(?Send)]
async fn get_profile_data_native(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.java2d.cmm.lcms.LCMS.getProfileDataNative(J[B)V")
}

#[async_recursion(?Send)]
async fn get_profile_id(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.cmm.lcms.LCMS.getProfileID(Ljava/awt/color/ICC_Profile;)Lsun/java2d/cmm/lcms/LCMSProfile;")
}

#[async_recursion(?Send)]
async fn get_profile_size_native(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.java2d.cmm.lcms.LCMS.getProfileSizeNative(J)I")
}

#[async_recursion(?Send)]
async fn get_tag_native(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.cmm.lcms.LCMS.getTagNative(JI)[B")
}

#[async_recursion(?Send)]
async fn init_lcms(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.cmm.lcms.LCMS.initLCMS(Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;)V")
}

#[async_recursion(?Send)]
async fn load_profile_native(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.cmm.lcms.LCMS.loadProfileNative([BLjava/lang/Object;)J")
}

#[async_recursion(?Send)]
async fn set_tag_data_native(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.cmm.lcms.LCMS.setTagDataNative(JI[B)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.cmm.lcms.LCMS.colorConvert(Lsun/java2d/cmm/lcms/LCMSTransform;Lsun/java2d/cmm/lcms/LCMSImageLayout;Lsun/java2d/cmm/lcms/LCMSImageLayout;)V"
    )]
    async fn test_color_convert() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = color_convert(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.cmm.lcms.LCMS.createNativeTransform([JIIILjava/lang/Object;)J"
    )]
    async fn test_create_native_transform() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = create_native_transform(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.java2d.cmm.lcms.LCMS.freeTransform(J)V")]
    async fn test_free_transform() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = free_transform(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.cmm.lcms.LCMS.getProfileDataNative(J[B)V"
    )]
    async fn test_get_profile_data_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_profile_data_native(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.cmm.lcms.LCMS.getProfileID(Ljava/awt/color/ICC_Profile;)Lsun/java2d/cmm/lcms/LCMSProfile;"
    )]
    async fn test_get_profile_id() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_profile_id(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.cmm.lcms.LCMS.getProfileSizeNative(J)I"
    )]
    async fn test_get_profile_size_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_profile_size_native(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.java2d.cmm.lcms.LCMS.getTagNative(JI)[B")]
    async fn test_get_tag_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_tag_native(thread, Arguments::default()).await;
    }
}
