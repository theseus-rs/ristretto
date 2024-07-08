use crate::class_access_flags::ClassAccessFlags;
use crate::class_file::ClassFile;
use crate::method::Method;
use crate::method_access_flags::MethodAccessFlags;
use crate::version::Version;
use crate::Error::InvalidMethodAccessFlags;
use crate::Result;

const VERSION_52_0: Version = Version::Java8 { minor: 0 };

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

    #[test]
    fn test_method_success() {
        let class_file = ClassFile::default();
        let method = Method::default();

        assert_eq!(Ok(()), verify(&class_file, &method));
    }

    fn test_method_invalid_flag_error(access_flags: MethodAccessFlags) {
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
    fn test_invalid_flag() {
        test_method_invalid_flag_error(
            MethodAccessFlags::PUBLIC | MethodAccessFlags::PROTECTED | MethodAccessFlags::PRIVATE,
        );
        test_method_invalid_flag_error(MethodAccessFlags::PUBLIC | MethodAccessFlags::PROTECTED);
        test_method_invalid_flag_error(MethodAccessFlags::PUBLIC | MethodAccessFlags::PRIVATE);
        test_method_invalid_flag_error(MethodAccessFlags::PROTECTED | MethodAccessFlags::PRIVATE);
    }
}
