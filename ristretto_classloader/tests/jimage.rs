//! Integration tests for `ristretto_jimage` crate. The test are included in this crate in order to
//! make use of the class loader utilities for locating the `JImage` files without cyclical
//! dependencies.

use ristretto_classfile::ClassFile;
use ristretto_classloader::ALL_LTS_VERSIONS;
use ristretto_classloader::runtime::{default_class_loader, version_class_loader};
use ristretto_jimage::{Image, Result};
use std::io::Cursor;

/// Loads the default Image for testing.
async fn get_test_image() -> Result<Image> {
    let (java_home, _java_version, _class_loader) =
        default_class_loader().await.expect("java home");
    let path = java_home.join("lib").join("modules");
    let image = Image::from_file(&path)?;
    Ok(image)
}

#[tokio::test]
async fn test_image_for_all_java_lts_versions() -> Result<()> {
    for version in ALL_LTS_VERSIONS {
        let major_version = version
            .split('.')
            .next()
            .and_then(|version| version.parse::<u32>().ok())
            .expect("major version");
        if major_version <= 8 {
            continue;
        }
        let (java_home, _java_version, _class_loader) =
            version_class_loader(version).await.expect("java home");
        let path = java_home.join("lib").join("modules");
        let image = Image::from_file(&path)?;

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
async fn test_get_resource_and_parse_classfile() -> Result<()> {
    let image = get_test_image().await?;
    let resource_name = "/java.base/java/lang/Object.class";
    let resource = image.get_resource(resource_name)?;
    assert_eq!(resource_name, resource.full_name());

    let mut bytes = Cursor::new(resource.data());
    let class_file = ClassFile::from_bytes(&mut bytes).expect("read classfile");
    let class_name = class_file.class_name().expect("class name");
    assert_eq!(class_name, "java/lang/Object");
    Ok(())
}

#[tokio::test]
async fn test_image_iterator() -> Result<()> {
    let image = get_test_image().await?;
    let iterator = image.iter();
    let total_resources = iterator.len();

    assert_eq!(
        iterator.size_hint(),
        (total_resources, Some(total_resources))
    );

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

#[tokio::test]
async fn test_get_resource_offset() -> Result<()> {
    let image = get_test_image().await?;
    let resource_name = "/java.base/java/lang/Object.class";
    let resource = image.get_resource(resource_name)?;
    assert_eq!(resource_name, resource.full_name());
    Ok(())
}

#[tokio::test]
async fn test_get_resource_offset_invalid() -> Result<()> {
    let image = get_test_image().await?;
    let result = image.get_resource("/foo/42");
    assert!(result.is_err());
    Ok(())
}
