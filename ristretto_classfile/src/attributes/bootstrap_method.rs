use crate::error::Result;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fmt;
use std::io::Cursor;

/// Implementation of `BootstrapMethod`.
///
/// See: <https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.7.23>
#[derive(Clone, Debug, PartialEq)]
pub struct BootstrapMethod {
    pub bootstrap_method_ref: u16,
    pub arguments: Vec<u16>,
}

impl BootstrapMethod {
    /// Deserialize the bootstrap method from bytes.
    ///
    /// # Errors
    /// Should not occur; reserved for future use.
    pub fn from_bytes(bytes: &mut Cursor<Vec<u8>>) -> Result<BootstrapMethod> {
        let bootstrap_method_ref = bytes.read_u16::<BigEndian>()?;
        let arguments_count = bytes.read_u16::<BigEndian>()? as usize;
        let mut arguments = Vec::with_capacity(arguments_count);
        for _ in 0..arguments_count {
            arguments.push(bytes.read_u16::<BigEndian>()?);
        }
        let bootstrap_method = BootstrapMethod {
            bootstrap_method_ref,
            arguments,
        };
        Ok(bootstrap_method)
    }

    /// Serialize the bootstrap method to bytes.
    ///
    /// # Errors
    /// If there are more than 65,534 arguments.
    pub fn to_bytes(&self, bytes: &mut Vec<u8>) -> Result<()> {
        bytes.write_u16::<BigEndian>(self.bootstrap_method_ref)?;

        let arguments_length = u16::try_from(self.arguments.len())?;
        bytes.write_u16::<BigEndian>(arguments_length)?;
        for argument in &self.arguments {
            bytes.write_u16::<BigEndian>(*argument)?;
        }
        Ok(())
    }
}

impl fmt::Display for BootstrapMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "bootstrap_method_ref: {}, arguments: {:?}",
            self.bootstrap_method_ref, self.arguments
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test_log::test]
    fn test_to_string() {
        let bootstrap_method = BootstrapMethod {
            bootstrap_method_ref: 3,
            arguments: vec![42],
        };
        assert_eq!(
            "bootstrap_method_ref: 3, arguments: [42]",
            bootstrap_method.to_string()
        );
    }

    #[test_log::test]
    fn test_serialization() -> Result<()> {
        let bootstrap_method = BootstrapMethod {
            bootstrap_method_ref: 3,
            arguments: vec![42],
        };
        let expected_value = [0, 3, 0, 1, 0, 42];

        let mut bytes = Vec::new();
        bootstrap_method.clone().to_bytes(&mut bytes)?;
        assert_eq!(expected_value, &bytes[..]);

        let mut bytes = Cursor::new(expected_value.to_vec());
        assert_eq!(bootstrap_method, BootstrapMethod::from_bytes(&mut bytes)?);
        Ok(())
    }
}
