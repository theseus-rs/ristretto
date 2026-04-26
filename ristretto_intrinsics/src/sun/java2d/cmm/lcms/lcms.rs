use ristretto_classfile::VersionSpecification::{
    Any, Equal, GreaterThan, GreaterThanOrEqual, LessThanOrEqual,
};
use ristretto_classfile::{JAVA_8, JAVA_11, JAVA_17, JAVA_21};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/java2d/cmm/lcms/LCMS.colorConvert(Lsun/java2d/cmm/lcms/LCMSTransform;Lsun/java2d/cmm/lcms/LCMSImageLayout;Lsun/java2d/cmm/lcms/LCMSImageLayout;)V",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn color_convert_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _dest = parameters.pop_reference()?;
    let _src = parameters.pop_reference()?;
    let _trans = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun.java2d.cmm.lcms.LCMS.colorConvert(Lsun/java2d/cmm/lcms/LCMSTransform;Lsun/java2d/cmm/lcms/LCMSImageLayout;Lsun/java2d/cmm/lcms/LCMSImageLayout;)V".to_string()).into())
}

#[intrinsic_method(
    "sun/java2d/cmm/lcms/LCMS.colorConvert(JLsun/java2d/cmm/lcms/LCMSImageLayout;Lsun/java2d/cmm/lcms/LCMSImageLayout;)V",
    Equal(JAVA_17)
)]
#[async_method]
pub async fn color_convert_1<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _dest = parameters.pop_reference()?;
    let _src = parameters.pop_reference()?;
    let _trans = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun.java2d.cmm.lcms.LCMS.colorConvert(JLsun/java2d/cmm/lcms/LCMSImageLayout;Lsun/java2d/cmm/lcms/LCMSImageLayout;)V".to_string()).into())
}

#[intrinsic_method(
    "sun/java2d/cmm/lcms/LCMS.colorConvert(JIIIIIILjava/lang/Object;Ljava/lang/Object;II)V",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn color_convert_2<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg10 = parameters.pop_int()?;
    let _arg9 = parameters.pop_int()?;
    let _arg8 = parameters.pop_reference()?;
    let _arg7 = parameters.pop_reference()?;
    let _arg6 = parameters.pop_int()?;
    let _arg5 = parameters.pop_int()?;
    let _arg4 = parameters.pop_int()?;
    let _arg3 = parameters.pop_int()?;
    let _arg2 = parameters.pop_int()?;
    let _arg1 = parameters.pop_int()?;
    let _arg0 = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.cmm.lcms.LCMS.colorConvert(JIIIIIILjava/lang/Object;Ljava/lang/Object;II)V"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/cmm/lcms/LCMS.createNativeTransform([JIIZIZLjava/lang/Object;)J",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn create_native_transform_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _disposer_ref = parameters.pop_reference()?;
    let _is_out_int_packed = parameters.pop_bool()?;
    let _out_formatter = parameters.pop_int()?;
    let _is_in_int_packed = parameters.pop_bool()?;
    let _in_formatter = parameters.pop_int()?;
    let _render_type = parameters.pop_int()?;
    let _profile_i_ds = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.cmm.lcms.LCMS.createNativeTransform([JIIZIZLjava/lang/Object;)J".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/cmm/lcms/LCMS.createNativeTransform([JIIILjava/lang/Object;)J",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn create_native_transform_1<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _disposer_ref = parameters.pop_reference()?;
    let _out_formatter = parameters.pop_int()?;
    let _in_formatter = parameters.pop_int()?;
    let _rendering_intent = parameters.pop_int()?;
    let _profile_i_ds = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.cmm.lcms.LCMS.createNativeTransform([JIIILjava/lang/Object;)J".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/java2d/cmm/lcms/LCMS.freeTransform(J)V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn free_transform<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _id = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun.java2d.cmm.lcms.LCMS.freeTransform(J)V".to_string())
            .into(),
    )
}

#[intrinsic_method(
    "sun/java2d/cmm/lcms/LCMS.getProfileDataNative(J[B)V",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_profile_data_native_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _data = parameters.pop_reference()?;
    let _ptr = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.cmm.lcms.LCMS.getProfileDataNative(J[B)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/cmm/lcms/LCMS.getProfileDataNative(J)[B",
    GreaterThan(JAVA_11)
)]
#[async_method]
pub async fn get_profile_data_native_1<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _ptr = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.cmm.lcms.LCMS.getProfileDataNative(J)[B".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/cmm/lcms/LCMS.getProfileID(Ljava/awt/color/ICC_Profile;)Lsun/java2d/cmm/lcms/LCMSProfile;",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn get_profile_id<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _profile = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun.java2d.cmm.lcms.LCMS.getProfileID(Ljava/awt/color/ICC_Profile;)Lsun/java2d/cmm/lcms/LCMSProfile;".to_string()).into())
}

#[intrinsic_method(
    "sun/java2d/cmm/lcms/LCMS.getProfileSizeNative(J)I",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_profile_size_native<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _ptr = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.cmm.lcms.LCMS.getProfileSizeNative(J)I".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/java2d/cmm/lcms/LCMS.getTagNative(JI)[B", Any)]
#[async_method]
pub async fn get_tag_native<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _signature = parameters.pop_int()?;
    let _profile_id = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun.java2d.cmm.lcms.LCMS.getTagNative(JI)[B".to_string())
            .into(),
    )
}

#[intrinsic_method(
    "sun/java2d/cmm/lcms/LCMS.initLCMS(Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn init_lcms<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _pf = parameters.pop_reference()?;
    let _il = parameters.pop_reference()?;
    let _trans = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.cmm.lcms.LCMS.initLCMS(Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;)V"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/cmm/lcms/LCMS.loadProfileNative([BLjava/lang/Object;)J",
    Any
)]
#[async_method]
pub async fn load_profile_native<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _disposer_ref = parameters.pop_reference()?;
    let _data = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.cmm.lcms.LCMS.loadProfileNative([BLjava/lang/Object;)J".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/java2d/cmm/lcms/LCMS.setTagDataNative(JI[B)V", Any)]
#[async_method]
pub async fn set_tag_data_native<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _data = parameters.pop_reference()?;
    let _tag_signature = parameters.pop_int()?;
    let _ptr = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.cmm.lcms.LCMS.setTagDataNative(JI[B)V".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_color_convert_0() {
        let (_vm, thread) = crate::test::java11_thread().await.expect("thread");
        let result = color_convert_0(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "sun.java2d.cmm.lcms.LCMS.colorConvert(Lsun/java2d/cmm/lcms/LCMSTransform;Lsun/java2d/cmm/lcms/LCMSImageLayout;Lsun/java2d/cmm/lcms/LCMSImageLayout;)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_color_convert_1() {
        let (_vm, thread) = crate::test::java17_thread().await.expect("thread");
        let result = color_convert_1(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Object(None),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "sun.java2d.cmm.lcms.LCMS.colorConvert(JLsun/java2d/cmm/lcms/LCMSImageLayout;Lsun/java2d/cmm/lcms/LCMSImageLayout;)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_color_convert_2() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = color_convert_2(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.java2d.cmm.lcms.LCMS.colorConvert(JIIIIIILjava/lang/Object;Ljava/lang/Object;II)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_create_native_transform_0() {
        let (_vm, thread) = crate::test::java17_thread().await.expect("thread");
        let result = create_native_transform_0(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::from(false),
                Value::Int(0),
                Value::from(false),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "sun.java2d.cmm.lcms.LCMS.createNativeTransform([JIIZIZLjava/lang/Object;)J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_create_native_transform_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create_native_transform_1(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "sun.java2d.cmm.lcms.LCMS.createNativeTransform([JIIILjava/lang/Object;)J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_free_transform() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = free_transform(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.java2d.cmm.lcms.LCMS.freeTransform(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_profile_data_native_0() {
        let (_vm, thread) = crate::test::java11_thread().await.expect("thread");
        let result = get_profile_data_native_0(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun.java2d.cmm.lcms.LCMS.getProfileDataNative(J[B)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_profile_data_native_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_profile_data_native_1(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.java2d.cmm.lcms.LCMS.getProfileDataNative(J)[B",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_profile_id() {
        let (_vm, thread) = crate::test::java17_thread().await.expect("thread");
        let result = get_profile_id(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun.java2d.cmm.lcms.LCMS.getProfileID(Ljava/awt/color/ICC_Profile;)Lsun/java2d/cmm/lcms/LCMSProfile;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_profile_size_native() {
        let (_vm, thread) = crate::test::java11_thread().await.expect("thread");
        let result = get_profile_size_native(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.java2d.cmm.lcms.LCMS.getProfileSizeNative(J)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_tag_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            get_tag_native(thread, Parameters::new(vec![Value::Long(0), Value::Int(0)])).await;
        assert_eq!(
            "sun.java2d.cmm.lcms.LCMS.getTagNative(JI)[B",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_init_lcms() {
        let (_vm, thread) = crate::test::java17_thread().await.expect("thread");
        let result = init_lcms(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "sun.java2d.cmm.lcms.LCMS.initLCMS(Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_load_profile_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = load_profile_native(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun.java2d.cmm.lcms.LCMS.loadProfileNative([BLjava/lang/Object;)J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_set_tag_data_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_tag_data_native(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Int(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun.java2d.cmm.lcms.LCMS.setTagDataNative(JI[B)V",
            result.unwrap_err().to_string()
        );
    }
}
