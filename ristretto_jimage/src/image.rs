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
    /// ```rust
    /// # use ristretto_classloader::runtime::default_class_loader;
    /// use ristretto_jimage::Image;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> ristretto_jimage::Result<()> {
    /// # let (java_home, _java_version, _class_loader) = default_class_loader().await.expect("java home");
    /// # let path = java_home.join("lib").join("modules");
    /// let image = Image::from_file(&path)?;
    /// # Ok(())
    /// # }
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
    /// ```rust
    /// # use ristretto_classloader::runtime::default_class_loader;
    /// use ristretto_jimage::Image;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> ristretto_jimage::Result<()> {
    /// # let (java_home, _java_version, _class_loader) = default_class_loader().await.expect("java home");
    /// # let path = java_home.join("lib").join("modules");
    /// let image = Image::from_file(&path)?;
    /// let resource_name = "/java.base/java/lang/Object.class";
    /// let resource = image.get_resource(resource_name)?;
    /// assert_eq!(resource_name, resource.full_name());
    /// # Ok(())
    /// # }
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
    /// ```rust
    /// # use ristretto_classloader::runtime::default_class_loader;
    /// use ristretto_jimage::Image;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> ristretto_jimage::Result<()> {
    /// # let (java_home, _java_version, _class_loader) = default_class_loader().await.expect("java home");
    /// # let path = java_home.join("lib").join("modules");
    /// let image = Image::from_file(&path)?;
    /// for (index, resource) in image.iter().enumerate() {
    ///     let resource = resource?;
    ///     println!("Resource [{index}]: {}", resource.full_name());
    /// }
    /// # Ok(())
    /// # }
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
    use ristretto_classfile::ClassFile;
    use ristretto_classloader::runtime::{default_class_loader, version_class_loader};
    use std::io::Cursor;

    /// Loads the default Image for testing.
    async fn get_test_image() -> Result<Image> {
        let (java_home, _java_version, _class_loader) =
            default_class_loader().await.expect("java home");
        let path = java_home.join("lib").join("modules");
        let image = Image::from_file(&path)?;
        Ok(image)
    }

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

    #[tokio::test]
    async fn test_image_for_all_java_lts_versions() -> Result<()> {
        let versions = ["11.0.29.7.1", "17.0.17.10.1", "21.0.9.10.1", "25.0.1.8.1"];
        for version in &versions {
            let (java_home, _java_version, _class_loader) =
                version_class_loader(version).await.expect("java home");
            let path = java_home.join("lib").join("modules");
            let _path_str = path.to_str().unwrap_or_default();
            let image = Image::from_file(&path)?;
            let header = image.header();
            assert_eq!(header.version(), (&1, &0));

            // Verify the Object class can be found for each version
            let resource_name = "/java.base/java/lang/Object.class";
            let resource = image.get_resource(resource_name)?;
            assert_eq!(resource_name, resource.full_name());

            // Verify that we can iterate all resources
            for resource in &image {
                let resource = resource?;
                assert!(!resource.name().is_empty());
            }
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_get_resource() -> Result<()> {
        let image = get_test_image().await?;
        let resource_name = "/java.base/java/lang/Object.class";
        let resource = image.get_resource(resource_name)?;
        assert_eq!(resource_name, resource.full_name());

        let mut bytes = Cursor::new(resource.data().to_vec());
        let class_file = ClassFile::from_bytes(&mut bytes).expect("read classfile");
        let class_name = class_file.class_name().expect("class name");
        assert_eq!(class_name, "java/lang/Object");
        Ok(())
    }

    #[tokio::test]
    async fn test_image_iterator() -> Result<()> {
        let image = get_test_image().await?;
        let total_resources = image.header().resources_count();
        let iterator = image.iter();

        assert_eq!(
            iterator.size_hint(),
            (total_resources, Some(total_resources))
        );
        assert_eq!(iterator.len(), total_resources);

        let count = image.iter().count();
        assert_eq!(count, total_resources);

        let mut iterator_count = 0;
        for resource in &image {
            let resource = resource?;
            assert!(!resource.full_name().is_empty());
            iterator_count += 1;
        }
        assert_eq!(iterator_count, total_resources);

        Ok(())
    }
}
