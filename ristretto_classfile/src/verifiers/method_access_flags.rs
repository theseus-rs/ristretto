use crate::Error::InvalidMethodAccessFlags;
use crate::class_access_flags::ClassAccessFlags;
use crate::class_file::ClassFile;
use crate::method::Method;
use crate::method_access_flags::MethodAccessFlags;
use crate::version::Version;
use crate::{JAVA_8, Result};

const VERSION_52_0: Version = JAVA_8;

/// Verify the method `MethodAccessFlags`.
pub fn verify(class_file: &ClassFile, method: &Method) -> Result<()> {
    let access_flags = method.access_flags;
    let public_set = access_flags.contains(MethodAccessFlags::PUBLIC);
    let protected_set = access_flags.contains(MethodAccessFlags::PROTECTED);
    let private_set = access_flags.contains(MethodAccessFlags::PRIVATE);

    if u8::from(public_set) + u8::from(protected_set) + u8::from(private_set) > 1 {
        return Err(InvalidMethodAccessFlags(access_flags.bits()));
    }

    if class_file
        .access_flags
        .contains(ClassAccessFlags::INTERFACE)
        && (access_flags.contains(MethodAccessFlags::PROTECTED)
            || access_flags.contains(MethodAccessFlags::FINAL)
            || access_flags.contains(MethodAccessFlags::SYNCHRONIZED)
            || access_flags.contains(MethodAccessFlags::NATIVE))
    {
        return Err(InvalidMethodAccessFlags(access_flags.bits()));
    }

    if access_flags.contains(MethodAccessFlags::ABSTRACT) {
        if access_flags.contains(MethodAccessFlags::PRIVATE)
            || access_flags.contains(MethodAccessFlags::STATIC)
            || access_flags.contains(MethodAccessFlags::FINAL)
            || access_flags.contains(MethodAccessFlags::SYNCHRONIZED)
            || access_flags.contains(MethodAccessFlags::NATIVE)
        {
            return Err(InvalidMethodAccessFlags(access_flags.bits()));
        }

        if class_file.version.major() >= 46
            && class_file.version.major() <= 60
            && access_flags.contains(MethodAccessFlags::STRICT)
        {
            return Err(InvalidMethodAccessFlags(access_flags.bits()));
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::class_file::ClassFile;
    use crate::{
        JAVA_1_2, JAVA_1_3, JAVA_1_4, JAVA_5, JAVA_6, JAVA_7, JAVA_9, JAVA_10, JAVA_11, JAVA_12,
        JAVA_13, JAVA_14, JAVA_15, JAVA_16,
    };

    #[test]
    fn test_method_success() {
        let class_file = ClassFile::default();
        let method = Method::default();

        assert_eq!(Ok(()), verify(&class_file, &method));
    }

    fn test_method_flag_error(access_flags: MethodAccessFlags) {
        let class_file = ClassFile::default();
        let method = Method {
            access_flags,
            ..Default::default()
        };

        assert_eq!(
            Err(InvalidMethodAccessFlags(access_flags.bits())),
            verify(&class_file, &method)
        );
    }

    #[test]
    fn test_visibility_errors() {
        test_method_flag_error(
            MethodAccessFlags::PUBLIC | MethodAccessFlags::PROTECTED | MethodAccessFlags::PRIVATE,
        );
        test_method_flag_error(MethodAccessFlags::PUBLIC | MethodAccessFlags::PROTECTED);
        test_method_flag_error(MethodAccessFlags::PUBLIC | MethodAccessFlags::PRIVATE);
        test_method_flag_error(MethodAccessFlags::PROTECTED | MethodAccessFlags::PRIVATE);
    }

    fn test_interface_method_error(access_flags: MethodAccessFlags) {
        let class_file = ClassFile {
            access_flags: ClassAccessFlags::INTERFACE,
            ..Default::default()
        };
        let method = Method {
            access_flags,
            ..Default::default()
        };

        assert_eq!(
            Err(InvalidMethodAccessFlags(access_flags.bits())),
            verify(&class_file, &method)
        );
    }

    #[test]
    fn test_interface_access_flag_errors() {
        test_interface_method_error(MethodAccessFlags::PROTECTED);
        test_interface_method_error(MethodAccessFlags::FINAL);
        test_interface_method_error(MethodAccessFlags::SYNCHRONIZED);
        test_interface_method_error(MethodAccessFlags::NATIVE);
    }

    #[test]
    fn test_abstract_access_flag_errors() {
        test_method_flag_error(MethodAccessFlags::ABSTRACT | MethodAccessFlags::PRIVATE);
        test_method_flag_error(MethodAccessFlags::ABSTRACT | MethodAccessFlags::STATIC);
        test_method_flag_error(MethodAccessFlags::ABSTRACT | MethodAccessFlags::FINAL);
        test_method_flag_error(MethodAccessFlags::ABSTRACT | MethodAccessFlags::SYNCHRONIZED);
        test_method_flag_error(MethodAccessFlags::ABSTRACT | MethodAccessFlags::NATIVE);
    }

    fn test_strict_version_error(version: Version) {
        let class_file = ClassFile {
            version,
            ..Default::default()
        };
        let access_flags = MethodAccessFlags::ABSTRACT | MethodAccessFlags::STRICT;
        let method = Method {
            access_flags,
            ..Default::default()
        };

        assert_eq!(
            Err(InvalidMethodAccessFlags(access_flags.bits())),
            verify(&class_file, &method)
        );
    }

    #[test]
    fn test_strict_version_errors() {
        test_strict_version_error(JAVA_1_2);
        test_strict_version_error(JAVA_1_3);
        test_strict_version_error(JAVA_1_4);
        test_strict_version_error(JAVA_5);
        test_strict_version_error(JAVA_6);
        test_strict_version_error(JAVA_7);
        test_strict_version_error(JAVA_8);
        test_strict_version_error(JAVA_9);
        test_strict_version_error(JAVA_10);
        test_strict_version_error(JAVA_11);
        test_strict_version_error(JAVA_12);
        test_strict_version_error(JAVA_13);
        test_strict_version_error(JAVA_14);
        test_strict_version_error(JAVA_15);
        test_strict_version_error(JAVA_16);
    }
}
