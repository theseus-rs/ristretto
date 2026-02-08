use ristretto_classfile::VersionSpecification::{
    Any, Equal, GreaterThan, GreaterThanOrEqual, LessThanOrEqual,
};
use ristretto_classfile::{JAVA_8, JAVA_11, JAVA_17, JAVA_21};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/java2d/cmm/lcms/LCMS.colorConvert(Lsun/java2d/cmm/lcms/LCMSTransform;Lsun/java2d/cmm/lcms/LCMSImageLayout;Lsun/java2d/cmm/lcms/LCMSImageLayout;)V",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn color_convert_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.java2d.cmm.lcms.LCMS.colorConvert(Lsun/java2d/cmm/lcms/LCMSTransform;Lsun/java2d/cmm/lcms/LCMSImageLayout;Lsun/java2d/cmm/lcms/LCMSImageLayout;)V"
    )
}

#[intrinsic_method(
    "sun/java2d/cmm/lcms/LCMS.colorConvert(JLsun/java2d/cmm/lcms/LCMSImageLayout;Lsun/java2d/cmm/lcms/LCMSImageLayout;)V",
    Equal(JAVA_17)
)]
#[async_method]
pub async fn color_convert_1<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.java2d.cmm.lcms.LCMS.colorConvert(JLsun/java2d/cmm/lcms/LCMSImageLayout;Lsun/java2d/cmm/lcms/LCMSImageLayout;)V"
    )
}

#[intrinsic_method(
    "sun/java2d/cmm/lcms/LCMS.colorConvert(JIIIIIILjava/lang/Object;Ljava/lang/Object;II)V",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn color_convert_2<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.cmm.lcms.LCMS.colorConvert(JIIIIIILjava/lang/Object;Ljava/lang/Object;II)V")
}

#[intrinsic_method(
    "sun/java2d/cmm/lcms/LCMS.createNativeTransform([JIIZIZLjava/lang/Object;)J",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn create_native_transform_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.cmm.lcms.LCMS.createNativeTransform([JIIZIZLjava/lang/Object;)J")
}

#[intrinsic_method(
    "sun/java2d/cmm/lcms/LCMS.createNativeTransform([JIIILjava/lang/Object;)J",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn create_native_transform_1<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.cmm.lcms.LCMS.createNativeTransform([JIIILjava/lang/Object;)J")
}

#[intrinsic_method("sun/java2d/cmm/lcms/LCMS.freeTransform(J)V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn free_transform<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.cmm.lcms.LCMS.freeTransform(J)V")
}

#[intrinsic_method(
    "sun/java2d/cmm/lcms/LCMS.getProfileDataNative(J[B)V",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_profile_data_native_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.cmm.lcms.LCMS.getProfileDataNative(J[B)V")
}

#[intrinsic_method(
    "sun/java2d/cmm/lcms/LCMS.getProfileDataNative(J)[B",
    GreaterThan(JAVA_11)
)]
#[async_method]
pub async fn get_profile_data_native_1<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.cmm.lcms.LCMS.getProfileDataNative(J)[B")
}

#[intrinsic_method(
    "sun/java2d/cmm/lcms/LCMS.getProfileID(Ljava/awt/color/ICC_Profile;)Lsun/java2d/cmm/lcms/LCMSProfile;",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn get_profile_id<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.java2d.cmm.lcms.LCMS.getProfileID(Ljava/awt/color/ICC_Profile;)Lsun/java2d/cmm/lcms/LCMSProfile;"
    )
}

#[intrinsic_method(
    "sun/java2d/cmm/lcms/LCMS.getProfileSizeNative(J)I",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_profile_size_native<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.cmm.lcms.LCMS.getProfileSizeNative(J)I")
}

#[intrinsic_method("sun/java2d/cmm/lcms/LCMS.getTagNative(JI)[B", Any)]
#[async_method]
pub async fn get_tag_native<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.cmm.lcms.LCMS.getTagNative(JI)[B")
}

#[intrinsic_method(
    "sun/java2d/cmm/lcms/LCMS.initLCMS(Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn init_lcms<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.cmm.lcms.LCMS.initLCMS(Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;)V")
}

#[intrinsic_method(
    "sun/java2d/cmm/lcms/LCMS.loadProfileNative([BLjava/lang/Object;)J",
    Any
)]
#[async_method]
pub async fn load_profile_native<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.cmm.lcms.LCMS.loadProfileNative([BLjava/lang/Object;)J")
}

#[intrinsic_method("sun/java2d/cmm/lcms/LCMS.setTagDataNative(JI[B)V", Any)]
#[async_method]
pub async fn set_tag_data_native<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.cmm.lcms.LCMS.setTagDataNative(JI[B)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.cmm.lcms.LCMS.colorConvert(Lsun/java2d/cmm/lcms/LCMSTransform;Lsun/java2d/cmm/lcms/LCMSImageLayout;Lsun/java2d/cmm/lcms/LCMSImageLayout;)V"
    )]
    async fn test_color_convert_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = color_convert_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.cmm.lcms.LCMS.colorConvert(JLsun/java2d/cmm/lcms/LCMSImageLayout;Lsun/java2d/cmm/lcms/LCMSImageLayout;)V"
    )]
    async fn test_color_convert_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = color_convert_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.cmm.lcms.LCMS.colorConvert(JIIIIIILjava/lang/Object;Ljava/lang/Object;II)V"
    )]
    async fn test_color_convert_2() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = color_convert_2(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.cmm.lcms.LCMS.createNativeTransform([JIIZIZLjava/lang/Object;)J"
    )]
    async fn test_create_native_transform_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = create_native_transform_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.cmm.lcms.LCMS.createNativeTransform([JIIILjava/lang/Object;)J"
    )]
    async fn test_create_native_transform_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = create_native_transform_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.java2d.cmm.lcms.LCMS.freeTransform(J)V")]
    async fn test_free_transform() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = free_transform(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.cmm.lcms.LCMS.getProfileDataNative(J[B)V"
    )]
    async fn test_get_profile_data_native_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_profile_data_native_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.cmm.lcms.LCMS.getProfileDataNative(J)[B"
    )]
    async fn test_get_profile_data_native_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_profile_data_native_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.cmm.lcms.LCMS.getProfileID(Ljava/awt/color/ICC_Profile;)Lsun/java2d/cmm/lcms/LCMSProfile;"
    )]
    async fn test_get_profile_id() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_profile_id(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.cmm.lcms.LCMS.getProfileSizeNative(J)I"
    )]
    async fn test_get_profile_size_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_profile_size_native(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.java2d.cmm.lcms.LCMS.getTagNative(JI)[B")]
    async fn test_get_tag_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_tag_native(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.cmm.lcms.LCMS.initLCMS(Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;)V"
    )]
    async fn test_init_lcms() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_lcms(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.cmm.lcms.LCMS.loadProfileNative([BLjava/lang/Object;)J"
    )]
    async fn test_load_profile_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = load_profile_native(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.cmm.lcms.LCMS.setTagDataNative(JI[B)V"
    )]
    async fn test_set_tag_data_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_tag_data_native(thread, Parameters::default()).await;
    }
}
