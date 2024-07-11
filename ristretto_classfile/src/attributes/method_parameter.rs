use crate::error::Result;
use crate::method_access_flags::MethodAccessFlags;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fmt;
use std::io::Cursor;

/// Implementation of `MethodParameter`.
///
/// See: <https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.7.24>
#[derive(Clone, Debug, PartialEq)]
pub struct MethodParameter {
    pub name_index: u16,
    pub access_flags: MethodAccessFlags,
}

impl MethodParameter {
    /// Deserialize the method parameters from bytes.
    ///
    /// # Errors
    /// Should not occur; reserved for future use.
    pub fn from_bytes(bytes: &mut Cursor<Vec<u8>>) -> Result<MethodParameter> {
        let name_index = bytes.read_u16::<BigEndian>()?;
        let access_flags = MethodAccessFlags::from_bytes(bytes)?;
        let bootstrap_method = MethodParameter {
            name_index,
            access_flags,
        };
        Ok(bootstrap_method)
    }

    /// Serialize the method parameters to bytes.
    ///
    /// # Errors
    /// If method access flags cannot be serialized.
    pub fn to_bytes(&self, bytes: &mut Vec<u8>) -> Result<()> {
        bytes.write_u16::<BigEndian>(self.name_index)?;
        self.access_flags.to_bytes(bytes)
    }
}

impl fmt::Display for MethodParameter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "name_index: #{}, access_flags: {}",
            self.name_index, self.access_flags
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_to_string() {
        let method_parameter = MethodParameter {
            name_index: 3,
            access_flags: MethodAccessFlags::PUBLIC,
        };
        assert_eq!(
            "name_index: #3, access_flags: (0x0001) ACC_PUBLIC",
            method_parameter.to_string()
        );
    }

    #[test]
    fn test_serialization() -> Result<()> {
        let method_parameter = MethodParameter {
            name_index: 3,
            access_flags: MethodAccessFlags::PUBLIC,
        };
        let expected_value = [0, 3, 0, 1];
        let mut bytes = Vec::new();
        method_parameter.clone().to_bytes(&mut bytes)?;
        assert_eq!(expected_value, &bytes[..]);

        let mut bytes = Cursor::new(expected_value.to_vec());
        assert_eq!(method_parameter, MethodParameter::from_bytes(&mut bytes)?);
        Ok(())
    }
}
