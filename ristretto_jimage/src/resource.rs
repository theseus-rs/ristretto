//! Resource types and iterator for Image files.

use crate::Image;
use crate::Result;
use crate::index::Index;
use byteorder::ByteOrder;
use std::borrow::Cow;

/// A resource entry in the Image file.
#[derive(Clone, Debug)]
pub struct Resource<'a> {
    /// The module name of the resource
    module: Cow<'a, str>,
    /// The parent directory path
    parent: Cow<'a, str>,
    /// The base name of the resource
    base: Cow<'a, str>,
    /// The extension of the resource
    extension: Cow<'a, str>,
    /// The resource data
    data: Cow<'a, [u8]>,
}

impl<'a> Resource<'a> {
    /// Constructs a `Resource` from raw bytes at the given index.
    pub(crate) fn from_bytes<T: ByteOrder>(image: &'a Image, index: usize) -> Result<Self> {
        let attribute_offset = Index::get_attribute_offset::<T>(image, index)?;
        let attributes = Index::get_attributes(image, attribute_offset)?;
        let module = Self::get_string_from_attribute(image, attributes.module_offset())?;
        let parent = Self::get_string_from_attribute(image, attributes.parent_offset())?;
        let base = Self::get_string_from_attribute(image, attributes.base_offset())?;
        let extension = Self::get_string_from_attribute(image, attributes.extension_offset())?;
        let data = Index::get_data(image, &attributes)?;
        Ok(Self {
            module,
            parent,
            base,
            extension,
            data,
        })
    }

    /// Retrieves a string from the attribute data at the given offset or `None` if the offset is
    /// zero.
    ///
    /// # Errors
    ///
    /// Returns an error if the attribute data cannot be read.
    fn get_string_from_attribute(image: &'_ Image, offset: usize) -> Result<Cow<'_, str>> {
        if offset == 0 {
            return Ok(Cow::Owned(String::new()));
        }
        Index::get_string(image, offset)
    }

    /// Returns the module of the resource
    #[must_use]
    pub fn module(&self) -> &str {
        &self.module
    }

    /// Returns the parent of the resource
    #[must_use]
    pub fn parent(&self) -> &str {
        &self.parent
    }

    /// Returns the base of the resource
    #[must_use]
    pub fn base(&self) -> &str {
        &self.base
    }

    /// Returns the extension of the resource
    #[must_use]
    pub fn extension(&self) -> &str {
        &self.extension
    }

    /// Returns the resource data
    #[must_use]
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// Returns the resource name excluding the module.
    #[must_use]
    pub fn name(&self) -> String {
        let capacity = self.parent.len() + self.base.len() + self.extension.len() + 2;
        let mut name = String::with_capacity(capacity);
        let mut needs_separator = false;

        if !self.parent.is_empty() {
            name.push_str(&self.parent);
            needs_separator = true;
        }

        if needs_separator {
            name.push('/');
        }
        name.push_str(&self.base);

        if !self.extension.is_empty() {
            name.push('.');
            name.push_str(&self.extension);
        }

        name
    }

    /// Returns the full resource name including the module.
    #[must_use]
    pub fn full_name(&self) -> String {
        if self.module.is_empty() {
            return self.name();
        }

        let name = self.name();
        let capacity = self.module.len() + name.len() + 2;
        let mut full_name = String::with_capacity(capacity);
        full_name.push('/');
        full_name.push_str(&self.module);
        full_name.push('/');
        full_name.push_str(&name);
        full_name
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_name() {
        let resource = Resource {
            module: Cow::Owned("java.base".to_string()),
            parent: Cow::Owned("java/lang".to_string()),
            base: Cow::Owned("String".to_string()),
            extension: Cow::Owned("class".to_string()),
            data: Cow::Owned(Vec::new()),
        };
        assert_eq!(resource.name(), "java/lang/String.class");

        let resource_no_parent = Resource {
            module: Cow::Owned("java.base".to_string()),
            parent: Cow::Owned(String::new()),
            base: Cow::Owned("no_parent".to_string()),
            extension: Cow::Owned("txt".to_string()),
            data: Cow::Owned(Vec::new()),
        };
        assert_eq!(resource_no_parent.name(), "no_parent.txt");

        let resource_no_extension = Resource {
            module: Cow::Owned("java.base".to_string()),
            parent: Cow::Owned("java/lang".to_string()),
            base: Cow::Owned("no_extension".to_string()),
            extension: Cow::Owned(String::new()),
            data: Cow::Owned(Vec::new()),
        };
        assert_eq!(resource_no_extension.name(), "java/lang/no_extension");

        let resource_no_module = Resource {
            module: Cow::Owned(String::new()),
            parent: Cow::Owned("java/lang".to_string()),
            base: Cow::Owned("String".to_string()),
            extension: Cow::Owned("class".to_string()),
            data: Cow::Owned(Vec::new()),
        };
        assert_eq!(resource_no_module.name(), "java/lang/String.class");

        let resource_only_base = Resource {
            module: Cow::Owned(String::new()),
            parent: Cow::Owned(String::new()),
            base: Cow::Owned("README".to_string()),
            extension: Cow::Owned(String::new()),
            data: Cow::Owned(Vec::new()),
        };
        assert_eq!(resource_only_base.name(), "README");
    }

    #[test]
    fn test_full_name() {
        let resource = Resource {
            module: Cow::Owned("java.base".to_string()),
            parent: Cow::Owned("java/lang".to_string()),
            base: Cow::Owned("String".to_string()),
            extension: Cow::Owned("class".to_string()),
            data: Cow::Owned(Vec::new()),
        };
        assert_eq!(resource.full_name(), "/java.base/java/lang/String.class");

        let resource_no_parent = Resource {
            module: Cow::Owned("java.base".to_string()),
            parent: Cow::Owned(String::new()),
            base: Cow::Owned("no_parent".to_string()),
            extension: Cow::Owned("txt".to_string()),
            data: Cow::Owned(Vec::new()),
        };
        assert_eq!(resource_no_parent.full_name(), "/java.base/no_parent.txt");

        let resource_no_extension = Resource {
            module: Cow::Owned("java.base".to_string()),
            parent: Cow::Owned("java/lang".to_string()),
            base: Cow::Owned("no_extension".to_string()),
            extension: Cow::Owned(String::new()),
            data: Cow::Owned(Vec::new()),
        };
        assert_eq!(
            resource_no_extension.full_name(),
            "/java.base/java/lang/no_extension"
        );

        let resource_no_module = Resource {
            module: Cow::Owned(String::new()),
            parent: Cow::Owned("java/lang".to_string()),
            base: Cow::Owned("String".to_string()),
            extension: Cow::Owned("class".to_string()),
            data: Cow::Owned(Vec::new()),
        };
        assert_eq!(resource_no_module.full_name(), "java/lang/String.class");

        let resource_only_base = Resource {
            module: Cow::Owned(String::new()),
            parent: Cow::Owned(String::new()),
            base: Cow::Owned("README".to_string()),
            extension: Cow::Owned(String::new()),
            data: Cow::Owned(Vec::new()),
        };
        assert_eq!(resource_only_base.full_name(), "README");
    }
}
