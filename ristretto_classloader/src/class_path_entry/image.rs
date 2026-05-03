use crate::Error::{ArchiveError, ClassNotFound};
use crate::Result;
use ahash::{AHashMap, RandomState};
use dashmap::DashMap;
use parking_lot::RwLock;
use ristretto_classfile::ClassFile;
use ristretto_jimage::Image as JImage;
use std::ffi::{OsStr, OsString};
use std::fmt::Debug;
use std::path::PathBuf;
use std::sync::{Arc, LazyLock};

type ClassCache = DashMap<Box<str>, Arc<ClassFile<'static>>, RandomState>;

#[derive(Clone)]
struct CachedImage {
    image: Arc<JImage>,
    packages: Arc<AHashMap<Box<str>, u16>>,
    modules: Arc<Vec<String>>,
    classes: Arc<ClassCache>,
}

/// Runtime images are immutable for the lifetime of a VM process. Reusing the parsed image, its
/// package index, and parsed class templates avoids repeating that work for every VM instance.
static IMAGE_CACHE: LazyLock<RwLock<AHashMap<OsString, CachedImage>>> =
    LazyLock::new(|| RwLock::new(AHashMap::new()));

/// A jimage in the class path.
#[derive(Debug)]
pub struct Image {
    name: OsString,
    #[expect(clippy::struct_field_names)]
    image: Arc<JImage>,
    packages: Arc<AHashMap<Box<str>, u16>>,
    modules: Arc<Vec<String>>,
    classes: Arc<ClassCache>,
}

impl Image {
    /// Create new image from a path.
    ///
    /// # Errors
    ///
    /// if the image cannot be read.
    pub fn new<S: AsRef<OsStr>>(path: S) -> Result<Self> {
        let path = path.as_ref();
        if let Some(cached) = IMAGE_CACHE.read().get(path).cloned() {
            return Ok(Self {
                name: path.to_os_string(),
                image: cached.image,
                packages: cached.packages,
                modules: cached.modules,
                classes: cached.classes,
            });
        }

        // Keep the write lock while constructing the entry so concurrent VM startups do not each
        // parse and index the same large runtime image.
        let mut cache = IMAGE_CACHE.write();
        if let Some(cached) = cache.get(path).cloned() {
            return Ok(Self {
                name: path.to_os_string(),
                image: cached.image,
                packages: cached.packages,
                modules: cached.modules,
                classes: cached.classes,
            });
        }

        let image = JImage::from_file(PathBuf::from(path).as_path())
            .map_err(|error| ArchiveError(error.to_string()))?;
        let mut packages = AHashMap::with_capacity_and_hasher(1_000, RandomState::new());
        let mut modules = Vec::with_capacity(100);
        let mut module_indices: AHashMap<String, u16> =
            AHashMap::with_capacity_and_hasher(100, RandomState::new());

        for resource in &image {
            let resource = resource.map_err(|error| ArchiveError(error.to_string()))?;
            if resource.extension() != "class" || resource.base() == "module-info" {
                continue;
            }

            let package = resource.parent();
            if packages.contains_key(package) {
                continue;
            }

            let module = resource.module();
            let module_index = if let Some(&index) = module_indices.get(module) {
                index
            } else {
                let index = u16::try_from(modules.len())
                    .map_err(|_| ArchiveError("Too many modules in image".to_string()))?;
                let module_string = module.to_string();
                modules.push(module_string.clone());
                module_indices.insert(module_string, index);
                index
            };
            packages.insert(Box::from(package), module_index);
        }

        let cached = CachedImage {
            image: Arc::new(image),
            packages: Arc::new(packages),
            modules: Arc::new(modules),
            classes: Arc::new(DashMap::with_hasher(RandomState::new())),
        };
        cache.insert(path.to_os_string(), cached.clone());

        Ok(Self {
            name: path.to_os_string(),
            image: cached.image,
            packages: cached.packages,
            modules: cached.modules,
            classes: cached.classes,
        })
    }

    /// Get the name of the image.
    pub fn name(&self) -> &OsString {
        &self.name
    }

    /// Read a class from the image.
    ///
    /// # Errors
    ///
    /// if the class file is not found or cannot be read.
    #[expect(clippy::unused_async)]
    pub async fn read_class<S: AsRef<str>>(&self, name: S) -> Result<ClassFile<'static>> {
        let name = name.as_ref();
        if let Some(class_file) = self
            .classes
            .get(name)
            .map(|entry| Arc::clone(entry.value()))
        {
            return Ok((*class_file).clone());
        }

        let (package, _class_name) = name.rsplit_once('/').unwrap_or(("", name));

        let Some(&module_index) = self.packages.get(package) else {
            return Err(ClassNotFound(name.to_string()));
        };
        let module = self
            .modules
            .get(module_index as usize)
            .ok_or_else(|| ClassNotFound(name.to_string()))?;

        // Construct the full resource name: /<module>/<name>.class
        let capacity = module.len() + name.len() + 8;
        let mut full_name = String::with_capacity(capacity);
        full_name.push('/');
        full_name.push_str(module);
        full_name.push('/');
        full_name.push_str(name);
        full_name.push_str(".class");

        let resource = self
            .image
            .get_resource(&full_name)
            .map_err(|error| ArchiveError(error.to_string()))?;

        let class_file = Arc::new(ClassFile::from_bytes(resource.data())?);
        let cached_class_file = match self.classes.entry(Box::from(name)) {
            dashmap::mapref::entry::Entry::Occupied(entry) => Arc::clone(entry.get()),
            dashmap::mapref::entry::Entry::Vacant(entry) => {
                let cached_class_file = Arc::clone(&class_file);
                entry.insert(class_file);
                cached_class_file
            }
        };
        Ok((*cached_class_file).clone())
    }

    /// Read a resource from the image.
    ///
    /// # Errors
    ///
    /// if the resource cannot be read.
    #[expect(clippy::unused_async)]
    pub async fn read_resource<S: AsRef<str>>(
        &self,
        module: Option<&str>,
        name: S,
    ) -> Result<Option<Vec<u8>>> {
        let name = name.as_ref().trim_start_matches('/');
        let Some(module) = module else {
            return Ok(None);
        };

        let mut full_name = String::with_capacity(module.len() + name.len() + 2);
        full_name.push('/');
        full_name.push_str(module);
        full_name.push('/');
        full_name.push_str(name);

        let Ok(resource) = self.image.get_resource(&full_name) else {
            return Ok(None);
        };
        Ok(Some(resource.data().to_vec()))
    }

    /// Get the class names in the image.
    ///
    /// # Errors
    ///
    /// if the class names cannot be read.
    #[expect(clippy::unused_async)]
    pub async fn class_names(&self) -> Result<Vec<String>> {
        let resources = self.image.iter();
        let mut classes = Vec::with_capacity(resources.len());
        for resource in resources {
            let resource = resource.map_err(|error| ArchiveError(error.to_string()))?;
            if resource.extension() == "class" && resource.base() != "module-info" {
                let mut name = String::new();
                if !resource.parent().is_empty() {
                    name.push_str(resource.parent());
                    name.push('/');
                }
                name.push_str(resource.base());
                classes.push(name);
            }
        }
        classes.sort();
        Ok(classes)
    }
}

impl Clone for Image {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            image: Arc::clone(&self.image),
            packages: Arc::clone(&self.packages),
            modules: Arc::clone(&self.modules),
            classes: Arc::clone(&self.classes),
        }
    }
}

impl PartialEq for Image {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::default_class_loader;

    #[tokio::test]
    async fn test_new() -> Result<()> {
        let (java_home, _java_version, _class_loader) = default_class_loader().await?;
        let image_path = java_home.join("lib").join("modules");
        if image_path.exists() {
            let image = Image::new(&image_path)?;
            assert_eq!(image.name(), image_path.as_os_str());
            let cached_image = Image::new(&image_path)?;
            assert!(Arc::ptr_eq(&image.image, &cached_image.image));
            assert!(Arc::ptr_eq(&image.packages, &cached_image.packages));
            assert!(Arc::ptr_eq(&image.modules, &cached_image.modules));
            assert!(Arc::ptr_eq(&image.classes, &cached_image.classes));
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_read_class() -> Result<()> {
        let (java_home, _java_version, _class_loader) = default_class_loader().await?;
        let image_path = java_home.join("lib").join("modules");
        if image_path.exists() {
            let image = Image::new(&image_path)?;
            let class_file = image.read_class("java/lang/Object").await?;
            assert_eq!("java/lang/Object", class_file.class_name()?);
            assert!(image.classes.contains_key("java/lang/Object"));
            let cached_class_file = image.read_class("java/lang/Object").await?;
            assert_eq!(class_file, cached_class_file);
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_class_names() -> Result<()> {
        let (java_home, _java_version, _class_loader) = default_class_loader().await?;
        let image_path = java_home.join("lib").join("modules");
        if image_path.exists() {
            let image = Image::new(&image_path)?;
            let class_names = image.class_names().await?;
            assert!(class_names.contains(&"java/lang/Object".to_string()));
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_read_inner_class() -> Result<()> {
        let (java_home, _java_version, _class_loader) = default_class_loader().await?;
        let image_path = java_home.join("lib").join("modules");
        if image_path.exists() {
            let image = Image::new(&image_path)?;
            let class_file = image.read_class("java/io/ObjectInputFilter$Config").await?;
            assert_eq!("java/io/ObjectInputFilter$Config", class_file.class_name()?);
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_read_resource() -> Result<()> {
        let (java_home, _java_version, _class_loader) = default_class_loader().await?;
        let image_path = java_home.join("lib").join("modules");
        if image_path.exists() {
            let image = Image::new(&image_path)?;
            assert!(
                image
                    .read_resource(Some("java.base"), "/java/lang/Object.class")
                    .await?
                    .is_some()
            );
            assert!(
                image
                    .read_resource(Some("java.base"), "missing.resource")
                    .await?
                    .is_none()
            );
            assert!(
                image
                    .read_resource(None, "java/lang/Object.class")
                    .await?
                    .is_none()
            );
        }
        Ok(())
    }
}
