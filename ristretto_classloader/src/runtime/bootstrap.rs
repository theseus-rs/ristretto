use crate::runtime::util;
use crate::{Class, ClassLoader, ClassPath, Error, Result};
use flate2::bufread::GzDecoder;
use ristretto_classfile::Error::IoError;
use ristretto_classfile::{ClassAccessFlags, ClassFile, ConstantPool, JAVA_1_0_2};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Weak};
use std::{env, io};
use tar::Archive;
use tracing::{debug, warn};

/// The default Java version used by the class loader. This is the version that will be used if no
/// version is specified when creating a class loader.
pub const DEFAULT_JAVA_VERSION: &str = JAVA_25_VERSION;

/// Java 8 LTS version
pub const JAVA_8_VERSION: &str = "8.482.08.1";

/// Java 11 LTS version
pub const JAVA_11_VERSION: &str = "11.0.30.7.1";

/// Java 17 LTS version
pub const JAVA_17_VERSION: &str = "17.0.18.8.1";

/// Java 21 LTS version
pub const JAVA_21_VERSION: &str = "21.0.10.7.1";

/// Java 25 LTS version (also the default)
pub const JAVA_25_VERSION: &str = "25.0.2.10.1";

/// Array of all supported Java LTS versions for iteration in tests
pub const ALL_LTS_VERSIONS: &[&str] = &[
    JAVA_11_VERSION,
    JAVA_17_VERSION,
    JAVA_21_VERSION,
    JAVA_25_VERSION,
];

/// The access flags for primitive classes in the Java runtime. These classes are public, final, and
/// abstract, meaning they cannot be instantiated and do not have any methods or fields.
const PRIMITIVE_CLASS_ACCESS_FLAGS: ClassAccessFlags = ClassAccessFlags::from_bits_retain(
    ClassAccessFlags::PUBLIC.bits()
        | ClassAccessFlags::FINAL.bits()
        | ClassAccessFlags::ABSTRACT.bits(),
);

/// Get a class loader for the default Java runtime version. If the version is not installed, the
/// archive will be downloaded and extracted.
///
/// # Errors
///
/// An error will be returned if the class loader cannot be created.
pub async fn default_class_loader() -> Result<(PathBuf, String, Arc<ClassLoader>)> {
    version_class_loader(DEFAULT_JAVA_VERSION).await
}

/// Get a class loader for the given Java home.
///
/// # Errors
///
/// An error will be returned if the class loader cannot be created.
pub async fn home_class_loader(java_home: &Path) -> Result<(PathBuf, String, Arc<ClassLoader>)> {
    let version_file = java_home.join("version.txt");
    // Corretto version 8 does not have a release file, but includes a version.txt file. Since most
    // versions of Corretto include a version.txt file, and it should be faster to process, we can
    // use this file to determine the version.  The version.txt file also includes the full version
    // number (e.g. 21.0.5.11.1) instead of a partial version in the release file JAVA_VERSION
    // property (e.g. 21.0.5).
    let java_version = if version_file.exists() {
        let version_file = java_home.join("version.txt");
        #[cfg(target_family = "wasm")]
        let java_version = std::fs::read_to_string(version_file)?;
        #[cfg(not(target_family = "wasm"))]
        let java_version = tokio::fs::read_to_string(version_file).await?;
        java_version.trim().to_string()
    } else {
        let release_file = java_home.join("release");

        #[cfg(target_family = "wasm")]
        let release = std::fs::read_to_string(release_file)?;
        #[cfg(not(target_family = "wasm"))]
        let release = tokio::fs::read_to_string(release_file).await?;

        let Some(java_version_line) = release
            .lines()
            .find(|line| line.starts_with("JAVA_VERSION"))
        else {
            return Err(IoError("JAVA_VERSION not found in release file".to_string()).into());
        };
        java_version_line
            .split('=')
            .next_back()
            .unwrap_or_default()
            .replace('"', "")
    };

    let class_path = get_class_path(&java_version, java_home)?;
    let class_loader = ClassLoader::new("bootstrap", class_path);
    register_primitives(&class_loader).await?;
    Ok((java_home.to_path_buf(), java_version, class_loader))
}

/// Get a class loader for the given Java runtime version. If the version is not installed, the
/// archive will be downloaded and extracted. A version can be a partial version, a major version,
/// or a `*` to get the latest LTS release supported by the runtime.
///
/// # Errors
///
/// An error will be returned if the class loader cannot be created.
pub async fn version_class_loader(version: &str) -> Result<(PathBuf, String, Arc<ClassLoader>)> {
    let mut version = version.to_string();
    #[cfg(target_family = "wasm")]
    let home_dir = env::current_dir().unwrap_or_default();
    #[cfg(not(target_family = "wasm"))]
    let home_dir = env::home_dir().unwrap_or_else(|| env::current_dir().unwrap_or_default());

    let base_path = home_dir.join(".ristretto");
    let mut installation_dir = base_path.join(&version);
    if !installation_dir.exists() {
        let (extracted_version, file_name, archive) = util::get_runtime_archive(&version).await?;
        installation_dir = extract_archive(&version, &file_name, &archive, &base_path).await?;
        version = extracted_version;
    }

    #[cfg(target_os = "macos")]
    let installation_dir = installation_dir.join("Contents").join("Home");

    let class_path = get_class_path(&version, &installation_dir)?;
    let class_loader = ClassLoader::new("bootstrap", class_path);
    register_primitives(&class_loader).await?;
    Ok((installation_dir, version, class_loader))
}

/// Register all primitive classes in the class loader.
async fn register_primitives(class_loader: &Arc<ClassLoader>) -> Result<()> {
    let class_loader_weak_ref = Arc::downgrade(class_loader);
    let classes = [
        create_primitive_class(&class_loader_weak_ref, "boolean")?,
        create_primitive_class(&class_loader_weak_ref, "byte")?,
        create_primitive_class(&class_loader_weak_ref, "char")?,
        create_primitive_class(&class_loader_weak_ref, "double")?,
        create_primitive_class(&class_loader_weak_ref, "float")?,
        create_primitive_class(&class_loader_weak_ref, "int")?,
        create_primitive_class(&class_loader_weak_ref, "long")?,
        create_primitive_class(&class_loader_weak_ref, "short")?,
        create_primitive_class(&class_loader_weak_ref, "void")?,
    ];
    class_loader.register_all(classes).await?;
    Ok(())
}

/// Creates a primitive class in the class loader.
fn create_primitive_class(class_loader: &Weak<ClassLoader>, primitive: &str) -> Result<Arc<Class>> {
    let mut constant_pool = ConstantPool::new();
    let this_class_index = constant_pool.add_class(primitive)?;
    let class_file = ClassFile {
        version: JAVA_1_0_2,
        constant_pool,
        access_flags: PRIMITIVE_CLASS_ACCESS_FLAGS,
        this_class: this_class_index,
        ..Default::default()
    };
    let class = Class::from(Some(class_loader.clone()), class_file)?;
    Ok(class)
}

/// Get the class path for the given version.
///
/// # Errors
///
/// An error will be returned if the class path cannot be determined.
fn get_class_path(version: &str, installation_dir: &Path) -> Result<ClassPath> {
    let class_path = if util::parse_major_version(version) <= 8 {
        let rt_jar_path = installation_dir.join("jre").join("lib").join("rt.jar");
        vec![rt_jar_path]
    } else {
        let modules_path = installation_dir.join("lib").join("modules");
        if modules_path.exists() {
            vec![modules_path]
        } else {
            let jmods_path = installation_dir.join("jmods");
            let jmod_files = std::fs::read_dir(jmods_path)?
                .filter_map(|entry| {
                    let entry = entry.ok()?;
                    let path = entry.path();
                    if path.extension()?.to_string_lossy() == "jmod" {
                        Some(path)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            let mut class_paths = jmod_files.into_iter().collect::<Vec<_>>();
            class_paths.sort_by(Ord::cmp);
            class_paths
        }
    };
    Ok(ClassPath::from(&class_path))
}

/// Extract the archive to the installation directory.
///
/// # Errors
///
/// An error will be returned if the archive cannot be extracted.
async fn extract_archive(
    version: &str,
    file_name: &str,
    archive: &Vec<u8>,
    out_dir: &PathBuf,
) -> Result<PathBuf> {
    #[cfg(target_family = "wasm")]
    std::fs::create_dir_all(out_dir)?;
    #[cfg(not(target_family = "wasm"))]
    tokio::fs::create_dir_all(out_dir).await?;

    let Some(extension) = file_name.split('.').next_back() else {
        return Err(Error::ArchiveError(
            "No extension found in file name".to_string(),
        ));
    };

    let archive = io::Cursor::new(archive);
    let extract_dir = tempfile::tempdir_in(out_dir)?.keep();
    debug!(
        "Extracting {file_name} to {}",
        extract_dir.to_string_lossy()
    );

    if extension == "zip" {
        let mut zip = zip::ZipArchive::new(archive)?;
        zip.extract(&extract_dir)?;
    } else {
        let decoder = GzDecoder::new(archive);
        let mut tar = Archive::new(decoder);
        tar.unpack(extract_dir.clone())?;
    }

    #[cfg(target_family = "wasm")]
    let runtime_dir = {
        let mut entries = std::fs::read_dir(&extract_dir)?;
        let Some(runtime_dir) = entries.next() else {
            return Err(Error::ArchiveError(
                "No directory found in archive".to_string(),
            ));
        };
        runtime_dir?
    };
    #[cfg(not(target_family = "wasm"))]
    let runtime_dir = {
        let mut entries = tokio::fs::read_dir(&extract_dir).await?;
        let Some(runtime_dir) = entries.next_entry().await? else {
            return Err(Error::ArchiveError(
                "No directory found in archive".to_string(),
            ));
        };
        runtime_dir
    };

    let runtime_dir = runtime_dir.path();
    let installation_dir = out_dir.join(version);

    // Rename the runtime directory to the installation directory. Another process may have
    // already installed the runtime, so we need to check if the installation directory exists.
    // If it does, we can ignore the error.
    #[cfg(target_family = "wasm")]
    let rename_result = std::fs::rename(runtime_dir.clone(), installation_dir.clone());
    #[cfg(not(target_family = "wasm"))]
    let rename_result = tokio::fs::rename(runtime_dir.clone(), installation_dir.clone()).await;

    if let Err(error) = rename_result {
        warn!(
            "Failed to rename {} to {}",
            runtime_dir.to_string_lossy(),
            installation_dir.to_string_lossy(),
        );
        if !installation_dir.exists() {
            return Err(Error::ArchiveError(error.to_string()));
        }
    }

    #[cfg(target_family = "wasm")]
    std::fs::remove_dir_all(&extract_dir)?;
    #[cfg(not(target_family = "wasm"))]
    tokio::fs::remove_dir_all(&extract_dir).await?;

    debug!(
        "Installed {version} to: {}",
        installation_dir.to_string_lossy()
    );

    Ok(installation_dir)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Load a class using the default class loader.
    async fn load_class(class_name: &str) -> Result<Arc<Class>> {
        let (_path, _version, class_loader) = default_class_loader().await?;
        let (class, _loaded_previously) = class_loader.load_with_status(class_name).await?;
        Ok(class)
    }

    #[tokio::test]
    async fn test_class_loader_v8() -> Result<()> {
        let version = JAVA_8_VERSION;
        let (_java_home, java_version, class_loader) = version_class_loader(version).await?;
        assert_eq!(version, java_version);
        assert_eq!("bootstrap", class_loader.name());
        Ok(())
    }

    #[tokio::test]
    async fn test_class_loader_v21() -> Result<()> {
        let version = JAVA_21_VERSION;
        let (_java_home, java_version, class_loader) = version_class_loader(version).await?;
        assert_eq!(version, java_version);
        assert_eq!("bootstrap", class_loader.name());
        Ok(())
    }

    #[tokio::test]
    async fn test_load_primitive_classes() -> Result<()> {
        let class_names = [
            "boolean", "byte", "char", "double", "float", "int", "long", "short", "void",
        ];
        for class_name in class_names {
            let class = load_class(class_name).await?;
            assert_eq!(class.name(), class_name);
            assert!(class.is_primitive());
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_load_primitive_array_classes() -> Result<()> {
        let class_names = ["[Z", "[B", "[C", "[D", "[F", "[I", "[J", "[S"];
        for class_name in class_names {
            let class = load_class(class_name).await?;
            assert_eq!(class.name(), class_name);
            assert!(class.is_array());
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_load_object_class() -> Result<()> {
        let class_name = "java.lang.Object";
        let class = load_class(class_name).await?;
        assert_eq!(class.name(), "java/lang/Object");
        Ok(())
    }

    #[tokio::test]
    async fn test_load_object_array_class() -> Result<()> {
        let class_name = "[Ljava.lang.Object;";
        let class = load_class(class_name).await?;
        assert_eq!(class.name(), "[Ljava/lang/Object;");
        Ok(())
    }

    #[tokio::test]
    async fn test_load_object_multi_array_class() -> Result<()> {
        let class_name = "[[Ljava.lang.Object;";
        let class = load_class(class_name).await?;
        assert_eq!(class.name(), "[[Ljava/lang/Object;");
        Ok(())
    }

    #[tokio::test]
    async fn test_load_big_integer_class() -> Result<()> {
        let class_name = "java.math.BigInteger";
        let class = load_class(class_name).await?;
        assert_eq!(class.name(), "java/math/BigInteger");
        Ok(())
    }

    #[tokio::test]
    async fn test_load_big_integer_array_class() -> Result<()> {
        let class_name = "[Ljava.math.BigInteger;";
        let class = load_class(class_name).await?;
        assert_eq!(class.name(), "[Ljava/math/BigInteger;");
        Ok(())
    }

    #[tokio::test]
    async fn test_load_big_integer_multi_array_class() -> Result<()> {
        let class_name = "[[Ljava.math.BigInteger;";
        let class = load_class(class_name).await?;
        assert_eq!(class.name(), "[[Ljava/math/BigInteger;");
        Ok(())
    }
}
