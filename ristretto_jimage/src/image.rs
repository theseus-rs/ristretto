//! A module for reading and parsing Image files.
//!
//! ## File layout
//!
//! ```text
//! ╭───────────────────╮
//! │      Header       │
//! ├───────────────────┤
//! │   Redirect Table  │
//! ├───────────────────┤
//! │ Attribute Offsets │
//! ├───────────────────┤
//! │   Attribute Data  │
//! ├───────────────────┤
//! │      Strings      │
//! ├───────────────────┤
//! │       Data        │
//! ╰───────────────────╯
//! ```

use crate::byte_source::ByteSource;
use crate::header::{Header, IMAGE_MAGIC, IMAGE_MAGIC_INVERTED};
use crate::index::Index;
use crate::{Error, Resource, Result};
use byteorder::{BigEndian, LittleEndian};
use std::path::Path;

/// Endian detected from the magic number.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Endian {
    Little,
    Big,
}

/// A representation of an Image file.
#[derive(Debug)]
pub struct Image {
    endian: Endian,
    byte_source: ByteSource,
    header: Header,
}

impl Image {
    /// Creates a new `Image` instance from the given path.
    ///
    /// # Errors
    ///
    /// Returns an error if the path cannot be read or if it is not a valid Image path.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use ristretto_jimage::Image;
    /// use std::path::PathBuf;
    ///
    /// fn main() -> ristretto_jimage::Result<()> {
    ///     // Typically found at $JAVA_HOME/lib/modules
    ///     let path = PathBuf::from("/path/to/java/lib/modules");
    ///     let image = Image::from_file(&path)?;
    ///     Ok(())
    /// }
    /// ```
    pub fn from_file(path: &Path) -> Result<Self> {
        let byte_source = ByteSource::from(path)?;
        let bytes = byte_source.get_bytes(0..28)?;
        let endian = endian_from_magic(&bytes[..4])?;
        let header = match endian {
            Endian::Little => Header::from_bytes::<LittleEndian>(&bytes[..28])?,
            Endian::Big => Header::from_bytes::<BigEndian>(&bytes[..28])?,
        };

        let image = Self {
            endian,
            byte_source,
            header,
        };
        Ok(image)
    }

    /// Returns a reference to the underlying byte source.
    pub(crate) fn byte_source(&self) -> &ByteSource {
        &self.byte_source
    }

    /// Returns a reference to the header of the Image file.
    pub(crate) fn header(&self) -> &Header {
        &self.header
    }

    /// Retrieves a resource by name.
    ///
    /// # Errors
    ///
    /// Returns an error if the resource cannot be found or if there is an issue reading it.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use ristretto_jimage::Image;
    /// use std::path::PathBuf;
    ///
    /// fn main() -> ristretto_jimage::Result<()> {
    ///     // Typically found at $JAVA_HOME/lib/modules
    ///     let path = PathBuf::from("/path/to/java/lib/modules");
    ///     let image = Image::from_file(&path)?;
    ///     let resource_name = "/java.base/java/lang/Object.class";
    ///     let resource = image.get_resource(resource_name)?;
    ///     assert_eq!(resource_name, resource.full_name());
    ///     Ok(())
    /// }
    /// ```
    pub fn get_resource(&'_ self, name: &str) -> Result<Resource<'_>> {
        let resource = match self.endian {
            Endian::Little => {
                let resource_offset = Index::get_resource_offset::<LittleEndian>(self, name)?;
                Resource::from_bytes::<LittleEndian>(self, resource_offset)?
            }
            Endian::Big => {
                let resource_offset = Index::get_resource_offset::<BigEndian>(self, name)?;
                Resource::from_bytes::<BigEndian>(self, resource_offset)?
            }
        };

        let resource_name = resource.full_name();
        if resource_name != name {
            return Err(Error::ResourceNameMismatch {
                expected: name.to_string(),
                actual: resource_name.clone(),
            });
        }

        Ok(resource)
    }

    /// Returns an iterator over all resources in the Image file.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use ristretto_jimage::Image;
    /// use std::path::PathBuf;
    ///
    /// fn main() -> ristretto_jimage::Result<()> {
    ///     // Typically found at $JAVA_HOME/lib/modules
    ///     let path = PathBuf::from("/path/to/java/lib/modules");
    ///     let image = Image::from_file(&path)?;
    ///     for (index, resource) in image.iter().enumerate() {
    ///         let resource = resource?;
    ///         println!("Resource [{index}]: {}", resource.full_name());
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub fn iter(&self) -> ImageIterator<'_> {
        ImageIterator {
            image: self,
            current_index: 0,
            total_resources: self.header.table_length(),
        }
    }
}

/// Determines the endian from the magic bytes.
///
/// # Errors
/// Returns an error if the magic bytes do not match any known endian.
fn endian_from_magic(bytes: &[u8]) -> Result<Endian> {
    if bytes == IMAGE_MAGIC_INVERTED {
        return Ok(Endian::Little);
    } else if bytes == IMAGE_MAGIC {
        return Ok(Endian::Big);
    }
    Err(Error::InvalidMagicBytes(bytes.to_vec()))
}

/// Iterator for Image resources.
#[derive(Debug)]
pub struct ImageIterator<'a> {
    image: &'a Image,
    current_index: usize,
    total_resources: usize,
}

impl<'a> Iterator for ImageIterator<'a> {
    type Item = Result<Resource<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index >= self.total_resources {
            return None;
        }
        let resource_result = match self.image.endian {
            Endian::Little => Resource::from_bytes::<LittleEndian>(self.image, self.current_index),
            Endian::Big => Resource::from_bytes::<BigEndian>(self.image, self.current_index),
        };
        self.current_index += 1;
        Some(resource_result)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.total_resources.saturating_sub(self.current_index);
        (remaining, Some(remaining))
    }
}

impl<'a> IntoIterator for &'a Image {
    type Item = Result<Resource<'a>>;
    type IntoIter = ImageIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl ExactSizeIterator for ImageIterator<'_> {
    fn len(&self) -> usize {
        self.total_resources.saturating_sub(self.current_index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_endian_from_magic() -> Result<()> {
        assert_eq!(
            endian_from_magic(&[0xDA, 0xDA, 0xFE, 0xCA])?,
            Endian::Little
        );
        assert_eq!(endian_from_magic(&[0xCA, 0xFE, 0xDA, 0xDA])?, Endian::Big);
        assert!(endian_from_magic(&[0x00, 0x00, 0x00, 0x00]).is_err());

        Ok(())
    }
}
