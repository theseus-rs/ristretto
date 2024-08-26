use crate::class_access_flags::ClassAccessFlags;
use crate::class_file::ClassFile;
use crate::field::Field;
use crate::field_access_flags::FieldAccessFlags;
use crate::Error::InvalidFieldAccessFlags;
use crate::Result;

/// Verify the field `FieldAccessFlags`.
pub fn verify(class_file: &ClassFile, field: &Field) -> Result<()> {
    let access_flags = field.access_flags;
    let public_set = access_flags.contains(FieldAccessFlags::PUBLIC);
    let protected_set = access_flags.contains(FieldAccessFlags::PROTECTED);
    let private_set = access_flags.contains(FieldAccessFlags::PRIVATE);

    if u8::from(public_set) + u8::from(protected_set) + u8::from(private_set) > 1 {
        return Err(InvalidFieldAccessFlags(access_flags.bits()));
    }

    if class_file
        .access_flags
        .contains(ClassAccessFlags::INTERFACE)
    {
        if !access_flags
            .contains(FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL)
            || access_flags.contains(FieldAccessFlags::PRIVATE)
            || access_flags.contains(FieldAccessFlags::PROTECTED)
            || access_flags.contains(FieldAccessFlags::VOLATILE)
            || access_flags.contains(FieldAccessFlags::TRANSIENT)
            || access_flags.contains(FieldAccessFlags::ENUM)
        {
            return Err(InvalidFieldAccessFlags(access_flags.bits()));
        }
    } else if access_flags.contains(FieldAccessFlags::FINAL | FieldAccessFlags::VOLATILE) {
        return Err(InvalidFieldAccessFlags(access_flags.bits()));
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::class_file::ClassFile;
    use crate::{BaseType, FieldType};

    #[test]
    fn test_interface_success() {
        let class_file = ClassFile {
            access_flags: ClassAccessFlags::INTERFACE,
            ..Default::default()
        };
        let field = Field {
            access_flags: FieldAccessFlags::PUBLIC
                | FieldAccessFlags::STATIC
                | FieldAccessFlags::FINAL,
            name_index: 1,
            descriptor_index: 2,
            field_type: FieldType::Base(BaseType::Int),
            attributes: vec![],
        };

        assert_eq!(Ok(()), verify(&class_file, &field));
    }

    fn test_invalid_flag_error(access_flags: FieldAccessFlags) {
        let class_file = ClassFile::default();
        let field = Field {
            access_flags,
            name_index: 1,
            descriptor_index: 2,
            field_type: FieldType::Base(BaseType::Int),
            attributes: vec![],
        };

        assert_eq!(
            Err(InvalidFieldAccessFlags(access_flags.bits())),
            verify(&class_file, &field)
        );
    }

    #[test]
    fn test_class_invalid_flag() {
        test_invalid_flag_error(
            FieldAccessFlags::PUBLIC | FieldAccessFlags::PROTECTED | FieldAccessFlags::PRIVATE,
        );
        test_invalid_flag_error(FieldAccessFlags::PUBLIC | FieldAccessFlags::PROTECTED);
        test_invalid_flag_error(FieldAccessFlags::PUBLIC | FieldAccessFlags::PRIVATE);
        test_invalid_flag_error(FieldAccessFlags::PROTECTED | FieldAccessFlags::PRIVATE);
    }

    #[test]
    fn test_interface_invalid_signature() {
        let class_file = ClassFile {
            access_flags: ClassAccessFlags::INTERFACE,
            ..Default::default()
        };
        let field = Field {
            access_flags: FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC,
            name_index: 1,
            descriptor_index: 2,
            field_type: FieldType::Base(BaseType::Int),
            attributes: vec![],
        };

        assert_eq!(Err(InvalidFieldAccessFlags(9)), verify(&class_file, &field));
    }

    fn test_interface_invalid_flag_error(access_flags: FieldAccessFlags) {
        let class_file = ClassFile {
            access_flags: ClassAccessFlags::INTERFACE,
            ..Default::default()
        };
        let field = Field {
            access_flags,
            name_index: 1,
            descriptor_index: 2,
            field_type: FieldType::Base(BaseType::Int),
            attributes: vec![],
        };

        assert_eq!(
            Err(InvalidFieldAccessFlags(access_flags.bits())),
            verify(&class_file, &field)
        );
    }

    #[test]
    fn test_interface_invalid_flag() {
        test_interface_invalid_flag_error(FieldAccessFlags::PRIVATE);
        test_interface_invalid_flag_error(FieldAccessFlags::PROTECTED);
        test_interface_invalid_flag_error(FieldAccessFlags::VOLATILE);
        test_interface_invalid_flag_error(FieldAccessFlags::TRANSIENT);
        test_interface_invalid_flag_error(FieldAccessFlags::ENUM);
    }

    #[test]
    fn test_class_success() {
        let class_file = ClassFile::default();
        let field = Field {
            access_flags: FieldAccessFlags::empty(),
            name_index: 1,
            descriptor_index: 2,
            field_type: FieldType::Base(BaseType::Int),
            attributes: vec![],
        };

        assert_eq!(Ok(()), verify(&class_file, &field));
    }

    #[test]
    fn test_class_final_and_volatile_error() {
        let class_file = ClassFile::default();
        let field = Field {
            access_flags: FieldAccessFlags::FINAL | FieldAccessFlags::VOLATILE,
            name_index: 1,
            descriptor_index: 2,
            field_type: FieldType::Base(BaseType::Int),
            attributes: vec![],
        };

        assert_eq!(
            Err(InvalidFieldAccessFlags(80)),
            verify(&class_file, &field)
        );
    }
}
