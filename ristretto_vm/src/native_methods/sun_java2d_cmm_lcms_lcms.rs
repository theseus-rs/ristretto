use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_8: Version = Version::Java8 { minor: 0 };
const JAVA_11: Version = Version::Java11 { minor: 0 };
const JAVA_17: Version = Version::Java17 { minor: 0 };
const JAVA_19: Version = Version::Java19 { minor: 0 };

/// Register all native methods for `sun.java2d.cmm.lcms.LCMS`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/java2d/cmm/lcms/LCMS";
    let java_version = registry.java_version().clone();

    if java_version <= JAVA_8 {
        registry.register(class_name, "freeTransform", "(J)V", free_transform);
    }

    if java_version <= JAVA_11 {
        registry.register(class_name, "colorConvert", "(Lsun/java2d/cmm/lcms/LCMSTransform;Lsun/java2d/cmm/lcms/LCMSImageLayout;Lsun/java2d/cmm/lcms/LCMSImageLayout;)V", color_convert);
        registry.register(
            class_name,
            "getProfileDataNative",
            "(J[B)V",
            get_profile_data_native,
        );
        registry.register(
            class_name,
            "getProfileSizeNative",
            "(J)I",
            get_profile_size_native,
        );
    } else {
        registry.register(
            class_name,
            "colorConvert",
            "(JLsun/java2d/cmm/lcms/LCMSImageLayout;Lsun/java2d/cmm/lcms/LCMSImageLayout;)V",
            color_convert,
        );
        registry.register(
            class_name,
            "getProfileDataNative",
            "(J)[B",
            get_profile_data_native,
        );
    }

    if java_version == JAVA_17 {
        registry.register(
            class_name,
            "colorConvert",
            "(JLsun/java2d/cmm/lcms/LCMSImageLayout;Lsun/java2d/cmm/lcms/LCMSImageLayout;)V",
            color_convert,
        );
    }

    if java_version <= JAVA_17 {
        registry.register(
            class_name,
            "initLCMS",
            "(Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;)V",
            init_lcms,
        );
    } else if java_version <= JAVA_19 {
        registry.register(
            class_name,
            "colorConvert",
            "(JIIIIIIZZLjava/lang/Object;Ljava/lang/Object;II)V",
            color_convert,
        );
    }

    if java_version <= JAVA_19 {
        registry.register(
            class_name,
            "getProfileID",
            "(Ljava/awt/color/ICC_Profile;)Lsun/java2d/cmm/lcms/LCMSProfile;",
            get_profile_id,
        );
    } else {
        registry.register(
            class_name,
            "colorConvert",
            "(JIIIIIILjava/lang/Object;Ljava/lang/Object;II)V",
            color_convert,
        );
    }

    registry.register(
        class_name,
        "createNativeTransform",
        "([JIIZIZLjava/lang/Object;)J",
        create_native_transform,
    );
    registry.register(class_name, "getTagNative", "(JI)[B", get_tag_native);
    registry.register(
        class_name,
        "loadProfileNative",
        "([BLjava/lang/Object;)J",
        load_profile_native,
    );
    registry.register(
        class_name,
        "setTagDataNative",
        "(JI[B)V",
        set_tag_data_native,
    );
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn color_convert(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn create_native_transform(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn free_transform(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_profile_data_native(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_profile_id(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_profile_size_native(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_tag_native(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn init_lcms(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn load_profile_native(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn set_tag_data_native(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
