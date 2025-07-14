use crate::runtime::util;
use crate::{ClassLoader, ClassPath, Error, Result};
use flate2::bufread::GzDecoder;
use ristretto_classfile::Error::IoError;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::{env, io};
use tar::Archive;
use tracing::{debug, instrument, warn};

pub const DEFAULT_JAVA_VERSION: &str = "21.0.7.6.1";

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
#[instrument(level = "debug")]
pub async fn home_class_loader(java_home: &PathBuf) -> Result<(PathBuf, String, Arc<ClassLoader>)> {
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
    Ok((java_home.clone(), java_version, class_loader))
}

/// Get a class loader for the given Java runtime version. If the version is not installed, the
/// archive will be downloaded and extracted. A version can be a partial version, a major version,
/// or a `*` to get the latest LTS release supported by the runtime.
///
/// # Errors
///
/// An error will be returned if the class loader cannot be created.
#[instrument(level = "debug")]
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
        installation_dir =
            extract_archive(version.as_str(), file_name.as_str(), &archive, &base_path).await?;
        version = extracted_version;
    }

    #[cfg(target_os = "macos")]
    let installation_dir = installation_dir.join("Contents").join("Home");

    let class_path = get_class_path(&version, &installation_dir)?;
    let class_loader = ClassLoader::new("bootstrap", class_path);
    Ok((installation_dir, version, class_loader))
}

/// Get the class path for the given version.
///
/// # Errors
///
/// An error will be returned if the class path cannot be determined.
fn get_class_path(version: &str, installation_dir: &Path) -> Result<ClassPath> {
    let class_path = if util::parse_major_version(version) <= 8 {
        let rt_jar_path = installation_dir.join("jre").join("lib").join("rt.jar");
        let class_path = rt_jar_path.to_string_lossy();
        class_path.to_string()
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
        let mut class_paths = jmod_files
            .iter()
            .map(|path| path.to_string_lossy().to_string())
            .collect::<Vec<_>>();
        class_paths.sort_by(Ord::cmp);
        class_paths.join(":")
    };
    Ok(ClassPath::from(class_path))
}

/// Extract the archive to the installation directory.
///
/// # Errors
///
/// An error will be returned if the archive cannot be extracted.
#[instrument(level = "debug", skip(archive))]
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

    #[tokio::test]
    async fn test_class_loader_v8() -> Result<()> {
        let version = "8.452.09.1";
        let (_java_home, java_version, class_loader) = version_class_loader(version).await?;
        assert_eq!(version, java_version);
        assert_eq!("bootstrap", class_loader.name());
        Ok(())
    }

    #[tokio::test]
    async fn test_class_loader_v21() -> Result<()> {
        let version = "21.0.7.6.1";
        let (_java_home, java_version, class_loader) = version_class_loader(version).await?;
        assert_eq!(version, java_version);
        assert_eq!("bootstrap", class_loader.name());
        Ok(())
    }
}
