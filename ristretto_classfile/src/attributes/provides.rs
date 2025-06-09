use crate::error::Result;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fmt;
use std::io::Cursor;

/// Represents a service implementation declaration within a module descriptor.
///
/// The `Provides` struct corresponds to a "provides" directive in a Java module declaration, which
/// specifies that a given service (identified by `index`) is implemented by one or more classes
/// (identified by the indices in `with_index`).
///
/// In the context of the Module attribute in a class file, this structure represents an entry in
/// the `provides_table` that indicates which services are provided by the module and which classes
/// implement those services.
///
/// # Fields
///
/// * `index` - The index into the constant pool representing the service interface
///   that is being provided.
/// * `with_index` - A list of indices into the constant pool representing classes
///   that implement the service interface.
///
/// # Examples
///
/// ```rust
/// use ristretto_classfile::attributes::Provides;
///
/// // Create a provides entry where the service interface is at constant pool index 5
/// // and it's implemented by classes at indices 10 and 12
/// let provides = Provides {
///     index: 5,
///     with_index: vec![10, 12],
/// };
///
/// // Serialize to bytes
/// let mut bytes = Vec::new();
/// provides.to_bytes(&mut bytes)?;
///
/// // Deserialize from bytes
/// let mut cursor = std::io::Cursor::new(bytes);
/// let deserialized = Provides::from_bytes(&mut cursor)?;
/// assert_eq!(provides, deserialized);
///
/// // Display the provides entry
/// assert_eq!(provides.to_string(), "Provides[index=5, with_index=[10, 12]]");
/// # Ok::<(), ristretto_classfile::Error>(())
/// ```
///
/// # References
///
/// - [JVM Specification §4.7.25](https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.7.25)
#[derive(Clone, Debug, PartialEq)]
pub struct Provides {
    pub index: u16,
    pub with_index: Vec<u16>,
}

impl Provides {
    /// Deserialize the provides from bytes.
    ///
    /// # Errors
    ///
    /// Should not occur; reserved for future use.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::Provides;
    /// use std::io::Cursor;
    ///
    /// // index: 1, with_index: [2]
    /// let mut bytes = Cursor::new(vec![0x00, 0x01, 0x00, 0x01, 0x00, 0x02]);
    /// let provides = Provides::from_bytes(&mut bytes)?;
    /// assert_eq!(provides.index, 1);
    /// assert_eq!(provides.with_index, vec![2]);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    pub fn from_bytes(bytes: &mut Cursor<Vec<u8>>) -> Result<Provides> {
        let index = bytes.read_u16::<BigEndian>()?;
        let to_index_count = bytes.read_u16::<BigEndian>()?;
        let mut with_index = Vec::with_capacity(to_index_count as usize);
        for _ in 0..to_index_count {
            with_index.push(bytes.read_u16::<BigEndian>()?);
        }
        let requires = Provides { index, with_index };
        Ok(requires)
    }

    /// Serialize the provides to bytes.
    ///
    /// # Errors
    ///
    /// If there are more than 65,534 `with_index` values.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::Provides;
    ///
    /// let provides = Provides {
    ///    index: 1,
    ///    with_index: vec![2],
    /// };
    /// let mut bytes = Vec::new();
    /// provides.to_bytes(&mut bytes)?;
    /// assert_eq!(bytes, vec![0x00, 0x01, 0x00, 0x01, 0x00, 0x02]);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    pub fn to_bytes(&self, bytes: &mut Vec<u8>) -> Result<()> {
        bytes.write_u16::<BigEndian>(self.index)?;

        let with_index_length = u16::try_from(self.with_index.len())?;
        bytes.write_u16::<BigEndian>(with_index_length)?;
        for index in &self.with_index {
            bytes.write_u16::<BigEndian>(*index)?;
        }

        Ok(())
    }
}

impl fmt::Display for Provides {
    /// Formats the `Provides` structure for display.
    ///
    /// This implementation returns a string in the format
    /// `Provides[index={index}, with_index={with_index}]`, where `{index}` is the numeric index
    /// value and `{with_index}` is the debug representation of the vector.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::attributes::Provides;
    ///
    /// let provides = Provides {
    ///     index: 1,
    ///     with_index: vec![2, 3],
    /// };
    ///
    /// let output = provides.to_string();
    /// assert_eq!(output, "Provides[index=1, with_index=[2, 3]]");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Provides[index={}, with_index={:?}]",
            self.index, self.with_index
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_to_string() {
        let provides = Provides {
            index: 1,
            with_index: vec![2],
        };
        assert_eq!("Provides[index=1, with_index=[2]]", provides.to_string());
    }

    #[test]
    fn test_serialization() -> Result<()> {
        let provides = Provides {
            index: 1,
            with_index: vec![2],
        };
        let expected_value = [0, 1, 0, 1, 0, 2];
        let mut bytes = Vec::new();
        provides.clone().to_bytes(&mut bytes)?;
        assert_eq!(expected_value, &bytes[..]);

        let mut bytes = Cursor::new(expected_value.to_vec());
        assert_eq!(provides, Provides::from_bytes(&mut bytes)?);
        Ok(())
    }
}
