use crate::attributes::Attribute;
use crate::class_access_flags::ClassAccessFlags;
use crate::constant::Constant;
use crate::constant_pool::ConstantPool;
use crate::error::Error::{InvalidMagicNumber, VerificationError};
use crate::error::Result;
use crate::field::Field;
use crate::method::Method;
use crate::verifiers::verifier;
use crate::version::Version;
use crate::Error::InvalidConstantPoolIndexType;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::Cursor;

const MAGIC: u32 = 0xCAFE_BABE;

/// `ClassFile` represents the content of a Java .class file.
///
/// See: <https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.1>
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ClassFile {
    pub version: Version,
    pub constant_pool: ConstantPool,
    pub access_flags: ClassAccessFlags,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces: Vec<u16>,
    pub fields: Vec<Field>,
    pub methods: Vec<Method>,
    pub attributes: Vec<Attribute>,
}

impl ClassFile {
    /// Get the class name.
    ///
    /// # Errors
    /// Returns an error if the class name is not found.
    pub fn class_name(&self) -> Result<&String> {
        let class_constant = self.constant_pool.try_get(self.this_class)?;
        let Constant::Class { name_index } = class_constant else {
            return Err(InvalidConstantPoolIndexType(self.this_class));
        };

        self.constant_pool.try_get_utf8(*name_index)
    }

    /// Verify the `ClassFile`.
    ///
    /// # Errors
    /// Returns a `VerificationError` if the verification fails.
    pub fn verify(&self) -> Result<()> {
        match verifier::verify(self) {
            Ok(()) => Ok(()),
            Err(error) => {
                let context = self.class_name()?;
                let verification_error = VerificationError {
                    context: context.to_string(),
                    message: error.to_string(),
                };
                Err(verification_error)
            }
        }
    }

    /// Deserialize the `ClassFile` from bytes.
    ///
    /// # Errors
    /// Returns an error if the bytes are not a valid class file.
    pub fn from_bytes(bytes: &mut Cursor<Vec<u8>>) -> Result<ClassFile> {
        let magic = bytes.read_u32::<BigEndian>()?;
        if magic != MAGIC {
            return Err(InvalidMagicNumber(magic));
        }

        let version = Version::from_bytes(bytes)?;
        let constant_pool = ConstantPool::from_bytes(bytes)?;
        let access_flags = ClassAccessFlags::from_bytes(bytes)?;
        let this_class = bytes.read_u16::<BigEndian>()?;
        let super_class = bytes.read_u16::<BigEndian>()?;

        let interfaces_count = bytes.read_u16::<BigEndian>()?;
        let mut interfaces = Vec::with_capacity(interfaces_count as usize);
        for _ in 0..interfaces_count {
            interfaces.push(bytes.read_u16::<BigEndian>()?);
        }

        let field_count = bytes.read_u16::<BigEndian>()?;
        let mut fields = Vec::with_capacity(field_count as usize);
        for _ in 0..field_count {
            let field = Field::from_bytes(&constant_pool, bytes)?;
            fields.push(field);
        }

        let method_count = bytes.read_u16::<BigEndian>()?;
        let mut methods = Vec::with_capacity(method_count as usize);
        for _ in 0..method_count {
            let method = Method::from_bytes(&constant_pool, bytes)?;
            methods.push(method);
        }

        let attribute_count = bytes.read_u16::<BigEndian>()?;
        let mut attributes = Vec::with_capacity(attribute_count as usize);
        for _ in 0..attribute_count {
            let attribute = Attribute::from_bytes(&constant_pool, bytes)?;
            attributes.push(attribute);
        }

        let class_file = ClassFile {
            version,
            constant_pool,
            access_flags,
            this_class,
            super_class,
            interfaces,
            fields,
            methods,
            attributes,
        };
        Ok(class_file)
    }

    /// Serialize the `ClassFile` to bytes.
    ///
    /// # Errors
    /// - If there are more than 65,535 interfaces, fields, methods, or attributes.
    pub fn to_bytes(self, bytes: &mut Vec<u8>) -> Result<()> {
        bytes.write_u32::<BigEndian>(MAGIC)?;
        self.version.to_bytes(bytes)?;
        self.constant_pool.to_bytes(bytes)?;
        self.access_flags.to_bytes(bytes)?;
        bytes.write_u16::<BigEndian>(self.this_class)?;
        bytes.write_u16::<BigEndian>(self.super_class)?;

        let interfaces_length = u16::try_from(self.interfaces.len())?;
        bytes.write_u16::<BigEndian>(interfaces_length)?;
        for interface in self.interfaces {
            bytes.write_u16::<BigEndian>(interface)?;
        }

        let fields_length = u16::try_from(self.fields.len())?;
        bytes.write_u16::<BigEndian>(fields_length)?;
        for field in self.fields {
            field.to_bytes(bytes)?;
        }

        let methods_length = u16::try_from(self.methods.len())?;
        bytes.write_u16::<BigEndian>(methods_length)?;
        for method in self.methods {
            method.to_bytes(bytes)?;
        }

        let attributes_length = u16::try_from(self.attributes.len())?;
        bytes.write_u16::<BigEndian>(attributes_length)?;
        for attribute in self.attributes {
            attribute.to_bytes(bytes)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::error::Result;

    #[test]
    fn test_invalid_magic() {
        let invalid_magic: u32 = 0x0102_0304;
        let mut bytes = Cursor::new(invalid_magic.to_be_bytes().to_vec());
        assert_eq!(
            Err(InvalidMagicNumber(invalid_magic)),
            ClassFile::from_bytes(&mut bytes)
        );
    }

    #[test]
    fn test_verify() -> Result<()> {
        let class_bytes = include_bytes!("../classes/Simple.class");
        let expected_bytes = class_bytes.to_vec();
        let class_file = ClassFile::from_bytes(&mut Cursor::new(expected_bytes.clone()))?;

        assert!(class_file.verify().is_ok());
        Ok(())
    }

    #[test]
    fn test_verify_error() -> Result<()> {
        let mut constant_pool = ConstantPool::default();
        constant_pool.add(Constant::Utf8("Test".to_string()));
        constant_pool.add(Constant::Class {
            name_index: u16::try_from(constant_pool.len())?,
        });
        let class_index = u16::try_from(constant_pool.len())?;
        // Add an invalid constant to trigger a verification error.
        constant_pool.add(Constant::Class {
            name_index: u16::MAX,
        });
        let class_file = ClassFile {
            constant_pool: constant_pool.clone(),
            this_class: class_index,
            ..Default::default()
        };

        assert_eq!(
            Err(VerificationError {
                context: "Test".to_string(),
                message: "Invalid constant pool index 2".to_string()
            }),
            class_file.verify()
        );
        Ok(())
    }

    #[test]
    fn test_minimum_serialization() -> Result<()> {
        let class_bytes = include_bytes!("../classes/Minimum.class");
        let expected_bytes = class_bytes.to_vec();
        let class_file = ClassFile::from_bytes(&mut Cursor::new(expected_bytes.clone()))?;

        assert_eq!(Version::Java21 { minor: 0 }, class_file.version);
        assert_eq!(
            ClassAccessFlags::PUBLIC | ClassAccessFlags::SUPER,
            class_file.access_flags
        );
        assert_eq!("Minimum", class_file.class_name()?);

        let mut bytes = Vec::with_capacity(4);
        class_file.to_bytes(&mut bytes)?;
        assert_eq!(expected_bytes, bytes);
        Ok(())
    }

    #[test]
    fn test_simple_serialization() -> Result<()> {
        let class_bytes = include_bytes!("../classes/Simple.class");
        let expected_bytes = class_bytes.to_vec();
        let class_file = ClassFile::from_bytes(&mut Cursor::new(expected_bytes.clone()))?;

        assert_eq!(Version::Java21 { minor: 0 }, class_file.version);
        assert_eq!(
            ClassAccessFlags::PUBLIC | ClassAccessFlags::SUPER,
            class_file.access_flags
        );
        assert_eq!("Simple", class_file.class_name()?);

        let mut bytes = Vec::with_capacity(4);
        class_file.to_bytes(&mut bytes)?;
        assert_eq!(expected_bytes, bytes);
        Ok(())
    }
}
