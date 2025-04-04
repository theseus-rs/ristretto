use crate::error::Result;
use byteorder::{ReadBytesExt, WriteBytesExt};
use std::fmt;
use std::io::Cursor;

/// Implementation of `TargetPath`.
///
/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.7.20.2>
#[derive(Clone, Debug, PartialEq)]
pub struct TargetPath {
    pub type_path_kind: u8,
    pub type_argument_index: u8,
}

impl TargetPath {
    /// Deserialize the target path from bytes.
    ///
    /// # Errors
    /// Should not occur; reserved for future use.
    pub fn from_bytes(bytes: &mut Cursor<Vec<u8>>) -> Result<TargetPath> {
        let type_path_kind = bytes.read_u8()?;
        let type_argument_index = bytes.read_u8()?;

        let target_path = TargetPath {
            type_path_kind,
            type_argument_index,
        };
        Ok(target_path)
    }

    /// Serialize the target path to bytes.
    ///
    /// # Errors
    /// Should not occur; reserved for future use.
    pub fn to_bytes(&self, bytes: &mut Vec<u8>) -> Result<()> {
        bytes.write_u8(self.type_path_kind)?;
        bytes.write_u8(self.type_argument_index)?;
        Ok(())
    }
}

impl fmt::Display for TargetPath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "TargetPath[type_path_kind={}, type_argument_index={}]",
            self.type_path_kind, self.type_argument_index
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_serialization() -> Result<()> {
        let target_path = TargetPath {
            type_path_kind: 1,
            type_argument_index: 2,
        };
        let expected_value = [1, 2];

        assert_eq!(
            "TargetPath[type_path_kind=1, type_argument_index=2]",
            target_path.to_string()
        );

        let mut bytes = Vec::new();
        target_path.clone().to_bytes(&mut bytes)?;
        assert_eq!(expected_value, &bytes[..]);

        let mut bytes = Cursor::new(expected_value.to_vec());
        assert_eq!(target_path, TargetPath::from_bytes(&mut bytes)?);
        Ok(())
    }
}
